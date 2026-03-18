## Why

The workflow module currently defines workflow structure, steps, and dependency graphs, but lacks the execution engine to actually run workflows. Users can define workflows with DAG-based dependencies, conditions, and parallelism settings, but cannot execute them. This change introduces a complete orchestration layer that transforms workflow definitions into running executions with proper dependency resolution, parallel execution, condition evaluation, and result collection.

## What Changes

- New `src/workflows/execution/` module containing:
  - `orchestrator.rs`: `WorkflowOrchestrator` for DAG-based workflow execution management
  - `executor.rs`: `StepExecutor` for executing individual step types (KubeJob, KubePod, Python, Rust, Lua, Wasm)
  - `workflow_execution.rs`: `WorkflowExecution` struct for tracking execution state and lifecycle
  - `scheduler.rs`: `Scheduler` for parallel step execution with rate limiting
- Topological sort integration for execution order determination
- Parallel execution within dependency constraints using tokio tasks
- Condition evaluation using closures
- Execution state management (Pending, Running, Succeeded, Failed, Cancelled)
- Checkpoint support for pause/resume functionality
- Comprehensive unit and integration tests

## Capabilities

### New Capabilities

- `workflow-orchestration`: Core orchestration engine for executing DAG-based workflows with dependency resolution, parallel execution, and condition evaluation
- `step-execution`: Individual step executor supporting KubeJob, KubePod, Python, Rust, Lua, and Wasm step types
- `workflow-lifecycle`: Workflow execution lifecycle management including wait, pause, resume, cancel, and checkpoint operations
- `parallel-scheduling`: Scheduler for parallel step execution with configurable parallelism and rate limiting

### Modified Capabilities

- None - this adds new capabilities without modifying existing requirements

## Impact

- **New Files**:
  - `src/workflows/execution/mod.rs`
  - `src/workflows/execution/orchestrator.rs`
  - `src/workflows/execution/executor.rs`
  - `src/workflows/execution/workflow_execution.rs`
  - `src/workflows/execution/scheduler.rs`
- **Modified Files**:
  - `src/workflows/mod.rs` - add `pub mod execution`
  - `Cargo.toml` - may need additional dependencies for async runtime utilities
- **Dependencies**: Uses existing `DependencyGraph`, `StepResult`, `Workflow`, and step traits
- **APIs**: New public API for workflow execution orchestration
- **Testing**: Unit tests in each module + integration tests with Kind cluster
