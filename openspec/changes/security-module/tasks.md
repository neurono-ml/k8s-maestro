## 1. Module Structure and Exports

- [ ] 1.1 Create `src/security/mod.rs` with module declarations and public exports
- [ ] 1.2 Add `pub mod security;` to `src/lib.rs`
- [ ] 1.3 Add `thiserror` dependency to Cargo.toml for library error types

## 2. Network Policy Implementation

- [ ] 2.1 Create `src/security/network_policy.rs` with imports and error types
- [ ] 2.2 Implement `PolicyType` enum (Ingress, Egress, Both)
- [ ] 2.3 Implement `NetworkPolicyRule` struct with from/to selectors and ports
- [ ] 2.4 Implement `NetworkPolicyBuilder` with new(), with_pod_selector(), with_ingress_rule(), with_egress_rule(), with_policy_types()
- [ ] 2.5 Implement build() method returning Result<NetworkPolicy>
- [ ] 2.6 Implement preset: `NetworkPolicyBuilder::deny_all(name, namespace)`
- [ ] 2.7 Implement preset: `NetworkPolicyBuilder::allow_all(name, namespace)`
- [ ] 2.8 Implement preset: `NetworkPolicyBuilder::allow_within_namespace(name, namespace)`
- [ ] 2.9 Add unit tests for NetworkPolicyBuilder
- [ ] 2.10 Add unit tests for preset policies

## 3. Resource Quota Implementation

- [ ] 3.1 Create `src/security/resource_quota.rs` with imports and error types
- [ ] 3.2 Implement `QuotaScope` enum (Terminating, NotTerminating, BestEffort, NotBestEffort)
- [ ] 3.3 Implement `ResourceQuotaBuilder` with new(), with_hard_limits(), with_scopes()
- [ ] 3.4 Implement build() method returning Result<ResourceQuota>
- [ ] 3.5 Implement preset: `ResourceQuotaBuilder::small_workload(name, namespace)`
- [ ] 3.6 Implement preset: `ResourceQuotaBuilder::medium_workload(name, namespace)`
- [ ] 3.7 Implement preset: `ResourceQuotaBuilder::large_workload(name, namespace)`
- [ ] 3.8 Add unit tests for ResourceQuotaBuilder
- [ ] 3.9 Add unit tests for preset quotas

## 4. Security Context Implementation

- [ ] 4.1 Create `src/security/security_context.rs` with imports
- [ ] 4.2 Implement `SecurityContextConfig` struct with all fields (run_as_user, run_as_group, etc.)
- [ ] 4.3 Implement `PodSecurityContext` with builder methods and conversion to K8s type
- [ ] 4.4 Implement `ContainerSecurityContext` with builder methods and conversion to K8s type
- [ ] 4.5 Implement capabilities handling (add/drop capabilities)
- [ ] 4.6 Implement seccomp profile configuration
- [ ] 4.7 Implement preset: `SecurityContextConfig::restricted()`
- [ ] 4.8 Implement preset: `SecurityContextConfig::baseline()`
- [ ] 4.9 Implement preset: `SecurityContextConfig::privileged()`
- [ ] 4.10 Add unit tests for security context builders
- [ ] 4.11 Add unit tests for preset contexts

## 5. RBAC Management Implementation

- [ ] 5.1 Create `src/security/rbac.rs` with imports and error types
- [ ] 5.2 Implement `PolicyRule` struct with api_groups, resources, verbs, resource_names
- [ ] 5.3 Implement `ServiceAccountBuilder` with new(), with_annotations(), build()
- [ ] 5.4 Implement `RoleBuilder` with new(), with_rules(), build()
- [ ] 5.5 Implement `RoleBindingBuilder` with new(), with_subject(), with_role_ref(), build()
- [ ] 5.6 Implement `ClusterRoleBuilder` with new(), with_rules(), build()
- [ ] 5.7 Implement `ClusterRoleBindingBuilder` with new(), with_subject(), with_role_ref(), build()
- [ ] 5.8 Implement preset: `RoleBuilder::workflow_executor(name, namespace)`
- [ ] 5.9 Implement preset: `RoleBuilder::workflow_viewer(name, namespace)`
- [ ] 5.10 Implement preset: `RoleBuilder::admin(name, namespace)`
- [ ] 5.11 Add unit tests for all RBAC builders
- [ ] 5.12 Add unit tests for preset roles

## 6. Limit Range Implementation

- [ ] 6.1 Create `src/security/limits.rs` with imports and error types
- [ ] 6.2 Implement limit range type enum (Container, Pod, PersistentVolumeClaim)
- [ ] 6.3 Implement `LimitRangeBuilder` with new(), with_default_limit(), with_max_limit(), build()
- [ ] 6.4 Implement support for min, max, default, default_request constraints
- [ ] 6.5 Add unit tests for LimitRangeBuilder
- [ ] 6.6 Add unit tests for different limit types

## 7. Integration Tests with Kind

- [ ] 7.1 Create `src/security/tests/` directory structure
- [ ] 7.2 Create Kind cluster fixture for security tests
- [ ] 7.3 Add integration test: Apply network policy and verify isolation
- [ ] 7.4 Add integration test: Apply resource quota and verify enforcement
- [ ] 7.5 Add integration test: Apply security context and verify restrictions
- [ ] 7.6 Add integration test: Apply RBAC and verify permissions

## 8. Documentation and Finalization

- [ ] 8.1 Update CHANGELOG.md with security module additions
- [ ] 8.2 Add inline documentation (rustdoc) for all public APIs
- [ ] 8.3 Run cargo clippy and fix all warnings
- [ ] 8.4 Run cargo fmt --check and ensure formatting compliance
- [ ] 8.5 Run cargo test --verbose and ensure all tests pass
