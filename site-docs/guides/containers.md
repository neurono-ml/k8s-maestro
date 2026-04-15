# MaestroContainer

`MaestroContainer` is a builder for Kubernetes container specifications. It provides a fluent API for configuring container images, arguments, environment variables, and resource limits. `MaestroContainer` integrates with the `ContainerLike` trait to produce standard Kubernetes `Container` objects.

## When to Use MaestroContainer

Use `MaestroContainer` when you need to:

- Define containers for Kubernetes Jobs and Pods
- Configure container arguments and environment variables
- Set CPU and memory resource limits
- Implement custom container types via the `ContainerLike` trait

## Overview

`MaestroContainer` follows the builder pattern, allowing you to construct container configurations through method chaining:

```rust
use k8s_maestro::entities::MaestroContainer;
use std::collections::BTreeMap;

let container = MaestroContainer::new("nginx:latest", "web")
    .set_arguments(&["nginx", "-g", "daemon off;"].map(String::from).to_vec())
    .set_environment_variables(BTreeMap::from([
        "PORT".to_string(), "8080".to_string(),
    ]));
```

## Quick Reference

```rust
use k8s_maestro::entities::{MaestroContainer, ContainerLike};

let container = MaestroContainer::new("nginx:latest", "main");
let k8s_container = container.as_container();
```

## Builder API Reference

| Method | Description | Returns |
|--------|-------------|---------|
| `new(image, name)` | Creates a new container with image and name | `MaestroContainer` |
| `set_arguments(args)` | Sets the container command arguments | `Self` |
| `set_environment_variables(env_vars)` | Sets environment variables as a BTreeMap | `Self` |
| `set_resource_bounds(bounds)` | Sets CPU and memory resource limits | `Self` |

## Usage Examples

### Basic Container with Image and Name

Create a simple container with just an image and name:

```rust
use k8s_maestro::entities::{MaestroContainer, ContainerLike};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let container = MaestroContainer::new("busybox", "main");
    let k8s_container = container.as_container();

    println!("Container name: {}", k8s_container.name);
    println!("Container image: {}", k8s_container.image.unwrap());

    Ok(())
}
```

Output:
```
Container name: main
Container image: busybox
```

### Container with Arguments and Command

Configure container arguments for running specific commands:

```rust
use k8s_maestro::entities::{MaestroContainer, ContainerLike};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = vec![
        "python".to_string(),
        "app.py".to_string(),
        "--mode".to_string(),
        "production".to_string(),
    ];

    let container = MaestroContainer::new("python:3.11-slim", "main")
        .set_arguments(&args);

    let k8s_container = container.as_container();
    println!("Arguments: {:?}", k8s_container.args);

    Ok(())
}
```

### Container with Environment Variables

Pass environment variables to the container:

```rust
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut env_vars = BTreeMap::new();
    env_vars.insert("DATABASE_URL".to_string(), "postgres://localhost:5432/app".to_string());
    env_vars.insert("LOG_LEVEL".to_string(), "debug".to_string());
    env_vars.insert("MAX_WORKERS".to_string(), "4".to_string());
    env_vars.insert("ENABLE_CACHE".to_string(), "true".to_string());

    let container = MaestroContainer::new("my-app:latest", "main")
        .set_environment_variables(env_vars);

    let k8s_container = container.as_container();
    println!("Environment variables: {:?}", k8s_container.env);

    Ok(())
}
```

Output:
```
Environment variables: Some([EnvVar { name: "DATABASE_URL", ... }, EnvVar { name: "ENABLE_CACHE", ... }, ...])
```

### Container with Resource Limits

Configure CPU and memory resource limits:

```rust
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use k8s_maestro::steps::ResourceLimits;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let limits = ResourceLimits::new()
        .with_cpu("2000m")
        .with_memory("4Gi")
        .with_cpu_request("1000m")
        .with_memory_request("2Gi");

    let container = MaestroContainer::new("java:17", "main")
        .set_resource_bounds(limits);

    let k8s_container = container.as_container();
    println!("Resources: {:?}", k8s_container.resources);

    Ok(())
}
```

### Container with All Options Combined

A complete example combining all builder options:

```rust
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use k8s_maestro::steps::ResourceLimits;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut env_vars = BTreeMap::new();
    env_vars.insert("APP_ENV".to_string(), "production".to_string());
    env_vars.insert("PORT".to_string(), "8080".to_string());

    let limits = ResourceLimits::new()
        .with_cpu("1000m")
        .with_memory("2Gi")
        .with_ephemeral_storage("10Gi")
        .with_cpu_request("500m")
        .with_memory_request("1Gi");

    let args = vec![
        "bash".to_string(),
        "-c".to_string(),
        "echo Starting app && ./app serve".to_string(),
    ];

    let container = MaestroContainer::new("my-app:v1.2.3", "main")
        .set_arguments(&args)
        .set_environment_variables(env_vars)
        .set_resource_bounds(limits);

    let k8s_container = container.as_container();

    println!("Container: {}", k8s_container.name);
    println!("Image: {}", k8s_container.image.unwrap());
    println!("Args: {:?}", k8s_container.args);
    println!("Env count: {}", k8s_container.env.map(|e| e.len()).unwrap_or(0));
    println!("Resources: {:?}", k8s_container.resources.is_some());

    Ok(())
}
```

## SidecarContainer

`SidecarContainer` is a specialized container type for Kubernetes sidecars. It shares the same API as `MaestroContainer` but is designed for sidecar containers that run alongside main containers in a pod.

### Creating a SidecarContainer

```rust
use k8s_maestro::entities::{SidecarContainer, ContainerLike};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut env_vars = BTreeMap::new();
    env_vars.insert("LOG_LEVEL".to_string(), "info".to_string());

    let log_sidecar = SidecarContainer::new("busybox", "logger")
        .set_arguments(&["sh".to_string(), "-c".to_string(), "tail -f /var/log/app.log".to_string()])
        .set_environment_variables(env_vars);

    let k8s_container = log_sidecar.as_container();
    println!("Sidecar name: {}", k8s_container.name);

    Ok(())
}
```

### Common Sidecar Patterns

#### Log Collector Sidecar

```rust
use k8s_maestro::entities::{SidecarContainer, ContainerLike};

let log_collector = SidecarContainer::new("busybox", "log-collector")
    .set_arguments(&[
        "sh".to_string(),
        "-c".to_string(),
        "tail -f /var/log/containers/*.log".to_string(),
    ]);
```

#### Metrics Exporter Sidecar

```rust
use k8s_maestro::entities::{SidecarContainer, ContainerLike};

let metrics_exporter = SidecarContainer::new("prom/node-exporter:latest", "metrics")
    .set_arguments(&[
        "--path.procfs=/host/proc".to_string(),
        "--path.sysfs=/host/sys".to_string(),
        "--collector.filesystem.mount-points-include=/dev".to_string(),
    ]);
```

#### Proxy Sidecar

```rust
use k8s_maestro::entities::{SidecarContainer, ContainerLike};

let proxy_sidecar = SidecarContainer::new("envoyproxy/envoy:latest", "envoy")
    .set_arguments(&["-c".to_string(), "/etc/envoy/envoy.yaml".to_string()]);
```

## ContainerLike Trait

The `ContainerLike` trait enables custom container implementations. Any type implementing `ContainerLike` can be used with `KubeJobStep` and other Maestro components.

### Implementing ContainerLike

```rust
use k8s_maestro::entities::ContainerLike;
use k8s_openapi::api::core::v1::Container;
use std::collections::BTreeMap;

pub struct CustomContainer {
    image: String,
    name: String,
    args: Option<Vec<String>>,
    env: Option<BTreeMap<String, String>>,
}

impl CustomContainer {
    pub fn new(image: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            name: name.into(),
            args: None,
            env: None,
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = Some(args);
        self
    }

    pub fn with_env(mut self, env: BTreeMap<String, String>) -> Self {
        self.env = Some(env);
        self
    }
}

impl ContainerLike for CustomContainer {
    fn as_container(&self) -> Container {
        let env = self.env.as_ref().map(|vars| {
            vars.iter()
                .map(|(k, v)| k8s_openapi::api::core::v1::EnvVar {
                    name: k.clone(),
                    value: Some(v.clone()),
                    ..Default::default()
                })
                .collect()
        });

        Container {
            name: self.name.clone(),
            image: Some(self.image.clone()),
            args: self.args.clone(),
            env,
            ..Default::default()
        }
    }
}
```

### Using Custom Container with KubeJobStep

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::ContainerLike;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use std::collections::BTreeMap;

// Define custom container (from previous example)
pub struct CustomContainer {
    image: String,
    name: String,
    args: Option<Vec<String>>,
    env: Option<BTreeMap<String, String>>,
}

// ... impl block ...

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let custom = CustomContainer::new("my-custom-image:latest", "custom")
        .with_args(vec!["--config".to_string(), "/etc/config.yaml".to_string()])
        .with_env(BTreeMap::from([
            "MODE".to_string(), "production".to_string(),
        ]));

    let job = KubeJobStepBuilder::new()
        .with_name("custom-container-job")
        .with_namespace("default")
        .add_container(Box::new(custom))
        .with_client(client)
        .build()?;

    println!("Created job with custom container: {}", job.resource_name());

    Ok(())
}
```

## Combined Examples

### Using MaestroContainer with KubeJobStep

Create a complete Kubernetes Job with a MaestroContainer:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use k8s_maestro::steps::ResourceLimits;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let mut env_vars = BTreeMap::new();
    env_vars.insert("DATABASE_URL".to_string(), "postgres://db:5432/app".to_string());
    env_vars.insert("SECRET_TOKEN".to_string(), "abc123".to_string());

    let limits = ResourceLimits::new()
        .with_cpu("2000m")
        .with_memory("4Gi")
        .with_cpu_request("1000m")
        .with_memory_request("2Gi");

    let container = MaestroContainer::new("my-app:v2.0.0", "main")
        .set_arguments(&["python".to_string(), "main.py".to_string()])
        .set_environment_variables(env_vars)
        .set_resource_bounds(limits);

    let job = KubeJobStepBuilder::new()
        .with_name("complete-app-job")
        .with_namespace("production")
        .add_container(Box::new(container))
        .with_client(client)
        .build()?;

    println!("Created job: {}", job.resource_name());
    Ok(())
}
```

### Multiple Containers with Sidecars

Combine main container with multiple sidecars:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, SidecarContainer, ContainerLike};
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

#[tokio::test]
async fn test_multiple_containers() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let main = MaestroContainer::new("my-app:latest", "main")
        .set_arguments(&["python".to_string(), "app.py".to_string()]);

    let logger = SidecarContainer::new("busybox", "logger")
        .set_arguments(&["sh".to_string(), "-c".to_string(), "tail -f /var/log/app.log".to_string()]);

    let metrics = SidecarContainer::new("prom/statsd-exporter:latest", "metrics")
        .set_arguments(&["--mapper.mapping-type=histogram".to_string()]);

    let job = KubeJobStepBuilder::new()
        .with_name("multi-container-job")
        .with_namespace("default")
        .add_container(Box::new(main))
        .add_sidecar(Box::new(logger))
        .add_sidecar(Box::new(metrics))
        .with_client(client)
        .build()?;

    Ok(())
}
```

### Complete Pipeline with Containers

Create a workflow with multiple job steps, each using different containers:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use k8s_maestro::workflow::WorkFlowBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    // Step 1: Data processing
    let process_container = MaestroContainer::new("data-processor:v1.0.0", "processor")
        .set_arguments(&["process".to_string(), "--input".to_string(), "/data/input".to_string()])
        .set_environment_variables(std::collections::BTreeMap::from([
            "BATCH_SIZE".to_string(), "1000".to_string(),
        ]));

    // Step 2: Model training
    let train_container = MaestroContainer::new("ml-trainer:v2.0.0", "trainer")
        .set_arguments(&["train".to_string(), "--model".to_string(), "ResNet50".to_string()])
        .set_environment_variables(std::collections::BTreeMap::from([
            "EPOCHS".to_string(), "100".to_string(),
            "LEARNING_RATE".to_string(), "0.001".to_string(),
        ]));

    // Step 3: Model evaluation
    let eval_container = MaestroContainer::new("ml-evaluator:v1.0.0", "evaluator")
        .set_arguments(&["evaluate".to_string(), "--threshold".to_string(), "0.95".to_string()]);

    let process_job = KubeJobStepBuilder::new()
        .with_name("data-process")
        .with_namespace("ml-pipeline")
        .add_container(Box::new(process_container))
        .with_client(client.clone())
        .build()?;

    let train_job = KubeJobStepBuilder::new()
        .with_name("model-train")
        .with_namespace("ml-pipeline")
        .add_container(Box::new(train_container))
        .with_client(client.clone())
        .build()?;

    let eval_job = KubeJobStepBuilder::new()
        .with_name("model-evaluate")
        .with_namespace("ml-pipeline")
        .add_container(Box::new(eval_container))
        .with_client(client.clone())
        .build()?;

    let workflow = WorkFlowBuilder::new()
        .with_name("ml-pipeline")
        .add_step(process_job)
        .add_step(train_job)
        .add_step(eval_job)
        .build();

    println!("Created workflow with {} steps", workflow.steps().len());

    Ok(())
}
```

## Error Handling

### Common Error Scenarios

**Duplicate Container Names**

```rust
// Error: Containers in the same pod must have unique names
let container1 = MaestroContainer::new("image1", "main");
let container2 = MaestroContainer::new("image2", "main"); // Duplicate name!
```

**Invalid Image Name**

```rust
// An empty image name will cause issues when the container is created
let container = MaestroContainer::new("", "main");
let k8s_container = container.as_container(); // Image will be empty
```

**Missing Required Fields**

```rust
// All containers require at minimum a name and image
let container = MaestroContainer::new("nginx", "web");
let k8s_container = container.as_container();
assert!(k8s_container.name == "web");
assert!(k8s_container.image.is_some());
```

## Related Resources

- [KubeJobStep](./k8s-job-step.md) - Using containers with Kubernetes Jobs
- [Basic Workflows](./basic-workflow.md) - Workflow patterns
- [Configuration Reference](../reference/configuration.md) - Configuration options
- [Troubleshooting](../reference/troubleshooting.md) - Common issues and solutions

## API Links

- **Source Code**: [`src/entities/containers.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/entities/containers.rs)
- **ResourceLimits**: [`src/steps/traits.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/steps/traits.rs)
- **docs.rs**: [MaestroContainer documentation](https://docs.rs/k8s-maestro/latest/k8s_maestro/entities/struct.MaestroContainer.html)
- **docs.rs**: [ContainerLike documentation](https://docs.rs/k8s-maestro/latest/k8s_maestro/entities/trait.ContainerLike.html)