#[cfg(test)]
mod tests {
    use crate::IngressBuilder;
    use crate::IngressPath;
    use crate::PathType;
    use crate::TLSConfig;
    use std::collections::BTreeMap;

    #[test]
    fn test_ingress_builder_basic() {
        let ingress = IngressBuilder::new()
            .with_name("test-ingress")
            .with_namespace("default")
            .with_host("example.com")
            .with_path("/", "test-service", 80)
            .build()
            .expect("Should build ingress");

        assert_eq!(ingress.metadata.name, Some("test-ingress".to_string()));
        assert_eq!(ingress.metadata.namespace, Some("default".to_string()));
        assert_eq!(
            ingress.spec.as_ref().unwrap().rules.as_ref().unwrap().len(),
            1
        );
    }

    #[test]
    fn test_ingress_builder_tls() {
        let ingress = IngressBuilder::new()
            .with_name("tls-ingress")
            .with_namespace("default")
            .with_host("secure.example.com")
            .with_path("/", "secure-service", 443)
            .with_tls_secret("tls-secret")
            .build()
            .expect("Should build ingress with TLS");

        assert!(ingress.spec.as_ref().unwrap().tls.is_some());
        let tls = ingress.spec.as_ref().unwrap().tls.as_ref().unwrap();
        assert_eq!(tls.len(), 1);
        assert_eq!(tls[0].secret_name.as_ref().unwrap(), "tls-secret");
    }

    #[test]
    fn test_ingress_builder_multiple_paths() {
        let paths = vec![
            IngressPath {
                path: "/api".to_string(),
                path_type: PathType::Prefix,
                service_name: "api-service".to_string(),
                service_port: 8080,
            },
            IngressPath {
                path: "/static".to_string(),
                path_type: PathType::Prefix,
                service_name: "static-service".to_string(),
                service_port: 80,
            },
        ];

        let ingress = IngressBuilder::new()
            .with_name("multi-path-ingress")
            .with_namespace("default")
            .with_host("example.com")
            .with_paths(paths)
            .build()
            .expect("Should build ingress with multiple paths");

        assert_eq!(
            ingress.spec.as_ref().unwrap().rules.as_ref().unwrap()[0]
                .http
                .as_ref()
                .unwrap()
                .paths
                .len(),
            2
        );
    }

    #[test]
    fn test_ingress_builder_validation_missing_name() {
        let result = IngressBuilder::new()
            .with_namespace("default")
            .with_host("example.com")
            .with_path("/", "test-service", 80)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name is required"));
    }

    #[test]
    fn test_ingress_builder_validation_missing_namespace() {
        let result = IngressBuilder::new()
            .with_name("test-ingress")
            .with_host("example.com")
            .with_path("/", "test-service", 80)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("namespace is required"));
    }

    #[test]
    fn test_ingress_builder_validation_missing_host() {
        let result = IngressBuilder::new()
            .with_name("test-ingress")
            .with_namespace("default")
            .with_path("/", "test-service", 80)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Host is required"));
    }

    #[test]
    fn test_ingress_builder_validation_missing_paths() {
        let result = IngressBuilder::new()
            .with_name("test-ingress")
            .with_namespace("default")
            .with_host("example.com")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("path is required"));
    }

    #[test]
    fn test_ingress_builder_with_annotations() {
        let mut annotations = BTreeMap::new();
        annotations.insert(
            "nginx.ingress.kubernetes.io/rewrite-target".to_string(),
            "/$2".to_string(),
        );
        annotations.insert(
            "cert-manager.io/cluster-issuer".to_string(),
            "letsencrypt-prod".to_string(),
        );

        let ingress = IngressBuilder::new()
            .with_name("annotated-ingress")
            .with_namespace("default")
            .with_host("example.com")
            .with_path("/", "test-service", 80)
            .with_annotations(annotations.clone())
            .build()
            .expect("Should build ingress with annotations");

        assert_eq!(ingress.metadata.annotations.as_ref().unwrap(), &annotations);
    }

    #[test]
    fn test_ingress_builder_with_ingress_class() {
        let ingress = IngressBuilder::new()
            .with_name("class-ingress")
            .with_namespace("default")
            .with_host("example.com")
            .with_path("/", "test-service", 80)
            .with_ingress_class("nginx")
            .build()
            .expect("Should build ingress with ingress class");

        assert_eq!(
            ingress
                .spec
                .as_ref()
                .unwrap()
                .ingress_class_name
                .as_ref()
                .unwrap(),
            "nginx"
        );
    }

    #[test]
    fn test_ingress_path_default() {
        let path = IngressPath::default();
        assert_eq!(path.path, "/");
        assert_eq!(path.path_type, PathType::Prefix);
        assert_eq!(path.service_name, "");
        assert_eq!(path.service_port, 80);
    }

    #[test]
    fn test_tls_config_default() {
        let tls = TLSConfig::default();
        assert!(tls.hosts.is_empty());
        assert_eq!(tls.secret_name, "");
    }
}
