## Context

k8s-maestro is a Kubernetes job orchestrator that provides fluent builders for creating Kubernetes resources. Currently, the library supports building Jobs, Containers, and Volumes, but lacks support for ConfigMaps and Secrets - two fundamental Kubernetes resources for configuration management.

Workflows often need to:
- Inject configuration data into containers via ConfigMaps
- Mount sensitive data via Secrets
- Configure image pull credentials for private registries

This design introduces type-safe builders following the existing patterns in the codebase (e.g., `JobBuilder`, `MaestroContainer`).

## Goals / Non-Goals

**Goals:**
- Provide fluent builder APIs for ConfigMap, Secret, and ImagePullSecret resources
- Support all common ConfigMap and Secret fields (data, binaryData, stringData, labels, annotations, immutable)
- Provide helper functions for common operations (loading from files, creating TLS secrets, Docker auth)
- Maintain consistency with existing builder patterns in k8s-maestro
- Ensure type safety through `SecretType` enum
- Support conversion to native Kubernetes types (`k8s_openapi::api::core::v1::{ConfigMap, Secret}`)

**Non-Goals:**
- Client operations (create, update, delete) - these belong in a separate client module
- Watching or event handling for ConfigMaps/Secrets
- Validation beyond type safety (e.g., secret value validation)

## Decisions

### 1. Module Structure
**Decision**: Create `src/entities/config/` module with submodules for each builder type.

**Rationale**: Follows existing pattern where related entities are grouped (e.g., `entities::container`). The `config` module groups configuration-related resources.

**Structure**:
```
src/entities/config/
├── mod.rs           # Re-exports
├── configmap.rs     # ConfigMapBuilder + helpers
├── secret.rs        # SecretBuilder + SecretType + helpers
└── image_pull_secret.rs  # ImagePullSecretBuilder
```

### 2. Builder Pattern
**Decision**: Use fluent builder pattern with `&mut self` methods returning `&mut Self`.

**Rationale**: Consistent with existing `JobBuilder` and `MaestroContainer` patterns. Allows method chaining while supporting optional fields.

**Alternative**: Consuming builder (`self` → `Self`) was considered but rejected to match existing codebase style.

### 3. Error Handling
**Decision**: Use `anyhow::Result` for helper functions, return `Self` from builder methods.

**Rationale**: Builder methods don't fail (they just store data). Only `build()` and file-loading helpers can fail, using `anyhow::Result` per AGENTS.md guidelines.

### 4. SecretType Enum
**Decision**: Create `SecretType` enum covering common Kubernetes secret types.

**Rationale**: Type-safe representation prevents typos in secret type strings. Covers the most common types:
- `Opaque` (default)
- `ServiceAccountToken`
- `Dockercfg`
- `DockerConfigJson`
- `BasicAuth`
- `SshAuth`
- `Tls`
- `BootstrapToken`

### 5. ImagePullSecret Specialization
**Decision**: Create dedicated `ImagePullSecretBuilder` instead of using `SecretBuilder` directly.

**Rationale**: Image pull secrets have a specific structure (`.dockerconfigjson` key with base64-encoded auth). A dedicated builder provides better UX and ensures correct format.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Binary data handling complexity | Use `Vec<u8>` for binary data, convert to base64 internally |
| Large file loading memory impact | Document that file helpers load entire file into memory; users with large files should use streaming |
| Secret type string format | `SecretType` enum implements `Display` to produce correct Kubernetes type strings |
| Immutable field behavior | Builders support `with_immutable()`, but updates to immutable resources will fail at K8s level (documented) |
