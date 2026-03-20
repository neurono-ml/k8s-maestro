# k8s-maestro

[![Crates.io](https://img.shields.io/crates/v/k8s-maestro.svg)](https://crates.io/crates/k8s-maestro)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Docs.rs](https://docs.rs/k8s-maestro/badge.svg)](https://docs.rs/k8s-maestro)
[![Build Status](https://github.com/neurono-ml/k8s-maestro/workflows/CI/badge.svg)](https://github.com/neurono-ml/k8s-maestro/actions)

**A Kubernetes workflow orchestrator for Rust with minimal requirements and full power.**

k8s-maestro is a Rust library that provides a high-level, type-safe API for orchestrating complex workflows on Kubernetes. It abstracts away the complexity of managing Kubernetes resources while giving you full control when you need it.

## What k8s-maestro Does

k8s-maestro helps you:

- **Run Jobs on Kubernetes** - Execute containerized workloads as Kubernetes Jobs with automatic retries, resource limits, and cleanup
- **Build Multi-step Workflows** - Chain multiple jobs together with dependencies, parallel execution, and conditional logic
- **Expose Services** - Create Services and Ingress resources to make your workloads accessible
- **Manage Configuration** - Handle ConfigMaps, Secrets, and environment variables seamlessly
- **Add Sidecars** - Attach logging, monitoring, or utility containers to your workflows
- **Persist Data** - Use PersistentVolumeClaims for stateful workloads

## Key Features

| Feature | Description |
|---------|-------------|
| **Workflow Engine** | Define multi-step workflows with dependencies, parallelism, and conditional execution |
| **Step Types** | Kubernetes Jobs, exec steps, WASM modules, and custom step implementations |
| **Networking** | Built-in Service and Ingress builders with TLS support |
| **Sidecars** | Add logging, metrics, or custom sidecar containers to any step |
| **Checkpointing** | Automatic state persistence for long-running workflows |
| **Security** | RBAC, network policies, and pod security context support |
| **Builder Pattern** | Fluent, type-safe API for constructing resources |
| **Test Infrastructure** | Unit, integration, and E2E tests with Kind clusters |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
k8s-maestro = "1.0"
```

### Feature Flags

k8s-maestro uses feature flags to minimize dependencies and ensure compatibility with your Kubernetes cluster.

#### Kubernetes Version Features (Required)

You **must** enable one Kubernetes version feature matching your cluster version:

```toml
k8s-maestro = { version = "1.0", features = ["k8s_v1_28"] }
```

| Feature | Cluster Version | When to Use |
|---------|-----------------|-------------|
| `k8s_v1_28` | Kubernetes 1.28+ | Default choice for most clusters |
| `k8s_v1_29` | Kubernetes 1.29+ | If using 1.29-specific APIs |
| `k8s_v1_30` | Kubernetes 1.30+ | If using 1.30-specific APIs |
| `k8s_v1_31` | Kubernetes 1.31+ | If using 1.31-specific APIs |
| `k8s_v1_32` | Kubernetes 1.32+ | Latest Kubernetes features |

> **Note:** These features enable the `kube` and `k8s-openapi` crates with the correct API version. Using a lower version (e.g., `k8s_v1_28`) on a newer cluster works fine for common resources.

#### Optional Features

| Feature | Description | Dependencies Added |
|---------|-------------|-------------------|
| `exec-steps` | Execute local shell commands and git operations as workflow steps. Useful for CI/CD pipelines that need to clone repos, run scripts, or compute checksums before/after Kubernetes jobs. | `git2`, `sha2`, `tempfile` |

Example with exec-steps:

```toml
k8s-maestro = { version = "1.0", features = ["k8s_v1_28", "exec-steps"] }
```

#### Default Features

By default, k8s-maestro enables:
- `k8s_v1_28` - Kubernetes 1.28 API support
- `exec-steps` - Local command execution

To disable defaults and select only what you need:

```toml
k8s-maestro = { version = "1.0", default-features = false, features = ["k8s_v1_30"] }
```

### Migrating from v0.3.0?

See the [Migration Guide](docs/migration-guide.md) for updating to the new workflow-centric API.

## Quick Start

### Simple Job

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::clients::MaestroK8sClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;

    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .with_client(k8s_client.clone())
        .build()?;

    let workflow = WorkflowBuilder::new()
        .with_name("my-workflow")
        .add_step(KubeJobStep::new("my-job", "nginx:latest", k8s_client))
        .build()?;

    let execution = client.execute_workflow(&workflow).await?;
    println!("Workflow executed: {:?}", execution);

    Ok(())
}
```

### ETL Pipeline with Dependencies

```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::workflows::DependencyChain;
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::clients::MaestroK8sClient;

#[tokio::main]
async fn example() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_client(k8s_client.clone())
        .build()?;

    let workflow = WorkflowBuilder::new()
        .with_name("etl-pipeline")
        .add_step(KubeJobStep::new("extract", "python:3.11", k8s_client.clone()))
        .add_step(KubeJobStep::new("transform", "python:3.11", k8s_client.clone()))
        .add_step(KubeJobStep::new("load", "postgres:16", k8s_client.clone()))
        .with_parallelism(2)
        .build()?;

    client.execute_workflow(&workflow).await?;
    Ok(())
}
```

### Service with Ingress

```rust
use k8s_maestro::{WorkflowBuilder, ServiceBuilder, ServiceType, IngressBuilder, MaestroClientBuilder};
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::clients::MaestroK8sClient;
use std::collections::BTreeMap;

#[tokio::main]
async fn example() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_client(k8s_client.clone())
        .build()?;

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "my-app".to_string());

    let service = ServiceBuilder::new()
        .with_name("my-service")
        .with_port(80, 8080, "TCP")
        .with_selector(selector)
        .with_type(ServiceType::ClusterIP)
        .build()?;

    let workflow = WorkflowBuilder::new()
        .with_name("web-workflow")
        .add_step(KubeJobStep::new("web-app", "nginx:latest", k8s_client))
        .build()?;

    client.create_service(&service).await?;
    client.execute_workflow(&workflow).await?;
    Ok(())
}
```

## API Documentation

| Module | Description |
|--------|-------------|
| [Client API](https://docs.rs/k8s-maestro/latest/k8s_maestro/client/struct.MaestroClient.html) | Main client for interacting with Kubernetes |
| [Workflow API](https://docs.rs/k8s-maestro/latest/k8s_maestro/workflows/struct.Workflow.html) | Workflow definition and execution |
| [Steps API](https://docs.rs/k8s-maestro/latest/k8s_maestro/steps/) | Step types (Jobs, exec, custom) |
| [Networking API](https://docs.rs/k8s-maestro/latest/k8s_maestro/networking/) | Services, Ingress, DNS |

## Examples

The [examples directory](examples/) contains comprehensive examples:

| Example | Description |
|---------|-------------|
| [use_workflow_builder.rs](examples/use_workflow_builder.rs) | Building workflows with WorkflowBuilder |
| [apply_and_watch_workflow.rs](examples/apply_and_watch_workflow.rs) | Watching workflow execution |
| [delete_workflow.rs](examples/delete_workflow.rs) | Cleaning up workflow resources |
| [use_services.rs](examples/use_services.rs) | Creating and managing services |
| [use_sidecar.rs](examples/use_sidecar.rs) | Adding sidecar containers |
| [multi_step_workflow.rs](examples/multi_step_workflow.rs) | Multi-step workflows |
| [dependency_system.rs](examples/dependency_system.rs) | Using the dependency system |

## AI Assistant Integration

k8s-maestro includes a dedicated AI assistant skill to help you integrate the library into your projects. The **k8s-maestro-integrator** skill provides:

- **Smart Resource Selection** - Guidance on when to use Jobs, Pods, or Workflows
- **Integration Patterns** - API vs channel integration strategies
- **Feature Detection** - Automatic detection of required features based on cluster version
- **Code Generation** - Generate workflow and test code from descriptions
- **Testing Patterns** - Unit, integration, and E2E test patterns with Kind

### Installation

```bash
# Using skills.sh (recommended)
npx skills add https://github.com/neurono-ml/k8s-maestro --skill k8s-maestro-integrator
```

### Example Usage

```
You: I need to create an ETL pipeline that extracts data from an API,
     transforms it with Python, and loads it into PostgreSQL.

AI: [Provides complete Rust implementation with WorkflowBuilder,
     KubeJobStep, dependency chains, and proper error handling]
```

See the [skill documentation](skills/k8s-maestro-integrator/README.md) for details.

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for your changes (TDD approach)
4. Ensure all tests pass (`cargo test --verbose`)
5. Run clippy (`cargo clippy`)
6. Format your code (`cargo fmt`)
7. Commit and create a Pull Request

### Testing

```bash
# Unit tests (fast, no cluster needed)
cargo test --lib

# Integration tests (requires Docker)
cargo test --test '*' -- --ignored

# Specific test
cargo test integration_test_kubernetes -- --exact
```

## License

Dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

## Contact

- André Claudino - [@andreclaudino](https://github.com/andreclaudino)
- Pedro Braga - [@braga-rp](https://github.com/braga-rp)
- Romulo Tavares - [@tavaresrft](https://github.com/tavaresrft)

## Documentation

- [GitHub Pages](https://neurono-ml.github.io/k8s-maestro/)
- [Getting Started](site-docs/getting-started/)
- [API Reference](https://docs.rs/k8s-maestro)
