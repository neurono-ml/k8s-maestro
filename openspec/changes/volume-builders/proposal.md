## Why

The project currently has example code referencing volume builders that don't exist. Workflows need persistent storage, configuration injection, and secret management to be useful in production scenarios. A fluent API for volume management will enable developers to easily mount PVCs, ConfigMaps, Secrets, EmptyDir, and HostPath volumes in their Kubernetes jobs.

## What Changes

- Create new `entities` module with `volumes` submodule
- Add volume builder types:
  - `MaestroPVCMountVolumeBuilder` - Persistent Volume Claim mounts
  - `ConfigMapVolumeBuilder` - ConfigMap volume mounts
  - `SecretVolumeBuilder` - Secret volume mounts
  - `EmptyDirVolumeBuilder` - Temporary empty directory volumes
  - `HostPathVolumeBuilder` - Host path volumes
- Add supporting types:
  - `VolumeType` enum for volume type identification
  - `VolumeItem` for key/path/mode configuration
  - `AccessMode` enum for PVC access modes
  - `Medium` enum for EmptyDir medium types
  - `HostPathType` enum for host path types
- Add traits:
  - `VolumeMountLike` trait for volume mount abstraction
  - `VolumeSourceLike` trait for volume source abstraction
- Comprehensive unit tests for all builders
- Integration tests with Kind cluster

## Capabilities

### New Capabilities

- `volume-builders`: Fluent API for creating and mounting Kubernetes volumes (PVC, ConfigMap, Secret, EmptyDir, HostPath)

### Modified Capabilities

None - this is a new feature.

## Impact

- New module: `src/entities/volumes/`
- Updates: `src/lib.rs` to export new entities module
- Dependencies: Uses existing `k8s-openapi` types for Kubernetes API compatibility
- Examples: Existing `examples/use_volumes.rs` will work with new implementation
