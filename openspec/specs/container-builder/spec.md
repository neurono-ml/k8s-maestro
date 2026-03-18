# container-builder Specification

## Purpose
TBD - created by archiving change implement-maestro-container-builder. Update Purpose after archive.
## Requirements
### Requirement: MaestroContainer builder creates containers
The system SHALL provide a `MaestroContainerBuilder` that constructs `MaestroContainer` instances with a fluent API.

#### Scenario: Create container with required fields
- **WHEN** user creates `MaestroContainerBuilder::new("nginx:latest", "web")` and calls `build()`
- **THEN** system returns a `MaestroContainer` with image "nginx:latest" and name "web"

#### Scenario: Set container arguments
- **WHEN** user calls `set_arguments(&vec!["--port".to_string(), "8080".to_string()])` on builder
- **THEN** resulting container has args set to `["--port", "8080"]`

#### Scenario: Add container arguments incrementally
- **WHEN** user calls `add_arguments(&["--verbose", "--debug"])` on builder
- **THEN** resulting container has args appended to existing arguments

### Requirement: MaestroContainer builder validates required fields
The system SHALL require image and name to be provided before building.

#### Scenario: Build with missing name
- **WHEN** user attempts to build without providing name
- **THEN** system SHALL NOT compile (enforced at type level via `new()` parameters)

### Requirement: SidecarContainer builder creates sidecars
The system SHALL provide a `SidecarContainerBuilder` that constructs `SidecarContainer` instances with identical API to `MaestroContainerBuilder`.

#### Scenario: Create sidecar container
- **WHEN** user creates `SidecarContainerBuilder::new("busybox:latest", "log-collector")` and calls `build()`
- **THEN** system returns a `SidecarContainer` with image "busybox:latest" and name "log-collector"

### Requirement: Builder supports working directory
The system SHALL allow setting the container working directory.

#### Scenario: Set working directory
- **WHEN** user calls `set_working_dir("/app")` on builder
- **THEN** resulting container has `working_dir` set to "/app"

### Requirement: Builder supports command override
The system SHALL allow setting the container command (entrypoint).

#### Scenario: Set command
- **WHEN** user calls `set_command(&["/bin/sh".to_string(), "-c".to_string()])` on builder
- **THEN** resulting container has command set to `["/bin/sh", "-c"]`

