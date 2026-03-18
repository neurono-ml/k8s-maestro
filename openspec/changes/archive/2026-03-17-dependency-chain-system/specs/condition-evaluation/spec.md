## ADDED Requirements

### Requirement: Condition function type
The system SHALL define a `ConditionFn` type as `Box<dyn Fn(&[StepResult]) -> bool + Send + Sync>`.

#### Scenario: Condition receives dependency results
- **WHEN** a condition function is evaluated
- **THEN** it SHALL receive the results of all dependency steps as input

#### Scenario: Condition returns boolean
- **WHEN** a condition function is evaluated
- **THEN** it SHALL return true to allow execution or false to skip

### Requirement: ConditionBuilder for common patterns
The system SHALL provide a `ConditionBuilder` with predefined condition factories.

#### Scenario: All success condition
- **WHEN** `ConditionBuilder::all_success()` is used
- **THEN** the condition SHALL return true only if ALL dependency results are successful

#### Scenario: Any success condition
- **WHEN** `ConditionBuilder::any_success()` is used
- **THEN** the condition SHALL return true if ANY dependency result is successful

#### Scenario: All failure condition
- **WHEN** `ConditionBuilder::all_failure()` is used
- **THEN** the condition SHALL return true only if ALL dependency results have failed

#### Scenario: Any failure condition
- **WHEN** `ConditionBuilder::any_failure()` is used
- **THEN** the condition SHALL return true if ANY dependency result has failed

#### Scenario: Custom condition
- **WHEN** `ConditionBuilder::custom(fn)` is used
- **THEN** the provided closure SHALL be used as the condition function

### Requirement: Output-based conditions
The system SHALL provide conditions that evaluate step outputs.

#### Scenario: Output equals condition
- **WHEN** `ConditionBuilder::output_equals("step-a", "key", "value")` is used
- **THEN** the condition SHALL return true if step-a's output "key" equals "value"

#### Scenario: Output greater than condition
- **WHEN** `ConditionBuilder::output_greater_than("step-a", "count", 100)` is used
- **THEN** the condition SHALL return true if step-a's output "count" is greater than 100

### Requirement: StepResult type
The system SHALL define a `StepResult` type containing step execution information.

#### Scenario: Store success status
- **WHEN** a step completes
- **THEN** `StepResult` SHALL indicate success or failure

#### Scenario: Store step outputs
- **WHEN** a step produces outputs
- **THEN** `StepResult` SHALL store key-value output pairs

#### Scenario: Retrieve output value
- **WHEN** `result.get_output("key")` is called
- **THEN** the system SHALL return the output value or None if not present
