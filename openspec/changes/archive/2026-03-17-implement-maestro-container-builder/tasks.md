## 1. Setup and Module Structure

- [ ] 1.1 Create `src/entities/container/` directory structure
- [ ] 1.2 Create `src/entities/container/mod.rs` with module exports
- [ ] 1.3 Create `src/entities/mod.rs` to export container module
- [ ] 1.4 Update `src/lib.rs` to export entities module

## 2. Types and Traits

- [ ] 2.1 Create `src/entities/container/types.rs` with `ComputeResource` enum (Cpu, Memory, EphemeralStorage, Storage, Custom)
- [ ] 2.2 Add `FieldRef` and `ResourceFieldRef` structs for env var sources
- [ ] 2.3 Add `EnvironmentVariableSource` enum (Value, FieldRef, ResourceFieldRef)
- [ ] 2.4 Add `EnvironmentVariableFromObject` enum (ConfigMap, Secret)
- [ ] 2.5 Add `ContainerPort` struct with container_port, host_port, protocol, name fields
- [ ] 2.6 Add `VolumeMount` struct with name, mount_path, sub_path, read_only fields
- [ ] 2.7 Add `ResourceLimits` struct with cpu, memory, cpu_request, memory_request, ephemeral_storage fields
- [ ] 2.8 Create `src/entities/container/traits.rs` with `ContainerLike` trait (as_container, name, image methods)
- [ ] 2.9 Add `VolumeMountLike` trait to traits.rs (as_volume_mount method)

## 3. MaestroContainer Implementation

- [ ] 3.1 Create `src/entities/container/container.rs` with `MaestroContainer` struct
- [ ] 3.2 Add all fields to `MaestroContainer`: image, name, args, env, env_from, ports, volume_mounts, resource_limits, working_dir, command
- [ ] 3.3 Create `MaestroContainerBuilder` struct with same fields as Option types
- [ ] 3.4 Implement `MaestroContainerBuilder::new(image, name)` constructor
- [ ] 3.5 Implement `set_arguments(args: &[String])` method
- [ ] 3.6 Implement `add_arguments(args: &[&str])` method
- [ ] 3.7 Implement `set_environment_variables(env: BTreeMap<String, EnvironmentVariableSource>)` method
- [ ] 3.8 Implement `add_environment_variable(key, value)` method
- [ ] 3.9 Implement `set_environment_variables_from_objects(env_from: &[EnvironmentVariableFromObject])` method
- [ ] 3.10 Implement `add_environment_variables_from_object(env_from: &EnvironmentVariableFromObject)` method
- [ ] 3.11 Implement `set_resource_bounds(bounds: BTreeMap<ComputeResource, Quantity>)` method
- [ ] 3.12 Implement `add_volume_mount(volume: impl VolumeMountLike)` method
- [ ] 3.13 Implement `set_working_dir(dir: &str)` method
- [ ] 3.14 Implement `set_command(cmd: &[String])` method
- [ ] 3.15 Implement `set_ports(ports: Vec<ContainerPort>)` method
- [ ] 3.16 Implement `build()` method to construct `MaestroContainer`
- [ ] 3.17 Implement `ContainerLike` trait for `MaestroContainer`
- [ ] 3.18 Implement `as_container()` to convert to `k8s_openapi::api::core::v1::Container`

## 4. SidecarContainer Implementation

- [ ] 4.1 Create `src/entities/container/sidecar.rs` with `SidecarContainer` struct (same fields as MaestroContainer)
- [ ] 4.2 Create `SidecarContainerBuilder` with identical API to `MaestroContainerBuilder`
- [ ] 4.3 Implement all builder methods for `SidecarContainerBuilder` (same as MaestroContainerBuilder)
- [ ] 4.4 Implement `ContainerLike` trait for `SidecarContainer`

## 5. VolumeMountLike Implementation

- [ ] 5.1 Implement `VolumeMountLike` trait for `VolumeMount` struct
- [ ] 5.2 Implement `as_volume_mount()` to convert to `k8s_openapi::api::core::v1::VolumeMount`

## 6. Unit Tests

- [ ] 6.1 Add unit tests for `MaestroContainerBuilder` basic creation with image and name
- [ ] 6.2 Add unit tests for `set_arguments` and `add_arguments` methods
- [ ] 6.3 Add unit tests for environment variable handling (set, add, Value source)
- [ ] 6.4 Add unit tests for FieldRef and ResourceFieldRef env sources
- [ ] 6.5 Add unit tests for env_from ConfigMap and Secret sources
- [ ] 6.6 Add unit tests for resource bounds configuration
- [ ] 6.7 Add unit tests for `ComputeResource` enum variants
- [ ] 6.8 Add unit tests for volume mount configuration
- [ ] 6.9 Add unit tests for sub_path and read_only in volume mounts
- [ ] 6.10 Add unit tests for container port configuration
- [ ] 6.11 Add unit tests for working_dir and command configuration
- [ ] 6.12 Add unit tests for `ContainerLike::as_container()` conversion
- [ ] 6.13 Add unit tests for `SidecarContainerBuilder` basic functionality
- [ ] 6.14 Add unit tests for polymorphic usage with `Box<dyn ContainerLike>`

## 7. Integration Tests

- [ ] 7.1 Create integration test file `tests/container_integration_test.rs`
- [ ] 7.2 Add test: Create pod with MaestroContainer and verify container spec
- [ ] 7.3 Add test: Verify environment variables are set correctly in running pod
- [ ] 7.4 Add test: Verify resource limits are applied correctly
- [ ] 7.5 Add test: Verify volume mounts work with Kind cluster
- [ ] 7.6 Add test: Verify container with sidecar configuration
