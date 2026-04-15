# KubePodStep

`KubePodStep` is a Kubernetes Pod workflow step that enables running containerized workloads as standalone pods. Unlike Jobs, Pods are suitable for long-running services, daemons, and one-off tasks that don't require job completion tracking.

## Overview

`KubePodStep` provides a builder pattern for creating and managing Kubernetes Pods with support for:
- Multiple containers and sidecars
- Resource limits (CPU/memory)
- Restart policies
- Service exposure
- Ingress configuration
- Log streaming
- Dry-run mode for validation

## When to Use KubeJobStep vs KubePodStep

| Feature | KubeJobStep | KubePodStep |
|---------|-------------|-------------|
| **Use Case** | Batch jobs, ETL pipelines, one-time tasks | Long-running services, daemons, debug pods |
| **Completion Tracking** | Built-in (waits for completion) | Manual (use `wait()` to poll status) |
| **Restart Behavior** | Manages pod restarts on failure | Respects restart policy (Never/OnFailure/Always) |
| **Parallelism** | Supports multiple completions/parallelism | Single pod instance |
| **TTL** | Supports TTL after finishing | No TTL support |
| **Best For** | Data processing, CI/CD tasks | Web servers, background workers, debugging |

## Quick Reference

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let pod = KubePodStepBuilder::new()
        .with_name("my-pod")
        .with_namespace("default")
        .add_container(Box::new(
            k8s_maestro::entities::MaestroContainer::new("nginx:latest", "main")
        ))
        .with_client(client)
        .build()?;

    Ok(())
}
```

## Builder API Reference

| Method | Description | Returns |
|--------|-------------|---------|
| `new()` | Creates a new builder instance | `KubePodStepBuilder` |
| `with_name(name)` | Sets the pod name | `Self` |
| `with_name_type(name_type)` | Sets the pod name type (DefinedName or GenerateName) | `Self` |
| `with_namespace(namespace)` | Sets the Kubernetes namespace | `Self` |
| `add_container(container)` | Adds a main container to the pod | `Self` |
| `add_sidecar(sidecar)` | Adds a sidecar container to the pod | `Self` |
| `with_restart_policy(policy)` | Sets the restart policy (Never, OnFailure, Always) | `Self` |
| `with_resource_limits(limits)` | Sets CPU/memory resource limits | `Self` |
| `with_service_config(config)` | Configures service exposure | `Self` |
| `with_ingress_config(config)` | Configures ingress exposure | `Self` |
| `with_client(client)` | Sets the Kubernetes client (required) | `Self` |
| `with_dry_run(dry_run)` | Enables dry-run mode | `Self` |
| `build()` | Builds the KubePodStep | `Result<KubePodStep>` |

## Usage Examples

### Basic Pod Creation

Create a simple Kubernetes Pod with a single container:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let pod = KubePodStepBuilder::new()
        .with_name("hello-world-pod")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("busybox", "main")))
        .with_client(client)
        .build()?;

    println!("Created pod: {}", pod.resource_name());
    Ok(())
}
```

### With Resource Limits

Configure CPU and memory limits for the pod container:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;
use k8s_maestro::steps::ResourceLimits;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let resource_limits = ResourceLimits::new()
        .with_cpu("2000m")
        .with_memory("4Gi")
        .with_cpu_request("1000m")
        .with_memory_request("2Gi");

    let pod = KubePodStepBuilder::new()
        .with_name("resource-limited-pod")
        .with_namespace("production")
        .add_container(Box::new(MaestroContainer::new("nginx:latest", "main")))
        .with_resource_limits(resource_limits)
        .with_client(client)
        .build()?;

    Ok(())
}
```

### With Restart Policy

Configure restart policy for different pod behaviors:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::types::RestartPolicy;
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    // Use OnFailure for batch-style pods that may need retries
    let pod = KubePodStepBuilder::new()
        .with_name("batch-pod")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("my-batch-processor:latest", "main")))
        .with_restart_policy(RestartPolicy::OnFailure)
        .with_client(client)
        .build()?;

    // Use Always for long-running services
    let service_pod = KubePodStepBuilder::new()
        .with_name("web-pod")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("nginx:latest", "main")))
        .with_restart_policy(RestartPolicy::Always)
        .with_client(client)
        .build()?;

    // Use Never for one-off debug pods
    let debug_pod = KubePodStepBuilder::new()
        .with_name("debug-pod")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("busybox", "main")))
        .with_restart_policy(RestartPolicy::Never)
        .with_client(client)
        .build()?;

    Ok(())
}
```

### With Service Exposure

Expose the pod via a Kubernetes Service:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::types::ServiceConfig;
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;
use k8s_maestro::networking::ServiceType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "my-pod".to_string());

    let service_config = ServiceConfig::new("my-pod-service", 8080)
        .with_service_type(ServiceType::ClusterIP)
        .with_selector(selector);

    let pod = KubePodStepBuilder::new()
        .with_name("web-pod")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("nginx:latest", "main")))
        .with_service_config(service_config)
        .with_client(client)
        .build()?;

    // Expose the service
    let service_result = pod.expose_service("my-pod-service", 8080).await?;
    println!("{}", service_result);

    Ok(())
}
```

### With Ingress

Expose the pod via a Kubernetes Ingress:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::types::IngressConfig;
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let ingress_config = IngressConfig::new("my-pod-ingress", "example.com", "my-service", 8080)
        .with_path("/")
        .with_tls_secret("tls-secret");

    let pod = KubePodStepBuilder::new()
        .with_name("web-pod")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("nginx:latest", "main")))
        .with_ingress_config(ingress_config)
        .with_client(client)
        .build()?;

    // Expose the ingress
    let ingress_result = pod.expose_ingress("my-pod-ingress", "example.com", 8080).await?;
    println!("{}", ingress_result);

    Ok(())
}
```

### Dry-Run Mode

Use dry-run mode to validate pod configuration without creating actual resources:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let pod = KubePodStepBuilder::new()
        .with_name("dry-run-pod")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("nginx:latest", "main")))
        .with_client(client)
        .with_dry_run(true)
        .build()?;

    println!("Dry-run mode enabled - no actual resources will be created");

    // Deletion in dry-run mode won't actually delete the pod
    pod.delete_workflow(false).await?;

    Ok(())
}
```

## Trait Implementations

| Trait | Description |
|-------|-------------|
| `WorkFlowStep` | Base trait providing step identification and type erasure |
| `KubeWorkFlowStep` | Provides namespace and resource name access |
| `WaitableWorkFlowStep` | Enables waiting for pod completion (polls phase until Succeeded/Failed) |
| `DeletableWorkFlowStep` | Provides pod deletion |
| `LoggableWorkFlowStep` | Enables log streaming from pod containers |
| `ServableWorkFlowStep` | Provides service and ingress exposure |

## Error Handling

### Common Error Scenarios

**Missing Namespace:**

```rust
let result = KubePodStepBuilder::new()
    .with_name("my-pod")
    .add_container(Box::new(MaestroContainer::new("nginx", "main")))
    .with_client(client)
    .build();

// Error: "Namespace is required"
```

**Missing Container:**

```rust
let result = KubePodStepBuilder::new()
    .with_name("my-pod")
    .with_namespace("default")
    .with_client(client)
    .build();

// Error: "At least one container is required"
```

**Missing Client:**

```rust
let result = KubePodStepBuilder::new()
    .with_name("my-pod")
    .with_namespace("default")
    .add_container(Box::new(MaestroContainer::new("nginx", "main")))
    .build();

// Error: "Client is required"
```

**Empty Pod Name:**

```rust
let result = KubePodStepBuilder::new()
    .with_name("")
    .with_namespace("default")
    .add_container(Box::new(MaestroContainer::new("nginx", "main")))
    .with_client(client)
    .build();

// Error: "Pod name is required"
```

## Combined Examples

### Pod + Service + Logs (wait() + stream_logs())

Create a pod, expose it as a service, wait for completion, and stream logs:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::types::ServiceConfig;
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;
use k8s_maestro::networking::ServiceType;
use std::collections::BTreeMap;
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "my-app".to_string());

    let service_config = ServiceConfig::new("my-app-service", 8080)
        .with_service_type(ServiceType::ClusterIP)
        .with_selector(selector);

    let pod = KubePodStepBuilder::new()
        .with_name("my-app-pod")
        .with_namespace("default")
        .add_container(Box::new(
            MaestroContainer::new("my-app:latest", "main")
                .set_arguments(&vec!["sh".to_string(), "-c".to_string(), "echo 'App started' && sleep 30".to_string()])
        ))
        .with_service_config(service_config)
        .with_client(client)
        .build()?;

    // Expose service
    let service_result = pod.expose_service("my-app-service", 8080).await?;
    println!("Service: {}", service_result);

    // Wait for pod completion
    let result = pod.wait().await?;
    println!("Pod status: {:?}", result);

    // Stream logs
    let mut logs = pod.stream_logs(Default::default());
    while let Some(log_result) = logs.next().await {
        if let Ok(log) = log_result {
            println!("{}", log);
        }
    }

    // Cleanup
    pod.delete_workflow(false).await?;

    Ok(())
}
```

### Pod + Ingress + NetworkPolicy

Create a complete deployment with ingress and security:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::types::{IngressConfig, ServiceConfig};
use k8s_maestro::steps::kubernetes::KubePodStepBuilder;
use k8s_maestro::networking::ServiceType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    // Configure service
    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "secure-app".to_string());

    let service_config = ServiceConfig::new("secure-app-service", 443)
        .with_service_type(ServiceType::ClusterIP)
        .with_selector(selector)
        .with_target_port(8443);

    // Configure ingress with TLS
    let ingress_config = IngressConfig::new(
        "secure-app-ingress",
        "secure.example.com",
        "secure-app-service",
        443,
    )
    .with_path("/")
    .with_tls_secret("app-tls-secret");

    // Create pod with security context
    let mut env_vars = BTreeMap::new();
    env_vars.insert("SECRET_KEY".to_string(), "sensitive-value".to_string());

    let container = MaestroContainer::new("my-secure-app:latest", "main")
        .set_environment_variables(env_vars);

    let pod = KubePodStepBuilder::new()
        .with_name("secure-app-pod")
        .with_namespace("production")
        .add_container(Box::new(container))
        .with_service_config(service_config)
        .with_ingress_config(ingress_config)
        .with_client(client)
        .build()?;

    // Expose service
    let service_result = pod.expose_service("secure-app-service", 443).await?;
    println!("{}", service_result);

    // Expose ingress
    let ingress_result = pod.expose_ingress("secure-app-ingress", "secure.example.com", 443).await?;
    println!("{}", ingress_result);

    Ok(())
}
```

## Related Resources

- [KubeJobStep](./k8s-job-step.md) - For batch jobs and one-time tasks
- [Basic Workflows](./basic-workflow.md) - Learn about workflow patterns
- [Services & Ingress](./services-ingress.md) - Configure service exposure
- [Configuration Reference](../reference/configuration.md) - Configuration options
- [Troubleshooting](../reference/troubleshooting.md) - Common issues and solutions

## API Links

- **Source Code**: [`src/steps/kubernetes/pod.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/steps/kubernetes/pod.rs)
- **Types**: [`src/steps/kubernetes/types.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/steps/kubernetes/types.rs)
- **Tests**: [Integration tests](https://github.com/k8s-maestro/k8s-maestro/tree/main/tests/integration)
- **Examples**: [Examples directory](https://github.com/k8s-maestro/k8s-maestro/tree/main/examples)
- **docs.rs**: [KubePodStep documentation](https://docs.rs/k8s-maestro/latest/k8s_maestro/steps/kubernetes/struct.KubePodStep.html)