## ADDED Requirements

### Requirement: HTTP file retrieval
The system SHALL provide HTTP endpoints for file access when HTTP mode is enabled.

#### Scenario: Get file content
- **WHEN** GET /files/{path} is requested
- **THEN** the system SHALL return file content with appropriate Content-Type header

#### Scenario: File not found
- **WHEN** GET /files/{path} is requested for non-existent file
- **THEN** the system SHALL return 404 Not Found

### Requirement: HTTP file listing
The system SHALL provide an endpoint to list all cached files.

#### Scenario: List all files
- **WHEN** GET /files is requested
- **THEN** the system SHALL return JSON array of FileMetadata objects

#### Scenario: Empty cache listing
- **WHEN** GET /files is requested and cache is empty
- **THEN** the system SHALL return empty JSON array

### Requirement: HTTP metadata access
The system SHALL provide an endpoint for file metadata.

#### Scenario: Get file metadata
- **WHEN** GET /files/{path}/metadata is requested
- **THEN** the system SHALL return JSON FileMetadata object

#### Scenario: Metadata for non-existent file
- **WHEN** GET /files/{path}/metadata is requested for non-existent file
- **THEN** the system SHALL return 404 Not Found

### Requirement: HTTP existence check
The system SHALL provide HEAD endpoint for file existence.

#### Scenario: Check file exists
- **WHEN** HEAD /files/{path} is requested for existing file
- **THEN** the system SHALL return 200 OK with Content-Length header

#### Scenario: Check file not exists
- **WHEN** HEAD /files/{path} is requested for non-existent file
- **THEN** the system SHALL return 404 Not Found

### Requirement: HTTP service configuration
The system SHALL support configurable HTTP service settings.

#### Scenario: Configurable port
- **WHEN** HTTP service is started with custom port
- **THEN** the service SHALL listen on the specified port

#### Scenario: Service binds to all interfaces
- **WHEN** HTTP service starts
- **THEN** it SHALL bind to 0.0.0.0 for pod accessibility

### Requirement: HTTP service lifecycle
The system SHALL manage HTTP service lifecycle with the cache.

#### Scenario: Service shares cache instance
- **WHEN** HTTP service is started
- **THEN** it SHALL use the provided Arc<TieredCache> reference

#### Scenario: Graceful shutdown
- **WHEN** shutdown signal is received
- **THEN** the HTTP service SHALL complete in-flight requests before stopping
