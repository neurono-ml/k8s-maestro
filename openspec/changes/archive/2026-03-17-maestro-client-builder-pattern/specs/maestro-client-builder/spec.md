## ADDED Requirements

### Requirement: Builder creates initial configuration
The system SHALL provide a `MaestroClientBuilder` with a `new()` method that creates an empty builder ready for configuration.

#### Scenario: Create new builder
- **WHEN** user calls `MaestroClientBuilder::new()`
- **THEN** system returns a builder instance with all configuration fields unset

### Requirement: Builder accepts Kubernetes configuration path
The builder SHALL provide a `with_kube_config(config_path: impl AsRef<Path>)` method that sets the Kubernetes configuration file path.

#### Scenario: Set kube config path with string
- **WHEN** user calls `builder.with_kube_config("~/.kube/config")`
- **THEN** builder stores the provided path
- **AND** method returns builder for chaining

#### Scenario: Set kube config path with PathBuf
- **WHEN** user calls `builder.with_kube_config(PathBuf::from("/path/to/config"))`
- **THEN** builder stores the provided path
- **AND** method returns builder for chaining

### Requirement: Builder accepts namespace
The builder SHALL provide a `with_namespace(namespace: impl Into<String>)` method that sets the default Kubernetes namespace.

#### Scenario: Set namespace with string
- **WHEN** user calls `builder.with_namespace("default")`
- **THEN** builder stores "default" as namespace
- **AND** method returns builder for chaining

#### Scenario: Set namespace with String
- **WHEN** user calls `builder.with_namespace(String::from("production"))`
- **THEN** builder stores "production" as namespace
- **AND** method returns builder for chaining

### Requirement: Builder accepts dry run mode
The builder SHALL provide a `with_dry_run(dry_run: bool)` method that configures the client for test mode without executing operations.

#### Scenario: Enable dry run mode
- **WHEN** user calls `builder.with_dry_run(true)`
- **THEN** builder stores dry_run as true
- **AND** client will not execute actual Kubernetes operations
- **AND** method returns builder for chaining

#### Scenario: Disable dry run mode for production
- **WHEN** user calls `builder.with_dry_run(false)`
- **THEN** builder stores dry_run as false
- **AND** client will execute actual Kubernetes operations
- **AND** method returns builder for chaining

### Requirement: Builder accepts default timeout
The builder SHALL provide a `with_default_timeout(timeout: Duration)` method that sets the default timeout for operations.

#### Scenario: Set timeout duration
- **WHEN** user calls `builder.with_default_timeout(Duration::from_secs(300))`
- **THEN** builder stores 300 seconds as default timeout
- **AND** method returns builder for chaining

### Requirement: Builder accepts log level
The builder SHALL provide a `with_log_level(level: Level)` method that sets the logging verbosity.

#### Scenario: Set log level to Info
- **WHEN** user calls `builder.with_log_level(Level::INFO)`
- **THEN** builder stores Level::INFO as log level
- **AND** method returns builder for chaining

### Requirement: Builder accepts default resource limits
The builder SHALL provide a `with_default_resource_limits(limits: ResourceLimits)` method that sets default resource constraints for containers.

#### Scenario: Set resource limits
- **WHEN** user calls `builder.with_default_resource_limits(ResourceLimits::new(...))`
- **THEN** builder stores the provided resource limits
- **AND** method returns builder for chaining

### Requirement: Builder creates client with all configured options
The builder SHALL provide a `build()` method that constructs a `MaestroClient` with all configured options.

#### Scenario: Build client with all options
- **WHEN** user calls `builder.build()` after configuring all options
- **THEN** system returns a `MaestroClient` instance
- **AND** client contains all configured values
- **AND** client configuration is immutable

#### Scenario: Build client with minimal configuration
- **WHEN** user calls `builder.build()` with only dry_run configured
- **THEN** system returns a `MaestroClient` instance
- **AND** client uses sensible defaults for unconfigured options
- **AND** client configuration is immutable

### Requirement: Builder methods support fluent chaining
All builder configuration methods SHALL return `Self` to enable method chaining.

#### Scenario: Chain multiple builder methods
- **WHEN** user calls `builder.with_namespace("ns").with_dry_run(true).with_timeout(Duration::from_secs(60))`
- **THEN** each method returns the builder
- **AND** all configurations are applied
- **AND** final build() includes all chained configurations
