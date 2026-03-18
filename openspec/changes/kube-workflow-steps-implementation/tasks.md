## 1. Entity Types and Prerequisites

- [x] 1.1 Create entity types module (src/entities/mod.rs)
- [x] 1.2 Implement ContainerLike trait in src/entities/container.rs
- [x] 1.3 Implement MaestroContainer in src/entities/container.rs
- [x] 1.4 Implement SidecarContainer in src/entities/container.rs
- [x] 1.5 Verify ResourceLimits is available in src/steps/traits.rs
- [x] 1.6 Add entities module to lib.rs

## 2. Kube Client Module

- [x] 2.1 Create clients module (src/clients/mod.rs)
- [x] 2.2 Implement MaestroK8sClient wrapper around kube::Client
- [x] 2.3 Add client initialization methods
- [x] 2.4 Add clients module to lib.rs

## 3. Supporting Types

- [x] 3.1 Implement JobNameType enum (src/steps/kubernetes/types.rs)
- [x] 3.2 Implement RestartPolicy enum (src/steps/kubernetes/types.rs)
- [x] 3.3 Implement ServiceConfig type (src/steps/kubernetes/types.rs)
- [x] 3.4 Implement IngressConfig type (src/steps/kubernetes/types.rs)
- [x] 3.5 Create kubernetes module structure (src/steps/kubernetes/mod.rs)

## 4. KubeJobStep Implementation

- [x] 4.1 Create KubeJobStep struct definition
- [x] 4.2 Implement WorkFlowStep trait for KubeJobStep
- [x] 4.3 Implement KubeWorkFlowStep trait for KubeJobStep
- [x] 4.4 Implement KubeJobStepBuilder struct
- [x] 4.5 Implement builder methods: new(), with_name(), with_namespace()
- [x] 4.6 Implement builder methods: with_name_type(), add_container(), add_sidecar()
- [x] 4.7 Implement builder methods: with_backoff_limit(), with_restart_policy()
- [x] 4.8 Implement builder methods: with_ttl_seconds(), with_completions(), with_parallelism()
- [x] 4.9 Implement builder methods: with_resource_limits(), with_service_config(), with_ingress_config()
- [x] 4.10 Implement builder methods: expose_service(), expose_ingress(), build()
- [x] 4.11 Implement build() validation logic (required fields check)
- [x] 4.12 Implement K8s Job creation logic (convert step to k8s_openapi::Job)

## 5. KubeJobStep Trait Implementations

- [x] 5.1 Implement WaitableWorkFlowStep trait (wait() method with job status watching)
- [x] 5.2 Implement DeletableWorkFlowStep trait (delete_workflow() and delete_associated_pods())
- [x] 5.3 Implement LoggableWorkFlowStep trait (stream_logs() with pod log streaming)
- [x] 5.4 Implement ServableWorkFlowStep trait (expose_service() and expose_ingress())

## 6. KubePodStep Implementation

- [x] 6.1 Create KubePodStep struct definition
- [x] 6.2 Implement WorkFlowStep trait for KubePodStep
- [x] 6.3 Implement KubeWorkFlowStep trait for KubePodStep
- [x] 6.4 Implement KubePodStepBuilder struct
- [x] 6.5 Implement builder methods: new(), with_name(), with_namespace()
- [x] 6.6 Implement builder methods: with_name_type(), add_container(), add_sidecar()
- [x] 6.7 Implement builder methods: with_restart_policy(), with_resource_limits()
- [x] 6.8 Implement builder methods: with_service_config(), with_ingress_config()
- [x] 6.9 Implement builder methods: expose_service(), expose_ingress(), build()
- [x] 6.10 Implement build() validation logic
- [x] 6.11 Implement K8s Pod creation logic

## 7. KubePodStep Trait Implementations

- [x] 7.1 Implement WaitableWorkFlowStep trait (wait() method with pod status watching)
- [x] 7.2 Implement DeletableWorkFlowStep trait (delete_workflow())
- [x] 7.3 Implement LoggableWorkFlowStep trait (stream_logs() with pod log streaming)
- [x] 7.4 Implement ServableWorkFlowStep trait (expose_service() and expose_ingress())

## 8. Module Integration

- [x] 8.1 Export KubeJobStep and KubePodStep from src/steps/kubernetes/mod.rs
- [x] 8.2 Export supporting types from src/steps/kubernetes/mod.rs
- [x] 8.3 Add kubernetes module to src/steps/mod.rs
- [x] 8.4 Export types in lib.rs if needed for public API

## 9. Unit Tests

- [x] 9.1 Create src/steps/kubernetes/tests/mod.rs
- [x] 9.2 Add builder pattern tests for KubeJobStep (valid configurations)
- [x] 9.3 Add builder pattern tests for KubeJobStep (invalid configurations)
- [x] 9.4 Add builder pattern tests for KubePodStep (valid configurations)
- [x] 9.5 Add builder pattern tests for KubePodStep (invalid configurations)
- [x] 9.6 Add JobNameType enum tests
- [x] 9.7 Add RestartPolicy enum tests
- [x] 9.8 Add ServiceConfig tests
- [x] 9.9 Add IngressConfig tests

## 10. Integration Tests with Kind

- [x] 10.1 Set up testcontainers Kind cluster setup
- [x] 10.2 Create integration test: job creation and completion
- [x] 10.3 Create integration test: job failure scenario
- [x] 10.4 Create integration test: job deletion
- [x] 10.5 Create integration test: pod creation and completion
- [x] 10.6 Create integration test: pod deletion
- [x] 10.7 Create integration test: log streaming from job
- [x] 10.8 Create integration test: log streaming from pod
- [x] 10.9 Create integration test: service creation
- [x] 10.10 Create integration test: ingress creation
- [x] 10.11 Create integration test: external resource deletion during wait
- [x] 10.12 Create integration test: delete_associated_pods for jobs

## 11. Test Fixtures

- [x] 11.1 Create tests/common/fixtures/workflows/ directory
- [x] 11.2 Create simple_job.yaml fixture
- [x] 11.3 Create job_with_volumes.yaml fixture
- [x] 11.4 Create job_with_secrets.yaml fixture
- [x] 11.5 Create tests/common/fixtures/failure_scenarios/ directory
- [x] 11.6 Create invalid_job.yaml fixture

## 12. Documentation and Examples

- [x] 12.1 Add documentation to KubeJobStepBuilder
- [x] 12.2 Add documentation to KubePodStepBuilder
- [x] 12.3 Add usage example for KubeJobStep
- [x] 12.4 Add usage example for KubePodStep
- [x] 12.5 Update README.md if needed with new capabilities

## 13. Code Quality

- [x] 13.1 Run cargo fmt to ensure consistent formatting
- [x] 13.2 Run cargo clippy to fix any warnings
- [x] 13.3 Run cargo test to ensure all tests pass
- [x] 13.4 Verify no fake/mockup code remains
- [x] 13.5 Check line lengths are within 100 character limit
