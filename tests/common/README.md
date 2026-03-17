# Test Common Utilities

This directory contains shared test infrastructure for k8s-maestro integration and E2E tests.

## Modules

### `kind_cluster`
Kind cluster lifecycle management for integration tests.

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;

// Create a new Kind cluster
let cluster = KindCluster::new().await?;

// Get kubeconfig
let kubeconfig = cluster.kubeconfig()?;

// Get API endpoint
let endpoint = cluster.api_endpoint();

// Cleanup (automatic on drop, but can be explicit)
cluster.cleanup().await?;
```

### `fixtures`
YAML fixture loading and parsing for Kubernetes resources.

```rust
use k8s_maestro::tests::common::fixtures::{
    load_configmap_fixture,
    load_secret_fixture,
    load_pvc_fixture,
    load_job_fixture,
    load_workflow_fixture,
};

// Load fixtures by name
let cm = load_configmap_fixture("test-configmap")?;
let secret = load_secret_fixture("test-secret")?;
let pvc = load_pvc_fixture("test-pvc")?;
```

### `utilities`
Helper functions for resource creation, cleanup, and validation.

```rust
use k8s_maestro::tests::common::utilities::{
    create_configmap, create_secret, create_pvc, create_namespace,
    apply_resource, delete_resource_by_name, delete_namespace,
    verify_resource_exists, wait_for_resource_ready,
};

// Create resources programmatically
let cm = create_configmap("my-cm", "default", data);
let secret = create_secret("my-secret", "default", string_data);
let ns = create_namespace("test");

// Apply to cluster
apply_resource(&client, &cm, "default").await?;

// Verify and cleanup
assert!(verify_resource_exists::<ConfigMap>(&client, "my-cm", "default").await);
delete_resource_by_name::<ConfigMap>(&client, "my-cm", "default").await?;
```

### `mocking`
Mock K8s client for unit tests without cluster dependencies.

```rust
use k8s_maestro::tests::common::mocking::{
    MockK8sClient, MockError, mock_error, mock_resource_response,
};

// Create mock with predefined responses
let mut mock = MockK8sClient::new()
    .add_get_response(Ok(serde_json::json!({"name": "test"})))
    .add_create_response(Err(mock_error("AlreadyExists", "resource exists")));

// Use in tests
let response = mock.next_get_response()?;
assert_eq!(response["name"], "test");
```

## Test Categories

| Category | Location | Cluster | Speed |
|----------|----------|---------|-------|
| Unit | `src/**/*_test.rs` | No | Fast |
| Integration | `tests/integration/` | Yes | Medium |
| E2E | `tests/e2e/` | Yes | Medium |

## Running Tests

```bash
# Unit tests only (no Docker needed)
cargo test --lib

# Integration tests (requires Docker)
cargo test --test '*' -- --ignored

# All tests
cargo test -- --include-ignored
```
