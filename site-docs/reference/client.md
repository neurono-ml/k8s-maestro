# Client API Reference

The Client API provides the primary interface for interacting with Kubernetes clusters to create, manage, and orchestrate workflows.

## Overview

The client API is the main entry point for all Kubernetes operations in k8s-maestro. It provides:

- **MaestroClientBuilder**: A fluent builder API for configuring client instances
- **MaestroClient**: The main client for workflow management operations
- **MaestroK8sClient**: Legacy client for direct Kubernetes API access

## MaestroClientBuilder

The `MaestroClientBuilder` provides a fluent API for constructing configured `MaestroClient` instances.

### Creating a Builder

```rust
use k8s_maestro::MaestroClientBuilder;

let builder = MaestroClientBuilder::new();
```

### Builder Methods

#### `new()`

Creates a new builder with default values.

```rust
let builder = MaestroClientBuilder::new();
```

**Defaults:**
- `namespace`: `"default"`
- `dry_run`: `false`
- All other fields: `None`

---

#### `with_kube_config(path)`

Sets the path to the kubeconfig file.

```rust
use k8s_maestro::MaestroClientBuilder;

let client = MaestroClientBuilder::new()
    .with_kube_config("/path/to/kubeconfig")
    .build()
    .unwrap();
```

If not set, the default Kubernetes configuration locations are used:
- `$KUBECONFIG` environment variable
- `~/.kube/config`

---

#### `with_namespace(namespace)`

Sets the default namespace for operations.

```rust
let client = MaestroClientBuilder::new()
    .with_namespace("production")
    .build()
    .unwrap();
```

If not set, defaults to `"default"`.

---

#### `with_dry_run(dry_run)`

Enables or disables dry run mode.

```rust
let client = MaestroClientBuilder::new()
    .with_dry_run(true)
    .build()
    .unwrap();
```

In dry run mode, operations are validated but not executed against the cluster.

---

#### `with_default_timeout(timeout)`

Sets the default timeout for operations.

```rust
use std::time::Duration;

let client = MaestroClientBuilder::new()
    .with_default_timeout(Duration::from_secs(60))
    .build()
    .unwrap();
```

---

#### `with_log_level(level)`

Sets the log level for client operations.

```rust
let client = MaestroClientBuilder::new()
    .with_log_level("debug")
    .build()
    .unwrap();
```

Valid values: `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`

---

#### `with_default_resource_limits(limits)`

Sets default resource limits for workflows.

```rust
use k8s_maestro::steps::traits::ResourceLimits;

let limits = ResourceLimits::new()
    .with_cpu("500m")
    .with_memory("512Mi");

let client = MaestroClientBuilder::new()
    .with_default_resource_limits(limits)
    .build()
    .unwrap();
```

---

#### `build()`

Builds and returns a configured `MaestroClient`.

```rust
let client = MaestroClientBuilder::new()
    .with_namespace("production")
    .with_dry_run(false)
    .build()
    .unwrap();
```

**Returns:** `Result<MaestroClient>`

**Errors:** Returns an error if the configuration is invalid.

---

## MaestroClient

The main client for managing Kubernetes workflows.

```rust
use k8s_maestro::{MaestroClientBuilder, MaestroClient};

let client: MaestroClient = MaestroClientBuilder::new()
    .with_namespace("default")
    .build()
    .unwrap();
```

### Client Methods

#### `namespace(&self) -> &str`

Returns the default namespace for operations.

```rust
let ns = client.namespace();
println!("Using namespace: {}", ns);
```

---

#### `dry_run(&self) -> bool`

Returns whether the client is in dry run mode.

```rust
if client.dry_run() {
    println!("Dry run mode enabled");
}
```

---

#### `default_timeout(&self) -> Option<&Duration>`

Returns the default timeout for operations.

```rust
if let Some(timeout) = client.default_timeout() {
    println!("Default timeout: {:?}", timeout);
}
```

---

#### `log_level(&self) -> Option<&str>`

Returns the log level for client operations.

```rust
if let Some(level) = client.log_level() {
    println!("Log level: {}", level);
}
```

---

#### `default_resource_limits(&self) -> Option<&ResourceLimits>`

Returns the default resource limits for workflows.

```rust
if let Some(limits) = client.default_resource_limits() {
    println!("Default CPU: {}", limits.cpu.as_ref().unwrap());
    println!("Default Memory: {}", limits.memory.as_ref().unwrap());
}
```

---

#### `kube_config_path(&self) -> Option<&PathBuf>`

Returns the path to the kubeconfig file.

```rust
if let Some(path) = client.kube_config_path() {
    println!("Using kubeconfig: {}", path.display());
}
```

---

#### `create_workflow(&self, workflow: Workflow) -> Result<CreatedWorkflow>`

Creates a new workflow.

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};

// Create the client
let client = MaestroClientBuilder::new()
    .with_namespace("production")
    .build()
    .unwrap();

// Build a workflow
let workflow = WorkflowBuilder::new()
    .with_name("my-workflow")
    .with_namespace("default")
    .add_step(step)
    .build()
    .unwrap();

// Create the workflow
let created = client.create_workflow(workflow).unwrap();
```

In dry run mode, the workflow is validated but not created in the cluster.

**Returns:** `Result<CreatedWorkflow>`

**Errors:** Returns an error if the workflow is invalid.

---

#### `get_workflow(&self, id: &str) -> Result<Option<CreatedWorkflow>>`

Retrieves a workflow by ID.

```rust
let result = client.get_workflow("workflow-123").unwrap();

if let Some(workflow) = result {
    println!("Found workflow: {}", workflow.name());
    println!("Namespace: {}", workflow.namespace());
} else {
    println!("Workflow not found");
}
```

**Returns:** `Result<Option<CreatedWorkflow>>`

Returns `Ok(None)` if the workflow is not found.

---

### Understanding the CreatedWorkflow Enum

The `CreatedWorkflow` enum represents a workflow that has been created, either in dry run mode or runtime mode.

```rust
use k8s_maestro::client::{CreatedWorkflow, DryRunWorkflow, RuntimeWorkflow};
```

#### Variants

| Variant | Description |
|---------|-------------|
| `DryRun(DryRunWorkflow)` | Workflow validated but not executed |
| `Runtime(RuntimeWorkflow)` | Workflow executed in the cluster |

#### Methods

##### `id(&self) -> &str`

Returns the workflow ID.

```rust
let workflow_id = created.id();
```

---

##### `name(&self) -> &str`

Returns the workflow name.

```rust
let name = created.name();
```

---

##### `namespace(&self) -> &str`

Returns the workflow namespace.

```rust
let ns = created.namespace();
```

---

##### `is_dry_run(&self) -> bool`

Returns whether this is a dry run workflow.

```rust
if created.is_dry_run() {
    println!("Dry run mode - workflow not actually created");
}
```

---

## MaestroK8sClient

The legacy Kubernetes client for direct API access.

```rust
use k8s_maestro::clients::MaestroK8sClient;
```

### Methods

#### `new() -> Result<Self>`

Creates a new client by inferring the cluster configuration.

```rust
use k8s_maestro::clients::MaestroK8sClient;

let client = MaestroK8sClient::new().await.unwrap();
```

**Returns:** `Result<Self>`

**Errors:**
- Failed to infer cluster configuration
- Failed to create Kubernetes client

---

#### `as_client() -> KubeClient`

Returns the underlying kube client.

```rust
let kube_client = client.as_client();
```

**Returns:** `KubeClient`

---

## Usage Examples

### Example 1: Basic Client Setup with MaestroClientBuilder

The simplest way to create a client with default settings:

```rust
use k8s_maestro::MaestroClientBuilder;

fn main() -> anyhow::Result<()> {
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    println!("Client created for namespace: {}", client.namespace());
    println!("Dry run mode: {}", client.dry_run());

    Ok(())
}
```

This creates a client configured to:
- Use the default namespace
- Execute operations against the cluster (not dry run)
- Use default kubeconfig locations

---

### Example 2: With Custom Kubeconfig

For connecting to a specific cluster using a custom kubeconfig:

```rust
use k8s_maestro::MaestroClientBuilder;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let kubeconfig_path = PathBuf::from("/path/to/custom/kubeconfig");

    let client = MaestroClientBuilder::new()
        .with_kube_config(kubeconfig_path)
        .with_namespace("production")
        .build()?;

    println!("Connected to cluster with kubeconfig: {:?}", client.kube_config_path());

    Ok(())
}
```

This is useful for:
- CI/CD environments with multiple clusters
- Development environments connecting to remote clusters
- Testing with kind clusters

---

### Example 3: Creating a Workflow and Executing It

Create and execute a complete workflow:

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
use k8s_maestro::steps::traits::{WorkFlowStep, ResourceLimitedStep};
use k8s_maestro::steps::PythonStepBuilder;

#[derive(Debug, Clone)]
struct MyStep {
    id: String,
}

impl MyStep {
    fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl WorkFlowStep for MyStep {
    fn step_id(&self) -> &str {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ResourceLimitedStep for MyStep {
    fn with_resource_limits(self, _limits: crate::steps::traits::ResourceLimits) -> Self {
        self
    }

    fn resource_limits(&self) -> Option<&crate::steps::traits::ResourceLimits> {
        None
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    let step = MyStep::new("step-1");
    let workflow = WorkflowBuilder::new()
        .with_name("my-workflow")
        .with_namespace("default")
        .add_step(step)
        .build()?;

    let created = client.create_workflow(workflow)?;

    println!("Workflow '{}' created", created.name());
    println!("ID: {}", created.id());
    println!("Namespace: {}", created.namespace());

    Ok(())
}
```

---

### Example 4: Dry-Run Mode

Use dry-run mode to validate workflows without executing them:

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};

#[derive(Debug, Clone)]
struct MockStep {
    id: String,
}

impl MockStep {
    fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl WorkFlowStep for MockStep {
    fn step_id(&self) -> &str {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ResourceLimitedStep for MockStep {
    fn with_resource_limits(self, _limits: crate::steps::traits::ResourceLimits) -> Self {
        self
    }

    fn resource_limits(&self) -> Option<&crate::steps::traits::ResourceLimits> {
        None
    }
}

fn main() -> anyhow::Result<()> {
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .with_dry_run(true)
        .build()?;

    let step = MockStep::new("validate-step");
    let workflow = WorkflowBuilder::new()
        .with_name("test-workflow")
        .with_namespace("default")
        .add_step(step)
        .build()?;

    let created = client.create_workflow(workflow)?;

    if created.is_dry_run() {
        println!("Workflow validated successfully (dry run)");
        println!("Would create: {} in namespace: {}",
            created.name(), created.namespace());
    }

    Ok(())
}
```

Dry-run mode is useful for:
- Validating workflow configurations before deployment
- Testing in CI/CD pipelines
- Checking workflow syntax without cluster access

---

### Example 5: Full Configuration

Using all builder options together:

```rust
use k8s_maestro::MaestroClientBuilder;
use k8s_maestro::steps::traits::ResourceLimits;
use std::time::Duration;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let limits = ResourceLimits::new()
        .with_cpu("1000m")
        .with_memory("1Gi");

    let client = MaestroClientBuilder::new()
        .with_kube_config(PathBuf::from("/custom/kubeconfig"))
        .with_namespace("production")
        .with_dry_run(false)
        .with_default_timeout(Duration::from_secs(120))
        .with_log_level("info")
        .with_default_resource_limits(limits)
        .build()?;

    println!("Fully configured client:");
    println!("  Namespace: {}", client.namespace());
    println!("  Dry run: {}", client.dry_run());
    println!("  Timeout: {:?}", client.default_timeout());
    println!("  Log level: {:?}", client.log_level());

    Ok(())
}
```

---

## Error Handling

### Common Error Scenarios

#### 1. Invalid Namespace

```rust
use k8s_maestro::MaestroClientBuilder;

// Empty namespace is not explicitly validated
// but may cause issues when creating resources
let result = MaestroClientBuilder::new()
    .with_namespace("")
    .build();

if result.is_err() {
    // Handle error
}
```

**Solution:** Always use valid Kubernetes namespace names (lowercase alphanumeric with hyphens).

---

#### 2. Invalid Kubeconfig Path

```rust
use k8s_maestro::MaestroClientBuilder;

let result = MaestroClientBuilder::new()
    .with_kube_config("/nonexistent/path/config")
    .build();

// The error occurs when attempting operations,
// not during client creation
```

**Solution:** Verify the kubeconfig file exists before creating the client.

---

#### 3. Cluster Connection Failure

```rust
use k8s_maestro::clients::MaestroK8sClient;

let result = MaestroK8sClient::new().await;

match result {
    Ok(client) => println!("Connected"),
    Err(e) => println!("Connection failed: {}", e),
}
```

**Solution:**
- Verify the cluster is running
- Check kubeconfig is correct
- Ensure network connectivity

---

#### 4. Invalid Workflow

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};

let client = MaestroClientBuilder::new()
    .with_namespace("default")
    .build()?;

// Workflow without steps or invalid configuration
let workflow = WorkflowBuilder::new()
    .with_name("")  // Empty name - invalid
    .build();

let result = match workflow {
    Ok(wf) => client.create_workflow(wf),
    Err(e) => Err(e),
};

if result.is_err() {
    println!("Invalid workflow: {:?}", result.err());
}
```

**Solution:** Always validate workflows before creation:
```rust
workflow.validate()?;
```

---

#### 5. Resource Limit Validation

```rust
use k8s_maestro::steps::traits::ResourceLimits;

let invalid_limits = ResourceLimits::new()
    .with_cpu("invalid-cpu")  // Invalid format
    .with_memory("512Mi");

let result = MaestroClientBuilder::new()
    .with_default_resource_limits(invalid_limits)
    .build();

// Note: The builder accepts any limits,
// but they may fail when applied to resources
```

**Solution:** Use valid resource formats:
- CPU: `"100m"`, `"1"`, `"2"`
- Memory: `"128Mi"`, `"1Gi"`, `"512Mi"`

---

#### 6. Timeout Handling

```rust
use k8s_maestro::MaestroClientBuilder;
use std::time::Duration;

let client = MaestroClientBuilder::new()
    .with_default_timeout(Duration::from_secs(30))
    .build()?;

// Long-running operations may timeout
// depending on cluster configuration
```

**Solution:** Set appropriate timeouts based on workload complexity.