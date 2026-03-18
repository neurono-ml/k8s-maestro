## Context

The k8s-maestro crate provides a fluent API for creating and managing Kubernetes Jobs. Currently, users cannot expose workflow steps as network services without manually creating Kubernetes Service and Ingress resources. This design adds a networking module that follows the existing builder patterns used for jobs and containers.

## Goals / Non-Goals

**Goals:**
- Provide fluent builders for Service and Ingress resources
- Support all Kubernetes service types (ClusterIP, Headless, NodePort, LoadBalancer)
- Enable TLS configuration for Ingress
- Provide DNS utilities for service discovery
- Follow existing code patterns (MaestroContainer, JobBuilder)

**Non-Goals:**
- NetworkPolicy support (future enhancement)
- Gateway API support (future enhancement)
- Service mesh integration (future enhancement)
- Advanced traffic management (canary, blue-green)

## Decisions

### Module Structure
Create `src/networking/` with three submodules:
- `service.rs`: ServiceBuilder and related types
- `ingress.rs`: IngressBuilder and related types  
- `dns.rs`: DNS name generation utilities

**Rationale**: Separation of concerns, matches k8s-openapi structure, allows independent testing and usage.

### ServiceType Enum
Use a custom enum instead of raw strings:
```rust
pub enum ServiceType {
    ClusterIP,
    Headless,    // ClusterIP = "None"
    NodePort,
    LoadBalancer,
}
```

**Rationale**: Type safety, clearer API, automatic handling of headless services.

### Headless Service Handling
Headless services use `cluster_ip = "None"`. The builder will:
- Accept `with_cluster_ip("None")` directly
- Map `ServiceType::Headless` to `cluster_ip: "None"`

**Rationale**: Explicit API for both patterns, matches Kubernetes semantics.

### Ingress Path Type
Default to `Prefix` path type for simplicity, allow override:
```rust
pub struct IngressPath {
    pub path: String,
    pub path_type: PathType,
    pub service_name: String,
    pub service_port: i32,
}

pub enum PathType {
    Exact,
    Prefix,
    ImplementationSpecific,
}
```

**Rationale**: Matches k8s-openapi, type-safe, sensible default.

### DNS Utilities Pattern
Return formatted FQDN strings:
- `service_dns_name("my-service", "default")` → `"my-service.default.svc.cluster.local"`
- `pod_dns_name("my-pod", "default")` → `"my-pod.default.pod.cluster.local"`
- `headless_service_dns_pattern("my-service", "default")` → `"*.my-service.default.svc.cluster.local"`

**Rationale**: Standard Kubernetes DNS format, matches in-cluster DNS resolution.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Ingress API v1 vs v1beta1 | Use stable `networking.k8s.io/v1` Ingress API only |
| Missing required fields | Builder returns `Result<Service>` with validation errors |
| Breaking changes to k8s-openapi | Pin version, update in controlled manner |
| Complex ingress rules | Keep builder simple, allow raw annotations for advanced cases |
