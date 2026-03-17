## Why

Multi-tenant Kubernetes environments require robust security isolation to prevent workloads from interfering with each other and to enforce resource boundaries. Currently, k8s-maestro lacks built-in security primitives, forcing users to manually construct NetworkPolicy, ResourceQuota, SecurityContext, RBAC, and LimitRange resources. This creates inconsistency, increases error-proneness, and makes it difficult to establish secure-by-default workflow execution patterns.

## What Changes

- **New security module** (`src/security/`) with builders for Kubernetes security resources
- **NetworkPolicyBuilder** for workflow network isolation with preset policies (deny_all, allow_all, allow_within_namespace)
- **ResourceQuotaBuilder** for namespace-level resource limits with preset quotas (small, medium, large workloads)
- **SecurityContextConfig** for pod and container security settings with preset contexts (restricted, baseline, privileged)
- **RBAC builders** (ServiceAccount, Role, RoleBinding, ClusterRole, ClusterRoleBinding) with preset roles (workflow-executor, workflow-viewer, admin)
- **LimitRangeBuilder** for enforcing min/max resource constraints per container/pod/PVC
- Comprehensive unit and integration tests with Kind cluster validation

## Capabilities

### New Capabilities

- `network-policy`: NetworkPolicy builders and presets for controlling ingress/egress traffic at pod level
- `resource-quota`: ResourceQuota builders and presets for namespace-level resource accounting and limits
- `security-context`: SecurityContext configurations for pod and container-level security settings
- `rbac-management`: ServiceAccount, Role, RoleBinding, ClusterRole, and ClusterRoleBinding builders with preset roles
- `limit-range`: LimitRange builders for enforcing default/min/max resource constraints

### Modified Capabilities

(None - this is a new module with no existing spec modifications)

## Impact

- **New module**: `src/security/` with 6 submodules (mod.rs, network_policy.rs, resource_quota.rs, security_context.rs, rbac.rs, limits.rs)
- **Exports**: New `security` module added to `src/lib.rs`
- **Dependencies**: Uses existing `k8s-openapi` and `kube` crates for K8s types
- **Testing**: Unit tests in each module, integration tests in `src/security/tests/` using Kind
- **No breaking changes**: Purely additive functionality
