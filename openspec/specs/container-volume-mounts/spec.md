# container-volume-mounts Specification

## Purpose
TBD - created by archiving change implement-maestro-container-builder. Update Purpose after archive.
## Requirements
### Requirement: Add volume mount to container
The system SHALL allow adding volume mounts via `add_volume_mount(volume: impl VolumeMountLike)`.

#### Scenario: Add volume mount
- **WHEN** user calls `add_volume_mount(&VolumeMount { name: "data", mount_path: "/data", sub_path: None, read_only: false })`
- **THEN** resulting container has volume mount with name "data" at path "/data"

### Requirement: VolumeMount supports sub-path
The system SHALL support sub-path configuration in volume mounts.

#### Scenario: Set volume mount sub-path
- **WHEN** user creates `VolumeMount { name: "config", mount_path: "/etc/config", sub_path: Some("app.conf".to_string()), read_only: true }`
- **THEN** resulting k8s VolumeMount has `sub_path` set to "app.conf"

### Requirement: VolumeMount supports read-only flag
The system SHALL support read-only configuration in volume mounts.

#### Scenario: Set read-only volume mount
- **WHEN** user creates `VolumeMount` with `read_only: true`
- **THEN** resulting k8s VolumeMount has `read_only` set to true

### Requirement: ContainerPort configuration
The system SHALL support container port configuration.

#### Scenario: Set container ports
- **WHEN** user calls `set_ports(vec![ContainerPort { container_port: 8080, host_port: None, protocol: "TCP".to_string(), name: "http".to_string() }])`
- **THEN** resulting container has port 8080 with protocol TCP and name "http"

### Requirement: ContainerPort with host port
The system SHALL support host port configuration.

#### Scenario: Set container port with host port
- **WHEN** user creates `ContainerPort` with `host_port: Some(8080)`
- **THEN** resulting k8s ContainerPort has `host_port` set to 8080

### Requirement: Multiple volume mounts
The system SHALL support multiple volume mounts on a single container.

#### Scenario: Add multiple volume mounts
- **WHEN** user calls `add_volume_mount` multiple times with different mounts
- **THEN** resulting container has all volume mounts in the `volume_mounts` list

