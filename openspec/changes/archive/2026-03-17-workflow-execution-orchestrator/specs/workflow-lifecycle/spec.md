## ADDED Requirements

### Requirement: WorkflowExecution tracks execution state

The system SHALL provide a `WorkflowExecution` struct tracking workflow state including workflow_id, status, step_results, timestamps, and error information.

#### Scenario: Create WorkflowExecution
- **WHEN** a workflow begins execution
- **THEN** a WorkflowExecution SHALL be created with status Running
- **AND** started_at SHALL be set to the current time

### Requirement: Workflow status enumeration

The system SHALL support workflow statuses: Pending, Running, Succeeded, Failed, and Cancelled.

#### Scenario: Status transitions to Succeeded
- **WHEN** all workflow steps complete successfully
- **THEN** workflow status SHALL transition to Succeeded

#### Scenario: Status transitions to Failed
- **WHEN** a critical step fails
- **THEN** workflow status SHALL transition to Failed

#### Scenario: Status transitions to Cancelled
- **WHEN** cancel() is called on a running workflow
- **THEN** workflow status SHALL transition to Cancelled

### Requirement: Wait for workflow completion

The system SHALL provide `wait()` to block until workflow execution completes.

#### Scenario: Wait for successful workflow
- **WHEN** wait() is called on a running workflow
- **THEN** the system SHALL block until all steps complete
- **AND** return Ok(())

#### Scenario: Wait for failed workflow
- **WHEN** wait() is called and workflow fails
- **THEN** the system SHALL return an error with failure details

### Requirement: Pause workflow execution

The system SHALL provide `pause()` to pause workflow execution.

#### Scenario: Pause running workflow
- **WHEN** pause() is called on a running workflow
- **THEN** the system SHALL stop scheduling new steps
- **AND** wait for running steps to complete
- **AND** save a checkpoint

### Requirement: Resume paused workflow

The system SHALL provide `resume()` to resume a paused workflow.

#### Scenario: Resume paused workflow
- **WHEN** resume() is called on a paused workflow
- **THEN** the system SHALL restore state from checkpoint
- **AND** continue execution from where it was paused

### Requirement: Cancel workflow execution

The system SHALL provide `cancel()` to cancel workflow execution.

#### Scenario: Cancel running workflow
- **WHEN** cancel() is called on a running workflow
- **THEN** the system SHALL cancel all running steps
- **AND** set status to Cancelled

### Requirement: Get workflow status

The system SHALL provide `get_status()` to return current workflow status.

#### Scenario: Get status
- **WHEN** get_status() is called
- **THEN** the system SHALL return the current WorkflowStatus

### Requirement: Get step result by ID

The system SHALL provide `get_step_result(step_id)` to retrieve results for a specific step.

#### Scenario: Get completed step result
- **WHEN** get_step_result("step-a") is called for a completed step
- **THEN** the system SHALL return Some(&StepResult)

#### Scenario: Get pending step result
- **WHEN** get_step_result("pending-step") is called for a pending step
- **THEN** the system SHALL return None

### Requirement: Delete workflow resources

The system SHALL provide `delete()` to clean up all Kubernetes resources created by the workflow.

#### Scenario: Delete workflow
- **WHEN** delete() is called
- **THEN** the system SHALL delete all jobs and pods created by the workflow

### Requirement: Get checkpoint

The system SHALL provide `get_checkpoint()` to retrieve the current execution checkpoint.

#### Scenario: Get checkpoint for running workflow
- **WHEN** get_checkpoint() is called
- **THEN** the system SHALL return an optional Checkpoint with current state
