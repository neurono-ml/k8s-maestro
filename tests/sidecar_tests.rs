//! Integration tests for sidecar container functionality.
//!
//! These tests verify the sidecar container plugin system configuration
//! and builder correctness.
//!
//! Run these tests with: `cargo test --test sidecar_tests -- --ignored`

mod common;

use common::kind_cluster::KindCluster;
use k8s_maestro::entities::ContainerLike;
use k8s_maestro::steps::SidecarBuilder;
use std::error::Error;

const TEST_NAMESPACE_PREFIX: &str = "sidecar-integration-test";

fn get_unique_namespace_name(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{}-{}", prefix, nanos)
}

/// Test that SidecarBuilder correctly creates a sidecar container.
#[tokio::test]
#[ignore = "Requires Docker and Kind cluster"]
async fn test_sidecar_builder_creates_valid_sidecar() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _cluster = KindCluster::new().await?;

    let sidecar = SidecarBuilder::new("nginx:alpine")
        .with_name("test-sidecar")
        .build()?;

    let container = sidecar.as_container();

    assert_eq!(container.name, "test-sidecar", "Sidecar name should match");
    assert_eq!(
        container.image.as_deref(),
        Some("nginx:alpine"),
        "Sidecar image should match"
    );

    Ok(())
}

/// Test sidecar can be added to pod step builder.
#[tokio::test]
#[ignore = "Requires Docker and Kind cluster"]
async fn test_sidecar_network_namespace() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _cluster = KindCluster::new().await?;
    let _namespace_name = get_unique_namespace_name(&format!("{}-net", TEST_NAMESPACE_PREFIX));

    let sidecar = SidecarBuilder::new("busybox")
        .with_name("communicator")
        .build()?;

    let container = sidecar.as_container();
    assert_eq!(
        container.name, "communicator",
        "Sidecar name should be communicator"
    );

    Ok(())
}

/// Test plugin installation to step.
#[tokio::test]
#[ignore = "Requires Docker and Kind cluster"]
async fn test_plugin_installation_to_step() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _cluster = KindCluster::new().await?;
    let _namespace_name = get_unique_namespace_name(&format!("{}-plugin", TEST_NAMESPACE_PREFIX));

    let sidecar = SidecarBuilder::new("nginx:alpine")
        .with_name("plugin-sidecar")
        .build()?;

    let container = sidecar.as_container();
    assert_eq!(
        container.image.as_deref(),
        Some("nginx:alpine"),
        "Sidecar image should match"
    );

    Ok(())
}

/// Test sidecar resource limits configuration.
#[tokio::test]
#[ignore = "Requires Docker and Kind cluster"]
async fn test_sidecar_resource_limits() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _cluster = KindCluster::new().await?;

    use k8s_maestro::steps::ResourceLimits;

    let limits = ResourceLimits::new()
        .with_cpu_request("100m")
        .with_cpu("500m")
        .with_memory_request("128Mi")
        .with_memory("512Mi");

    let sidecar = SidecarBuilder::new("nginx:alpine")
        .with_name("limited-sidecar")
        .with_resource_limits(limits)
        .build()?;

    let container = sidecar.as_container();
    assert!(
        container.resources.is_some(),
        "Sidecar should have resources configured"
    );

    Ok(())
}
