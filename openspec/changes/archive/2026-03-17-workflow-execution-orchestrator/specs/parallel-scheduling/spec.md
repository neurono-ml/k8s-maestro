## ADDED Requirements

### Requirement: Schedule steps with parallelism limit

The system SHALL provide a `Scheduler` that executes steps in parallel up to a configured parallelism limit.

#### Scenario: Schedule with parallelism of 3
- **WHEN** schedule_steps is called with 5 steps and parallelism of 3
- **THEN** the system SHALL execute at most 3 steps concurrently
- **AND** start remaining steps as slots become available

### Requirement: Respect dependency constraints

The system SHALL only schedule steps whose dependencies have completed successfully.

#### Scenario: Schedule with satisfied dependencies
- **WHEN** step B depends on step A
- **AND** step A has completed successfully
- **THEN** the system SHALL schedule step B

#### Scenario: Schedule with unsatisfied dependencies
- **WHEN** step B depends on step A
- **AND** step A has not completed
- **THEN** the system SHALL NOT schedule step B

### Requirement: Use tokio tasks for parallel execution

The system SHALL use tokio tasks for concurrent step execution.

#### Scenario: Parallel task spawning
- **WHEN** multiple steps are ready for execution
- **THEN** the system SHALL spawn a tokio task for each step
- **AND** execute them concurrently

### Requirement: Rate limiting per parallelism setting

The system SHALL enforce rate limiting using a semaphore based on the parallelism setting.

#### Scenario: Semaphore limits concurrency
- **WHEN** parallelism is set to 2
- **THEN** the semaphore SHALL allow at most 2 concurrent executions
- **AND** additional steps SHALL wait for permits

### Requirement: Handle step failures

The system SHALL handle step failures according to configured failure strategy (continue or stop).

#### Scenario: Stop on failure
- **WHEN** a step fails and failure strategy is Stop
- **THEN** the system SHALL not schedule remaining steps
- **AND** the workflow SHALL fail

#### Scenario: Continue on failure
- **WHEN** a step fails and failure strategy is Continue
- **THEN** the system SHALL continue scheduling remaining eligible steps
- **AND** record the failure in results

### Requirement: Collect step results

The system SHALL collect results from all scheduled steps.

#### Scenario: Collect all results
- **WHEN** all scheduled steps complete
- **THEN** the system SHALL return a collection of all StepResults

### Requirement: Handle step timeouts

The system SHALL support configurable timeouts for individual steps.

#### Scenario: Step exceeds timeout
- **WHEN** a step runs longer than its configured timeout
- **THEN** the system SHALL cancel the step
- **AND** return a StepResult with status Failure

### Requirement: Execution levels from topological sort

The system SHALL execute steps level by level based on topological sort results.

#### Scenario: Execute by levels
- **WHEN** topological sort returns levels [[A], [B, C], [D]]
- **THEN** the system SHALL execute A first
- **AND** then execute B and C in parallel
- **AND** finally execute D after B and C complete
