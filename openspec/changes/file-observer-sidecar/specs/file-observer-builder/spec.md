## ADDED Requirements

### Requirement: Fluent builder API
The system SHALL provide a builder pattern for configuring FileObserverSidecar.

#### Scenario: Create new builder
- **WHEN** FileObserverBuilder::new() is called
- **THEN** the system SHALL return a builder with default configuration

#### Scenario: Set watch path
- **WHEN** watch_path(path) is called on builder
- **THEN** the builder SHALL store the path for directory watching

#### Scenario: Enable channel mode
- **WHEN** with_channel_mode(true) is called
- **THEN** the builder SHALL enable real-time event broadcasting

#### Scenario: Enable cache mode with config
- **WHEN** with_cache_mode(true, config) is called
- **THEN** the builder SHALL enable tiered caching with provided configuration

#### Scenario: Enable HTTP service with port
- **WHEN** with_http_service(true, port) is called
- **THEN** the builder SHALL enable HTTP service on specified port

#### Scenario: Set file filters
- **WHEN** with_filters(filters) is called
- **THEN** the builder SHALL apply file filtering configuration

### Requirement: Builder validation
The system SHALL validate builder configuration before construction.

#### Scenario: Missing watch path
- **WHEN** build() is called without watch_path
- **THEN** the system SHALL return an error indicating missing required field

#### Scenario: No modes enabled
- **WHEN** build() is called with all modes disabled
- **THEN** the system SHALL return an error indicating at least one mode required

#### Scenario: Valid configuration
- **WHEN** build() is called with valid configuration
- **THEN** the system SHALL return FileObserverSidecar instance

### Requirement: Default configurations
The system SHALL provide sensible defaults for optional configurations.

#### Scenario: Default memory cache config
- **WHEN** cache is enabled without custom config
- **THEN** memory tier SHALL default to 50MB, 100 files, 1h TTL

#### Scenario: Default disk cache config
- **WHEN** cache is enabled without custom config
- **THEN** disk tier SHALL default to 1GB, 1000 files, 24h TTL, LRU policy

#### Scenario: Default file filter
- **WHEN** no filters are specified
- **THEN** all files SHALL be observed with max size of 100MB

### Requirement: Observer modes configuration
The system SHALL support flexible mode combinations.

#### Scenario: Channel mode only
- **WHEN** only channel mode is enabled
- **THEN** events SHALL be broadcast but not cached

#### Scenario: Cache mode only
- **WHEN** only cache mode is enabled
- **THEN** files SHALL be cached but no events broadcast

#### Scenario: HTTP service only
- **WHEN** only HTTP service mode is enabled
- **THEN** files SHALL be accessible via HTTP but not cached or broadcast

#### Scenario: Multiple modes enabled
- **WHEN** multiple modes are enabled
- **THEN** all enabled modes SHALL function independently
