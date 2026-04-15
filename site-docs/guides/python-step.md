# PythonStep

`PythonStep` is a workflow step that executes Python code as a Kubernetes Pod. It provides a streamlined way to run Python scripts in a containerized environment with support for pip requirements, package sources, resource limits, and more.

## Overview

`PythonStep` enables running Python scripts on Kubernetes with:

- **Inline Python code** - Pass Python code directly to the step
- **pip requirements** - Specify Python dependencies to install
- **Package sources** - Load code from Git, remote URLs, or local paths
- **Resource limits** - Control CPU and memory allocation
- **Environment variables** - Pass configuration to your Python script
- **Volume mounts** - Access persistent storage
- **Timeout handling** - Configure execution timeout
- **Log streaming** - Retrieve pod logs for debugging

### When to Use PythonStep

| Use Case | Recommendation |
|----------|-----------------|
| Simple Python scripts | Use `with_code()` for inline scripts |
| Python with dependencies | Use `with_requirements()` for pip packages |
| External Python packages | Use `with_package()` with `PackageSource` |
| Data processing pipelines | Combine with resource limits and timeout |
| ML model inference | Use volume mounts for models and data |

## PythonStepBuilder

The builder pattern provides a fluent API for configuring `PythonStep`:

```rust
use k8s_maestro::steps::exec::PythonStepBuilder;
```

### Builder Methods

| Method | Description | Required |
|--------|-------------|----------|
| `new()` | Creates a new builder instance | Yes |
| `with_name(name)` | Sets the step and pod name | Yes |
| `with_namespace(namespace)` | Sets the Kubernetes namespace (default: "default") | No |
| `with_code(code)` | Sets the Python code to execute | No* |
| `with_requirements(reqs)` | Sets pip requirements (e.g., &["pandas", "numpy"]) | No |
| `with_package(source)` | Sets a package source for external code | No |
| `with_entry_point(file)` | Sets the entry point file name | No |
| `with_resource_limits(limits)` | Sets CPU/memory resource limits | No |
| `with_volume_mount(mount_path, volume_name)` | Adds a volume mount | No |
| `with_env(key, value)` | Adds an environment variable | No |
| `with_timeout(duration)` | Sets execution timeout (default: 300s) | No |
| `with_client(client)` | Sets the Kubernetes client | Yes |
| `with_dry_run(dry_run)` | Enables dry-run mode (default: false) | No |
| `build()` | Builds the PythonStep | Yes |

*Note: Either `with_code()` or `with_package()` is required.

## PackageLoader

`PackageLoader` handles loading Python packages from various sources with built-in caching.

### PackageSource

The `PackageSource` enum defines where to load Python packages from:

```rust
use k8s_maestro::steps::exec::PackageSource;
```

#### Git Source

Load code from a Git repository:

```rust
PackageSource::Git {
    url: "https://github.com/user/repo.git".to_string(),
    branch: Some("main".to_string()),  // Optional branch
    path: Some("packages/mylib".to_string()),  // Optional subdirectory
}
```

#### RemotePath

Download code from a remote URL:

```rust
PackageSource::RemotePath {
    url: "https://example.com/package.tar.gz".to_string(),
}
```

#### LocalPath

Use code from the local filesystem:

```rust
PackageSource::LocalPath {
    path: PathBuf::from("/local/packages/mylib"),
}
```

#### Registry

Note: Registry support is planned but not yet implemented.

```rust
PackageSource::Registry {
    registry: "https://pypi.org".to_string(),
    package_name: "pandas".to_string(),
    version: "2.0.0".to_string(),
}
// Currently returns: "Registry support not yet implemented"
```

### PackageCache

`PackageCache` provides caching for downloaded packages:

```rust
use k8s_maestro::steps::exec::PackageCache;

// Create a new cache
let cache = PackageCache::new()?;

// Get the cache path for a source
let path = cache.get_cache_path(&source);
```

Cache key is generated using SHA256 hash of the source details, ensuring unique cache entries for different configurations.

## Usage Examples

### Basic Python Execution

Execute a simple Python script:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::exec::PythonStepBuilder;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let step = PythonStepBuilder::new()
        .with_name("hello-world")
        .with_namespace("default")
        .with_code("print('Hello, World!')")
        .with_timeout(Duration::from_secs(60))
        .with_client(client)
        .build()?;

    // Execute the step
    let result = step.execute()?;
    println!("Step completed: {:?}", result);

    // Wait for completion
    let wait_result = step.wait().await?;
    println!("Wait result: {:?}", wait_result);

    // Stream logs
    let mut logs = step.stream_logs(Default::default());
    while let Some(log_result) = tokio::stream::StreamExt::next(&mut logs).await {
        if let Ok(log) = log_result {
            print!("{}", log);
        }
    }

    // Cleanup
    step.delete_workflow(false).await?;

    Ok(())
}
```

### With pip Requirements

Install and use Python packages like pandas and numpy:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::exec::PythonStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let code = r#"
import pandas as pd
import numpy as np

# Create a sample DataFrame
data = {'A': [1, 2, 3], 'B': [4, 5, 6]}
df = pd.DataFrame(data)

# Perform simple operations
print("DataFrame:")
print(df)
print("\nSum of column A:", df['A'].sum())
print("Mean of column B:", df['B'].mean())

# Use numpy
arr = np.array([1, 2, 3, 4, 5])
print("\nNumPy array:", arr)
print("NumPy mean:", np.mean(arr))
"#;

    let step = PythonStepBuilder::new()
        .with_name("data-processing")
        .with_namespace("default")
        .with_code(code)
        .with_requirements(&["pandas>=2.0.0", "numpy>=1.24.0"])
        .with_client(client)
        .build()?;

    let result = step.execute()?;
    println!("Result: {:?}", result);

    step.delete_workflow(false).await?;

    Ok(())
}
```

### With Package Source (Git Repository)

Load Python code from a Git repository:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::exec::{PackageSource, PythonStepBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let package_source = PackageSource::Git {
        url: "https://github.com/myorg/my-python-package.git".to_string(),
        branch: Some("main".to_string()),
        path: Some("src/package".to_string()),
    };

    let step = PythonStepBuilder::new()
        .with_name("git-package-step")
        .with_namespace("default")
        .with_package(package_source)
        .with_entry_point("main.py")  // Runs /workspace/main.py
        .with_env("PACKAGE_PATH", "/workspace")
        .with_client(client)
        .build()?;

    let result = step.execute()?;
    println!("Result: {:?}", result);

    step.delete_workflow(false).await?;

    Ok(())
}
```

### With Resource Limits and Timeout

Configure resource constraints for production workloads:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::exec::PythonStepBuilder;
use k8s_maestro::steps::ResourceLimits;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    // Define resource limits
    let limits = ResourceLimits::new()
        .with_cpu("2000m")        // 2 CPU cores
        .with_memory("4Gi")      // 4 GiB memory
        .with_cpu_request("1000m") // Request 1 CPU
        .with_memory_request("2Gi"); // Request 2 GiB

    let code = r#"
import time
import psutil
import os

print(f"Process ID: {os.getpid()}")
print(f"CPU Count: {psutil.cpu_count()}")
print(f"Memory Info: {psutil.virtual_memory()}")

# Simulate work
for i in range(5):
    print(f"Processing {i+1}/5...")
    time.sleep(1)

print("Done!")
"#;

    let step = PythonStepBuilder::new()
        .with_name("resource-limited-step")
        .with_namespace("production")
        .with_code(code)
        .with_requirements(&["psutil"])
        .with_resource_limits(limits)
        .with_timeout(Duration::from_secs(120)) // 2 minute timeout
        .with_env("LOG_LEVEL", "INFO")
        .with_client(client)
        .build()?;

    // Execute with timeout handling
    let result = step.execute()?;
    println!("Execution result: {:?}", result);

    // Wait for completion with timeout
    match tokio::time::timeout(Duration::from_secs(120), step.wait()).await {
        Ok(wait_result) => println!("Wait result: {:?}", wait_result),
        Err(_) => {
            println!("Timeout reached, canceling step");
            step.cancel()?;
        }
    }

    // Cleanup
    step.delete_workflow(false).await?;

    Ok(())
}
```

### Advanced: Volume Mounts and Environment Variables

Access persistent volumes and configure environment:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::exec::PythonStepBuilder;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let code = r#"
import os
import json

# Read environment variables
print("Environment Variables:")
for key, value in os.environ.items():
    if key.startswith('APP_'):
        print(f"  {key}={value}")

# Read configuration from mounted volume
config_path = os.environ.get('CONFIG_PATH', '/config/settings.json')
try:
    with open(config_path, 'r') as f:
        config = json.load(f)
    print(f"\nLoaded config: {config}")
except FileNotFoundError:
    print(f"Config file not found: {config_path}")

print("\nProcessing complete!")
"#;

    let step = PythonStepBuilder::new()
        .with_name("advanced-step")
        .with_namespace("default")
        .with_code(code)
        .with_env("APP_MODE", "production")
        .with_env("APP_DEBUG", "false")
        .with_env("CONFIG_PATH", "/data/config/settings.json")
        .with_volume_mount("/data", "my-pvc")  // Mount PVC at /data
        .with_client(client)
        .build()?;

    let result = step.execute()?;
    println!("Result: {:?}", result);

    step.delete_workflow(false).await?;

    Ok(())
}
```

### Dry-Run Mode

Validate step configuration without creating actual resources:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::exec::PythonStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let step = PythonStepBuilder::new()
        .with_name("dry-run-step")
        .with_namespace("default")
        .with_code("print('This is a dry run')")
        .with_requirements(&["requests"])
        .with_client(client)
        .with_dry_run(true)  // Enable dry-run mode
        .build()?;

    // Execute will log what would happen without creating resources
    let result = step.execute()?;
    println!("Dry-run result: {:?}", result);

    // Delete will also log without actually deleting
    step.delete_workflow(false).await?;

    println!("No actual resources were created or modified.");

    Ok(())
}
```

## Trait Implementations

PythonStep implements the following traits:

| Trait | Description | Methods Available |
|-------|-------------|-------------------|
| `WorkFlowStep` | Base trait for workflow steps | `step_id()`, `as_any()` |
| `ExecutableWorkFlowStep` | Enables execution | `execute()`, `cancel()` |
| `WaitableWorkFlowStep` | Enables waiting for completion | `wait()` |
| `DeletableWorkFlowStep` | Enables resource cleanup | `delete_workflow()`, `delete_associated_pods()` |
| `LoggableWorkFlowStep` | Enables log streaming | `stream_logs()` |

### Trait Method Examples

```rust
// Get step identifier
let id = step.step_id();

// Execute the step
let result = step.execute()?;

// Wait for completion
let wait_result = step.wait().await?;

// Stream logs
let mut logs = step.stream_logs(Default::default());

// Delete the step resources
step.delete_workflow(false).await?;

// Cancel execution
step.cancel()?;
```

## Error Handling

### Common Error Scenarios

#### Missing Step Name

```rust
let result = PythonStepBuilder::new()
    .with_code("print('hello')")
    .with_client(client)
    .build();

// Error: "step_id is required"
```

**Solution:** Always provide a name:

```rust
let step = PythonStepBuilder::new()
    .with_name("my-step")  // Required
    .with_code("print('hello')")
    .with_client(client)
    .build()?;
```

#### Missing Client

```rust
let result = PythonStepBuilder::new()
    .with_name("my-step")
    .with_code("print('hello')")
    .build();

// Error: "client is required"
```

**Solution:** Always provide a Kubernetes client:

```rust
let client = MaestroK8sClient::new().await?;
let step = PythonStepBuilder::new()
    .with_name("my-step")
    .with_code("print('hello')")
    .with_client(client)  // Required
    .build()?;
```

#### Neither Code Nor Package Provided

```rust
let result = PythonStepBuilder::new()
    .with_name("my-step")
    .with_client(client)
    .build();

// Result: OK, but execution will fail with no code to run
```

**Solution:** Provide either code or a package source:

```rust
// Option 1: Inline code
let step = PythonStepBuilder::new()
    .with_name("my-step")
    .with_code("print('hello')")
    .with_client(client)
    .build()?;

// Option 2: Package source
let step = PythonStepBuilder::new()
    .with_name("my-step")
    .with_package(PackageSource::Git { url: "...", branch: None, path: None })
    .with_client(client)
    .build()?;
```

#### Pod Creation Failures

```rust
let result = step.execute();

// Error: Kubernetes API error (pod creation failed)
```

**Common causes:**
- Invalid namespace
- Insufficient cluster resources
- Image pull failures
- Resource quota exceeded

**Debugging:**

```rust
// Enable logging
env_logger::init();

// Check pod status
let pods: Api<Pod> = Api::namespaced(client, &namespace);
let pod = pods.get(&name).await?;
println!("Pod status: {:?}", pod.status);

// Check events
let events = pods.list(&Default::default()).await?;
for event in events {
    println!("Event: {:?}", event);
}
```

#### Timeout Errors

```rust
let result = step.wait().await;

// Error: "timeout elapsed"
```

**Solution:** Increase timeout or handle gracefully:

```rust
use std::time::Duration;

let step = PythonStepBuilder::new()
    .with_name("my-step")
    .with_code("long_running_task()")
    .with_timeout(Duration::from_secs(600)) // 10 minutes
    .with_client(client)
    .build()?;

// Or handle timeout explicitly
match tokio::time::timeout(Duration::from_secs(300), step.wait()).await {
    Ok(result) => println!("Completed: {:?}", result),
    Err(_) => {
        println!("Step timed out!");
        step.cancel()?;
    }
}
```

#### Package Loading Errors

```rust
let package_source = PackageSource::Git {
    url: "https://github.com/invalid-repo.git".to_string(),
    branch: None,
    path: None,
};

let result = package_loader.load(&package_source);

// Error: "Failed to clone repository: ..."
// Error: "Failed to fetch: ..."
// Error: "Path does not exist: ..."
```

**Debugging:**

```rust
// For Git issues, verify the repository URL
// For RemotePath, check network connectivity
// For LocalPath, verify the path exists
let path = std::path::PathBuf::from("/local/path");
if !path.exists() {
    println!("Path does not exist!");
}
```

## Related Resources

- [KubeJobStep](./k8s-job-step.md) - For batch jobs with more complex orchestration
- [KubePodStep](./k8s-pod-step.md) - For running multiple containers
- [Basic Workflows](./basic-workflow.md) - Learn about workflow patterns
- [Resource Limits](./resource-limits.md) - Configure resource constraints
- [Configuration Reference](../reference/configuration.md) - Configuration options

## API Links

- **Source Code**: [`src/steps/exec/python.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/steps/exec/python.rs)
- **Package Loader**: [`src/steps/exec/package_loader.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/steps/exec/package_loader.rs)
- **docs.rs**: [PythonStep documentation](https://docs.rs/k8s-maestro/latest/k8s_maestro/steps/exec/struct.PythonStep.html)
- **docs.rs**: [PackageSource documentation](https://docs.rs/k8s-maestro/latest/k8s_maestro/steps/exec/enum.PackageSource.html)