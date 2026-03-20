# Testing Patterns Reference

This reference provides comprehensive guidance on testing k8s-maestro workflows, from unit tests to integration tests and E2E scenarios.

## Test Organization

k8s-maestro uses a three-tier test organization:

| Category | Directory | Cluster Required | Speed | Purpose |
|----------|-----------|------------------|-------|---------|
| **Unit** | `src/**/*_test.rs` or `tests/common/mocking/` | No | Fast (< 10s) | Test individual functions and logic |
| **Integration** | `tests/integration/` | Yes (Kind) | Medium (< 5min) | Test K8s API interactions |
| **E2E** | `tests/e2e/` | Yes (Kind) | Medium (< 5min) | Test complete workflow scenarios |

## Running Tests

### Unit Tests Only

```bash
# Run all unit tests (no Docker needed)
cargo test --lib

# Run specific unit test
cargo test test_create_configmap

# Run with output
cargo test --lib -- --nocapture

# Run specific module tests
cargo test configmap::tests
```

### Integration Tests

```bash
# Run all integration tests (requires Docker)
cargo test --test '*' -- --ignored

# Run specific integration test
cargo test --test kind_cluster_lifecycle -- --ignored

# Run with verbose output
cargo test --test '*' -- --ignored -- --nocapture
```

### All Tests

```bash
# Run all tests including ignored ones
cargo test -- --include-ignored

# Run with detailed output
cargo test --verbose -- --include-ignored -- --nocapture
```

### Test Filtering

```bash
# Run tests matching pattern
cargo test configmap

# Run tests in specific file
cargo test --test configmap_tests

# Run tests with specific name
cargo test test_pvc_creation
```

## Unit Testing

### Mocking K8s Client

Use the mocking module for unit tests that don't need a real cluster:

```rust
use k8s_maestro::tests::common::mocking::{MockK8sClient, mock_error};
use serde_json::json;

#[test]
fn test_create_configmap_success() {
    // Setup mock with success response
    let mut client = MockK8sClient::new()
        .add_create_response(Ok(json!({
            "kind": "ConfigMap",
            "metadata": {
                "name": "test-cm",
                "namespace": "default"
            }
        })));

    // Test create operation
    let response = client.next_create_response().unwrap();
    assert_eq!(response["kind"], "ConfigMap");
    assert_eq!(response["metadata"]["name"], "test-cm");
}

#[test]
fn test_create_configmap_already_exists() {
    // Setup mock with error response
    let mut client = MockK8sClient::new()
        .add_create_response(Err(mock_error(
            "AlreadyExists",
            "ConfigMap 'test-cm' already exists"
        )));

    // Test error handling
    let response = client.next_create_response();
    assert!(response.is_err());
    let error = response.unwrap_err();
    assert!(error.to_string().contains("already exists"));
}

#[test]
fn test_get_job_success() {
    // Setup mock with job response
    let mut client = MockK8sClient::new()
        .add_get_response(Ok(json!({
            "kind": "Job",
            "metadata": {
                "name": "test-job",
                "namespace": "default"
            },
            "spec": {
                "template": {
                    "spec": {
                        "containers": [{
                            "name": "main",
                            "image": "nginx:latest"
                        }]
                    }
                }
            }
        })));

    // Test get operation
    let response = client.next_get_response().unwrap();
    assert_eq!(response["kind"], "Job");
    assert_eq!(response["metadata"]["name"], "test-job");
}
```

### Testing Builder Patterns

```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::KubeJobStep;

#[test]
fn test_workflow_builder_basic() {
    let workflow = WorkflowBuilder::new()
        .with_name("test-workflow")
        .with_namespace("test-ns")
        .build();

    assert!(workflow.is_ok());

    let wf = workflow.unwrap();
    assert_eq!(wf.name, "test-workflow");
    assert_eq!(wf.namespace, "test-ns");
    assert_eq!(wf.steps.len(), 1);
}

#[test]
fn test_workflow_builder_missing_name() {
    let result = WorkflowBuilder::new()
        .with_namespace("test-ns")
        .build();

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("name is required"));
}

#[test]
fn test_resource_limits_builder() {
    let limits = ResourceLimits::new()
        .with_cpu("500m")
        .with_memory("512Mi")
        .with_storage("1Gi");

    assert_eq!(limits.cpu.as_ref().unwrap(), "500m");
    assert_eq!(limits.memory.as_ref().unwrap(), "512Mi");
    assert_eq!(limits.storage.as_ref().unwrap(), "1Gi");
}
```

### Testing Dependency Chains

```rust
use k8s_maestro::workflows::DependencyChain;

#[test]
fn test_dependency_chain_simple() {
    let mut chain = DependencyChain::new();
    chain.add_step("A");
    chain.add_step("B").with_dependency("A");
    chain.add_step("C").with_dependency("B");

    let graph = chain.build_dag().unwrap();
    let levels = graph.topological_sort().unwrap();

    assert_eq!(levels.len(), 3);
    assert_eq!(levels[0], vec!["A"]);
    assert_eq!(levels[1], vec!["B"]);
    assert_eq!(levels[2], vec!["C"]);
}

#[test]
fn test_dependency_chain_parallel() {
    let mut chain = DependencyChain::new();
    chain.add_step("A");
    chain.add_step("B");
    chain.add_step("C").with_dependency_any(vec!["A", "B"]);

    let graph = chain.build_dag().unwrap();
    let deps = graph.get_dependencies("C").unwrap();

    assert!(deps.contains(&"A"));
    assert!(deps.contains(&"B"));
}

#[test]
fn test_dependency_chain_cycle_detection() {
    let mut chain = DependencyChain::new();
    chain.add_step("A");
    chain.add_step("B").with_dependency("A");
    chain.add_step("C").with_dependency("B");
    chain.add_step("A").with_dependency("C"); // Creates cycle!

    let graph = chain.build_dag();
    assert!(graph.is_err());
}
```

## Integration Testing

### Kind Cluster Lifecycle

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;

#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_kind_cluster_lifecycle() {
    // Create cluster
    let cluster = KindCluster::new()
        .await
        .expect("Failed to create Kind cluster");

    // Verify cluster is healthy
    let endpoint = cluster.api_endpoint();
    assert!(endpoint.starts_with("https://"));

    // Verify kubeconfig is valid
    let kubeconfig = cluster.kubeconfig().expect("Failed to get kubeconfig");
    assert!(kubeconfig.contains("apiVersion: v1"));

    // Cleanup happens automatically when cluster is dropped
    cluster.cleanup().await.expect("Failed to cleanup cluster");
}
```

### Resource Creation and Verification

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;
use k8s_maestro::tests::common::utilities::{
    create_namespace, apply_resource, verify_resource_exists, delete_resource_by_name
};
use k8s_maestro::entities::ConfigMapBuilder;
use k8s_openapi::api::core::v1::ConfigMap;

#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_configmap_lifecycle() {
    // Setup cluster
    let cluster = KindCluster::new().await.expect("Failed to create cluster");
    let client = create_client_from_cluster(&cluster);

    // Create namespace
    create_namespace(&client, "test-ns").await.expect("Failed to create namespace");

    // Create ConfigMap
    let configmap = ConfigMapBuilder::new()
        .with_name("test-cm")
        .with_namespace("test-ns")
        .add_data("key1", "value1")
        .build()
        .expect("Failed to build ConfigMap");

    // Apply to cluster
    apply_resource(&client, &configmap, "test-ns")
        .await
        .expect("Failed to apply ConfigMap");

    // Verify it exists
    assert!(
        verify_resource_exists::<ConfigMap>(&client, "test-cm", "test-ns").await,
        "ConfigMap should exist"
    );

    // Cleanup
    delete_resource_by_name::<ConfigMap>(&client, "test-cm", "test-ns")
        .await
        .expect("Failed to delete ConfigMap");

    cluster.cleanup().await.expect("Failed to cleanup cluster");
}
```

### Job Execution and Monitoring

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;
use k8s_maestro::tests::common::utilities::{
    create_namespace, apply_resource, wait_for_resource_ready
};
use k8s_openapi::api::batch::v1::Job;

#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_job_execution() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    // Create namespace
    create_namespace(&client, "test-ns").await?;

    // Create job
    let job = build_test_job("test-job")?;

    // Apply job
    apply_resource(&client, &job, "test-ns").await?;

    // Wait for job to complete
    wait_for_resource_ready::<Job>(&client, "test-job", "test-ns", |job| {
        job.status
            .as_ref()
            .and_then(|s| s.succeeded)
            .unwrap_or(0)
            > 0
    }).await?;

    // Verify job completed successfully
    let jobs_api: Api<Job> = Api::namespaced(client, "test-ns");
    let job = jobs_api.get("test-job").await?;

    assert!(job.status.unwrap().succeeded.unwrap_or(0) > 0);

    cluster.cleanup().await?;
}
```

### Service and Ingress Testing

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;
use k8s_maestro::{ServiceBuilder, ServiceType, IngressBuilder};
use std::collections::BTreeMap;

#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_service_and_ingress() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    // Create namespace
    create_namespace(&client, "test-ns").await?;

    // Create deployment
    let deployment = create_test_deployment()?;
    apply_resource(&client, &deployment, "test-ns").await?;

    // Create service
    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "test".to_string());

    let service = ServiceBuilder::new()
        .with_name("test-service")
        .with_namespace("test-ns")
        .with_port(80, 8080, "TCP")
        .with_selector(selector)
        .with_type(ServiceType::ClusterIP)
        .build()?;

    apply_resource(&client, &service, "test-ns").await?;

    // Verify service exists
    assert!(verify_resource_exists::<Service>(&client, "test-service", "test-ns").await);

    // Create ingress
    let ingress = IngressBuilder::new()
        .with_name("test-ingress")
        .with_namespace("test-ns")
        .with_host("test.local")
        .with_path("/", "test-service", 80)
        .build()?;

    apply_resource(&client, &ingress, "test-ns").await?;

    // Verify ingress exists
    assert!(verify_resource_exists::<Ingress>(&client, "test-ingress", "test-ns").await);

    cluster.cleanup().await?;
}
```

## E2E Testing

### Complete Workflow Execution

```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::tests::common::kind_cluster::KindCluster;

#[tokio::test]
#[ignore = "Requires Docker"]
async fn e2e_workflow_execution() {
    // Setup
    let (client, cluster) = setup_e2e_test().await?;

    // Build workflow
    let workflow = WorkflowBuilder::new()
        .with_name("e2e-workflow")
        .add_step(KubeJobStep::new("step1", "nginx:latest", client.clone()))
        .add_step(KubeJobStep::new("step2", "busybox:latest", client.clone()))
        .build()?;

    // Execute workflow
    let execution = execute_workflow(&client, &workflow).await?;

    // Verify execution
    assert!(!execution.id().is_empty());
    assert_eq!(execution.name(), "e2e-workflow");

    // Wait for completion
    execution.wait_for_completion().await?;

    // Verify success
    assert!(execution.is_success());

    // Cleanup
    cluster.cleanup().await?;
}
```

### ETL Pipeline E2E Test

```rust
#[tokio::test]
#[ignore = "Requires Docker"]
async fn e2e_etl_pipeline() {
    let (client, cluster) = setup_e2e_test().await?;

    // Step 1: Create ConfigMap with test data
    let configmap = ConfigMapBuilder::new()
        .with_name("test-data")
        .add_data("input.csv", "id,name\n1,Alice\n2,Bob\n3,Charlie")
        .build()?;

    apply_resource(&client, &configmap, "default").await?;

    // Step 2: Create PVC for output
    let pvc = MaestroPVCMountVolumeBuilder::new()
        .with_name("output-pvc")
        .with_storage("1Gi")
        .build()?;

    apply_resource(&client, &pvc, "default").await?;

    // Step 3: Build ETL workflow
    let workflow = WorkflowBuilder::new()
        .with_name("etl-pipeline")
        .add_step(KubeJobStep::new("extract", "python:3.11", client.clone()))
        .add_step(KubeJobStep::new("transform", "python:3.11", client.clone()))
        .add_step(KubeJobStep::new("load", "postgres:16", client.clone()))
        .build()?;

    // Step 4: Setup dependency chain
    let mut chain = DependencyChain::new();
    chain.add_step("extract");
    chain.add_step("transform").with_dependency("extract");
    chain.add_step("load").with_dependency("transform");

    // Step 5: Execute workflow
    let execution = execute_workflow_with_deps(&client, &workflow, &chain).await?;

    // Step 6: Verify results
    assert!(execution.is_success());

    // Step 7: Check output PVC has data
    let output = verify_pvc_has_data(&client, "output-pvc", "default").await?;
    assert!(!output.is_empty());

    cluster.cleanup().await?;
}
```

## Test Utilities

### Fixtures

Load pre-defined YAML fixtures:

```rust
use k8s_maestro::tests::common::fixtures::{
    load_configmap_fixture,
    load_secret_fixture,
    load_pvc_fixture,
    load_workflow_fixture,
};

#[tokio::test]
async fn test_with_fixture() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    // Load ConfigMap from fixture
    let configmap = load_configmap_fixture("test-configmap").unwrap();

    // Load Secret from fixture
    let secret = load_secret_fixture("test-secret").unwrap();

    // Load PVC from fixture
    let pvc = load_pvc_fixture("test-pvc").unwrap();

    // Load workflow from fixture
    let workflow = load_workflow_fixture("simple-workflow").unwrap();

    // Apply to cluster
    apply_resource(&client, &configmap, "default").await?;

    cluster.cleanup().await?;
}
```

### Resource Helpers

Create resources programmatically:

```rust
use k8s_maestro::tests::common::utilities::{
    create_configmap,
    create_secret,
    create_pvc,
    create_namespace,
};
use std::collections::BTreeMap;

#[tokio::test]
async fn test_with_helpers() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    // Create resources
    let cm = create_configmap("my-cm", "default", BTreeMap::new());
    let secret = create_secret("my-secret", "default", BTreeMap::new());
    let ns = create_namespace("test");

    apply_resource(&client, &cm, "default").await?;
    apply_resource(&client, &secret, "default").await?;
    apply_resource(&client, &ns, "").await?;

    cluster.cleanup().await?;
}
```

### Validation Helpers

Verify resource states:

```rust
use k8s_maestro::tests::common::utilities::{
    verify_resource_exists,
    verify_resource_state,
    wait_for_resource_ready,
};

#[tokio::test]
async fn test_with_validation() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    // Create resource
    let cm = create_configmap("test-cm", "default", BTreeMap::new());
    apply_resource(&client, &cm, "default").await?;

    // Check if resource exists
    if verify_resource_exists::<ConfigMap>(&client, "test-cm", "default").await {
        println!("ConfigMap exists!");
    }

    // Wait for resource to be ready
    wait_for_resource_ready::<Pod>(&client, "my-pod", "default", |pod| {
        pod.status
            .as_ref()
            .map(|s| s.phase.as_deref() == Some("Running"))
            .unwrap_or(false)
    }).await.expect("Pod never became ready");

    cluster.cleanup().await?;
}
```

## Testing Best Practices

1. **Use the right test category**:
   - Unit tests for logic and builder patterns
   - Integration tests for K8s API interactions
   - E2E tests for complete workflow scenarios

2. **Always clean up resources**:
   ```rust
   // Use automatic cleanup via Drop
   let cluster = KindCluster::new().await?;

   // Or manual cleanup
   let result = test_logic();
   cluster.cleanup().await?;
   result
   ```

3. **Use unique names**:
   ```rust
   let unique_name = format!("test-{}", uuid::Uuid::new_v4());
   let cm = ConfigMapBuilder::new()
       .with_name(unique_name)
       .build()?;
   ```

4. **Mark tests appropriately**:
   ```rust
   #[tokio::test]
   #[ignore = "Requires Docker"]
   async fn integration_test() { }
   ```

5. **Keep fixtures minimal**:
   - Only include necessary fields
   - Use realistic but simple data
   - Document fixture purpose

6. **Test both success and failure cases**:
   ```rust
   #[test]
   fn test_success() { /* ... */ }

   #[test]
   fn test_failure() { /* ... */ }
   ```

7. **Use assertions with clear messages**:
   ```rust
   assert!(
       verify_resource_exists(&client, name, ns).await,
       "Resource {} should exist in namespace {}", name, ns
   );
   ```

8. **Handle async properly**:
   ```rust
   #[tokio::test]
   async fn async_test() {
       let result = async_operation().await?;
       Ok::<(), anyhow::Error>(())
   }
   ```

## Test Coverage Goals

- **Unit tests**: 80%+ coverage for builder patterns and utilities
- **Integration tests**: Cover all major resource types (Jobs, Pods, Services, ConfigMaps, Secrets, PVCs)
- **E2E tests**: Cover common workflow patterns (ETL, conditional execution, parallel processing)

## Continuous Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --lib

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --test '*' -- --ignored
```

## Debugging Failed Tests

### Enable Logging

```bash
RUST_LOG=debug cargo test -- --nocapture
```

### Inspect Cluster State

```bash
# List all resources
kubectl get all

# Describe specific resource
kubectl describe job my-job

# View logs
kubectl logs my-pod-xyz

# Get events
kubectl get events --sort-by='.lastTimestamp'
```

### Pause on Failure

```rust
#[tokio::test]
#[ignore = "Requires Docker and manual inspection"]
async fn test_with_pause() {
    let cluster = KindCluster::new().await?;

    // ... test logic ...

    // Pause to inspect cluster
    println!("Cluster running. Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    cluster.cleanup().await?;
}
```
