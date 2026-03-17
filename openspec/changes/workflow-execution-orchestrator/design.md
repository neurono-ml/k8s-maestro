## Context

The k8s-maestro project provides workflow definition capabilities through `Workflow`, `WorkflowBuilder`, and `DependencyGraph` types. Users can define workflows with steps, dependencies, conditions, and parallelism settings. However, there's no execution engine to actually run these workflows.

The existing infrastructure includes:
- `Workflow` struct with steps, resource limits, checkpoint config, and execution mode
- `DependencyGraph` with topological sort, cycle detection, and condition support
- `StepResult` and `StepStatus` for tracking step execution outcomes
- Various step traits: `ExecutableWorkFlowStep`, `WaitableWorkFlowStep`, `DeletableWorkFlowStep`

This design introduces a pure orchestration layer (no Kubernetes CRDs) that manages workflow execution.

## Goals / Non-Goals

**Goals:**
- Execute workflows based on DAG dependencies with topological ordering
- Support parallel execution within dependency constraints
- Evaluate conditions to determine step eligibility
- Track execution state and results for each step
- Support pause/resume via checkpoints
- Handle failures with configurable strategies (stop, continue, retry)
- Execute multiple step types (KubeJob, KubePod, Python, Rust, Lua, Wasm)

**Non-Goals:**
- Kubernetes CRD-based workflow resources (Argo Workflows style)
- Persistent storage of workflow state (filesystem only for checkpoints)
- Distributed workflow execution across multiple nodes
- Web UI for workflow visualization
- Workflow versioning or rollback

## Decisions

### D1: Execution Architecture

**Decision**: Three-layer architecture: Orchestrator → Scheduler → Executor

**Rationale**: 
- Orchestrator manages high-level workflow state and determines which steps can run
- Scheduler handles parallel execution and rate limiting
- Executor deals with step-type-specific execution logic

**Alternatives considered**:
- Single monolithic executor: Harder to test and extend
- Event-driven actor model: Overkill for current requirements, adds complexity

### D2: State Management

**Decision**: In-memory state with optional checkpoint serialization to filesystem

**Rationale**:
- Workflows run in a single process (no distributed coordination needed)
- Checkpoints enable pause/resume without complex state stores
- Filesystem storage is simple and portable

**Alternatives considered**:
- Database-backed state: Adds deployment complexity
- Redis/state server: Requires external infrastructure

### D3: Parallel Execution Model

**Decision**: Tokio task-based parallelism with semaphore for rate limiting

**Rationale**:
- Native async support in Rust via tokio
- Semaphores provide simple concurrency control
- Tasks are lightweight compared to OS threads

**Alternatives considered**:
- Rayon for CPU-bound work: Doesn't fit async K8s API calls
- Thread pool: Heavier weight, less idiomatic for async Rust

### D4: Condition Evaluation

**Decision**: Closure-based conditions (`ConditionFn`) evaluated against dependency results

**Rationale**:
- Flexible expression of conditions (success checks, output matching, etc.)
- Reuses existing `ConditionFn` type from dependency module
- No need for DSL or expression parser

**Alternatives considered**:
- CEL expressions: Adds dependency, learning curve
- JSON-based conditions: Less expressive, verbose

### D5: Step Type Dispatch

**Decision**: Enum-based step type detection with trait object downcasting

**Rationale**:
- Steps implement `WorkFlowStep` trait with `as_any()` for downcasting
- Each step type has specific execution logic
- Type-safe dispatch without runtime reflection overhead

**Alternatives considered**:
- Visitor pattern: More boilerplate for adding step types
- Dynamic dispatch only: Loses type-specific information

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Memory usage with large workflows | Limit parallelism, stream results |
| Checkpoint corruption | Use atomic writes, checksums |
| Step execution hangs | Configurable timeouts per step |
| K8s API rate limiting | Built-in retry with backoff |
| Partial failure cleanup | Track created resources, cleanup on failure |
| Long-running workflows | Support cancellation, checkpoint frequently |

## Migration Plan

Not applicable - this is a new feature with no existing behavior to migrate.

## Open Questions

1. **Resource cleanup on failure**: Should we support automatic rollback of completed steps? (TBD: Start with no rollback, add as needed)
2. **Step timeout defaults**: What should the default timeout be? (TBD: 30 minutes configurable)
3. **Checkpoint format**: Binary or JSON? (TBD: JSON for debuggability)
