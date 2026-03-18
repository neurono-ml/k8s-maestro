## ADDED Requirements

### Requirement: PVC Volume Builder
The system SHALL provide a `MaestroPVCMountVolumeBuilder` that creates PersistentVolumeClaim volume mounts with fluent API.

#### Scenario: Basic PVC mount creation
- **WHEN** user creates `MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")` and calls `build()`
- **THEN** system returns a `PVCVolume` with mount path "/data", PVC name "my-pvc", and volume name "data-volume"

#### Scenario: PVC with storage class
- **WHEN** user calls `.with_storage_class("fast-ssd")` on builder
- **THEN** resulting volume includes storage class "fast-ssd"

#### Scenario: PVC with access modes
- **WHEN** user calls `.with_access_modes(vec![AccessMode::ReadWriteOnce])` on builder
- **THEN** resulting volume includes specified access mode

#### Scenario: PVC with read-only flag
- **WHEN** user calls `.with_read_only(true)` on builder
- **THEN** resulting volume mount is marked as read-only

### Requirement: ConfigMap Volume Builder
The system SHALL provide a `ConfigMapVolumeBuilder` that creates ConfigMap volume mounts.

#### Scenario: Basic ConfigMap mount
- **WHEN** user creates `ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")` and calls `build()`
- **THEN** system returns a `ConfigMapVolume` mounting ConfigMap "app-config" at "/config"

#### Scenario: ConfigMap with specific items
- **WHEN** user calls `.with_items(vec![VolumeItem::new("config.yaml", "config.yaml")])`
- **THEN** only specified keys are mounted

#### Scenario: ConfigMap with default mode
- **WHEN** user calls `.with_default_mode(0o644)` on builder
- **THEN** all files in volume have mode 0o644

#### Scenario: ConfigMap as optional
- **WHEN** user calls `.with_optional(true)` on builder
- **THEN** volume mount succeeds even if ConfigMap doesn't exist

### Requirement: Secret Volume Builder
The system SHALL provide a `SecretVolumeBuilder` that creates Secret volume mounts.

#### Scenario: Basic Secret mount
- **WHEN** user creates `SecretVolumeBuilder::new("/secrets", "db-credentials", "secret-vol")` and calls `build()`
- **THEN** system returns a `SecretVolume` mounting Secret "db-credentials" at "/secrets"

#### Scenario: Secret with specific items
- **WHEN** user calls `.with_items(vec![VolumeItem::new("password", "db-password")])`
- **THEN** only specified keys are mounted

#### Scenario: Secret with default mode
- **WHEN** user calls `.with_default_mode(0o400)` on builder
- **THEN** all files in volume have restrictive mode 0o400

### Requirement: EmptyDir Volume Builder
The system SHALL provide an `EmptyDirVolumeBuilder` that creates temporary empty directory volumes.

#### Scenario: Basic EmptyDir mount
- **WHEN** user creates `EmptyDirVolumeBuilder::new("/tmp", "temp-vol")` and calls `build()`
- **THEN** system returns an `EmptyDirVolume` with temporary storage at "/tmp"

#### Scenario: EmptyDir with memory medium
- **WHEN** user calls `.with_medium(Medium::Memory)` on builder
- **THEN** volume is backed by memory (tmpfs)

#### Scenario: EmptyDir with size limit
- **WHEN** user calls `.with_size_limit("1Gi")` on builder
- **THEN** volume has maximum size of 1Gi

### Requirement: HostPath Volume Builder
The system SHALL provide a `HostPathVolumeBuilder` that creates host path volume mounts.

#### Scenario: Basic HostPath mount
- **WHEN** user creates `HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")` and calls `build()`
- **THEN** system returns a `HostPathVolume` mounting host path "/var/data" at "/host-data"

#### Scenario: HostPath with type
- **WHEN** user calls `.with_type(HostPathType::Directory)` on builder
- **THEN** volume is configured for directory type with validation

### Requirement: VolumeMountLike Trait
The system SHALL provide a `VolumeMountLike` trait for volume abstraction.

#### Scenario: Convert to VolumeMount
- **WHEN** volume implements `VolumeMountLike` and `as_volume_mount()` is called
- **THEN** system returns a `k8s_openapi::api::core::v1::VolumeMount`

#### Scenario: Convert to VolumeSource
- **WHEN** volume implements `VolumeMountLike` and `as_volume_source()` is called
- **THEN** system returns a `k8s_openapi::api::core::v1::VolumeSource`

### Requirement: Volume Types
The system SHALL provide supporting types for volume configuration.

#### Scenario: AccessMode enum values
- **WHEN** user uses `AccessMode` enum
- **THEN** available values are: `ReadWriteOnce`, `ReadOnlyMany`, `ReadWriteMany`, `ReadWriteOncePod`

#### Scenario: Medium enum values
- **WHEN** user uses `Medium` enum
- **THEN** available values are: `Default`, `Memory`

#### Scenario: HostPathType enum values
- **WHEN** user uses `HostPathType` enum
- **THEN** available values include: `Directory`, `File`, `Socket`, `BlockDevice`, `CharDevice`, `DirectoryOrCreate`, `FileOrCreate`

#### Scenario: VolumeItem creation
- **WHEN** user creates `VolumeItem::new("key", "path")`
- **THEN** system returns item with key "key", path "path", and optional mode

### Requirement: Unit Tests
The system SHALL have comprehensive unit tests for all volume builders.

#### Scenario: Test all builders
- **WHEN** running `cargo test`
- **THEN** all volume builder tests pass with coverage of builder methods and edge cases

### Requirement: Integration Tests
The system SHALL have integration tests with Kind cluster for volume operations.

#### Scenario: PVC integration test
- **WHEN** integration test creates PVC and mounts in pod
- **THEN** pod can read/write to mounted volume

#### Scenario: ConfigMap integration test
- **WHEN** integration test creates ConfigMap and mounts in pod
- **THEN** pod can read mounted configuration files

#### Scenario: Secret integration test
- **WHEN** integration test creates Secret and mounts in pod
- **THEN** pod can read mounted secret data

#### Scenario: EmptyDir integration test
- **WHEN** integration test uses EmptyDir volume in pod
- **THEN** pod can read/write to temporary volume
