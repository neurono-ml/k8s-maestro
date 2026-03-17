## 1. Module Structure and Types

- [ ] 1.1 Create `src/entities/` directory and `mod.rs` with exports
- [ ] 1.2 Create `src/entities/volumes/mod.rs` with module exports
- [ ] 1.3 Create `src/entities/volumes/types.rs` with VolumeType, VolumeItem, AccessMode, Medium, HostPathType enums
- [ ] 1.4 Create `src/entities/volumes/traits.rs` with VolumeMountLike and VolumeSourceLike traits
- [ ] 1.5 Update `src/lib.rs` to export entities module

## 2. PVC Volume Builder

- [ ] 2.1 Create `src/entities/volumes/pvc.rs` with PVCVolume struct
- [ ] 2.2 Implement MaestroPVCMountVolumeBuilder with new(), with_storage_class(), with_access_modes(), with_storage_size(), with_read_only(), build()
- [ ] 2.3 Implement VolumeMountLike trait for PVCVolume
- [ ] 2.4 Add unit tests for PVCVolume builder and conversions

## 3. ConfigMap Volume Builder

- [ ] 3.1 Create `src/entities/volumes/configmap.rs` with ConfigMapVolume struct
- [ ] 3.2 Implement ConfigMapVolumeBuilder with new(), with_items(), with_default_mode(), with_optional(), build()
- [ ] 3.3 Implement VolumeMountLike trait for ConfigMapVolume
- [ ] 3.4 Add unit tests for ConfigMapVolume builder and conversions

## 4. Secret Volume Builder

- [ ] 4.1 Create `src/entities/volumes/secret.rs` with SecretVolume struct
- [ ] 4.2 Implement SecretVolumeBuilder with new(), with_items(), with_default_mode(), with_optional(), build()
- [ ] 4.3 Implement VolumeMountLike trait for SecretVolume
- [ ] 4.4 Add unit tests for SecretVolume builder and conversions

## 5. EmptyDir Volume Builder

- [ ] 5.1 Create `src/entities/volumes/emptydir.rs` with EmptyDirVolume struct
- [ ] 5.2 Implement EmptyDirVolumeBuilder with new(), with_medium(), with_size_limit(), build()
- [ ] 5.3 Implement VolumeMountLike trait for EmptyDirVolume
- [ ] 5.4 Add unit tests for EmptyDirVolume builder and conversions

## 6. HostPath Volume Builder

- [ ] 6.1 Create `src/entities/volumes/hostpath.rs` with HostPathVolume struct
- [ ] 6.2 Implement HostPathVolumeBuilder with new(), with_type(), build()
- [ ] 6.3 Implement VolumeMountLike trait for HostPathVolume
- [ ] 6.4 Add unit tests for HostPathVolume builder and conversions

## 7. Integration Tests

- [ ] 7.1 Create integration test fixtures in `crates/k8s-maestro-k8s/src/kubernetes/tests/fixtures/`
- [ ] 7.2 Add PVC integration test with Kind cluster
- [ ] 7.3 Add ConfigMap integration test with Kind cluster
- [ ] 7.4 Add Secret integration test with Kind cluster
- [ ] 7.5 Add EmptyDir integration test with Kind cluster

## 8. Verification

- [ ] 8.1 Run `cargo clippy` and fix all warnings
- [ ] 8.2 Run `cargo fmt --check` and fix formatting
- [ ] 8.3 Run `cargo test` and ensure all tests pass
- [ ] 8.4 Verify examples/use_volumes.rs compiles and works
