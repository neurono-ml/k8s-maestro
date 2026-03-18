## ADDED Requirements

### Requirement: SidecarContainer struct definition
The system SHALL provide a `SidecarContainer` struct with fields for name, image, config, ports, environment variables, volume mounts, and resource limits.

#### Scenario: Create sidecar container with all fields
- **WHEN** a user creates a `SidecarContainer` with name "log-collector", image "fluentd:v1.14", config containing collector settings, port 24224, environment variable "LOG_LEVEL=debug", volume mount "/var/log", and resource limits
- **THEN** the container SHALL have all specified fields populated and be valid for use in a workflow step

#### Scenario: Create sidecar container with minimal fields
- **WHEN** a user creates a `SidecarContainer` with only name and image
- **THEN** the container SHALL have empty collections for ports, env, volume_mounts, and None for resource_limits

### Requirement: SidecarBuilder fluent API
The system SHALL provide a `SidecarBuilder` with fluent methods for constructing `SidecarContainer` instances.

#### Scenario: Build sidecar using fluent API
- **WHEN** a user calls `SidecarBuilder::new("nginx:latest").with_name("proxy").with_port(8080).with_env("PROXY_MODE", "reverse").build()`
- **THEN** the builder SHALL return a valid `SidecarContainer` with all configured values

#### Scenario: Builder method chaining
- **WHEN** a user chains multiple configuration methods on `SidecarBuilder`
- **THEN** each method SHALL return `&mut Self` enabling fluent chaining

### Requirement: SidecarBuilder with_config method
The system SHALL provide `with_config(key: &str, value: serde_json::Value)` for setting plugin-specific configuration.

#### Scenario: Set JSON config value
- **WHEN** a user calls `.with_config("buffer_size", json!({"value": 1024, "unit": "MB"}))`
- **THEN** the config map SHALL contain the key "buffer_size" with the specified JSON value

### Requirement: SidecarBuilder with_volume_mount method
The system SHALL provide `with_volume_mount(mount: VolumeMount)` for attaching volumes to sidecars.

#### Scenario: Add volume mount
- **WHEN** a user calls `.with_volume_mount(VolumeMount::new("/data", "data-volume"))`
- **THEN** the sidecar SHALL include the volume mount in its volume_mounts collection

### Requirement: SidecarBuilder with_resource_limits method
The system SHALL provide `with_resource_limits(limits: ResourceLimits)` for setting CPU and memory constraints.

#### Scenario: Set resource limits
- **WHEN** a user calls `.with_resource_limits(ResourceLimits { cpu: "500m", memory: "256Mi" })`
- **THEN** the sidecar SHALL have the specified resource limits

### Requirement: SidecarBuilder build validation
The system SHALL validate that built sidecars have required fields (name and image).

#### Scenario: Build without required fields
- **WHEN** a user attempts to build without setting a name or image
- **THEN** the build method SHALL return an error indicating the missing required field
