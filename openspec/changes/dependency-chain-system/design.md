## Context

K8s Maestro needs a pure-Rust workflow orchestration layer that enables DAG-based job execution without requiring Kubernetes CRDs or external workflow engines. The system must support conditional execution based on step results, parallel execution of independent steps, and complex dependency patterns.

## Goals / Non-Goals

**Goals:**
- Provide a fluent builder API for constructing dependency chains
- Support conditional dependencies using Rust closures
- Enable "any-of" dependencies (step triggers when ANY dependency succeeds)
- Compute parallel execution levels via topological sorting
- Detect and report cycles in dependency graphs
- Support disconnected graphs (multiple independent workflows)

**Non-Goals:**
- Runtime workflow execution engine (this module provides the DAG structure only)
- Persistence or serialization of dependency graphs
- Integration with external workflow systems (Argo, Tekton, etc.)
- Visual DAG representation or UI

## Decisions

### D1: Closure-based Conditions
**Decision**: Use `Box<dyn Fn(&[StepResult]) -> bool + Send + Sync>` for condition functions.

**Rationale**: Enables maximum flexibility while maintaining thread safety. Users can write arbitrary Rust logic for conditions without implementing traits.

**Alternatives**:
- Enum-based conditions: Less flexible, requires predefined condition types
- Trait objects: More boilerplate for simple conditions

### D2: Kahn's Algorithm for Topological Sort
**Decision**: Use Kahn's algorithm for computing execution levels.

**Rationale**: Naturally produces levels of parallel execution and detects cycles efficiently. O(V + E) complexity.

**Alternatives**:
- DFS-based sort: Doesn't naturally group by parallel levels
- Parallel sort algorithms: Overkill for typical workflow sizes

### D3: BTreeMap for Edge Conditions
**Decision**: Store conditions in `BTreeMap<(StepId, StepId), Option<ConditionFn>>`.

**Rationale**: Ordered keys enable deterministic iteration and efficient lookup by edge.

**Alternatives**:
- HashMap: Non-deterministic iteration order
- Vec of tuples: O(n) lookup per edge

### D4: Type Alias for StepId
**Decision**: Use `pub type StepId = String;` for step identifiers.

**Rationale**: Simple, ergonomic, and sufficient for the use case. Can be changed to a newtype wrapper later if needed.

**Alternatives**:
- Generic `StepId<T>`: Adds complexity without benefit
- Integer IDs: Requires ID management, less readable

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Closure conditions not serializable | Document limitation; users needing persistence can use predefined conditions |
| Large graphs may have many levels | Document best practices for workflow design |
| Closures capture environment | Document ownership semantics; encourage pure functions |
