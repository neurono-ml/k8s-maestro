## Why

The current codebase lacks a dedicated container abstraction for defining Kubernetes container specifications. Workflow steps that need to run containers must manually construct `k8s_openapi::api::core::v1::Container` objects, leading to verbose, error-prone code and inconsistent container definitions. A fluent builder pattern with type-safe interfaces will simplify container creation and ensure consistency across all workflow steps.

## What Changes

- Create new `src/entities/container/` module with container abstractions
- Add `MaestroContainer` struct with builder pattern for main containers
- Add `SidecarContainer` struct with builder pattern for sidecar containers  
- Define `ContainerLike` trait for polymorphic container handling
- Define `VolumeMountLike` trait for volume mount abstractions
- Add supporting types: `ComputeResource`, `EnvironmentVariableSource`, `EnvironmentVariableFromObject`, `ContainerPort`, `VolumeMount`
- Support environment variables (direct values, field refs, configmaps, secrets)
- Support resource limits and requests
- Support volume mounts with sub-paths and read-only options
- Support container ports configuration
- Add comprehensive unit and integration tests

## Capabilities

### New Capabilities
- `container-builder`: Fluent builder pattern for creating Kubernetes containers with MaestroContainer and SidecarContainer
- `container-like-trait`: Trait abstraction for polymorphic container handling
- `container-env-config`: Environment variable configuration from multiple sources (values, field refs, configmaps, secrets)
- `container-resources`: Resource limits and requests configuration for containers
- `container-volume-mounts`: Volume mount configuration with sub-path and read-only support

### Modified Capabilities

None - this is foundational infrastructure with no existing specs to modify.

## Impact

- **New module**: `src/entities/container/` with full container abstraction layer
- **Exports**: Add `entities` module to `lib.rs` with container types
- **Dependencies**: Uses existing `k8s_openapi` and `kube` crates
- **Testing**: Unit tests in module, integration tests with Kind cluster
- **Future**: This becomes the foundation for all container-based workflow steps
