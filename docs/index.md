# k8s-maestro Documentation

Welcome to the k8s-maestro documentation. k8s-maestro is a Kubernetes workflow orchestrator with minimal requirements and full power.

## What is k8s-maestro?

k8s-maestro provides a high-level, type-safe Rust API for orchestrating complex workflows on Kubernetes. Built with test-driven development principles, it offers a clean builder pattern for creating multi-step workflows with dependencies, conditional execution, and powerful networking capabilities.

### Key Features

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

## Quick Links

### Getting Started
- [Installation](getting-started/installation.md) - Install and configure k8s-maestro
- [Quick Start](getting-started/quick-start.md) - Run your first workflow in 5 minutes
- [Concepts](getting-started/concepts.md) - Understand workflows, steps, and dependencies

### Guides
- [Basic Workflows](guides/basic-workflow.md) - Create your first workflow
- [Dependencies](guides/dependencies.md) - Configure step dependencies and execution order
- [Services & Ingress](guides/services-ingress.md) - Expose services and configure ingress
- [Multi-language Steps](guides/multi-language.md) - Use Python, Rust, WASM in workflows
- [Checkpointing](guides/checkpointing.md) - Enable workflow checkpointing and recovery
- [Security](guides/security.md) - Configure multi-tenant security and RBAC

### API Reference
- [Client API](api/client.md) - MaestroClient reference
- [Workflow API](api/workflow.md) - Workflow and WorkflowBuilder reference
- [Steps API](api/steps.md) - Available step types and APIs
- [Networking API](api/networking.md) - Services, Ingress, and DNS utilities

### Examples
- [Spark Cluster](examples/spark-cluster.md) - Orchestrate Apache Spark on Kubernetes
- [ML Pipeline](examples/ml-pipeline.md) - Build machine learning pipelines
- [Data Processing](examples/data-processing.md) - ETL and data transformation workflows

### Reference
- [Configuration](reference/configuration.md) - Configuration options and environment variables
- [Troubleshooting](reference/troubleshooting.md) - Common issues and solutions

## Example Usage

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};

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

## Community & Support

- **GitHub Repository**: [https://github.com/andreclaudino/k8s-maestro](https://github.com/andreclaudino/k8s-maestro)
- **Issues**: Report bugs and request features
- **Contributing**: See the contributing guidelines in the main repository

## License

This project is dual-licensed under:
- [MIT License](https://opensource.org/licenses/MIT)
- [Apache License 2.0](https://opensource.org/licenses/Apache-2.0)
