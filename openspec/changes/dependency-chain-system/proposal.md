## Why

K8s Maestro currently lacks a built-in mechanism for expressing dependencies between workflow steps. Users must either manually orchestrate job execution or rely on external tools. This creates friction for complex workflows that require conditional execution based on previous step results, parallel execution of independent steps, or DAG-based orchestration.

## What Changes

- Introduce a new `workflows/dependency` module with types for building and executing dependency chains
- Add `DependencyChain` builder for defining step dependencies with a fluent API
- Add `DependencyGraph` (DAG) representation with edge storage and condition support
- Add `ConditionFn` type alias and `ConditionBuilder` for common conditional patterns
- Implement topological sorting with Kahn's algorithm for parallel execution levels
- Support three dependency types:
  - Simple: A → B (B waits for A)
  - Conditional: A → B with condition closure (B waits for A AND condition passes)
  - Any-of: A OR B → C (C waits for first successful dependency)

## Capabilities

### New Capabilities

- `dependency-chain`: Builder API for constructing dependency chains with step relationships and conditional execution rules
- `dag-execution`: DAG-based workflow execution with topological sorting and parallel execution level computation
- `condition-evaluation`: Conditional dependency evaluation using closures with predefined and custom conditions

### Modified Capabilities

(None - this is a new module with no existing spec changes)

## Impact

- **New Code**: `src/workflows/dependency/` module with 5 new files
- **Dependencies**: No new external dependencies (uses std and existing crate types)
- **API Surface**: New public types: `DependencyChain`, `DependencyGraph`, `ConditionBuilder`, `ConditionFn`, `StepResult`
- **Testing**: Comprehensive unit tests for each component
