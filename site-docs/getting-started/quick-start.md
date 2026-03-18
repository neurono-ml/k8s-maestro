# Quick Start

This guide will help you run your first workflow in just 5 minutes.

## Prerequisites

Ensure you have completed the [Installation](installation.md) guide and have:
- k8s-maestro installed
- A running Kubernetes cluster
- kubectl configured

## Your First Workflow

Create a new Rust project:

```bash
cargo new my-first-workflow
cd my-first-workflow
```

Add k8s-maestro to `Cargo.toml`:

```toml
[package]
name = "my-first-workflow"
version = "0.1.0"
edition = "2021"

[dependencies]
k8s-maestro = { version = "0.3", features = ["k8s_v1_30"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

Update `src/main.rs`:

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
use k8s_maestro::steps::kubernetes::JobStep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Creating k8s-maestro client...");

    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    println!("Building workflow...");

    let workflow = WorkflowBuilder::new()
        .with_name("hello-maestro")
        .add_step(JobStep::new("hello-job", "nginx:latest"))
        .build()?;

    println!("Executing workflow...");

    let created = client.create_workflow(workflow)?;
    println!("Workflow created with ID: {}", created.id());
    println!("Workflow name: {}", created.name());
    println!("Namespace: {}", created.namespace());

    Ok(())
}
```

Run the workflow:

```bash
cargo run
```

## Verifying Execution

Check the workflow status with kubectl:

```bash
# List pods in the workflow
kubectl get pods

# View pod logs
kubectl logs -l app=hello-maestro

# Check workflow status
kubectl describe jobs
```

## Example Output

You should see output similar to:

```
Creating k8s-maestro client...
Building workflow...
Executing workflow...
Workflow created with ID: 550e8400-e29b-41d4-a716-446655440000
Workflow name: hello-maestro
Namespace: default
```

## Next Steps

Congratulations! You've successfully run your first workflow. Continue to:

- [Basic Workflows](../guides/basic-workflow.md) - Learn workflow building patterns
- [Dependencies](../guides/dependencies.md) - Configure step execution order
- [Examples](../examples/) - Explore more workflow examples

## Troubleshooting

### "Namespace not found" Error

Create the namespace if it doesn't exist:

```bash
kubectl create namespace default
```

### "Connection refused" Error

Verify your Kubernetes cluster is running:

```bash
kubectl cluster-info
```

### Workflow stuck in "Pending" state

Check the pod status for more details:

```bash
kubectl describe pod <pod-name>
```

Common issues:
- Insufficient resources: Increase cluster capacity
- Image pull errors: Check image availability
- Node affinity: Ensure nodes match the workflow requirements
