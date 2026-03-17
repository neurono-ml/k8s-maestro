## ADDED Requirements

### Requirement: Checkpoint configuration struct
The system SHALL provide a CheckpointConfig struct that defines checkpoint behavior and storage settings.

#### Scenario: Create checkpoint configuration
- **WHEN** CheckpointConfig is initialized with default values
- **THEN** the config shall provide sensible defaults for checkpointing behavior

### Requirement: Checkpoint frequency options
The system SHALL support CheckpointFrequency enum with variants: OnStepCompletion, OnSuccess, and Periodic(Duration).

#### Scenario: Configure OnStepCompletion frequency
- **WHEN** CheckpointFrequency::OnStepCompletion is used
- **THEN** checkpoints shall be saved after each workflow step completes

#### Scenario: Configure OnSuccess frequency
- **WHEN** CheckpointFrequency::OnSuccess is used
- **THEN** checkpoints shall be saved only when the workflow completes successfully

#### Scenario: Configure Periodic frequency
- **WHEN** CheckpointFrequency::Periodic(duration) is used
- **THEN** checkpoints shall be saved at the specified time intervals

### Requirement: Storage backend configuration
The system SHALL support CheckpointStorageConfig enum with variants: Sqlite, Etcd, Redis, Postgres.

#### Scenario: Configure SQLite storage
- **WHEN** CheckpointStorageConfig::Sqlite is used
- **THEN** the system shall use SQLite as the storage backend

#### Scenario: Configure Etcd storage
- **WHEN** CheckpointStorageConfig::Etcd is used
- **THEN** the system shall use Etcd as the storage backend

#### Scenario: Configure Redis storage
- **WHEN** CheckpointStorageConfig::Redis is used
- **THEN** the system shall use Redis as the storage backend

#### Scenario: Configure Postgres storage
- **WHEN** CheckpointStorageConfig::Postgres is used
- **THEN** the system shall use Postgres as the storage backend

### Requirement: Storage backend parameters
The system SHALL allow configuration of storage backend parameters (connection string, namespace, etc.) for each backend type.

#### Scenario: Configure SQLite parameters
- **WHEN** CheckpointStorageConfig::Sqlite is configured with namespace and PVC size
- **THEN** the SQLite StatefulSet shall use the configured parameters

#### Scenario: Configure Etcd parameters
- **WHEN** CheckpointStorageConfig::Etcd is configured with connection endpoints
- **THEN** the Etcd client shall connect to the specified endpoints

### Requirement: Retention policy configuration
The system SHALL allow configuration of retention policies including time-to-live and max checkpoint count per workflow.

#### Scenario: Configure retention by time
- **WHEN** retention policy specifies a time-to-live duration
- **THEN** checkpoints older than the specified duration shall be eligible for cleanup

#### Scenario: Configure retention by count
- **WHEN** retention policy specifies a max checkpoint count per workflow
- **THEN** only the most recent N checkpoints per workflow shall be retained

### Requirement: Default configuration values
The system SHALL provide sensible default values for all configuration options.

#### Scenario: Use default configuration
- **WHEN** CheckpointConfig is created without explicit values
- **THEN** the config shall use defaults (e.g., Sqlite backend, OnStepCompletion frequency, 7 day retention)

### Requirement: Configuration validation
The system SHALL validate configuration values and return errors for invalid settings.

#### Scenario: Validate configuration
- **WHEN** CheckpointConfig is created with invalid values (e.g., negative duration)
- **THEN** the validation shall fail with a descriptive error

### Requirement: Configuration serialization
The system SHALL support serialization and deserialization of CheckpointConfig using serde.

#### Scenario: Serialize configuration
- **WHEN** CheckpointConfig is serialized to JSON or YAML
- **THEN** all configuration values shall be preserved and correctly formatted

#### Scenario: Deserialize configuration
- **WHEN** CheckpointConfig is deserialized from JSON or YAML
- **THEN** the configuration shall be loaded with correct values and validated

### Requirement: Configuration from environment variables
The system SHALL support loading CheckpointConfig from environment variables for deployment flexibility.

#### Scenario: Load config from environment
- **WHEN** CheckpointConfig is loaded from environment variables
- **THEN** the config shall reflect the environment variable values
