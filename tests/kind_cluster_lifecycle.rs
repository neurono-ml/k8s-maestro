//! Integration tests for KindCluster lifecycle management.
//!
//! These tests require Docker to be running as they create actual Kind clusters.

mod common;

use common::kind_cluster::KindCluster;

/// Tests that a Kind cluster can be created and is healthy.
///
/// This test verifies:
/// - Cluster provisioning works
/// - Health check passes
/// - API endpoint is available
#[tokio::test]
#[ignore = "Requires Docker and is slow - run with --ignored flag"]
async fn test_kind_cluster_lifecycle() {
    // Create cluster
    let cluster = KindCluster::new()
        .await
        .expect("Failed to create Kind cluster");

    // Verify API endpoint is set
    let endpoint = cluster.api_endpoint();
    assert!(
        endpoint.starts_with("https://"),
        "API endpoint should be HTTPS"
    );

    // Verify kubeconfig is generated
    let kubeconfig = cluster.kubeconfig().expect("Failed to get kubeconfig");
    assert!(
        kubeconfig.contains("apiVersion: v1"),
        "Kubeconfig should be valid YAML"
    );
    assert!(
        kubeconfig.contains("kind: Config"),
        "Kubeconfig should be a Config resource"
    );

    // Cleanup happens automatically when cluster is dropped
    cluster.cleanup().await.expect("Failed to cleanup cluster");
}

/// Tests that a Kind cluster can be created with a specific version.
#[tokio::test]
#[ignore = "Requires Docker and is slow - run with --ignored flag"]
async fn test_kind_cluster_with_version() {
    let cluster = KindCluster::with_version("v1.28.0")
        .await
        .expect("Failed to create Kind cluster with version");

    // Verify the cluster is healthy
    let endpoint = cluster.api_endpoint();
    assert!(!endpoint.is_empty(), "API endpoint should not be empty");

    // Cleanup
    cluster.cleanup().await.expect("Failed to cleanup cluster");
}
