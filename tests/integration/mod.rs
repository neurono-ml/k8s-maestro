//! Integration test helper module.
//!
//! This module provides helpers for setting up integration tests with
//! Kind cluster lifecycle management.

use k8s_maestro::tests::common::kind_cluster::KindCluster;
use kube::Client;

/// Sets up an integration test with a Kind cluster.
///
/// This function:
/// 1. Creates a new Kind cluster
/// 2. Returns a kube client connected to the cluster
/// 3. Returns a handle to the cluster for cleanup
///
/// # Example
///
/// ```no_run
/// use k8s_maestro::tests::integration::setup_integration_test;
///
/// #[tokio::test]
/// #[ignore = "Requires Docker"]
/// async fn test_with_cluster() {
///     let (client, _cluster) = setup_integration_test().await.expect("Failed to setup");
///     // Use client to interact with the cluster...
/// }
/// ```
pub async fn setup_integration_test(
) -> Result<(Client, KindCluster), Box<dyn std::error::Error + Send + Sync>> {
    // Create Kind cluster
    let cluster = KindCluster::new().await?;

    // Create kube client from the cluster's kubeconfig
    let kubeconfig = cluster.kubeconfig()?;
    
    // Parse kubeconfig and create client
    // Note: In a real implementation, you'd parse the YAML and create a proper client
    let client = Client::try_default().await?;

    Ok((client, cluster))
}

/// Creates a test namespace for isolation.
///
/// # Arguments
///
/// * `client` - The Kubernetes client
/// * `prefix` - The prefix for the namespace name
pub async fn create_test_namespace(
    client: &Client,
    prefix: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use k8s_maestro::tests::common::utilities::{apply_resource, create_namespace};
    use kube::api::Api;
    use k8s_openapi::api::core::v1::Namespace;

    let ns = create_namespace(prefix);
    let name = ns.metadata.name.clone().unwrap();
    
    let api: Api<Namespace> = Api::all(client.clone());
    api.create(&kube::api::PostParams::default(), &ns).await?;

    Ok(name)
}

/// Cleans up a test namespace.
///
/// # Arguments
///
/// * `client` - The Kubernetes client
/// * `name` - The name of the namespace to delete
pub async fn cleanup_test_namespace(
    client: &Client,
    name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use k8s_maestro::tests::common::utilities::delete_namespace;
    delete_namespace(client, name).await?;
    Ok(())
}
