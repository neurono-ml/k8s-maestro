## 1. Entity Types and Prerequisites

- [ ] 1.1 Create entity types module (src/entities/mod.rs)
- [ ] 1.2 Implement ContainerLike trait in src/entities/container.rs
- [ ] 1.3 Implement MaestroContainer in src/entities/container.rs
- [ ] 1.4 Implement SidecarContainer in src/entities/container.rs
- [ ] 1.5 Verify ResourceLimits is available in src/steps/traits.rs
- [ ] 1.6 Add entities module to lib.rs

## 2. Kube Client Module

- [ ] 2.1 Create clients module (src/clients/mod.rs)
- [ ] 2.2 Implement MaestroK8sClient wrapper around kube::Client
- [ ] 2.3 Add client initialization methods
- [ ] 2.4 Add clients module to lib.rs

## 3. Supporting Types

- [ ] 3.1 Implement JobNameType enum (src/steps/kubernetes/types.rs)
- [ ] 3.2 Implement RestartPolicy enum (src/steps/kubernetes/types.rs)
- [ ] 3.3 Implement ServiceConfig type (src/steps/kubernetes/types.rs)
- [ ] 3.4 Implement IngressConfig type (src/steps/kubernetes/types.rs)
- [ ] 3.5 Create kubernetes module structure (src/steps/kubernetes/mod.rs)

## 4. KubeJobStep Implementation

- [ ] 4.1 Create KubeJobStep struct definition
- [ ] 4.2 Implement WorkFlowStep trait for KubeJobStep
- [ ] 4.3 Implement KubeWorkFlowStep trait for KubeJobStep
- [ ] 4.4 Implement KubeJobStepBuilder struct
- [ ] 4.5 Implement builder methods: new(), with_name(), with_namespace()
- [ ] 4.6 Implement builder methods: with_name_type(), add_container(), add_sidecar()
- [ ] 4.7 Implement builder methods: with_backoff_limit(), with_restart_policy()
- [ ] 4.8 Implement builder methods: with_ttl_seconds(), with_completions(), with_parallelism()
- [ ] 4.9 Implement builder methods: with_resource_limits(), with_service_config(), with_ingress_config()
- [ ] 4.10 Implement builder methods: expose_service(), expose_ingress(), build()
- [ ] 4.11 Implement build() validation logic (required fields check)
- [ ] 4.12 Implement K8s Job creation logic (convert step to k8s_openapi::Job)

## 5. KubeJobStep Trait Implementations

- [ ] 5.1 Implement WaitableWorkFlowStep trait (wait() method with job status watching)
- [ ] 5.2 Implement DeletableWorkFlowStep trait (delete_workflow() and delete_associated_pods())
- [ ] 5.3 Implement LoggableWorkFlowStep trait (stream_logs() with pod log streaming)
- [ ] 5.4 Implement ServableWorkFlowStep trait (expose_service() and expose_ingress())

## 6. KubePodStep Implementation

- [ ] 6.1 Create KubePodStep struct definition
- [ ] 6.2 Implement WorkFlowStep trait for KubePodStep
- [ ] 6.3 Implement KubeWorkFlowStep trait for KubePodStep
- [ ] 6.4 Implement KubePodStepBuilder struct
- [ ] 6.5 Implement builder methods: new(), with_name(), with_namespace()
- [ ] 6.6 Implement builder methods: with_name_type(), add_container(), add_sidecar()
- [ ] 6.7 Implement builder methods: with_restart_policy(), with_resource_limits()
- [ ] 6.8 Implement builder methods: with_service_config(), with_ingress_config()
- [ ] 6.9 Implement builder methods: expose_service(), expose_ingress(), build()
- [ ] 6.10 Implement build() validation logic
- [ ] 6.11 Implement K8s Pod creation logic

## 7. KubePodStep Trait Implementations

- [ ] 7.1 Implement WaitableWorkFlowStep trait (wait() method with pod status watching)
- [ ] 7.2 Implement DeletableWorkFlowStep trait (delete_workflow())
- [ ] 7.3 Implement LoggableWorkFlowStep trait (stream_logs() with pod log streaming)
- [ ] 7.4 Implement ServableWorkFlowStep trait (expose_service() and expose_ingress())

## 8. Module Integration

- [ ] 8.1 Export KubeJobStep and KubePodStep from src/steps/kubernetes/mod.rs
- [ ] 8.2 Export supporting types from src/steps/kubernetes/mod.rs
- [ ] 8.3 Add kubernetes module to src/steps/mod.rs
- [ ] 8.4 Export types in lib.rs if needed for public API

## 9. Unit Tests

- [ ] 9.1 Create src/steps/kubernetes/tests/mod.rs
- [ ] 9.2 Add builder pattern tests for KubeJobStep (valid configurations)
- [ ] 9.3 Add builder pattern tests for KubeJobStep (invalid configurations)
- [ ] 9.4 Add builder pattern tests for KubePodStep (valid configurations)
- [ ] 9.5 Add builder pattern tests for KubePodStep (invalid configurations)
- [ ] 9.6 Add JobNameType enum tests
- [ ] 9.7 Add RestartPolicy enum tests
- [ ] 9.8 Add ServiceConfig tests
- [ ] 9.9 Add IngressConfig tests

## 10. Integration Tests with Kind

- [ ] 10.1 Set up testcontainers Kind cluster setup
- [ ] 10.2 Create integration test: job creation and completion
- [ ] 10.3 Create integration test: job failure scenario
- [ ] 10.4 Create integration test: job deletion
- [ ] 10.5 Create integration test: pod creation and completion
- [ ] 10.6 Create integration test: pod deletion
- [ ] 10.7 Create integration test: log streaming from job
- [ ] 10.8 Create integration test: log streaming from pod
- [ ] 10.9 Create integration test: service creation
- [ ] 10.10 Create integration test: ingress creation
- [ ] 10.11 Create integration test: external resource deletion during wait
- [ ] 10.12 Create integration test: delete_associated_pods for jobs

## 11. Test Fixtures

- [ ] 11.1 Create tests/common/fixtures/workflows/ directory
- [ ] 11.2 Create simple_job.yaml fixture
- [ ] 11.3 Create job_with_volumes.yaml fixture
- [ ] 11.4 Create job_with_secrets.yaml fixture
- [ ] 11.5 Create tests/common/fixtures/failure_scenarios/ directory
- [ ] 11.6 Create invalid_job.yaml fixture

## 12. Documentation and Examples

- [ ] 12.1 Add documentation to KubeJobStepBuilder
- [ ] 12.2 Add documentation to KubePodStepBuilder
- [ ] 12.3 Add usage example for KubeJobStep
- [ ] 12.4 Add usage example for KubePodStep
- [ ] 12.5 Update README.md if needed with new capabilities

## 13. Code Quality

- [ ] 13.1 Run cargo fmt to ensure consistent formatting
- [ ] 13.2 Run cargo clippy to fix any warnings
- [ ] 13.3 Run cargo test to ensure all tests pass
- [ ] 13.4 Verify no fake/mockup code remains
- [ ] 13.5 Check line lengths are within 100 character limit
