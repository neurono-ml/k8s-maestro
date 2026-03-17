## 1. Setup and Dependencies

- [x] 1.1 Add testcontainers-rs with Kind provider to k8s-maestro-k8s/Cargo.toml
- [x] 1.2 Add mocking dependency (mockall or similar) to k8s-maestro-k8s/Cargo.toml
- [x] 1.3 Create tests/common/ directory structure in k8s-maestro-k8s
- [x] 1.4 Create tests/common/mod.rs with public module exports

## 2. Kind Cluster Lifecycle Management

- [x] 2.1 Create tests/common/kind_cluster.rs module
- [x] 2.2 Implement KindCluster struct with lifecycle management
- [x] 2.3 Implement cluster provisioning function using testcontainers-rs
- [x] 2.4 Implement cluster health verification
- [x] 2.5 Implement cluster cleanup function
- [x] 2.6 Add function to extract kubeconfig and API endpoint
- [x] 2.7 Create integration test for KindCluster lifecycle

## 3. Test Fixtures Directory Structure

- [x] 3.1 Create tests/common/fixtures/ directory
- [x] 3.2 Create tests/common/fixtures/workflows/ subdirectory
- [x] 3.3 Create tests/common/fixtures/secrets/ subdirectory
- [x] 3.4 Create tests/common/fixtures/configmaps/ subdirectory
- [x] 3.5 Create tests/common/fixtures/pvcs/ subdirectory
- [x] 3.6 Create tests/common/fixtures/failure_scenarios/ subdirectory
- [x] 3.7 Add sample YAML fixture files for each resource type

## 4. Test Fixtures Management

- [x] 4.1 Create tests/common/fixtures/mod.rs module
- [x] 4.2 Implement YAML fixture loading function
- [x] 4.3 Implement fixture parsing from YAML to K8s resource objects
- [x] 4.4 Create example YAML fixtures for workflows
- [x] 4.5 Create example YAML fixtures for ConfigMaps
- [x] 4.6 Create example YAML fixtures for Secrets
- [x] 4.7 Create example YAML fixtures for PVCs
- [x] 4.8 Create example YAML fixtures for failure scenarios

## 5. Test Utilities - Resource Creation

- [x] 5.1 Create tests/common/utilities/mod.rs module
- [x] 5.2 Implement create_configmap helper function
- [x] 5.3 Implement create_secret helper function with proper encoding
- [x] 5.4 Implement create_pvc helper function
- [x] 5.5 Implement create_namespace helper function with unique naming
- [x] 5.6 Implement apply_resource helper function for cluster application
- [x] 6.1 Implement delete_resource_by_name helper function
- [x] 6.2 Implement delete_resources_by_label helper function
- [x] 6.3 Implement delete_namespace helper function with cascade delete
- [x] 6.4 Add timeout and retry logic to cleanup functions
- [x] 6.5 Ensure cleanup errors are logged but don't fail tests
- [x] 7.1 Implement wait_for_resource_ready helper function
- [x] 7.2 Implement verify_resource_exists helper function
- [x] 7.3 Implement verify_resource_state helper function with field assertions
- [x] 7.4 Add configurable timeouts to validation functions
- [x] 7.5 Create integration tests for validation helpers

## 8. Test Utilities - Selective Mocking

- [x] 8.1 Create tests/common/mocking/mod.rs module
- [x] 8.2 Define mock traits for K8s client operations
- [x] 8.3 Implement mock K8s client with controllable responses
- [x] 8.4 Implement mock error response generator
- [x] 8.5 Implement mock response sequence for multi-step tests
- [x] 8.6 Create unit test examples demonstrating mocking

## 9. Test Organization Structure

- [x] 9.1 Create tests/integration/ directory
- [x] 9.2 Create tests/e2e/ directory
- [x] 9.3 Add integration test helper for cluster lifecycle per test file
- [x] 9.4 Add E2E test helper for full workflow scenarios
- [x] 9.5 Create sample integration test file demonstrating cluster usage
- [x] 9.6 Create sample E2E test file demonstrating full workflow

## 10. Documentation and Examples

- [x] 10.1 Update AGENTS.md with test infrastructure usage instructions
- [x] 10.2 Create tests/common/README.md with utility documentation
- [x] 10.3 Document fixture creation patterns in tests/common/fixtures/README.md
- [x] 10.4 Add test examples to site-docs/
- [x] 10.5 Update CHANGELOG.md with test infrastructure changes

## 11. CI/CD Integration

- [x] 11.1 Add Kind setup to CI pipeline
- [x] 11.2 Configure unit tests to run without Docker/Kind
- [x] 11.3 Configure integration tests to run with Kind
- [x] 11.4 Configure E2E tests to run with Kind
- [x] 11.5 Add test category selectors to CI scripts

## 12. Verification and Testing

- [x] 12.1 Run unit tests to verify mocking utilities work
- [x] 12.2 Run integration tests to verify Kind cluster provisioning
- [x] 12.3 Run E2E tests to verify full workflow scenarios
- [x] 12.4 Verify test execution times meet expectations (unit < 10s, integration < 5min)
- [x] 12.5 Run cargo clippy and fix any linter warnings
- [x] 12.6 Run cargo fmt to ensure code formatting
