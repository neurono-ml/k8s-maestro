#[cfg(test)]
mod tests {
    use crate::ServiceBuilder;
    use crate::ServicePort;
    use crate::ServiceType;
    use std::collections::BTreeMap;

    #[test]
    fn test_service_builder_clusterip() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "test".to_string());

        let service = ServiceBuilder::new()
            .with_name("test-service")
            .with_namespace("default")
            .with_port(80, 8080, "TCP")
            .with_selector(selector.clone())
            .with_type(ServiceType::ClusterIP)
            .build()
            .expect("Should build service");

        assert_eq!(service.metadata.name, Some("test-service".to_string()));
        assert_eq!(service.metadata.namespace, Some("default".to_string()));
        assert_eq!(
            service.spec.as_ref().unwrap().type_,
            Some("ClusterIP".to_string())
        );
        assert_eq!(
            service.spec.as_ref().unwrap().ports.as_ref().unwrap().len(),
            1
        );
        assert_eq!(
            service.spec.as_ref().unwrap().selector.as_ref().unwrap(),
            &selector
        );
    }

    #[test]
    fn test_service_builder_headless() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "stateful".to_string());

        let service = ServiceBuilder::new()
            .with_name("headless-service")
            .with_namespace("default")
            .with_port(80, 8080, "TCP")
            .with_selector(selector)
            .with_type(ServiceType::Headless)
            .build()
            .expect("Should build headless service");

        assert_eq!(
            service.spec.as_ref().unwrap().cluster_ip,
            Some("None".to_string())
        );
        assert_eq!(
            service.spec.as_ref().unwrap().type_,
            Some("ClusterIP".to_string())
        );
    }

    #[test]
    fn test_service_builder_nodeport() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "test".to_string());

        let service = ServiceBuilder::new()
            .with_name("nodeport-service")
            .with_namespace("default")
            .with_port(80, 8080, "TCP")
            .with_selector(selector)
            .with_type(ServiceType::NodePort)
            .build()
            .expect("Should build NodePort service");

        assert_eq!(
            service.spec.as_ref().unwrap().type_,
            Some("NodePort".to_string())
        );
    }

    #[test]
    fn test_service_builder_loadbalancer() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "test".to_string());

        let service = ServiceBuilder::new()
            .with_name("lb-service")
            .with_namespace("default")
            .with_port(80, 8080, "TCP")
            .with_selector(selector)
            .with_type(ServiceType::LoadBalancer)
            .build()
            .expect("Should build LoadBalancer service");

        assert_eq!(
            service.spec.as_ref().unwrap().type_,
            Some("LoadBalancer".to_string())
        );
    }

    #[test]
    fn test_service_builder_validation_missing_name() {
        let result = ServiceBuilder::new()
            .with_namespace("default")
            .with_port(80, 8080, "TCP")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name is required"));
    }

    #[test]
    fn test_service_builder_validation_missing_namespace() {
        let result = ServiceBuilder::new()
            .with_name("test-service")
            .with_port(80, 8080, "TCP")
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("namespace is required"));
    }

    #[test]
    fn test_service_builder_multiple_ports() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "test".to_string());

        let ports = vec![
            ServicePort {
                port: 80,
                target_port: 8080,
                protocol: "TCP".to_string(),
                name: Some("http".to_string()),
            },
            ServicePort {
                port: 443,
                target_port: 8443,
                protocol: "TCP".to_string(),
                name: Some("https".to_string()),
            },
        ];

        let service = ServiceBuilder::new()
            .with_name("multi-port-service")
            .with_namespace("default")
            .with_ports(ports)
            .with_selector(selector)
            .build()
            .expect("Should build service with multiple ports");

        assert_eq!(
            service.spec.as_ref().unwrap().ports.as_ref().unwrap().len(),
            2
        );
    }

    #[test]
    fn test_service_builder_advanced_options() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "test".to_string());

        let service = ServiceBuilder::new()
            .with_name("advanced-service")
            .with_namespace("default")
            .with_port(80, 8080, "TCP")
            .with_selector(selector)
            .with_session_affinity("ClientIP")
            .with_external_traffic_policy("Local")
            .build()
            .expect("Should build service with advanced options");

        assert_eq!(
            service
                .spec
                .as_ref()
                .unwrap()
                .session_affinity
                .as_ref()
                .unwrap(),
            "ClientIP"
        );
        assert_eq!(
            service
                .spec
                .as_ref()
                .unwrap()
                .external_traffic_policy
                .as_ref()
                .unwrap(),
            "Local"
        );
    }

    #[test]
    fn test_service_port_default() {
        let port = ServicePort::default();
        assert_eq!(port.port, 80);
        assert_eq!(port.target_port, 8080);
        assert_eq!(port.protocol, "TCP");
        assert!(port.name.is_none());
    }
}
