## ADDED Requirements

### Requirement: SQLite StatefulSet deployment
The system SHALL create and manage a Kubernetes StatefulSet for SQLite checkpoint storage with the name "maestro-checkpoint-storage".

#### Scenario: Create StatefulSet
- **WHEN** the SQLite checkpoint storage is initialized
- **THEN** a StatefulSet named "maestro-checkpoint-storage" shall be created in the configured namespace

### Requirement: Persistent Volume Claim for SQLite
The system SHALL create a PersistentVolumeClaim attached to the StatefulSet for durable checkpoint storage.

#### Scenario: Create PVC
- **WHEN** the SQLite StatefulSet is created
- **THEN** a PVC shall be created and mounted to the SQLite container

### Requirement: SQLite container with HTTP API
The system SHALL run a container with SQLite database and provide HTTP endpoints for checkpoint CRUD operations.

#### Scenario: Access checkpoint data via HTTP
- **WHEN** an HTTP request is made to the SQLite pod's API
- **THEN** the request shall be processed against the SQLite database and return appropriate responses

### Requirement: REST API endpoints for checkpoints
The system SHALL provide REST API endpoints for checkpoint operations: POST /checkpoints, GET /checkpoints/{workflow_id}, PUT /checkpoints/{workflow_id}, DELETE /checkpoints/{workflow_id}, GET /checkpoints.

#### Scenario: Create checkpoint via API
- **WHEN** POST /checkpoints is called with checkpoint data
- **THEN** the checkpoint shall be saved and HTTP 201 returned

#### Scenario: Retrieve checkpoint via API
- **WHEN** GET /checkpoints/{workflow_id} is called for an existing checkpoint
- **THEN** the checkpoint data shall be returned with HTTP 200

#### Scenario: Update checkpoint via API
- **WHEN** PUT /checkpoints/{workflow_id} is called with checkpoint data
- **THEN** the checkpoint shall be updated and HTTP 200 returned

#### Scenario: Delete checkpoint via API
- **WHEN** DELETE /checkpoints/{workflow_id} is called for an existing checkpoint
- **THEN** the checkpoint shall be deleted and HTTP 204 returned

#### Scenario: List checkpoints via API
- **WHEN** GET /checkpoints is called
- **THEN** all checkpoint metadata shall be returned with HTTP 200

### Requirement: HTTP client for SQLite communication
The system SHALL implement an HTTP client to communicate with the SQLite StatefulSet pod's REST API.

#### Scenario: Send HTTP request to SQLite pod
- **WHEN** the SQLiteCheckpointStorage performs an operation
- **THEN** the HTTP client shall send a request to the SQLite pod's API endpoint

### Requirement: Wait for StatefulSet readiness
The system SHALL wait for the StatefulSet to be ready before performing checkpoint operations.

#### Scenario: Wait for StatefulSet readiness
- **WHEN** the SQLite checkpoint storage is initialized
- **THEN** the system shall wait until the StatefulSet reports as ready

### Requirement: SQLite database schema
The system SHALL use a SQLite database with tables for checkpoints, including columns for workflow_id, checkpoint_time, steps (JSON), metadata (JSON), and version.

#### Scenario: Store checkpoint in SQLite
- **WHEN** a checkpoint is saved to SQLite
- **THEN** the data shall be stored in the checkpoints table with all required columns

### Requirement: ACID compliance
The system SHALL ensure all SQLite operations are ACID compliant using database transactions.

#### Scenario: Transactional checkpoint save
- **WHEN** a checkpoint is saved
- **THEN** the operation shall be atomic, consistent, isolated, and durable

### Requirement: CheckpointStorage trait implementation
The system SHALL implement the CheckpointStorage trait for SQLiteCheckpointStorage using HTTP communication with the StatefulSet.

#### Scenario: Use SQLiteCheckpointStorage
- **WHEN** SQLiteCheckpointStorage is used
- **THEN** it shall fulfill the CheckpointStorage trait contract via HTTP API calls

### Requirement: Error handling for HTTP failures
The system SHALL handle HTTP failures with appropriate error messages and types.

#### Scenario: Handle HTTP error response
- **WHEN** the SQLite pod returns an HTTP error response
- **THEN** the error shall be propagated with context about the failure
