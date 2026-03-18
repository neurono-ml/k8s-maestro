# Basic Workflows

This guide teaches you how to build and execute basic workflows with k8s-maestro.

## Creating a Simple Workflow

The simplest workflow consists of a single step that runs a container to completion.

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
use k8s_maestro::steps::kubernetes::JobStep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    let workflow = WorkflowBuilder::new()
        .with_name("simple-workflow")
        .add_step(JobStep::new("hello-job", "nginx:latest"))
        .build()?;

    let created = client.create_workflow(workflow)?;
    println!("Workflow created: {}", created.id());

    Ok(())
}
```

## Configuring Container Options

Customize the step's container with arguments, environment variables, and resource limits.

```rust
use k8s_maestro::entities::MaestroContainer;
use std::collections::BTreeMap;

let container = MaestroContainer::new("python:3.11", "data-processor")
    .set_arguments(&vec![
        "python".to_string(),
        "-c".to_string(),
        "print('Processing data...')".to_string(),
    ])
    .set_environment_variables(vec![
        ("LOG_LEVEL".to_string(), "info".to_string()),
    ].into_iter().collect());

let mut resource_limits = BTreeMap::new();
resource_limits.insert("cpu".to_string(), "500m".to_string());
resource_limits.insert("memory".to_string(), "512Mi".to_string());

let workflow = WorkflowBuilder::new()
    .with_name("configured-workflow")
    .add_step(JobStep::new("process-job", "python:3.11")
        .with_container(container)
        .with_resource_limits(resource_limits))
    .build()?;
```

## Adding Multiple Steps

Create workflows with multiple independent steps that run in parallel.

```rust
let workflow = WorkflowBuilder::new()
    .with_name("multi-step-workflow")
    .with_parallelism(3)
    .add_step(JobStep::new("fetch-data", "curlimages/curl"))
    .add_step(JobStep::new("process-a", "python:3.11"))
    .add_step(JobStep::new("process-b", "python:3.11"))
    .build()?;
```

## Setting Workflow Metadata

Add labels and annotations for organization and tracking.

```rust
let workflow = WorkflowBuilder::new()
    .with_name("labeled-workflow")
    .with_label("environment", "production")
    .with_label("team", "data-science")
    .with_label("cost-center", "engineering")
    .with_annotation("owner", "team-lead@example.com")
    .with_annotation("jira-ticket", "OPS-1234")
    .add_step(JobStep::new("main-job", "python:3.11"))
    .build()?;
```

## Configuring Execution Mode

Control whether steps run sequentially or in parallel.

### Sequential Execution

```rust
let workflow = WorkflowBuilder::new()
    .with_name("sequential-workflow")
    .with_execution_mode(ExecutionMode::Sequential)
    .add_step(JobStep::new("step-1", "python:3.11"))
    .add_step(JobStep::new("step-2", "python:3.11"))
    .add_step(JobStep::new("step-3", "python:3.11"))
    .build()?;
```

### Parallel Execution

```rust
let workflow = WorkflowBuilder::new()
    .with_name("parallel-workflow")
    .with_execution_mode(ExecutionMode::Parallel(5))
    .add_step(JobStep::new("worker-1", "python:3.11"))
    .add_step(JobStep::new("worker-2", "python:3.11"))
    .add_step(JobStep::new("worker-3", "python:3.11"))
    .add_step(JobStep::new("worker-4", "python:3.11"))
    .add_step(JobStep::new("worker-5", "python:3.11"))
    .build()?;
```

## Working with Workflows

### Creating a Workflow

```rust
let workflow = WorkflowBuilder::new()
    .with_name("my-workflow")
    .add_step(JobStep::new("job-1", "nginx:latest"))
    .build()?;

let created = client.create_workflow(workflow)?;
```

### Retrieving a Workflow

```rust
if let Some(workflow) = client.get_workflow(&created.id())? {
    println!("Workflow: {}", workflow.name());
    println!("Status: {:?}", workflow.status());
}
```

### Listing Workflows

```rust
let workflows = client.list_workflows()?;

for workflow in workflows {
    println!("{}: {} ({})", workflow.id(), workflow.name(), workflow.namespace());
}
```

### Deleting a Workflow

```rust
client.delete_workflow(&workflow_id)?;
```

## Best Practices

1. **Use descriptive names** for workflows and steps
2. **Add labels** for organization and filtering
3. **Set appropriate resource limits** to prevent resource exhaustion
4. **Choose the right execution mode** for your use case
5. **Monitor workflow execution** with kubectl
6. **Use dry-run mode** for testing without execution

## Next Steps

- [Dependencies](dependencies.md) - Configure step execution order
- [Services & Ingress](services-ingress.md) - Expose workflow services
- [Examples](../examples/) - More workflow examples
