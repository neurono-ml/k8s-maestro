## ADDED Requirements

### Requirement: Dependency chain construction
The system SHALL provide a `DependencyChain` builder for defining step dependencies with a fluent API.

#### Scenario: Create empty chain
- **WHEN** user creates a new `DependencyChain::new()`
- **THEN** the chain SHALL be empty with no dependencies

#### Scenario: Add simple dependency
- **WHEN** user calls `chain.with_dependency("step-b", "step-a")`
- **THEN** step-b SHALL depend on step-a completing successfully

#### Scenario: Add conditional dependency
- **WHEN** user calls `chain.with_conditional_dependency("step-c", "step-b", condition_fn)`
- **THEN** step-c SHALL depend on step-b AND the condition function returning true

#### Scenario: Add any-of dependency
- **WHEN** user calls `chain.with_dependency_any("step-d", vec!["step-b", "step-c"])`
- **THEN** step-d SHALL execute when ANY of the specified dependencies complete successfully

#### Scenario: Build DAG from chain
- **WHEN** user calls `chain.build_dag()`
- **THEN** the system SHALL return a `DependencyGraph` with all dependencies and conditions

### Requirement: Cycle detection during build
The system SHALL detect and reject cyclic dependencies during DAG construction.

#### Scenario: Detect simple cycle
- **WHEN** user adds dependencies A → B → C → A
- **THEN** `build_dag()` SHALL return an error indicating a cycle was detected

#### Scenario: Detect self-dependency
- **WHEN** user adds a dependency from step A to step A
- **THEN** `build_dag()` SHALL return an error indicating a cycle was detected

#### Scenario: Accept valid complex graph
- **WHEN** user adds dependencies forming a valid DAG (no cycles)
- **THEN** `build_dag()` SHALL succeed and return the graph
