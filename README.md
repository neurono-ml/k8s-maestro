# k8s-maestro

[![Crates.io](https://img.shields.io/crates/v/k8s-maestro.svg)](https://crates.io/crates/k8s-maestro)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Docs.rs](https://docs.rs/k8s-maestro/badge.svg)](https://docs.rs/k8s-maestro)
[![Build Status](https://github.com/andreclaudino/k8s-maestro/workflows/CI/badge.svg)](https://github.com/andreclaudino/k8s-maestro/actions)

A Kubernetes workflow orchestrator with minimal requirements and full power.

k8s-maestro provides a high-level, type-safe Rust API for orchestrating complex workflows on Kubernetes. Built with test-driven development principles, it offers a clean builder pattern for creating multi-step workflows with dependencies, conditional execution, and powerful networking capabilities.

## Features

- **Multi-step Workflows**: Define complex workflows with multiple steps and dependencies
- **Conditional Execution**: Execute steps based on conditions (success, failure, output values)
- **Multiple Step Types**: Support for Kubernetes jobs, exec steps, WASM, and custom step types
- **Services & Ingress**: Built-in support for exposing services and configuring ingress
- **Sidecar Containers**: Easily add sidecar containers to workflow steps
- **File Observer**: Monitor file changes and trigger workflow execution
- **Checkpointing**: Automatic checkpointing and recovery for long-running workflows
- **Multi-tenant Security**: Role-based access control and namespace isolation
- **Builder Pattern**: Fluent API for easy workflow and resource construction
- **TDD Approach**: Extensive test coverage with unit, integration, and E2E tests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
k8s-maestro = "0.3"
```

Enable Kubernetes support with the appropriate version feature:

```toml
k8s-maestro = { version = "0.3", features = ["k8s_v1_28"] }
```

Available features: `k8s_v1_28`, `k8s_v1_29`, `k8s_v1_30`, `k8s_v1_31`, `k8s_v1_32`

## Quick Start

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
use k8s_maestro::steps::kubernetes::JobStep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    let workflow = WorkflowBuilder::new()
        .with_name("my-workflow")
        .add_step(JobStep::new("my-job", "nginx:latest"))
        .build()?;

    let execution = client.execute_workflow(&workflow).await?;
    println!("Workflow executed: {:?}", execution);

    Ok(())
}
```

## Usage Examples

### Basic Workflow

```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::kubernetes::JobStep;

let workflow = WorkflowBuilder::new()
    .with_name("basic-workflow")
    .with_namespace("production")
    .add_step(JobStep::new("data-fetch", "python:3.11"))
    .build()?;

let client = MaestroClientBuilder::new().build()?;
client.execute_workflow(&workflow).await?;
```

### Workflow with Dependencies

```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::workflows::{ConditionBuilder, DependencyChain};
use k8s_maestro::steps::kubernetes::JobStep;

let mut chain = DependencyChain::new();
chain.add_step("extract");
chain.add_step("transform").with_dependency("extract");
chain.add_step("load").with_dependency("transform");

let workflow = WorkflowBuilder::new()
    .with_name("etl-workflow")
    .add_step(JobStep::new("extract", "python:3.11"))
    .add_step(JobStep::new("transform", "python:3.11"))
    .add_step(JobStep::new("load", "postgres:16"))
    .with_parallelism(2)
    .build()?;

let client = MaestroClientBuilder::new().build()?;
let execution = client.execute_workflow(&workflow).await?;
```

### Workflow with Services

```rust
use k8s_maestro::{WorkflowBuilder, ServiceBuilder, ServiceType, MaestroClientBuilder};
use k8s_maestro::steps::kubernetes::JobStep;
use std::collections::BTreeMap;

let mut selector = BTreeMap::new();
selector.insert("app".to_string(), "my-app".to_string());

let service = ServiceBuilder::new()
    .with_name("my-service")
    .with_port(80, 8080, "TCP")
    .with_selector(selector)
    .with_type(ServiceType::ClusterIP)
    .build()?;

let workflow = WorkflowBuilder::new()
    .with_name("service-workflow")
    .add_step(JobStep::new("web-app", "nginx:latest"))
    .build()?;

let client = MaestroClientBuilder::new().build()?;
client.create_service(&service).await?;
client.execute_workflow(&workflow).await?;
```

## API Documentation

- [Client API](https://docs.rs/k8s-maestro/latest/k8s_maestro/client/struct.MaestroClient.html)
- [Workflow API](https://docs.rs/k8s-maestro/latest/k8s_maestro/workflows/struct.Workflow.html)
- [Steps API](https://docs.rs/k8s-maestro/latest/k8s_maestro/steps/)
- [Networking API](https://docs.rs/k8s-maestro/latest/k8s_maestro/networking/)

## Examples

Check out the [examples directory](examples/) for comprehensive examples including:
- [use_workflow_builder.rs](examples/use_workflow_builder.rs) - Building workflows with the WorkflowBuilder
- [apply_and_watch_workflow.rs](examples/apply_and_watch_workflow.rs) - Applying and watching workflow execution
- [delete_workflow.rs](examples/delete_workflow.rs) - Cleaning up workflow resources
- [use_services.rs](examples/use_services.rs) - Creating and managing services
- [use_sidecar.rs](examples/use_sidecar.rs) - Adding sidecar containers
- [multi_step_workflow.rs](examples/multi_step_workflow.rs) - Multi-step workflows with dependencies
- [dependency_system.rs](examples/dependency_system.rs) - Using the dependency system

## Contributing

We welcome contributions! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for your changes (TDD approach)
4. Ensure all tests pass (`cargo test --verbose`)
5. Run clippy (`cargo clippy`)
6. Format your code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Testing

```bash
# Run unit tests (fast, no cluster needed)
cargo test --lib

# Run integration tests (requires Docker and Kind)
cargo test --test '*' -- --ignored

# Run specific test
cargo test integration_test_kubernetes -- --exact
```

## License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

## Contact

For questions and support:
- André Claudino - [@andreclaudino](https://github.com/andreclaudino)
- Pedro Braga - [@braga-rp](https://github.com/braga-rp)
- Romulo Tavares - [@tavaresrft](https://github.com/tavaresrft)

## Documentation

Additional documentation is available at:
- [GitHub Pages](https://andreclaudino.github.io/k8s-maestro/)
- [Getting Started Guide](site-docs/getting-started/)
- [API Reference](site-docs/api/)
