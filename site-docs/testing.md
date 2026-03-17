# Testing Guide

This guide explains how to write and run tests for k8s-maestro.

## Test Categories

k8s-maestro uses a three-tier test organization:

| Category | Purpose | Cluster Required | Speed |
|----------|---------|------------------|-------|
| Unit | Test individual functions and logic | No | Fast (< 10s) |
| Integration | Test Kubernetes API interactions | Yes (Kind) | Medium (< 5min) |
| E2E | Test complete workflow scenarios | Yes (Kind) | Medium (< 5min) |

## Running Tests

### Unit Tests

```bash
# Run unit tests only (no Docker needed)
cargo test --lib

# Run a specific unit test
cargo test test_create_configmap
```

### Integration Tests

```bash
# Run integration tests (requires Docker)
cargo test --test '*' -- --ignored

# Run specific integration test
cargo test --test kind_cluster_lifecycle -- --ignored
```

### All Tests

```bash
# Run all tests including ignored ones
cargo test -- --include-ignored
```

## Writing Tests

### Unit Tests with Mocking

Use the mocking module for unit tests that don't need a real cluster:

```rust
use k8s_maestro::tests::common::mocking::{MockK8sClient, mock_error, mock_resource_response};

#[test]
fn test_create_resource_success() {
    let mut client = MockK8sClient::new()
        .add_create_response(Ok(mock_resource_response("ConfigMap", "test-cm", "default")));
    
    let response = client.next_create_response().unwrap();
    assert_eq!(response["kind"], "ConfigMap");
}

#[test]
fn test_create_resource_already_exists() {
    let mut client = MockK8sClient::new()
        .add_create_response(Err(mock_error("AlreadyExists", "resource exists")));
    
    let result = client.next_create_response();
    assert!(result.is_err());
}
```

### Integration Tests with Kind

Use the Kind cluster module for integration tests:

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;
use k8s_maestro::tests::common::utilities::{
    create_configmap, apply_resource, verify_resource_exists, delete_resource_by_name,
};
use k8s_openapi::api::core::v1::ConfigMap;

#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_configmap_lifecycle() {
    // Setup cluster
    let cluster = KindCluster::new().await.expect("Failed to create cluster");
    
    // Create and apply ConfigMap
    let cm = create_configmap("test-cm", "default", std::collections::BTreeMap::new());
    apply_resource(&client, &cm, "default").await.expect("Failed to apply");
    
    // Verify it exists
    assert!(verify_resource_exists::<ConfigMap>(&client, "test-cm", "default").await);
    
    // Cleanup
    delete_resource_by_name::<ConfigMap>(&client, "test-cm", "default").await.ok();
}
```

### E2E Tests for Workflows

Use the E2E helpers for complete workflow scenarios:

```rust
use k8s_maestro::tests::e2e::setup_e2e_test;
use k8s_maestro::tests::common::fixtures::load_workflow_fixture;

#[tokio::test]
#[ignore = "Requires Docker"]
async fn e2e_workflow_execution() {
    let (client, _cluster) = setup_e2e_test().await.expect("Failed to setup");
    
    // Load and apply workflow
    let workflow = load_workflow_fixture("simple-workflow").expect("Failed to load");
    
    // Verify workflow execution...
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
};

let cm = load_configmap_fixture("test-configmap").unwrap();
let secret = load_secret_fixture("test-secret").unwrap();
```

### Resource Helpers

Create resources programmatically:

```rust
use k8s_maestro::tests::common::utilities::{
    create_configmap, create_secret, create_pvc, create_namespace,
};
use std::collections::BTreeMap;

let cm = create_configmap("my-cm", "default", BTreeMap::new());
let secret = create_secret("my-secret", "default", BTreeMap::new());
let ns = create_namespace("test");
```

### Validation Helpers

Verify resource states:

```rust
use k8s_maestro::tests::common::utilities::{
    verify_resource_exists,
    verify_resource_state,
    wait_for_resource_ready,
};

// Check if resource exists
if verify_resource_exists::<ConfigMap>(&client, "my-cm", "default").await {
    println!("ConfigMap exists!");
}

// Wait for resource to be ready
wait_for_resource_ready::<Pod>(&client, "my-pod", "default", |pod| {
    pod.status.as_ref().map(|s| s.phase.as_deref() == Some("Running")).unwrap_or(false)
}).await.expect("Pod never became ready");
```

## Best Practices

1. **Use the right test category**: Unit tests for logic, integration for K8s API, E2E for workflows
2. **Clean up resources**: Always clean up created resources in tests
3. **Use unique names**: Create resources with unique names to avoid conflicts
4. **Mark tests appropriately**: Use `#[ignore]` for tests requiring Docker
5. **Keep fixtures minimal**: Only include necessary fields in fixtures
