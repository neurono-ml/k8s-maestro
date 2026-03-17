//! Test utilities for Kubernetes resource management.
//!
//! This module provides helper functions for creating Kubernetes resources
//! programmatically for use in tests.

use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::{
    ConfigMap, Namespace, PersistentVolumeClaim, ResourceRequirements, Secret,
};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

// ============================================================================
// Resource Creation Helpers
// ============================================================================

/// Creates a ConfigMap with the specified parameters.
///
/// # Arguments
///
/// * `name` - The name of the ConfigMap
/// * `namespace` - The namespace to create the ConfigMap in
/// * `data` - The data key-value pairs
///
/// # Example
///
/// ```
/// use std::collections::BTreeMap;
/// use k8s_maestro::tests::common::utilities::create_configmap;
///
/// let mut data = BTreeMap::new();
/// data.insert("key".to_string(), "value".to_string());
/// let cm = create_configmap("my-config", "default", data);
/// ```
pub fn create_configmap(name: &str, namespace: &str, data: BTreeMap<String, String>) -> ConfigMap {
    ConfigMap {
        metadata: ObjectMeta {
            name: Some(name.to_string()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        data: Some(data),
        ..Default::default()
    }
}

/// Creates a Secret with the specified parameters.
///
/// # Arguments
///
/// * `name` - The name of the Secret
/// * `namespace` - The namespace to create the Secret in
/// * `string_data` - The string data (will be base64 encoded automatically)
///
/// # Note
///
/// Using `string_data` allows passing plain text values that will be
/// automatically encoded to base64 by Kubernetes.
pub fn create_secret(name: &str, namespace: &str, string_data: BTreeMap<String, String>) -> Secret {
    Secret {
        metadata: ObjectMeta {
            name: Some(name.to_string()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        string_data: Some(string_data),
        ..Default::default()
    }
}

/// Creates a PersistentVolumeClaim with the specified parameters.
///
/// # Arguments
///
/// * `name` - The name of the PVC
/// * `namespace` - The namespace to create the PVC in
/// * `storage` - The storage size (e.g., "1Gi")
/// * `access_modes` - The access modes (e.g., ["ReadWriteOnce"])
pub fn create_pvc(
    name: &str,
    namespace: &str,
    storage: &str,
    access_modes: Vec<String>,
) -> PersistentVolumeClaim {
    let mut requests = BTreeMap::new();
    requests.insert("storage".to_string(), Quantity(storage.to_string()));

    PersistentVolumeClaim {
        metadata: ObjectMeta {
            name: Some(name.to_string()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        spec: Some(k8s_openapi::api::core::v1::PersistentVolumeClaimSpec {
            access_modes: Some(access_modes),
            resources: Some(ResourceRequirements {
                requests: Some(requests),
                limits: None,
                claims: None,
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// Creates a Namespace with a unique name based on a prefix and timestamp.
///
/// # Arguments
///
/// * `prefix` - The prefix for the namespace name
///
/// # Returns
///
/// A Namespace struct with a unique name.
pub fn create_namespace(prefix: &str) -> Namespace {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let name = format!("{}-{}", prefix, timestamp);

    Namespace {
        metadata: ObjectMeta {
            name: Some(name),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_configmap() {
        let mut data = BTreeMap::new();
        data.insert("key".to_string(), "value".to_string());
        let cm = create_configmap("test-cm", "default", data);

        assert_eq!(cm.metadata.name, Some("test-cm".to_string()));
        assert_eq!(cm.metadata.namespace, Some("default".to_string()));
        assert!(cm.data.is_some());
    }

    #[test]
    fn test_create_secret() {
        let mut data = BTreeMap::new();
        data.insert("password".to_string(), "secret123".to_string());
        let secret = create_secret("test-secret", "default", data);

        assert_eq!(secret.metadata.name, Some("test-secret".to_string()));
        assert_eq!(secret.metadata.namespace, Some("default".to_string()));
        assert!(secret.string_data.is_some());
    }

    #[test]
    fn test_create_pvc() {
        let pvc = create_pvc(
            "test-pvc",
            "default",
            "1Gi",
            vec!["ReadWriteOnce".to_string()],
        );

        assert_eq!(pvc.metadata.name, Some("test-pvc".to_string()));
        assert_eq!(pvc.metadata.namespace, Some("default".to_string()));
        assert!(pvc.spec.is_some());
    }

    #[test]
    fn test_create_namespace() {
        let ns = create_namespace("test");

        assert!(ns.metadata.name.unwrap().starts_with("test-"));
    }
}
