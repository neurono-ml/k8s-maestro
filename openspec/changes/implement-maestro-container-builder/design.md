## Context

The k8s-maestro library needs a type-safe, fluent API for defining Kubernetes containers. Currently, users must directly construct `k8s_openapi::api::core::v1::Container` objects which is verbose and error-prone. This design introduces a new `entities/container` module with builder patterns and traits that abstract container configuration while maintaining full compatibility with Kubernetes API objects.

The design follows existing patterns in the codebase:
- Builder pattern similar to `WorkflowBuilder`
- Trait-based abstraction like `WorkFlowStep` traits
- Fluent API methods that return `Self`

## Goals / Non-Goals

**Goals:**
- Provide `MaestroContainer` and `SidecarContainer` builders with fluent APIs
- Define `ContainerLike` trait for polymorphic container handling
- Support all common container configurations: env vars, resources, volumes, ports
- Convert to `k8s_openapi::api::core::v1::Container` via trait method
- Enable type-safe configuration with compile-time validation

**Non-Goals:**
- Full coverage of all Kubernetes container fields (focus on workflow use cases)
- Container lifecycle management (handled by workflow steps)
- Validation of resource values (delegated to Kubernetes API)

## Decisions

### D1: Module Structure
**Decision**: Create `src/entities/container/` with separate files:
- `mod.rs` - module exports
- `container.rs` - `MaestroContainer` and `MaestroContainerBuilder`
- `sidecar.rs` - `SidecarContainer` and `SidecarContainerBuilder`
- `traits.rs` - `ContainerLike` and `VolumeMountLike` traits
- `types.rs` - supporting types (enums, structs)

**Alternatives considered**:
- Single file: Too large, hard to navigate
- Flat `entities/` module: Better organization with sub-module

### D2: Builder Pattern Implementation
**Decision**: Use consuming builder pattern where methods return `Self`

```rust
let container = MaestroContainerBuilder::new("nginx:latest", "web")
    .add_environment_variable("PORT", "8080")
    .set_resource_bounds(resource_bounds)
    .build();
```

**Alternatives considered**:
- `&mut self` builder: Less ergonomic, doesn't chain well
- Type-state builder: Overkill for this use case

### D3: Resource Bounds Type
**Decision**: Use `BTreeMap<ComputeResource, Quantity>` for flexibility

```rust
pub enum ComputeResource {
    Cpu,
    Memory,
    EphemeralStorage,
    Storage,
    Custom(String),
}
```

**Alternatives considered**:
- Dedicated struct with optional fields: Less extensible
- String keys: No type safety

### D4: Environment Variable Sources
**Decision**: Use enum for different source types

```rust
pub enum EnvironmentVariableSource {
    Value(String),
    FieldRef(FieldRef),
    ResourceFieldRef(ResourceFieldRef),
}
```

**Alternatives considered**:
- Multiple methods per source type: More API surface
- Trait objects: Unnecessary complexity

### D5: Separate MaestroContainer and SidecarContainer
**Decision**: Two distinct types implementing the same trait

**Rationale**: Semantic clarity - main containers vs sidecars have different roles in workflows, even if structurally similar.

**Alternatives considered**:
- Single type with `is_sidecar` flag: Loses type-level distinction
- Generic container: No semantic differentiation

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Builder API surface grows large | Keep methods focused, use `set_`/`add_` naming convention |
| Missing Kubernetes container fields | Focus on workflow use cases, extend as needed |
| Resource quantity validation | Defer to Kubernetes API, provide helpers for common formats |
| Trait object overhead | Use `Box<dyn ContainerLike>` only when needed |
