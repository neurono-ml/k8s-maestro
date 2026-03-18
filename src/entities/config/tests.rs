use crate::entities::config::{
    ConfigMapBuilder, ImagePullSecretBuilder, SecretBuilder, SecretType,
};
use std::collections::BTreeMap;

    #[test]
    fn test_configmap_builder_basic() {
        let configmap = ConfigMapBuilder::new("test-config")
            .build()
            .expect("Failed to build ConfigMap");

        assert_eq!(configmap.metadata.name, Some("test-config".to_string()));
    }

    #[test]
    fn test_configmap_builder_with_options() {
        let configmap = ConfigMapBuilder::new("test-config")
            .with_namespace("test-ns")
            .with_data("key1", "value1")
            .with_labels(BTreeMap::from([("app".to_string(), "test".to_string())]))
            .build()
            .expect("Failed to build ConfigMap");

        assert_eq!(configmap.metadata.name, Some("test-config".to_string()));
        assert_eq!(configmap.metadata.namespace, Some("test-ns".to_string()));
        assert_eq!(configmap.data.is_some(), true);
    }

    #[test]
    fn test_secret_builder_basic() {
        let secret = SecretBuilder::new("test-secret")
            .build()
            .expect("Failed to build Secret");

        assert_eq!(secret.metadata.name, Some("test-secret".to_string()));
        assert_eq!(secret.type_, Some("Opaque".to_string()));
    }

    #[test]
    fn test_secret_type_display() {
        assert_eq!(SecretType::Opaque.to_string(), "Opaque");
        assert_eq!(SecretType::Tls.to_string(), "kubernetes.io/tls");
        assert_eq!(
            SecretType::DockerConfigJson.to_string(),
            "kubernetes.io/dockerconfigjson"
        );
    }

    #[test]
    fn test_image_pull_secret_builder() {
        let secret = ImagePullSecretBuilder::new("test-registry-secret")
            .with_registry("https://index.docker.io/v1/")
            .with_username("testuser")
            .with_password("testpass")
            .with_email("test@example.com")
            .build()
            .expect("Failed to build ImagePullSecret");

        assert_eq!(
            secret.metadata.name,
            Some("test-registry-secret".to_string())
        );
        assert_eq!(
            secret.type_,
            Some("kubernetes.io/dockerconfigjson".to_string())
        );
    }

    #[test]
    fn test_image_pull_secret_builder_missing_registry() {
        let result = ImagePullSecretBuilder::new("test-secret")
            .with_username("testuser")
            .with_password("testpass")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("registry"));
    }

    #[test]
    fn test_image_pull_secret_builder_missing_username() {
        let result = ImagePullSecretBuilder::new("test-secret")
            .with_registry("https://index.docker.io/v1/")
            .with_password("testpass")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("username"));
    }

    #[test]
    fn test_image_pull_secret_builder_missing_password() {
        let result = ImagePullSecretBuilder::new("test-secret")
            .with_registry("https://index.docker.io/v1/")
            .with_username("testuser")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("password"));
    }
}

    #[test]
    fn test_configmap_builder_with_options() {
        let configmap = ConfigMapBuilder::new("test-config")
            .with_namespace("test-ns")
            .with_data("key1", "value1")
            .with_labels(BTreeMap::from([("app".to_string(), "test".to_string())]))
            .build()
            .expect("Failed to build ConfigMap");

        assert_eq!(configmap.metadata.name, Some("test-config".to_string()));
        assert_eq!(configmap.metadata.namespace, Some("test-ns".to_string()));
        assert!(configmap.data.is_some());
    }

    #[test]
    fn test_secret_builder_basic() {
        let secret = SecretBuilder::new("test-secret")
            .build()
            .expect("Failed to build Secret");

        assert_eq!(secret.metadata.name, Some("test-secret".to_string()));
        assert_eq!(secret.type_, Some("Opaque".to_string()));
    }

    #[test]
    fn test_secret_type_display() {
        assert_eq!(SecretType::Opaque.to_string(), "Opaque");
        assert_eq!(SecretType::Tls.to_string(), "kubernetes.io/tls");
        assert_eq!(
            SecretType::DockerConfigJson.to_string(),
            "kubernetes.io/dockerconfigjson"
        );
    }

    #[test]
    fn test_image_pull_secret_builder() {
        let secret = ImagePullSecretBuilder::new("test-registry-secret")
            .with_registry("https://index.docker.io/v1/")
            .with_username("testuser")
            .with_password("testpass")
            .with_email("test@example.com")
            .build()
            .expect("Failed to build ImagePullSecret");

        assert_eq!(
            secret.metadata.name,
            Some("test-registry-secret".to_string())
        );
        assert_eq!(
            secret.type_,
            Some("kubernetes.io/dockerconfigjson".to_string())
        );
    }

    #[test]
    fn test_image_pull_secret_builder_missing_registry() {
        let result = ImagePullSecretBuilder::new("test-secret")
            .with_username("testuser")
            .with_password("testpass")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("registry"));
    }

    #[test]
    fn test_image_pull_secret_builder_missing_username() {
        let result = ImagePullSecretBuilder::new("test-secret")
            .with_registry("https://index.docker.io/v1/")
            .with_password("testpass")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("username"));
    }
