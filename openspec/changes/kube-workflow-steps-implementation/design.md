## Context

The k8s-maestro library provides a trait-based workflow orchestration system with interfaces for `WorkFlowStep`, `KubeWorkFlowStep`, `WaitableWorkFlowStep`, `DeletableWorkFlowStep`, `LoggableWorkFlowStep`, and `ServableWorkFlowStep`. However, there are no concrete implementations for managing Kubernetes resources.

Current state:
- Traits are defined but only have mock implementations for testing
- Examples reference `k8s_maestro::clients` and `k8s_maestro::entities` which don't exist yet
- No real K8s client integration
- No builder patterns for creating jobs or pods
- No resource lifecycle management

Constraints:
- Must use existing trait interfaces (WorkFlowStep, KubeWorkFlowStep, etc.)
- Must integrate with k8s-openapi and kube crates (already in dependencies)
- Must follow the existing code style (4 spaces, 100 char lines, specific imports)
- Must provide comprehensive testing with Kind
- Entity types (ContainerLike, SidecarContainer, ResourceLimits) need to be available

## Goals / Non-Goals

**Goals:**
- Implement KubeJobStep with full K8s Job lifecycle management
- Implement KubePodStep with full K8s Pod lifecycle management
- Provide fluent builder APIs for both step types
- Implement ServiceConfig and IngressConfig for service/ingress management
- Implement JobNameType and RestartPolicy enums
- Provide comprehensive unit and integration tests
- Support watching job status until completion
- Support streaming logs from pods
- Support deleting jobs/pods and associated resources
- Support exposing services and ingress for workflow steps

**Non-Goals:**
- Creating entity types (ContainerLike, SidecarContainer, ResourceLimits) - assume they exist or will be created separately
- Implementing the clients module - assume MaestroK8sClient exists or will be created separately
- Advanced job scheduling features (cron jobs, etc.)
- Custom resource definitions (CRDs)
- Multi-cluster support

## Decisions

**KubeJobStep vs KubePodStep separation**
- Chose separate types for Job and Pod because they have different semantics and lifecycle management
- Jobs manage completions, parallelism, and backoff limits
- Pods are simpler, one-off tasks
- Both implement the same traits but with different internal implementations
- Reusing code via shared helper functions for common operations

**Builder pattern for step configuration**
- Chose fluent builder API because it provides clear, type-safe configuration
- Enables optional parameters while ensuring required fields are set at build time
- Follows existing patterns seen in the example (JobBuilder)
- Returns Result<> from build() to catch configuration errors early

**Integration with kube crate**
- Chose kube crate because it provides high-level Rust abstractions for K8s API
- Using Api<> type for typed K8s resource management
- Using Watcher for streaming job status
- Using Pod::log() for streaming pod logs
- Aligns with existing dependencies in Cargo.toml

**Service and Ingress management**
- ServiceConfig and IngressConfig as separate types to keep step definitions clean
- Service creation happens when expose_service() is called
- Ingress creation happens when expose_ingress() is called
- Services and ingress are managed separately from job/pod lifecycle
- This allows step to be used with or without service/ingress exposure

**JobNameType enum**
- DefinedName for explicitly named jobs (required when you need to reference by name later)
- GenerateName for auto-generated names (useful for one-off jobs)
- Provides flexibility while maintaining type safety
- Maps directly to K8s Job name and generateName fields

**RestartPolicy enum**
- Never: Pod never restarts (default for jobs)
- OnFailure: Pod restarts on failure
- Always: Pod always restarts (more common for deployments)
- Maps to K8s RestartPolicy enum
- Critical for controlling job behavior

**Test approach with Kind**
- Using testcontainers with Kind for integration testing
- Tests verify actual K8s API interactions
- Covers builder validation, resource creation, lifecycle management, error handling
- Test fixtures in YAML for easy modification and understanding

**Trait implementation approach**
- KubeJobStep implements all required traits synchronously where possible
- Async methods (wait, delete_workflow, delete_associated_pods, expose_service, expose_ingress) return futures
- stream_logs() returns a Stream for incremental log reading
- Using Pin<Box<...>> for complex trait object returns to satisfy the trait definitions

**Error handling**
- Using anyhow::Result for application-level errors
- Propagating kube::Error as needed
- Builder returns Result<> from build() method
- Validation happens at build time, not runtime

## Risks / Trade-offs

**Missing entity types** → Mitigation: Create placeholder entity types or document assumption that they exist. For this implementation, will create minimal entity types needed for compilation.

**Missing client implementation** → Mitigation: Create minimal MaestroK8sClient wrapper around kube::Client. For this implementation, will use kube::Client directly.

**Test complexity** → Mitigation: Start with unit tests for builder and logic, add integration tests with Kind incrementally. Use test fixtures to keep tests maintainable.

**Streaming complexity** → Mitigation: Use async-stream crate for streaming logs, following patterns in existing trait tests. Keep stream implementations simple and focused.

**Service/Ingress lifecycle** → Mitigation: Services and ingress are created on-demand but not automatically cleaned up. Document this expectation and provide delete methods.

**Builder validation** → Mitigation: Validate required fields (name/namespace, at least one container) in build() method. Return clear error messages for invalid configurations.

**External intervention** → Mitigation: Watch job status continuously, handle cases where job/pod is deleted externally. Return appropriate errors when expected resources don't exist.

## Migration Plan

- No migration needed (new implementation)
- Incremental rollout:
  1. Create entity types and client module if needed
  2. Implement KubeJobStep builder and core functionality
  3. Implement KubePodStep builder and core functionality
  4. Add trait implementations
  5. Add unit tests
  6. Add integration tests with Kind
  7. Update examples to use new implementations

Rollback strategy:
- Changes are additive, no breaking changes to existing code
- Can revert by removing the new modules
- No data migration needed

## Open Questions

- Should entity types (ContainerLike, SidecarContainer, ResourceLimits) be part of this implementation or a separate change?
- Should MaestroK8sClient be part of this implementation or separate?
- Should services and ingress be automatically deleted when job/pod is deleted, or managed separately?
- Should there be automatic retry logic for transient K8s API failures?
- Should there be default values for optional parameters (namespace, restart policy, etc.)?