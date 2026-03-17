## Why

The k8s-maestro library needs concrete workflow step implementations to orchestrate Kubernetes jobs and pods. Currently, the trait system is defined but there are no implementations for creating and managing actual K8s resources. This change provides the core execution layer that all workflows will use to interact with Kubernetes.

## What Changes

- Create `src/steps/kubernetes/job.rs` with `KubeJobStep` struct implementing `WorkFlowStep`, `KubeWorkFlowStep`, `WaitableWorkFlowStep`, `DeletableWorkFlowStep`, `LoggableWorkFlowStep`, `ServableWorkFlowStep`
- Create `KubeJobStepBuilder` with fluent API for configuring job properties, containers, sidecars, service configs, and ingress configs
- Create `src/steps/kubernetes/pod.rs` with `KubePodStep` struct and builder (similar to job but creates Pod directly)
- Implement `ServiceConfig` and `IngressConfig` types for service and ingress management
- Implement `JobNameType` enum (DefinedName, GenerateName) for flexible job naming
- Implement `RestartPolicy` enum (Never, OnFailure, Always)
- Add comprehensive unit and integration tests with Kind for K8s resource lifecycle
- Create test fixtures for various job configurations and failure scenarios
- Ensure existing entity types (ContainerLike, SidecarContainer, ResourceLimits) are available for use

## Capabilities

### New Capabilities
- `kube-job-step`: Kubernetes Job workflow step with full lifecycle management (create, wait, delete, logs, service/ingress exposure)
- `kube-pod-step`: Kubernetes Pod workflow step for one-off tasks (similar lifecycle to job step)
- `service-config`: Service configuration for exposing workflow steps as K8s Services
- `ingress-config`: Ingress configuration for exposing workflow steps via Ingress resources
- `job-naming`: Flexible job naming with defined names or auto-generated names
- `restart-policies`: Configurable restart policies for Kubernetes pods/jobs

### Modified Capabilities
- None (this is foundational implementation, no existing spec-level requirements changing)

## Impact

- Core workflow execution layer that all workflows will depend on
- Adds `k8s-openapi` and `kube` dependencies for K8s API interaction
- Integration with existing traits: WorkFlowStep, KubeWorkFlowStep, WaitableWorkFlowStep, DeletableWorkFlowStep, LoggableWorkFlowStep, ServableWorkFlowStep
- Requires entity types (ContainerLike, SidecarContainer, ResourceLimits) to be implemented or available
- Enables end-to-end testing of Kubernetes resource orchestration
- No breaking changes (adding new implementation modules)
