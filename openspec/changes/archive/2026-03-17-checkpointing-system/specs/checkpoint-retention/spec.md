## ADDED Requirements

### Requirement: Retention policy enforcement
The system SHALL enforce retention policies to automatically clean up checkpoints based on configured rules.

#### Scenario: Enforce time-based retention
- **WHEN** retention policy specifies max age
- **THEN** checkpoints older than the max age shall be automatically deleted

#### Scenario: Enforce count-based retention
- **WHEN** retention policy specifies max count per workflow
- **THEN** only the most recent N checkpoints per workflow shall be retained

### Requirement: Checkpoint age tracking
The system SHALL track the age of checkpoints based on checkpoint_time timestamp for retention evaluation.

#### Scenario: Track checkpoint age
- **WHEN** a checkpoint is saved
- **THEN** the checkpoint_time shall be recorded and used for age calculations

### Requirement: Checkpoint metadata
The system SHALL maintain checkpoint metadata including workflow_id, checkpoint_time, and version for efficient retention queries.

#### Scenario: Query checkpoint metadata
- **WHEN** retention policies are evaluated
- **THEN** checkpoint metadata shall be queried efficiently without loading full checkpoint data

### Requirement: Background cleanup job
The system SHALL provide a background cleanup job that periodically evaluates and enforces retention policies.

#### Scenario: Run background cleanup
- **WHEN** the background cleanup job runs
- **THEN** it shall evaluate all checkpoints and delete those exceeding retention limits

#### Scenario: Schedule cleanup job
- **WHEN** the cleanup job is started
- **THEN** it shall run at configurable intervals (e.g., hourly)

### Requirement: Manual cleanup trigger
The system SHALL allow manual triggering of cleanup operations on-demand.

#### Scenario: Trigger manual cleanup
- **WHEN** cleanup() is called manually
- **THEN** the system shall immediately evaluate and enforce retention policies

### Requirement: Configurable retention policies
The system SHALL allow configuration of retention policies including max_age and max_count per workflow.

#### Scenario: Configure max_age policy
- **WHEN** retention policy is configured with max_age duration
- **THEN** the system shall enforce the specified age limit

#### Scenario: Configure max_count policy
- **WHEN** retention policy is configured with max_count
- **THEN** the system shall retain only the specified number of checkpoints per workflow

#### Scenario: Configure combined policies
- **WHEN** retention policy is configured with both max_age and max_count
- **THEN** the system shall enforce both policies (delete checkpoints exceeding either limit)

### Requirement: Default retention policy
The system SHALL provide a default retention policy (e.g., 7 days, 10 checkpoints per workflow) when not explicitly configured.

#### Scenario: Use default retention
- **WHEN** no retention policy is configured
- **THEN** the system shall use the default policy (7 days, 10 checkpoints per workflow)

### Requirement: Retention policy validation
The system SHALL validate retention policy values and return errors for invalid configurations.

#### Scenario: Validate retention policy
- **WHEN** a retention policy is configured with invalid values (e.g., negative count, zero duration)
- **THEN** the validation shall fail with a descriptive error

### Requirement: Checkpoint deletion safety
The system SHALL ensure checkpoint deletion operations are atomic and do not leave orphaned data.

#### Scenario: Safe checkpoint deletion
- **WHEN** a checkpoint is deleted as part of retention enforcement
- **THEN** the deletion shall be atomic and all related data removed

### Requirement: Retention policy metrics
The system SHALL track and report metrics on retention enforcement (checkpoints deleted, retention policy violations).

#### Scenario: Track deletion metrics
- **WHEN** checkpoints are deleted by retention policy
- **THEN** the system shall record metrics on the number of checkpoints deleted

#### Scenario: Report retention status
- **WHEN** retention status is queried
- **THEN** the system shall return metrics on total checkpoints, deleted checkpoints, and policy violations

### Requirement: Checkpoint expiration time calculation
The system SHALL calculate checkpoint expiration time based on checkpoint_time and max_age policy.

#### Scenario: Calculate expiration
- **WHEN** a checkpoint's expiration is evaluated
- **THEN** the expiration time shall be checkpoint_time + max_age

### Requirement: Per-workflow retention
The system SHALL apply retention policies on a per-workflow basis to ensure isolation between workflows.

#### Scenario: Enforce per-workflow retention
- **WHEN** retention policies are enforced
- **THEN** each workflow's checkpoints shall be evaluated independently

### Requirement: Retention policy dry run
The system SHALL support a dry-run mode that evaluates retention policies without actually deleting checkpoints.

#### Scenario: Dry run retention
- **WHEN** retention cleanup is run in dry-run mode
- **THEN** the system shall report which checkpoints would be deleted without performing deletions
