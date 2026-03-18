# API Migration Guide: v0.3.0 → v0.4.0

This guide helps you migrate your code from the job-centric API (v0.3.x) to the new workflow-centric API (v0.4.0).

**Last Updated:** March 18, 2026  
**Target Version:** 0.4.0

## Overview

k8s-maestro 0.4.0 introduces a major API evolution focused on **multi-step workflows** with advanced features like dependency management, conditional execution, and trait-based step composition.

### Why the Change?

The job-centric API was designed for single Kubernetes Job management. As use cases evolved, users needed:
- Multi-step workflows with dependencies
- Flexible step types beyond Kubernetes Jobs
- Conditional execution based on step results
- Better resource management at workflow and step levels
- Trait-based composition for reusable behavior

### Key Benefits of the New API

✅ **Multi-step workflows**: Execute complex pipelines with dependencies  
✅ **Trait-based composition**: Mix and match behaviors (waitable, deletable, loggable)  
✅ **Dependency management**: Build DAG-based workflows with conditions  
✅ **Builder pattern consistency**: Fluent API across all components  
✅ **Centralized configuration**: Dry run, timeouts, and resources at client level  
✅ **Checkpointing**: Built-in support for workflow recovery  

## Breaking Changes

### 1. Job → Workflow Type Rename

**Old API (v0.3.x):**
```rust
use k8s_maestro::entities::job::Job;
use k8s_maestro::entities::job::JobBuilder;

let job = JobBuilder::new()
    .with_name("my-job")
    .build()?;
```

**New API (v0.4.0):**
```rust
use k8s_maestro::workflows::{Workflow, WorkflowBuilder};

let workflow = WorkflowBuilder::new()
    .with_name("my-workflow")
    .add_step(JobStep::new("job-1", "nginx:latest"))
    .build()?;
```

**Migration Notes:**
- `Job` is now `Workflow`
- `JobBuilder` is now `WorkflowBuilder`
- Workflows must contain at least one step
- Use `JobStep` for Kubernetes Jobs or implement custom step types

### 2. MaestroK8sClient → MaestroClient Builder Pattern

**Old API (v0.3.x):**
```rust
use k8s_maestro::MaestroK8sClient;

let client = MaestroK8sClient::new().await?;
client.create_job(&job, namespace, dry_run).await?;
```

**New API (v0.4.0):**
```rust
use k8s_maestro::{MaestroClientBuilder, MaestroClient};

let client = MaestroClientBuilder::new()
    .with_namespace("default")
    .with_dry_run(false)
    .build()?;

let created = client.create_workflow(workflow)?;
```

**Migration Notes:**
- `MaestroK8sClient::new().await` → `MaestroClientBuilder::new().build()`
- Builder pattern allows pre-configuration of namespace, dry_run, timeouts
- Client is no longer async to create
- Namespace configured once at client level

### 3. dry_run Parameter Moved to Client Builder

**Old API (v0.3.x):**
```rust
client.create_job(&job, namespace, false).await?;
client.delete_job(&job_name, namespace, true).await?;
```

**New API (v0.4.0):**
```rust
let client = MaestroClientBuilder::new()
    .with_dry_run(false)
    .build()?;

client.create_workflow(workflow)?;
```

**Migration Notes:**
- `dry_run` is now a client-level configuration
- Set once during client construction
- All operations inherit the dry_run setting
- No need to pass per-call

### 4. create_job() → create_workflow()

**Old API (v0.3.x):**
```rust
let result = client.create_job(&job, namespace, dry_run).await?;
result.wait().await?;
result.delete_job(dry_run).await?;
```

**New API (v0.4.0):**
```rust
let created = client.create_workflow(workflow)?;

match created {
    CreatedWorkflow::Runtime(w) => {
        w.wait().await?;
        w.delete().await?;
    }
    CreatedWorkflow::DryRun(_) => {
        println!("Dry run - nothing executed");
    }
}
```

**Migration Notes:**
- Method renamed to `create_workflow`
- Returns `CreatedWorkflow` enum (DryRun or Runtime variant)
- Use pattern matching to handle both modes
- `create_workflow` is synchronous, but execution may be async

### 5. entities::job → workflows Module Change

**Old API (v0.3.x):**
```rust
use k8s_maestro::entities::job::{Job, JobBuilder, JobConfig};
use k8s_maestro::entities::job::JobExecutionMode;
```

**New API (v0.4.0):**
```rust
use k8s_maestro::workflows::{Workflow, WorkflowBuilder, ExecutionMode};
use k8s_maestro::workflows::checkpointing::CheckpointConfig;
use k8s_maestro::workflows::dependency::DependencyChain;
```

**Migration Notes:**
- Job types moved to `workflows` module
- Checkpointing now in `workflows::checkpointing` submodule
- Dependency management in `workflows::dependency` submodule
- Execution modes renamed (see below)

### 6. New Traits System

The new API uses traits to compose step behavior:

**WorkFlowStep (Base Trait):**
```rust
pub trait WorkFlowStep: Send + Sync + Any {
    fn step_id(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}
```

**KubeWorkFlowStep (Kubernetes-specific):**
```rust
pub trait KubeWorkFlowStep: WorkFlowStep {
    fn namespace(&self) -> &str;
    fn resource_name(&self) -> &str;
}
```

**WaitableWorkFlowStep (Wait for completion):**
```rust
pub trait WaitableWorkFlowStep: WorkFlowStep {
    fn wait(&self) -> impl Future<Output = Result<StepResult>> + Send;
}
```

**DeletableWorkFlowStep (Cleanup resources):**
```rust
pub trait DeletableWorkFlowStep: WorkFlowStep {
    fn delete_workflow(&self, dry_run: bool) -> impl Future<Output = Result<()>> + Send;
}
```

**ResourceLimitedStep (Resource constraints):**
```rust
pub trait ResourceLimitedStep: WorkFlowStep {
    fn with_resource_limits(self, limits: ResourceLimits) -> Self;
    fn resource_limits(&self) -> Option<&ResourceLimits>;
}
```

**Migration Notes:**
- Steps are now polymorphic via trait objects
- Implement the traits you need for each step type
- `Box<dyn WorkFlowStep>` is used in workflows
- Multiple traits can be combined (e.g., `KubeJobStep` implements 6+ traits)

### 7. New Features

#### Execution Mode

**Old API (v0.3.x):**
```rust
let job = JobBuilder::new()
    .with_parallelism(4)
    .build()?;
```

**New API (v0.4.0):**
```rust
use k8s_maestro::workflows::ExecutionMode;

let workflow = WorkflowBuilder::new()
    .with_parallelism(4)
    .with_execution_mode(ExecutionMode::Parallel(2))
    .build()?;
```

#### Checkpointing

**New API (v0.4.0):**
```rust
use k8s_maestro::workflows::{CheckpointConfig, CheckpointFrequency};

let checkpoint = CheckpointConfig::new()
    .with_frequency(CheckpointFrequency::OnStepCompletion)
    .with_retention(RetentionPolicy::Count(10));

let workflow = WorkflowBuilder::new()
    .with_checkpointing(checkpoint)
    .build()?;
```

#### Dependency Chains

**New API (v0.4.0):**
```rust
use k8s_maestro::workflows::{ConditionBuilder, DependencyChain};

let mut chain = DependencyChain::new();
chain.add_step("extract");
chain.add_step("transform").with_dependency("extract");
chain.add_step("load").with_dependency("transform", ConditionBuilder::all_success());

let workflow = WorkflowBuilder::new()
    .with_name("etl-workflow")
    .add_step(JobStep::new("extract", "python:3.11"))
    .add_step(JobStep::new("transform", "python:3.11"))
    .add_step(JobStep::new("load", "postgres:16"))
    .build()?;
```

## Migration Guide

### Step 1: Update Imports

Replace old imports:

```diff
- use k8s_maestro::MaestroK8sClient;
- use k8s_maestro::entities::job::{Job, JobBuilder};
+ use k8s_maestro::{MaestroClientBuilder, MaestroClient};
+ use k8s_maestro::workflows::{Workflow, WorkflowBuilder};
+ use k8s_maestro::steps::kubernetes::JobStep;
```

### Step 2: Update Client Creation

Replace client initialization:

```diff
- let client = MaestroK8sClient::new().await?;
+ let client = MaestroClientBuilder::new()
+     .with_namespace("default")
+     .with_dry_run(false)
+     .build()?;
```

### Step 3: Update Job → Workflow

Replace job creation:

```diff
- let job = JobBuilder::new()
-     .with_name("my-job")
-     .with_namespace("default")
-     .with_container(container)
-     .build()?;
+ 
+ let workflow = WorkflowBuilder::new()
+     .with_name("my-workflow")
+     .with_namespace("default")
+     .add_step(JobStep::new("job-1", "nginx:latest")
+         .with_container(container))
+     .build()?;
```

### Step 4: Update Execution

Replace job execution:

```diff
- let result = client.create_job(&job, namespace, dry_run).await?;
- result.wait().await?;
+ 
+ let created = client.create_workflow(workflow)?;
+ 
+ match created {
+     CreatedWorkflow::Runtime(runtime) => {
+         runtime.wait().await?;
+     }
+     CreatedWorkflow::DryRun(_) => {
+         println!("Dry run complete");
+     }
+ }
```

### Step 5: Update Cleanup

Replace deletion logic:

```diff
- result.delete_job(dry_run).await?;
+ match created {
+     CreatedWorkflow::Runtime(runtime) => {
+         runtime.delete().await?;
+     }
+     CreatedWorkflow::DryRun(_) => {}
+ }
```

### Step 6: Remove Per-Call dry_run

Remove dry_run parameters from method calls:

```diff
- client.create_job(&job, namespace, false).await?;
- client.create_job(&test_job, namespace, true).await?;
+ 
+ let prod_client = MaestroClientBuilder::new()
+     .with_namespace("production")
+     .build()?;
+ 
+ let test_client = MaestroClientBuilder::new()
+     .with_namespace("production")
+     .with_dry_run(true)
+     .build()?;
```

## Module Structure Mapping

| Old Location (v0.3.x) | New Location (v0.4.0) |
|----------------------|------------------------|
| `entities::job::Job` | `workflows::Workflow` |
| `entities::job::JobBuilder` | `workflows::WorkflowBuilder` |
| `entities::job::JobConfig` | `workflows::WorkflowMetadata` |
| `entities::job::JobExecutionMode` | `workflows::ExecutionMode` |
| `MaestroK8sClient` | `client::MaestroClient` |
| `MaestroK8sClient::new()` | `MaestroClientBuilder::new().build()` |
| `entities::container` | `entities::container` (unchanged) |
| `entities::config` | `entities::config` (unchanged) |
| `entities::volumes` | `entities::volumes` (unchanged) |

**New Modules in v0.4.0:**
- `workflows::checkpointing` - Checkpointing and recovery
- `workflows::dependency` - Dependency management and DAG
- `workflows::execution` - Workflow execution engine
- `steps::kubernetes` - Kubernetes step types (Job, Pod)
- `steps::exec` - Execution step types (Python)
- `steps::observers` - File observer steps
- `client` - Client builder and security client

## Code Examples

### Client Creation

**Old API (v0.3.x):**
```rust
use k8s_maestro::MaestroK8sClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    Ok(())
}
```

**New API (v0.4.0):**
```rust
use k8s_maestro::MaestroClientBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;
    Ok(())
}
```

### Workflow/Job Creation

**Old API (v0.3.x):**
```rust
use k8s_maestro::entities::job::JobBuilder;
use k8s_maestro::entities::container::MaestroContainer;

let container = MaestroContainer::new("nginx:latest", "nginx")
    .set_image_pull_policy("IfNotPresent");

let job = JobBuilder::new()
    .with_name("my-job")
    .with_namespace("default")
    .with_container(container)
    .build()?;
```

**New API (v0.4.0):**
```rust
use k8s_maestro::workflows::WorkflowBuilder;
use k8s_maestro::steps::kubernetes::{JobStep, JobStepBuilder};

let step = JobStepBuilder::new()
    .with_name("job-1")
    .with_image("nginx:latest")
    .with_namespace("default")
    .build()?;

let workflow = WorkflowBuilder::new()
    .with_name("my-workflow")
    .with_namespace("default")
    .add_step(step)
    .build()?;
```

### Execution and Waiting

**Old API (v0.3.x):**
```rust
let result = client.create_job(&job, namespace, false).await?;
result.wait().await?;
println!("Job completed: {:?}", result);
```

**New API (v0.4.0):**
```rust
let created = client.create_workflow(workflow)?;

match created {
    CreatedWorkflow::Runtime(runtime) => {
        runtime.wait().await?;
        println!("Workflow completed");
    }
    CreatedWorkflow::DryRun(_) => {
        println!("Dry run validated");
    }
}
```

### Cleanup/Deletion

**Old API (v0.3.x):**
```rust
result.delete_job(dry_run).await?;
```

**New API (v0.4.0):**
```rust
match created {
    CreatedWorkflow::Runtime(runtime) => {
        runtime.delete().await?;
    }
    CreatedWorkflow::DryRun(_) => {}
}
```

### dry_run Configuration

**Old API (v0.3.x) - Per-call:**
```rust
client.create_job(&job, namespace, false).await?;
client.create_job(&test_job, namespace, true).await?;
```

**New API (v0.4.0) - Client-level:**
```rust
let prod_client = MaestroClientBuilder::new()
    .with_namespace("production")
    .build()?;

let dry_run_client = MaestroClientBuilder::new()
    .with_namespace("production")
    .with_dry_run(true)
    .build()?;

prod_client.create_workflow(prod_workflow)?;
dry_run_client.create_workflow(test_workflow)?;
```

### Container Configuration with Traits

**New API (v0.4.0):**
```rust
use k8s_maestro::steps::kubernetes::{JobStep, JobStepBuilder};
use k8s_maestro::steps::traits::{ResourceLimits, ResourceLimitedStep};

let limits = ResourceLimits::new()
    .with_cpu("500m")
    .with_memory("512Mi")
    .with_cpu_request("100m")
    .with_memory_request("256Mi");

let step = JobStepBuilder::new()
    .with_name("limited-job")
    .with_image("python:3.11")
    .with_resource_limits(limits)
    .build()?;
```

### Complete End-to-End Migration

**Old API (v0.3.x):**
```rust
use k8s_maestro::{MaestroK8sClient, MaestroContainer};
use k8s_maestro::entities::job::JobBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    
    let container = MaestroContainer::new("nginx:latest", "nginx")
        .set_image_pull_policy("IfNotPresent");
    
    let job = JobBuilder::new()
        .with_name("web-server")
        .with_namespace("default")
        .with_container(container)
        .build()?;
    
    let result = client.create_job(&job, "default", false).await?;
    result.wait().await?;
    
    result.delete_job(false).await?;
    
    Ok(())
}
```

**New API (v0.4.0):**
```rust
use k8s_maestro::{MaestroClientBuilder, CreatedWorkflow};
use k8s_maestro::workflows::WorkflowBuilder;
use k8s_maestro::steps::kubernetes::JobStep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;
    
    let workflow = WorkflowBuilder::new()
        .with_name("web-workflow")
        .with_namespace("default")
        .add_step(JobStep::new("web-server", "nginx:latest"))
        .build()?;
    
    let created = client.create_workflow(workflow)?;
    
    match created {
        CreatedWorkflow::Runtime(runtime) => {
            runtime.wait().await?;
            runtime.delete().await?;
        }
        CreatedWorkflow::DryRun(_) => {
            println!("Dry run validated successfully");
        }
    }
    
    Ok(())
}
```

## Common Pitfalls

### Pitfall 1: Forgetting to Call `.build()` on Client Builder

**Problem:**
```rust
let builder = MaestroClientBuilder::new()
    .with_namespace("production");
client.create_workflow(workflow)?;  // Error: builder is not a client
```

**Solution:**
```rust
let client = MaestroClientBuilder::new()
    .with_namespace("production")
    .build()?;
```

### Pitfall 2: Using Old Namespace Patterns

**Problem:**
```rust
let client = MaestroClientBuilder::new().build()?;
client.create_workflow(workflow, "default")?;  // Error: no namespace parameter
```

**Solution:**
```rust
let client = MaestroClientBuilder::new()
    .with_namespace("default")
    .build()?;

client.create_workflow(workflow)?;
```

**Note:** Namespace is configured once at client level, not per-call.

### Pitfall 3: Missing Required Workflow Name

**Problem:**
```rust
let workflow = WorkflowBuilder::new()
    .add_step(JobStep::new("job-1", "nginx:latest"))
    .build()?;  // Error: name is required
```

**Solution:**
```rust
let workflow = WorkflowBuilder::new()
    .with_name("my-workflow")
    .add_step(JobStep::new("job-1", "nginx:latest"))
    .build()?;
```

### Pitfall 4: Incorrect Step Implementation for New Traits

**Problem:**
```rust
struct MyStep {
    id: String,
}

impl WorkFlowStep for MyStep {
    fn step_id(&self) -> &str {
        &self.id
    }
    
    fn as_any(&self) -> &dyn Any {  // Missing import
        self
    }
}
```

**Solution:**
```rust
use std::any::Any;
use k8s_maestro::steps::traits::WorkFlowStep;

struct MyStep {
    id: String,
}

impl WorkFlowStep for MyStep {
    fn step_id(&self) -> &str {
        &self.id
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

### Pitfall 5: dry_run Configuration Placement

**Problem:**
```rust
let client = MaestroClientBuilder::new().build()?;
client.create_workflow(workflow, true)?;  // Error: no dry_run parameter
```

**Solution:**
```rust
let client = MaestroClientBuilder::new()
    .with_dry_run(true)
    .build()?;

client.create_workflow(workflow)?;
```

**Note:** Configure dry_run at client creation, not during workflow creation.

### Pitfall 6: Forgetting Pattern Matching on CreatedWorkflow

**Problem:**
```rust
let created = client.create_workflow(workflow)?;
created.wait().await?;  // Error: CreatedWorkflow doesn't have wait()
```

**Solution:**
```rust
let created = client.create_workflow(workflow)?;

match created {
    CreatedWorkflow::Runtime(runtime) => {
        runtime.wait().await?;
    }
    CreatedWorkflow::DryRun(_) => {
        println!("Dry run - nothing to wait for");
    }
}
```

## FAQ

### Q: Do I need to migrate immediately?

**A:** No, but we recommend migrating as soon as possible. The v0.3.x API will remain supported for at least 6 months, but new features will only be added to the v0.4.x workflow-centric API.

### Q: Will my old code continue to work?

**A:** If you're using v0.3.x, your code will continue to work. However, you cannot mix v0.3.x and v0.4.x APIs in the same project. Choose one version and stick with it.

### Q: How do I handle multi-step workflows?

**A:** In v0.4.0, multi-step workflows are first-class citizens:

```rust
use k8s_maestro::workflows::WorkflowBuilder;

let workflow = WorkflowBuilder::new()
    .with_name("multi-step")
    .add_step(JobStep::new("build", "rust:latest"))
    .add_step(JobStep::new("test", "rust:latest"))
    .add_step(JobStep::new("deploy", "kubectl:latest"))
    .build()?;
```

You can also add dependencies between steps:

```rust
use k8s_maestro::workflows::{ConditionBuilder, DependencyChain};

let mut chain = DependencyChain::new();
chain.add_step("build");
chain.add_step("test").with_dependency("build");
chain.add_step("deploy").with_dependency("test");
```

### Q: What about existing Kubernetes Job resources?

**A:** The v0.4.0 API still creates Kubernetes Jobs under the hood when you use `JobStep`. The abstraction has changed, but the underlying Kubernetes resources are the same. Your existing jobs will work with the new API.

### Q: How do I test the migration?

**A:** We recommend a phased approach:

1. **Dry run first:** Create a client with `dry_run: true` to validate your workflows
2. **Test in dev:** Run migrated workflows in a development namespace
3. **Monitor logs:** Check for deprecation warnings and errors
4. **Gradual rollout:** Migrate one workflow at a time

Example:
```rust
let test_client = MaestroClientBuilder::new()
    .with_namespace("development")
    .with_dry_run(true)  // Validate without executing
    .build()?;

test_client.create_workflow(test_workflow)?;
```

### Q: Where can I get help?

**A:** Resources for help:

- **Documentation:** [docs.rs/k8s-maestro](https://docs.rs/k8s-maestro)
- **GitHub Issues:** [github.com/andreclaudino/k8s-maestro/issues](https://github.com/andreclaudino/k8s-maestro/issues)
- **Examples:** Check the [examples/](../examples/) directory
- **Migration utilities:** See `k8s_maestro::migration` for type aliases and helpers

## Additional Resources

- [API Documentation](https://docs.rs/k8s-maestro)
- [Examples Directory](../examples/)
- [Workflow Guide](../site-docs/workflows.md)
- [Step Types Reference](../site-docs/steps.md)
- [Dependency System](../site-docs/dependencies.md)

## Summary Checklist

- [ ] Update all imports from `entities::job` to `workflows`
- [ ] Replace `MaestroK8sClient::new().await` with `MaestroClientBuilder::new().build()`
- [ ] Move `dry_run` configuration to client builder
- [ ] Rename `Job`/`JobBuilder` to `Workflow`/`WorkflowBuilder`
- [ ] Add at least one step to each workflow
- [ ] Update method calls to use new API signatures
- [ ] Add pattern matching for `CreatedWorkflow` enum
- [ ] Remove per-call namespace parameters
- [ ] Update resource limit configuration for steps
- [ ] Test with `dry_run: true` first
- [ ] Verify workflow execution in development environment
- [ ] Monitor logs for deprecation warnings

Happy migrating! 🚀
