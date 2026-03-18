#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_dns_name() {
        let dns = service_dns_name("my-service", "default");
        assert_eq!(dns, "my-service.default.svc.cluster.local");
    }

    #[test]
    fn test_service_dns_name_custom_namespace() {
        let dns = service_dns_name("api-server", "production");
        assert_eq!(dns, "api-server.production.svc.cluster.local");
    }

    #[test]
    fn test_pod_dns_name() {
        let dns = pod_dns_name("my-pod", "default");
        assert_eq!(dns, "my-pod.default.pod.cluster.local");
    }

    #[test]
    fn test_headless_service_dns_pattern() {
        let dns = headless_service_dns_pattern("stateful-set", "default");
        assert_eq!(dns, "*.stateful-set.default.svc.cluster.local");
    }

    #[test]
    fn test_dns_with_special_characters() {
        let dns = service_dns_name("my-service-123", "my-namespace");
        assert_eq!(dns, "my-service-123.my-namespace.svc.cluster.local");
    }
}
