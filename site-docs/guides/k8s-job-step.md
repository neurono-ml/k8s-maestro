# KubeJobStep

`KubeJobStep` is a Kubernetes Job workflow step that enables running containerized workloads to completion. It provides a builder pattern for creating and managing Kubernetes Jobs with support for multiple containers, sidecars, resource limits, service exposure, and ingress configuration.

## When to Use KubeJobStep

Use `KubeJobStep` when you need to:

- Run a containerized task that executes to completion
- Batch process data or run ETL pipelines
- Execute one-time or scheduled computational workloads
- Run parallel jobs with configurable completions and parallelism
- Expose job workloads via services and ingress

## Quick Reference

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let job = KubeJobStepBuilder::new()
        .with_name("my-job")
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
| `new()` | Creates a new builder instance | `KubeJobStepBuilder` |
| `with_name(name)` | Sets the job name | `Self` |
| `with_name_type(name_type)` | Sets the job name type (DefinedName or GenerateName) | `Self` |
| `with_namespace(namespace)` | Sets the Kubernetes namespace | `Self` |
| `add_container(container)` | Adds a main container to the job | `Self` |
| `add_sidecar(sidecar)` | Adds a sidecar container to the job | `Self` |
| `with_backoff_limit(limit)` | Sets the job backoff limit | `Self` |
| `with_restart_policy(policy)` | Sets the restart policy (Never, OnFailure, Always) | `Self` |
| `with_ttl_seconds(ttl)` | Sets TTL seconds after job finishes | `Self` |
| `with_completions(completions)` | Sets the number of successful completions required | `Self` |
| `with_parallelism(parallelism)` | Sets the number of parallel pods | `Self` |
| `with_resource_limits(limits)` | Sets CPU/memory resource limits | `Self` |
| `with_service_config(config)` | Configures service exposure | `Self` |
| `with_ingress_config(config)` | Configures ingress exposure | `Self` |
| `with_client(client)` | Sets the Kubernetes client (required) | `Self` |
| `with_dry_run(dry_run)` | Enables dry-run mode | `Self` |
| `build()` | Builds the KubeJobStep | `Result<KubeJobStep>` |

## Usage Examples

### Basic Job Creation

Create a simple Kubernetes Job with a single container:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let job = KubeJobStepBuilder::new()
        .with_name("hello-world-job")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("busybox", "main")))
        .with_client(client)
        .build()?;

    println!("Created job: {}", job.resource_name());
    Ok(())
}
```

### With Resource Limits

Configure CPU and memory limits for the job container:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use k8s_maestro::steps::ResourceLimits;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let resource_limits = ResourceLimits::new()
        .with_cpu("1000m")
        .with_memory("2Gi")
        .with_cpu_request("500m")
        .with_memory_request("1Gi");

    let job = KubeJobStepBuilder::new()
        .with_name("resource-limited-job")
        .with_namespace("production")
        .add_container(Box::new(MaestroContainer::new("python:3.11", "main")))
        .with_resource_limits(resource_limits)
        .with_client(client)
        .build()?;

    Ok(())
}
```

### With Environment Variables

Pass environment variables to the container:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let mut env_vars = BTreeMap::new();
    env_vars.insert("DATABASE_URL".to_string(), "postgres://localhost:5432/db".to_string());
    env_vars.insert("LOG_LEVEL".to_string(), "debug".to_string());
    env_vars.insert("MAX_RETRIES".to_string(), "3".to_string());

    let container = MaestroContainer::new("my-app:latest", "main")
        .set_environment_variables(env_vars);

    let job = KubeJobStepBuilder::new()
        .with_name("env-job")
        .with_namespace("default")
        .add_container(Box::new(container))
        .with_client(client)
        .build()?;

    Ok(())
}
```

### With Multiple Containers and Sidecars

Add multiple containers including main and sidecar containers:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let main_container = MaestroContainer::new("my-app:latest", "main")
        .set_arguments(&vec!["python".to_string(), "app.py".to_string()]);

    let log_sidecar = MaestroContainer::new("busybox", "logger")
        .set_arguments(&vec!["sh".to_string(), "-c".to_string(), "tail -f /var/log/app.log".to_string()]);

    let metrics_sidecar = MaestroContainer::new("prom/prometheus:latest", "metrics")
        .set_arguments(&vec!["--config.file=/etc/prometheus/prometheus.yml".to_string()]);

    let job = KubeJobStepBuilder::new()
        .with_name("multi-container-job")
        .with_namespace("default")
        .add_container(Box::new(main_container))
        .add_sidecar(Box::new(log_sidecar))
        .add_sidecar(Box::new(metrics_sidecar))
        .with_client(client)
        .build()?;

    Ok(())
}
```

### With Service and Ingress Exposure

Expose the job via Kubernetes Service and Ingress:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::types::{IngressConfig, ServiceConfig};
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use k8s_maestro::networking::ServiceType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "my-job".to_string());

    let service_config = ServiceConfig::new("my-job-service", 8080)
        .with_service_type(ServiceType::ClusterIP)
        .with_selector(selector);

    let ingress_config = IngressConfig::new("my-job-ingress", "example.com", "my-job-service", 8080)
        .with_path("/api")
        .with_tls_secret("tls-secret");

    let job = KubeJobStepBuilder::new()
        .with_name("web-job")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("nginx:latest", "main")))
        .with_service_config(service_config)
        .with_ingress_config(ingress_config)
        .with_client(client)
        .build()?;

    Ok(())
}
```

### With Job Configuration

Configure job-specific settings like backoff limit, completions, parallelism, and TTL:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::types::JobNameType;
use k8s_maestro::steps::kubernetes::types::RestartPolicy;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let job = KubeJobStepBuilder::new()
        .with_name("batch-processing-job")
        .with_name_type(JobNameType::GenerateName("batch-"))
        .with_namespace("batch-jobs")
        .add_container(Box::new(MaestroContainer::new("my-batch-processor:latest", "main")))
        .with_backoff_limit(5)
        .with_restart_policy(RestartPolicy::OnFailure)
        .with_ttl_seconds(3600)
        .with_completions(3)
        .with_parallelism(2)
        .with_client(client)
        .build()?;

    println!("Job configured with:");
    println!("  - Completions: 3");
    println!("  - Parallelism: 2");
    println!("  - Backoff limit: 5");
    println!("  - TTL seconds: 3600");

    Ok(())
}
```

### Dry-Run Mode

Use dry-run mode to validate job configuration without creating actual resources:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let job = KubeJobStepBuilder::new()
        .with_name("dry-run-job")
        .with_namespace("default")
        .add_container(Box::new(MaestroContainer::new("nginx:latest", "main")))
        .with_client(client)
        .with_dry_run(true)
        .build()?;

    println!("Dry-run mode enabled - no actual resources will be created");

    Ok(())
}
```

## Trait Implementations

| Trait | Description |
|-------|-------------|
| `WorkFlowStep` | Base trait providing step identification and type erasure |
| `KubeWorkFlowStep` | Provides namespace and resource name access |
| `WaitableWorkFlowStep` | Enables waiting for job completion |
| `DeletableWorkFlowStep` | Provides job and associated pod deletion |
| `LoggableWorkFlowStep` | Enables log streaming from job pods |
| `ServableWorkFlowStep` | Provides service and ingress exposure |

## Error Handling

### Common Error Scenarios

**Missing Namespace**

```rust
let result = KubeJobStepBuilder::new()
    .with_name("my-job")
    .add_container(Box::new(MaestroContainer::new("nginx", "main")))
    .with_client(client)
    .build();

// Error: "Namespace is required"
```

**Missing Container**

```rust
let result = KubeJobStepBuilder::new()
    .with_name("my-job")
    .with_namespace("default")
    .with_client(client)
    .build();

// Error: "At least one container is required"
```

**Missing Client**

```rust
let result = KubeJobStepBuilder::new()
    .with_name("my-job")
    .with_namespace("default")
    .add_container(Box::new(MaestroContainer::new("nginx", "main")))
    .build();

// Error: "Client is required"
```

**Empty Job Name**

```rust
let result = KubeJobStepBuilder::new()
    .with_name("")
    .with_namespace("default")
    .add_container(Box::new(MaestroContainer::new("nginx", "main")))
    .with_client(client)
    .build();

// Error: "Job name is required"
```

## Combined Examples

### Sidecars + Log Streaming

Combine multiple sidecars with log streaming capabilities:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let main_container = MaestroContainer::new("my-app:latest", "main")
        .set_arguments(&vec!["python".to_string(), "app.py".to_string()]);

    let log_sidecar = MaestroContainer::new("busybox", "logger")
        .set_arguments(&vec!["sh".to_string(), "-c".to_string(), "tail -f /var/log/app.log".to_string()]);

    let job = KubeJobStepBuilder::new()
        .with_name("log-streaming-job")
        .with_namespace("default")
        .add_container(Box::new(main_container))
        .add_sidecar(Box::new(log_sidecar))
        .with_client(client)
        .build()?;

    let mut logs = job.stream_logs(Default::default());
    while let Some(log_result) = logs.next().await {
        if let Ok(log) = log_result {
            println!("{}", log);
        }
    }

    Ok(())
}
```

### Job + Service + Ingress + SecurityContext

Create a complete deployment with service exposure, ingress, and security context:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::types::{IngressConfig, ServiceConfig};
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use k8s_maestro::networking::ServiceType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let mut env_vars = BTreeMap::new();
    env_vars.insert("SECRET_KEY".to_string(), "sensitive-value".to_string());

    let mut security_context = BTreeMap::new();
    security_context.insert("runAsNonRoot".to_string(), "true".to_string());
    security_context.insert("runAsUser".to_string(), "1000".to_string());
    security_context.insert("allowPrivilegeEscalation".to_string(), "false".to_string());

    let container = MaestroContainer::new("my-secure-app:latest", "main")
        .set_environment_variables(env_vars)
        .set_security_context(security_context);

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "secure-app".to_string());

    let service_config = ServiceConfig::new("secure-app-service", 443)
        .with_service_type(ServiceType::ClusterIP)
        .with_selector(selector)
        .with_target_port(8443);

    let ingress_config = IngressConfig::new("secure-app-ingress", "secure.example.com", "secure-app-service", 443)
        .with_tls_secret("app-tls-secret");

    let job = KubeJobStepBuilder::new()
        .with_name("secure-app-job")
        .with_namespace("production")
        .add_container(Box::new(container))
        .with_service_config(service_config)
        .with_ingress_config(ingress_config)
        .with_client(client)
        .build()?;

    // Expose service
    let service_result = job.expose_service("secure-app-service", 443).await?;
    println!("{}", service_result);

    // Expose ingress
    let ingress_result = job.expose_ingress("secure-app-ingress", "secure.example.com", 443).await?;
    println!("{}", ingress_result);

    Ok(())
}
```

## Related Resources

- [Basic Workflows](./basic-workflow.md) - Learn about workflow patterns
- [Services & Ingress](./services-ingress.md) - Configure service exposure
- [Configuration Reference](../reference/configuration.md) - Configuration options
- [Troubleshooting](../reference/troubleshooting.md) - Common issues and solutions

## API Links

- **Source Code**: [`src/steps/kubernetes/job.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/steps/kubernetes/job.rs)
- **Types**: [`src/steps/kubernetes/types.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/steps/kubernetes/types.rs)
- **Tests**: [Integration tests](https://github.com/k8s-maestro/k8s-maestro/tree/main/tests/integration)
- **Examples**: [Examples directory](https://github.com/k8s-maestro/k8s-maestro/tree/main/examples)
- **docs.rs**: [KubeJobStep documentation](https://docs.rs/k8s-maestro/latest/k8s_maestro/steps/kubernetes/struct.KubeJobStep.html)