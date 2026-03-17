## ADDED Requirements

### Requirement: File event detection
The system SHALL detect file creation, modification, and deletion events in watched directories.

#### Scenario: File created event
- **WHEN** a new file is created in the watched directory
- **THEN** the system SHALL emit a `FileEvent::Created` event with filename, path, content, MIME type, size, and timestamp

#### Scenario: File modified event
- **WHEN** an existing file is modified in the watched directory
- **THEN** the system SHALL emit a `FileEvent::Modified` event with filename, path, size, and modification timestamp

#### Scenario: File deleted event
- **WHEN** a file is deleted from the watched directory
- **THEN** the system SHALL emit a `FileEvent::Deleted` event with filename, path, and deletion timestamp

### Requirement: File metadata tracking
The system SHALL maintain metadata for all observed files.

#### Scenario: Metadata includes file properties
- **WHEN** a file is observed
- **THEN** the system SHALL track filename, path, MIME type, size, created_at, and modified_at

### Requirement: File content capture
The system SHALL capture file content for created files.

#### Scenario: Content captured on creation
- **WHEN** a file is created
- **THEN** the system SHALL read and store the file content as bytes along with metadata

### Requirement: Channel-based event distribution
The system SHALL distribute file events via broadcast channels when channel mode is enabled.

#### Scenario: Multiple subscribers receive events
- **WHEN** multiple subscribers are listening to the event channel
- **THEN** all subscribers SHALL receive each file event

#### Scenario: Slow consumer handling
- **WHEN** a subscriber cannot keep up with event rate
- **THEN** the system SHALL drop oldest events for that subscriber and continue

### Requirement: File filtering
The system SHALL filter files based on configured patterns and size limits.

#### Scenario: Include pattern matching
- **WHEN** include patterns are configured
- **THEN** only files matching at least one pattern SHALL be observed

#### Scenario: Exclude pattern matching
- **WHEN** exclude patterns are configured
- **THEN** files matching any exclude pattern SHALL NOT be observed

#### Scenario: Max file size enforcement
- **WHEN** max_file_size_mb is configured
- **THEN** files exceeding the size limit SHALL NOT be observed and SHALL log a warning
