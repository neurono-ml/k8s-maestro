# k8s-maestro Examples

This directory contains comprehensive examples demonstrating various features of k8s-maestro.

## Table of Contents

### Core Workflow Examples

- **[use_workflow_builder.rs](use_workflow_builder.rs)** - Demonstrates building workflows using the WorkflowBuilder API, including step creation, resource configuration, and metadata handling.

- **[apply_and_watch_workflow.rs](apply_and_watch_workflow.rs)** - Shows how to apply workflows to a Kubernetes cluster and watch their execution in real-time.

- **[delete_workflow.rs](delete_workflow.rs)** - Demonstrates proper cleanup of workflow resources, including jobs and associated pods.

- **[multi_step_workflow.rs](multi_step_workflow.rs)** - Advanced example showing multi-step workflows with complex dependency chains and parallel execution.

### Step Types and Dependencies

- **[dependency_system.rs](dependency_system.rs)** - Comprehensive examples of the dependency chain system, including conditional execution, cycle detection, and custom conditions.

- **[use_volumes.rs](use_volumes.rs)** - Shows how to configure volume mounts in workflow steps using PVC and other volume types.

### Networking Examples

- **[use_services.rs](use_services.rs)** - Demonstrates creating and managing Kubernetes Services with the ServiceBuilder API, including different service types.

- **[use_sidecar.rs](use_sidecar.rs)** - Shows how to add sidecar containers to workflow steps using the SidecarPlugin system.

- **[use_service_builder.rs](use_service_builder.rs)** - Low-level service configuration examples with custom selectors, ports, and annotations.

- **[use_ingress_builder.rs](use_ingress_builder.rs)** - Demonstrates creating Ingress resources for external access to services, including TLS configuration.

### Language-Specific Step Examples (Aspirational)

- **[python_step.rs](python_step.rs)** - Example demonstrating Python step execution within workflows.

- **[rust_step.rs](rust_step.rs)** - Example demonstrating Rust step execution within workflows.

- **[wasm_step.rs](wasm_step.rs)** - Example demonstrating WASM step execution for portable, lightweight step execution.

### Utility Examples

- **[use_image_pull_secret.rs](use_image_pull_secret.rs)** - Shows how to configure image pull secrets for private container registries.

## Running the Examples

Before running examples, ensure you have:
1. A running Kubernetes cluster (or use Kind for local testing)
2. Configured kubectl to access your cluster
3. Built the project: `cargo build --release`

To run an example:

```bash
# Run a specific example
cargo run --example use_workflow_builder

# Run with logging
RUST_LOG=debug cargo run --example use_workflow_builder
```

## Example Structure

Most examples follow this pattern:

1. **Setup**: Initialize client and configure namespace
2. **Build**: Create resources using builders
3. **Apply**: Apply resources to Kubernetes
4. **Monitor**: Watch execution (optional)
5. **Cleanup**: Remove resources (optional)

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Setup client
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    // 2. Build workflow
    let workflow = WorkflowBuilder::new()
        .with_name("example-workflow")
        .add_step(JobStep::new("example-job", "nginx:latest"))
        .build()?;

    // 3. Apply and execute
    let execution = client.execute_workflow(&workflow).await?;

    // 4. Monitor (optional)
    client.watch_workflow(&execution.id).await?;

    // 5. Cleanup (optional)
    client.delete_workflow(&execution.id).await?;

    Ok(())
}
```

## Learning Path

If you're new to k8s-maestro, we recommend following this order:

1. **Start with**: `use_workflow_builder.rs` - Understand the basic API
2. **Next**: `apply_and_watch_workflow.rs` - Learn execution and monitoring
3. **Then**: `dependency_system.rs` - Master workflow orchestration
4. **Finally**: Explore networking and advanced features

## Testing Examples

To ensure examples work correctly:

```bash
# Run examples as integration tests
cargo test --example use_workflow_builder

# Run with Kind cluster (requires Docker)
cargo test --test '*' -- --ignored
```

## Best Practices Demonstrated

- **Error Handling**: All examples use `anyhow::Result` for proper error handling
- **Resource Management**: Examples show proper cleanup of Kubernetes resources
- **Logging**: Log statements help understand workflow execution
- **Type Safety**: Leverage Rust's type system for compile-time checks
- **Builder Patterns**: Consistent use of fluent builder APIs
- **Testability**: Examples are structured to be testable

## Troubleshooting

If examples fail:
1. Check Kubernetes cluster is accessible: `kubectl cluster-info`
2. Verify namespace exists or create it: `kubectl create namespace default`
3. Check logs with verbose logging: `RUST_LOG=trace cargo run --example <name>`
4. Ensure image pull secrets are configured for private images

## Contributing Examples

When adding new examples:
1. Use descriptive, kebab-case filenames
2. Include comprehensive comments
3. Follow the existing code style
4. Demonstrate a single concept or feature
5. Add entry to this README
6. Test with both unit and integration tests
