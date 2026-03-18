## ADDED Requirements

### Requirement: WorkflowOrchestrator initializes with workflow and client

The system SHALL provide a `WorkflowOrchestrator` that accepts a `Workflow` definition and a Kubernetes client for execution.

#### Scenario: Create orchestrator with valid workflow
- **WHEN** a WorkflowOrchestrator is created with a valid Workflow and MaestroK8sClient
- **THEN** the orchestrator SHALL be ready to execute the workflow

### Requirement: Execute workflow returns WorkflowExecution

The system SHALL provide an `execute()` method that starts workflow execution and returns a `WorkflowExecution` handle.

#### Scenario: Execute simple workflow
- **WHEN** execute() is called on a workflow with a single step
- **THEN** the system SHALL return a WorkflowExecution with status Running
- **AND** the step SHALL begin execution

### Requirement: Execute individual step by ID

The system SHALL provide `execute_step(step_id)` to execute a specific step within the workflow.

#### Scenario: Execute specific step
- **WHEN** execute_step("step-a") is called
- **THEN** the system SHALL execute only the specified step
- **AND** return the StepResult for that step

#### Scenario: Execute step with unsatisfied dependencies
- **WHEN** execute_step("dependent-step") is called before its dependencies complete
- **THEN** the system SHALL return an error indicating unsatisfied dependencies

### Requirement: Evaluate conditions against dependency results

The system SHALL provide `evaluate_condition(condition, deps)` to determine if a step should execute based on its dependencies' results.

#### Scenario: Condition evaluates to true
- **WHEN** a condition checking for success status is evaluated against successful dependency results
- **THEN** the system SHALL return true

#### Scenario: Condition evaluates to false
- **WHEN** a condition checking for specific output is evaluated against results without that output
- **THEN** the system SHALL return false

### Requirement: Get next executable steps

The system SHALL provide `get_next_executable_steps()` returning steps whose dependencies are satisfied and conditions pass.

#### Scenario: Get executable steps at workflow start
- **WHEN** get_next_executable_steps() is called on a fresh workflow
- **THEN** the system SHALL return all steps with no dependencies

#### Scenario: Get executable steps after completion
- **WHEN** get_next_executable_steps() is called after step A completes
- **AND** step B depends on step A
- **THEN** the system SHALL include step B in the result

### Requirement: Mark step complete with result

The system SHALL provide `mark_step_complete(step_id, result)` to record step completion and update execution state.

#### Scenario: Mark successful step complete
- **WHEN** mark_step_complete("step-a", success_result) is called
- **THEN** the system SHALL record the result
- **AND** dependent steps SHALL become eligible for execution

#### Scenario: Mark failed step complete
- **WHEN** mark_step_complete("step-a", failure_result) is called
- **THEN** the system SHALL record the failure
- **AND** dependent steps with success conditions SHALL NOT become eligible

### Requirement: Topological sort for execution order

The system SHALL use topological sorting to determine execution order based on the dependency graph.

#### Scenario: Linear dependency chain
- **WHEN** workflow has steps A → B → C (A before B before C)
- **THEN** the system SHALL execute in order A, B, C

#### Scenario: Diamond dependency
- **WHEN** workflow has A → B, A → C, B → D, C → D
- **THEN** the system SHALL execute A first, then B and C in parallel, then D

### Requirement: Detect dependency cycles

The system SHALL detect and report cycles in the dependency graph before execution.

#### Scenario: Cycle detection
- **WHEN** a workflow contains a cycle (A → B → A)
- **THEN** the system SHALL return an error before starting execution
