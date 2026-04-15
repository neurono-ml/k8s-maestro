# Workflow API

The workflow API provides a high-level interface for defining and managing multi-step Kubernetes job workflows. It enables you to orchestrate complex job sequences with support for parallel execution, checkpointing, and resource management.

## Overview

The workflow system is built around three core components:

- **WorkflowBuilder** - A fluent builder API for constructing workflows
- **Workflow** - The main struct representing a complete workflow definition
- **WorkFlowStep** - A trait that represents individual workflow steps

Workflows can be executed in either sequential or parallel mode, with configurable checkpointing for long-running jobs and resource limits for Kubernetes pod scheduling.

## WorkflowBuilder

The `WorkflowBuilder` provides a fluent interface for constructing workflow definitions. All methods return `Self` to enable method chaining.

### `new()`

Creates a new workflow builder with default values:
- Namespace: `"default"`
- Parallelism: `1`
- Execution mode: `Sequential`

```rust
use k8s_maestro::workflows::WorkflowBuilder;

let builder = WorkflowBuilder::new();
```

### `with_name(name: impl Into<String>)`

Sets the workflow name. This is a required field.

```rust
let builder = WorkflowBuilder::new()
    .with_name("my-workflow");
```

### `with_namespace(namespace: impl Into<String>)`

Sets the Kubernetes namespace for the workflow. Defaults to `"default"` if not specified.

```rust
let builder = WorkflowBuilder::new()
    .with_namespace("production");
```

### `with_parallelism(parallelism: usize)`

Sets the maximum number of steps that can run concurrently. Must be greater than 0.

```rust
let builder = WorkflowBuilder::new()
    .with_parallelism(4);
```

### `with_resource_limits(limits: ResourceLimits)`

Sets resource limits (CPU, memory) for all steps in the workflow. Individual step limits can override these.

```rust
use k8s_maestro::steps::traits::ResourceLimits;

let limits = ResourceLimits::new()
    .with_cpu("1000m")
    .with_memory("1Gi");

let builder = WorkflowBuilder::new()
    .with_resource_limits(limits);
```

### `with_checkpointing(config: LegacyCheckpointConfig)`

Enables checkpointing with the specified configuration. Useful for long-running workflows that need to resume from failure.

```rust
use k8s_maestro::workflows::LegacyCheckpointConfig;

let checkpoint = LegacyCheckpointConfig::new()
    .enabled(true)
    .with_interval_secs(60)
    .with_retention_count(10);

let builder = WorkflowBuilder::new()
    .with_checkpointing(checkpoint);
```

### `with_execution_mode(mode: ExecutionMode)`

Sets the execution mode for the workflow. See [ExecutionMode](#executionmode) for details.

```rust
use k8s_maestro::workflows::ExecutionMode;

let builder = WorkflowBuilder::new()
    .with_execution_mode(ExecutionMode::Parallel(3));
```

### `with_label(key: impl Into<String>, value: impl Into<String>)`

Adds a label to the workflow metadata.

```rust
let builder = WorkflowBuilder::new()
    .with_label("env", "production")
    .with_label("team", "platform");
```

### `with_annotation(key: impl Into<String>, value: impl Into<String>)`

Adds an annotation to the workflow metadata.

```rust
let builder = WorkflowBuilder::new()
    .with_annotation("owner", "devops")
    .with_annotation("description", "CI/CD pipeline workflow");
```

### `add_step(step: impl WorkFlowStep + 'static)`

Adds a single step to the workflow. Steps are executed in the order they are added.

```rust
use k8s_maestro::steps::traits::WorkFlowStep;
use k8s_maestro::steps::container::MaestroContainer;

#[derive(Debug, Clone)]
struct MyStep {
    id: String,
}

impl WorkFlowStep for MyStep {
    fn step_id(&self) -> &str {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

let builder = WorkflowBuilder::new()
    .add_step(MyStep { id: "build".to_string() });
```

### `add_steps(steps: Vec<impl WorkFlowStep + 'static>)`

Adds multiple steps to the workflow at once.

```rust
let steps = vec![
    MyStep { id: "build".to_string() },
    MyStep { id: "test".to_string() },
    MyStep { id: "deploy".to_string() },
];

let builder = WorkflowBuilder::new()
    .add_steps(steps);
```

### `build() -> Result<Workflow>`

Builds the final workflow. Returns an error if:
- Name is not set
- No steps added
- Parallelism is 0

```rust
let workflow = WorkflowBuilder::new()
    .with_name("ci-pipeline")
    .with_namespace("production")
    .add_step(MyStep { id: "build".to_string() })
    .build()?;
```

## Workflow

The main workflow struct containing all configuration and metadata.

### Public Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | `String` | Unique identifier (UUID) |
| `name` | `String` | User-defined workflow name |
| `namespace` | `String` | Kubernetes namespace |
| `steps` | `Vec<Box<dyn WorkFlowStep>>` | Ordered list of workflow steps |
| `resource_limits` | `Option<ResourceLimits>` | Optional resource limits |
| `checkpoint_config` | `Option<LegacyCheckpointConfig>` | Optional checkpointing config |
| `metadata` | `WorkflowMetadata` | Labels and annotations |
| `parallelism` | `usize` | Maximum concurrent steps |
| `execution_mode` | `ExecutionMode` | Execution strategy |

### Methods

#### `name() -> &str`

Returns the workflow name.

```rust
let name = workflow.name();
```

#### `namespace() -> &str`

Returns the Kubernetes namespace.

```rust
let ns = workflow.namespace();
```

#### `steps() -> &[Box<dyn WorkFlowStep>]`

Returns a reference to the workflow steps.

```rust
for step in workflow.steps() {
    println!("Step: {}", step.step_id());
}
```

#### `metadata() -> &WorkflowMetadata`

Returns a reference to the workflow metadata.

```rust
let labels = &workflow.metadata().labels;
```

#### `execution_mode() -> &ExecutionMode`

Returns the execution mode.

```rust
if workflow.is_parallel() {
    // Handle parallel execution
}
```

#### `parallelism() -> usize`

Returns the configured parallelism.

```rust
let max_concurrent = workflow.parallelism();
```

#### `checkpoint_config() -> Option<&LegacyCheckpointConfig>`

Returns the checkpoint configuration if set.

```rust
if let Some(config) = workflow.checkpoint_config() {
    println!("Checkpoint interval: {}s", config.checkpoint_interval_secs);
}
```

#### `validate() -> Result<()>`

Validates the workflow configuration. Returns error if:
- Name is empty
- Namespace is empty
- No steps defined
- Parallelism is 0

```rust
workflow.validate()?;
```

#### `step_count() -> usize`

Returns the number of steps in the workflow.

```rust
let count = workflow.step_count();
```

#### `is_parallel() -> bool`

Returns `true` if the workflow is in parallel execution mode.

```rust
if workflow.is_parallel() {
    // Parallel execution path
}
```

#### `actual_parallelism() -> usize`

Returns the actual parallelism (minimum of configured parallelism and step count).

```rust
let actual = workflow.actual_parallelism();
```

#### `resolve_resource_limits(app_defaults: Option<&ResourceLimits>) -> ResourceLimits`

Resolves resource limits, prioritizing workflow-level limits over application defaults.

```rust
let app_defaults = ResourceLimits::new()
    .with_cpu("500m")
    .with_memory("512Mi");

let limits = workflow.resolve_resource_limits(Some(&app_defaults));
```

#### `step_resource_limits(step: &dyn WorkFlowStep, app_defaults: Option<&ResourceLimits>) -> ResourceLimits`

Gets resource limits for a specific step, applying workflow and application defaults.

```rust
let step_limits = workflow.step_resource_limits(&*workflow.steps[0], None);
```

## WorkflowMetadata

Contains metadata for the workflow including timestamps, labels, and annotations.

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `created_at` | `SystemTime` | Creation timestamp |
| `updated_at` | `SystemTime` | Last update timestamp |
| `labels` | `HashMap<String, String>` | User-defined labels |
| `annotations` | `HashMap<String, String>` | User-defined annotations |

### Default Implementation

```rust
let metadata = WorkflowMetadata::default();
assert!(metadata.labels.is_empty());
assert!(metadata.annotations.is_empty());
```

## ExecutionMode

Defines how workflow steps are executed.

### Variants

#### `Sequential`

Steps execute one at a time in order. This is the default mode.

```rust
let mode = ExecutionMode::Sequential;
```

#### `Parallel(usize)`

Steps execute concurrently up to the specified limit.

```rust
let mode = ExecutionMode::Parallel(3);  // Up to 3 steps concurrently
```

### Usage with WorkflowBuilder

```rust
let workflow = WorkflowBuilder::new()
    .with_name("parallel-workflow")
    .with_execution_mode(ExecutionMode::Parallel(4))
    .add_steps(steps)
    .build()?;
```

## LegacyCheckpointConfig

Configuration for workflow checkpointing (legacy implementation).

### Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `false` | Whether checkpointing is enabled |
| `checkpoint_interval_secs` | `u64` | `300` | Interval between checkpoints (seconds) |
| `retention_count` | `usize` | `10` | Number of checkpoints to retain |
| `storage_path` | `Option<String>` | `None` | Storage path for checkpoints |

### Builder Methods

```rust
let config = LegacyCheckpointConfig::new()
    .enabled(true)
    .with_interval_secs(120)
    .with_retention_count(20)
    .with_storage_path("/tmp/checkpoints");
```

### Default Configuration

```rust
let config = LegacyCheckpointConfig::default();
// config.enabled = false
// config.checkpoint_interval_secs = 300
// config.retention_count = 10
// config.storage_path = None
```

## Usage Examples

### Basic Workflow with Single Step

The simplest workflow with a single step:

```rust
use k8s_maestro::workflows::{WorkflowBuilder, Workflow};
use k8s_maestro::steps::traits::WorkFlowStep;

#[derive(Debug, Clone)]
struct SimpleStep {
    id: String,
}

impl WorkFlowStep for SimpleStep {
    fn step_id(&self) -> &str {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn main() -> anyhow::Result<()> {
    let workflow = WorkflowBuilder::new()
        .with_name("hello-world")
        .with_namespace("default")
        .add_step(SimpleStep { id: "greet".to_string() })
        .build()?;

    println!("Created workflow: {}", workflow.name());
    println!("Steps: {}", workflow.step_count());
    
    Ok(())
}
```

### Multi-Step Workflow with Parallelism

A workflow with multiple steps and parallel execution:

```rust
use k8s_maestro::workflows::{WorkflowBuilder, ExecutionMode};
use k8s_maestro::steps::traits::{WorkFlowStep, ResourceLimits};

#[derive(Debug, Clone)]
struct BuildStep { id: String }

impl WorkFlowStep for BuildStep {
    fn step_id(&self) -> &str { &self.id }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

#[derive(Debug, Clone)]
struct TestStep { id: String }

impl WorkFlowStep for TestStep {
    fn step_id(&self) -> &str { &self.id }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

#[derive(Debug, Clone)]
struct DeployStep { id: String }

impl WorkFlowStep for DeployStep {
    fn step_id(&self) -> &str { &self.id }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

fn main() -> anyhow::Result<()> {
    let steps = vec![
        BuildStep { id: "build".to_string() },
        TestStep { id: "test".to_string() },
        DeployStep { id: "deploy".to_string() },
    ];

    let limits = ResourceLimits::new()
        .with_cpu("2000m")
        .with_memory("2Gi");

    let workflow = WorkflowBuilder::new()
        .with_name("ci-cd-pipeline")
        .with_namespace("production")
        .with_parallelism(3)
        .with_resource_limits(limits)
        .with_execution_mode(ExecutionMode::Parallel(3))
        .with_label("app", "myapp")
        .with_label("env", "prod")
        .with_annotation("description", "CI/CD pipeline")
        .add_steps(steps)
        .build()?;

    println!("Workflow: {} in {}", workflow.name(), workflow.namespace());
    println!("Parallelism: {}", workflow.actual_parallelism());
    println!("Steps: {:?}", workflow.steps().iter().map(|s| s.step_id()).collect::<Vec<_>>());
    
    Ok(())
}
```

### Sequential Execution Mode

A workflow that executes steps one at a time in order:

```rust
use k8s_maestro::workflows::{WorkflowBuilder, ExecutionMode};
use k8s_maestro::steps::traits::WorkFlowStep;

#[derive(Debug, Clone)]
struct DataProcessingStep {
    id: String,
    order: i32,
}

impl WorkFlowStep for DataProcessingStep {
    fn step_id(&self) -> &str { &self.id }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

fn main() -> anyhow::Result<()> {
    let workflow = WorkflowBuilder::new()
        .with_name("data-pipeline")
        .with_namespace("dataops")
        .with_execution_mode(ExecutionMode::Sequential)  // Explicit sequential
        .add_step(DataProcessingStep { id: "extract".to_string(), order: 1 })
        .add_step(DataProcessingStep { id: "transform".to_string(), order: 2 })
        .add_step(DataProcessingStep { id: "load".to_string(), order: 3 })
        .with_label("pipeline", "etl")
        .build()?;

    assert!(!workflow.is_parallel());
    assert_eq!(workflow.actual_parallelism(), 1);
    assert_eq!(workflow.step_count(), 3);

    println!("Sequential pipeline '{}' has {} steps", workflow.name(), workflow.step_count());
    
    Ok(())
}
```

### Workflow with Checkpointing

A long-running workflow with checkpointing enabled for recovery:

```rust
use k8s_maestro::workflows::{WorkflowBuilder, LegacyCheckpointConfig, ExecutionMode};
use k8s_maestro::steps::traits::{WorkFlowStep, ResourceLimits};

#[derive(Debug, Clone)]
struct LongRunningStep { id: String }

impl WorkFlowStep for LongRunningStep {
    fn step_id(&self) -> &str { &self.id }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

fn main() -> anyhow::Result<()> {
    let checkpoint = LegacyCheckpointConfig::new()
        .enabled(true)
        .with_interval_secs(60)           // Checkpoint every 60 seconds
        .with_retention_count(5)          // Keep last 5 checkpoints
        .with_storage_path("/data/checkpoints");

    let resource_limits = ResourceLimits::new()
        .with_cpu("4000m")
        .with_memory("8Gi")
        .with_cpu_request("2000m")
        .with_memory_request("4Gi");

    let workflow = WorkflowBuilder::new()
        .with_name("training-workflow")
        .with_namespace("ml")
        .with_parallelism(2)
        .with_resource_limits(resource_limits)
        .with_checkpointing(checkpoint)
        .with_execution_mode(ExecutionMode::Parallel(2))
        .with_label("task", "ml-training")
        .with_annotation("owner", "data-science")
        .add_step(LongRunningStep { id: "download-data".to_string() })
        .add_step(LongRunningStep { id: "train-model".to_string() })
        .add_step(LongRunningStep { id: "validate-model".to_string() })
        .add_step(LongRunningStep { id: "deploy-model".to_string() })
        .build()?;

    // Verify checkpoint configuration
    if let Some(config) = workflow.checkpoint_config() {
        println!("Checkpointing enabled:");
        println!("  Interval: {} seconds", config.checkpoint_interval_secs);
        println!("  Retention: {} checkpoints", config.retention_count);
        println!("  Storage: {}", config.storage_path.as_ref().unwrap());
    }

    // Verify resource limits
    if let Some(limits) = &workflow.resource_limits {
        println!("Resource Limits:");
        println!("  CPU limit: {}", limits.cpu.as_ref().unwrap());
        println!("  Memory limit: {}", limits.memory.as_ref().unwrap());
    }

    Ok(())
}
```

## Error Handling

### Common Error Scenarios

The workflow builder and validation can fail in the following scenarios:

### Missing Workflow Name

```rust
let result = WorkflowBuilder::new()
    .with_namespace("default")
    .add_step(SimpleStep { id: "step1".to_string() })
    .build();

match result {
    Err(e) => println!("Error: {}", e),  // "Workflow name is required"
    Ok(_) => println!("Success"),
}
```

### No Steps Added

```rust
let result = WorkflowBuilder::new()
    .with_name("empty-workflow")
    .build();

match result {
    Err(e) => println!("Error: {}", e),  // "Workflow must have at least one step"
    Ok(_) => println!("Success"),
}
```

### Zero Parallelism

```rust
let result = WorkflowBuilder::new()
    .with_name("invalid-workflow")
    .with_parallelism(0)  // Invalid!
    .add_step(SimpleStep { id: "step1".to_string() })
    .build();

match result {
    Err(e) => println!("Error: {}", e),  // "Workflow parallelism must be greater than 0"
    Ok(_) => println!("Success"),
}
```

### Validation Errors at Runtime

```rust
let workflow = WorkflowBuilder::new()
    .with_name("test")
    .with_namespace("")
    .add_step(SimpleStep { id: "step1".to_string() })
    .build()?;

let validation_result = workflow.validate();
match validation_result {
    Err(e) => println!("Validation error: {}", e),
    Ok(_) => println!("Valid workflow"),
}
```

### Best Practices

1. **Always validate after building**:
   ```rust
   let workflow = builder.build()?;
   workflow.validate()?;
   ```

2. **Set resource limits for production**:
   ```rust
   let limits = ResourceLimits::new()
       .with_cpu("1000m")
       .with_memory("1Gi");
   ```

3. **Use checkpointing for long-running workflows**:
   ```rust
   let checkpoint = LegacyCheckpointConfig::new()
       .enabled(true)
       .with_interval_secs(300);
   ```

4. **Add labels and annotations for observability**:
   ```rust
   .with_label("team", "platform")
   .with_annotation("description", "Production pipeline")
   ```