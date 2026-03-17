## Context

k8s-maestro is a Kubernetes job orchestrator library that currently provides workflow management capabilities. The project needs a security module to enable secure multi-tenant workflow execution. This design covers the implementation of network policies, resource quotas, security contexts, RBAC management, and limit ranges using builder patterns consistent with the existing codebase style.

**Constraints:**
- Must use existing `k8s-openapi` and `kube` crate types for K8s resources
- Follow existing builder patterns (like `WorkflowBuilder`, `MaestroContainer`)
- All builders must produce valid K8s API objects
- Integration tests must use Kind clusters

## Goals / Non-Goals

**Goals:**
- Provide type-safe builders for all security-related K8s resources
- Offer preset configurations for common security scenarios
- Enable fluent API for chaining builder methods
- Support validation before resource creation
- Ensure all resources are namespace-scoped by default (except ClusterRole/ClusterRoleBinding)

**Non-Goals:**
- Runtime enforcement of security policies (delegated to K8s)
- Dynamic policy updates (resources are immutable after creation)
- Admission controller implementation
- Cross-namespace network policy management

## Decisions

### 1. Builder Pattern with Fluent API

**Decision:** Use fluent builder pattern returning `Self` for all setter methods.

**Rationale:** Consistent with existing `MaestroContainer` and `WorkflowBuilder` patterns. Enables method chaining and clear configuration.

**Alternatives considered:**
- Functional options pattern: More complex, less idiomatic in Rust
- Config struct: Less flexible, harder to extend

### 2. Preset Functions as Associated Functions

**Decision:** Implement presets as `pub fn preset_name() -> Self` associated functions on builders.

**Rationale:** Clear namespacing, discoverable via IDE, and returns mutable builder for further customization.

**Example:**
```rust
let policy = NetworkPolicyBuilder::deny_all("isolated", "production")?.build()?;
```

### 3. Error Handling with anyhow::Result

**Decision:** Use `anyhow::Result` for build methods and preset functions.

**Rationale:** Consistent with project's error handling strategy. Validation errors can provide context.

### 4. Separate Modules per Security Domain

**Decision:** Create separate files for each security concern (network_policy.rs, resource_quota.rs, etc.).

**Rationale:** Clean separation of concerns, easier to navigate, follows existing project structure.

### 5. Re-export K8s Types vs. Wrapper Types

**Decision:** Builders produce native `k8s_openapi` types directly (e.g., `NetworkPolicy`, `ResourceQuota`).

**Rationale:** No abstraction leak, users can use full K8s API capabilities, no maintenance burden of wrapper types.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| K8s API version changes may break presets | Pin k8s-openapi version, use feature flags for K8s versions |
| Complex RBAC rules may be error-prone | Provide preset roles for common use cases, validation in build() |
| NetworkPolicy behavior varies by CNI plugin | Document CNI requirements in presets, test with Kind (uses kindnet) |
| Security context presets may be too restrictive/lenient | Provide three tiers (restricted, baseline, privileged) |
