## Context

The k8s-maestro project is a Kubernetes job orchestrator that uses fluent builders for configuration. Examples reference volume builders (`MaestroPVCMountVolumeBuilder`, etc.) that don't exist yet. The project follows a pattern where entities have builders with fluent APIs and implement traits for polymorphism.

Current state:
- `MaestroContainer` uses fluent builder pattern
- `JobBuilder` follows same pattern
- Examples expect `entities::volumes` module with various volume builders
- No existing volume infrastructure

## Goals / Non-Goals

**Goals:**
- Create fluent API builders for all 5 volume types (PVC, ConfigMap, Secret, EmptyDir, HostPath)
- Follow existing project patterns (CamelCase types, snake_case functions, fluent builders)
- Implement `VolumeMountLike` trait for container integration
- Support all common Kubernetes volume options
- Provide comprehensive tests (unit + integration with Kind)

**Non-Goals:**
- CSI volume support (can be added later)
- Projected volumes (can be added later)
- Dynamic PVC creation (builders only create volume mounts, not PVCs themselves)
- Volume resizing operations

## Decisions

### Module Structure
**Decision:** Create `src/entities/volumes/` with separate files per volume type.

Rationale: Follows project pattern of organizing by domain. Each volume type has distinct configuration, making separate files cleaner than a monolithic module.

Files:
- `mod.rs` - exports and re-exports
- `traits.rs` - `VolumeMountLike` and `VolumeSourceLike` traits
- `types.rs` - shared enums and structs
- `pvc.rs` - PVC volume builder
- `configmap.rs` - ConfigMap volume builder
- `secret.rs` - Secret volume builder
- `emptydir.rs` - EmptyDir volume builder
- `hostpath.rs` - HostPath volume builder

### Trait Design
**Decision:** Use `VolumeMountLike` trait for container integration.

Rationale: Matches existing `ContainerLike` pattern. Allows `Box<dyn VolumeMountLike>` in containers, enabling polymorphic volume handling.

```rust
pub trait VolumeMountLike {
    fn volume_name(&self) -> &str;
    fn mount_path(&self) -> &str;
    fn read_only(&self) -> bool;
    fn as_volume_mount(&self) -> VolumeMount;
    fn as_volume_source(&self) -> VolumeSource;
}
```

### Builder Pattern
**Decision:** Each volume type has its own builder returning a concrete type.

Rationale: Type-specific configuration (e.g., `with_storage_class` only for PVC) is clearer than a generic builder with optional methods. Concrete types enable compile-time validation.

### K8s Types Integration
**Decision:** Builders produce types that convert to `k8s_openapi` types.

Rationale: Direct use of k8s-openapi types would be verbose. Our builders provide ergonomics while ensuring compatibility.

## Risks / Trade-offs

**Risk:** k8s-openapi version drift → Mitigation: Use feature flags already in Cargo.toml (k8s_v1_28 to k8s_v1_32)

**Risk:** Complex volume configurations may need many builder methods → Mitigation: Use sensible defaults, only expose common options

**Trade-off:** Separate files increases file count but improves maintainability and code navigation
