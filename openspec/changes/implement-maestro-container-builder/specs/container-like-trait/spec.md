## ADDED Requirements

### Requirement: ContainerLike trait converts to k8s Container
The system SHALL provide a `ContainerLike` trait with `as_container(&self) -> k8s_openapi::api::core::v1::Container` method.

#### Scenario: Convert MaestroContainer to k8s Container
- **WHEN** user calls `container.as_container()` on a `MaestroContainer`
- **THEN** system returns a valid `k8s_openapi::api::core::v1::Container` with all configured fields

#### Scenario: Convert SidecarContainer to k8s Container
- **WHEN** user calls `sidecar.as_container()` on a `SidecarContainer`
- **THEN** system returns a valid `k8s_openapi::api::core::v1::Container` with all configured fields

### Requirement: ContainerLike provides name accessor
The system SHALL provide `name(&self) -> &str` method in `ContainerLike` trait.

#### Scenario: Get container name
- **WHEN** user calls `container.name()` on any `ContainerLike` implementor
- **THEN** system returns the container name as string slice

### Requirement: ContainerLike provides image accessor
The system SHALL provide `image(&self) -> &str` method in `ContainerLike` trait.

#### Scenario: Get container image
- **WHEN** user calls `container.image()` on any `ContainerLike` implementor
- **THEN** system returns the container image as string slice

### Requirement: ContainerLike enables polymorphic usage
The system SHALL allow using different container types through trait objects.

#### Scenario: Store containers as trait objects
- **WHEN** user creates `Vec<Box<dyn ContainerLike>>` with both `MaestroContainer` and `SidecarContainer`
- **THEN** system allows iteration and calling trait methods on all containers

### Requirement: VolumeMountLike trait converts to k8s VolumeMount
The system SHALL provide a `VolumeMountLike` trait with `as_volume_mount(&self) -> k8s_openapi::api::core::v1::VolumeMount` method.

#### Scenario: Convert VolumeMount to k8s VolumeMount
- **WHEN** user calls `mount.as_volume_mount()` on a type implementing `VolumeMountLike`
- **THEN** system returns a valid `k8s_openapi::api::core::v1::VolumeMount`
