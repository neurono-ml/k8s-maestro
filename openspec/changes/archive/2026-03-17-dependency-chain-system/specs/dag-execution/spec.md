## ADDED Requirements

### Requirement: DAG representation
The system SHALL provide a `DependencyGraph` type that represents a directed acyclic graph of workflow steps.

#### Scenario: Store steps and edges
- **WHEN** a `DependencyGraph` is created
- **THEN** it SHALL store all step IDs and dependency edges

#### Scenario: Store edge conditions
- **WHEN** an edge has an associated condition
- **THEN** the graph SHALL store the condition function mapped to that edge

### Requirement: Query dependencies
The system SHALL provide methods to query step relationships.

#### Scenario: Get dependencies
- **WHEN** user calls `graph.get_dependencies("step-b")`
- **THEN** the system SHALL return all steps that step-b depends on (its predecessors)

#### Scenario: Get dependents
- **WHEN** user calls `graph.get_dependents("step-a")`
- **THEN** the system SHALL return all steps that depend on step-a (its successors)

### Requirement: Topological sorting
The system SHALL compute parallel execution levels using topological sorting.

#### Scenario: Linear chain produces sequential levels
- **WHEN** graph contains A → B → C
- **THEN** `get_execution_levels()` SHALL return `[[A], [B], [C]]`

#### Scenario: Diamond pattern produces correct levels
- **WHEN** graph contains A → B, A → C, B → D, C → D
- **THEN** `get_execution_levels()` SHALL return `[[A], [B, C], [D]]`

#### Scenario: Parallel starts in same level
- **WHEN** graph contains independent steps A, B, C with no dependencies
- **THEN** `get_execution_levels()` SHALL return `[[A, B, C]]`

#### Scenario: Disconnected graphs handled
- **WHEN** graph contains two disconnected chains A → B and C → D
- **THEN** `get_execution_levels()` SHALL return levels with both chains interleaved correctly

### Requirement: Kahn's algorithm implementation
The system SHALL use Kahn's algorithm for topological sorting.

#### Scenario: Algorithm produces valid topological order
- **WHEN** topological sort is computed
- **THEN** each step SHALL appear after all its dependencies

#### Scenario: Cycle detection during sort
- **WHEN** a cycle exists in the graph
- **THEN** `topological_sort()` SHALL return an error with cycle information
