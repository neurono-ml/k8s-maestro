use crate::networking::{IngressBuilder, ServiceBuilder, ServiceType};
use k8s_openapi::api::core::v1::Service;
use k8s_openapi::api::networking::v1::Ingress;
use std::collections::BTreeMap;

#[derive(Clone)]
pub enum JobNameType {
    DefinedName(String),
    GenerateName(String),
}

impl JobNameType {
    pub fn is_defined(&self) -> bool {
        matches!(self, JobNameType::DefinedName(_))
    }

    pub fn is_generated(&self) -> bool {
        matches!(self, JobNameType::GenerateName(_))
    }
}

pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

impl RestartPolicy {
    pub fn as_k8s_str(&self) -> &str {
        match self {
            RestartPolicy::Never => "Never",
            RestartPolicy::OnFailure => "OnFailure",
            RestartPolicy::Always => "Always",
        }
    }
}

pub struct ServiceConfig {
    pub service_name: String,
    pub port: u16,
    pub target_port: Option<u16>,
    pub service_type: ServiceType,
    pub selector: BTreeMap<String, String>,
}

impl ServiceConfig {
    pub fn new(service_name: impl Into<String>, port: u16) -> Self {
        Self {
            service_name: service_name.into(),
            port,
            target_port: None,
            service_type: ServiceType::ClusterIP,
            selector: BTreeMap::new(),
        }
    }

    pub fn with_target_port(mut self, target_port: u16) -> Self {
        self.target_port = Some(target_port);
        self
    }

    pub fn with_service_type(mut self, service_type: ServiceType) -> Self {
        self.service_type = service_type;
        self
    }

    pub fn with_selector(mut self, labels: BTreeMap<String, String>) -> Self {
        self.selector = labels;
        self
    }

    pub fn build_service(&self, namespace: &str) -> anyhow::Result<Service> {
        let port = self.target_port.unwrap_or(self.port) as i32;
        ServiceBuilder::new()
            .with_name(&self.service_name)
            .with_namespace(namespace)
            .with_port(self.port as i32, port, "TCP")
            .with_selector(self.selector.clone())
            .with_type(self.service_type)
            .build()
    }
}

pub struct IngressConfig {
    pub ingress_name: String,
    pub host: String,
    pub path: String,
    pub service_name: String,
    pub service_port: u16,
    pub tls_secret: Option<String>,
    pub annotations: Option<BTreeMap<String, String>>,
}

impl IngressConfig {
    pub fn new(
        ingress_name: impl Into<String>,
        host: impl Into<String>,
        service_name: impl Into<String>,
        service_port: u16,
    ) -> Self {
        Self {
            ingress_name: ingress_name.into(),
            host: host.into(),
            path: "/".to_string(),
            service_name: service_name.into(),
            service_port,
            tls_secret: None,
            annotations: None,
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    pub fn with_tls_secret(mut self, secret_name: impl Into<String>) -> Self {
        self.tls_secret = Some(secret_name.into());
        self
    }

    pub fn with_annotations(mut self, annotations: BTreeMap<String, String>) -> Self {
        self.annotations = Some(annotations);
        self
    }

    pub fn build_ingress(&self, namespace: &str) -> anyhow::Result<Ingress> {
        let mut builder = IngressBuilder::new()
            .with_name(&self.ingress_name)
            .with_namespace(namespace)
            .with_host(&self.host)
            .with_path(&self.path, &self.service_name, self.service_port as i32);

        if let Some(tls_secret) = &self.tls_secret {
            builder = builder.with_tls_secret(tls_secret);
        }

        if let Some(annotations) = &self.annotations {
            builder = builder.with_annotations(annotations.clone());
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_name_type_defined() {
        let name_type = JobNameType::DefinedName("my-job".to_string());
        assert!(name_type.is_defined());
        assert!(!name_type.is_generated());
    }

    #[test]
    fn test_job_name_type_generated() {
        let name_type = JobNameType::GenerateName("job-".to_string());
        assert!(!name_type.is_defined());
        assert!(name_type.is_generated());
    }

    #[test]
    fn test_restart_policy_k8s_str() {
        assert_eq!(RestartPolicy::Never.as_k8s_str(), "Never");
        assert_eq!(RestartPolicy::OnFailure.as_k8s_str(), "OnFailure");
        assert_eq!(RestartPolicy::Always.as_k8s_str(), "Always");
    }

    #[test]
    fn test_service_config_basic() {
        let config = ServiceConfig::new("my-service", 8080);
        assert_eq!(config.service_name, "my-service");
        assert_eq!(config.port, 8080);
        assert!(config.target_port.is_none());
    }

    #[test]
    fn test_service_config_with_target_port() {
        let config = ServiceConfig::new("my-service", 80).with_target_port(8080);
        assert_eq!(config.port, 80);
        assert_eq!(config.target_port, Some(8080));
    }

    #[test]
    fn test_service_config_with_selector() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "myapp".to_string());
        let config = ServiceConfig::new("my-service", 8080).with_selector(selector.clone());
        assert_eq!(config.selector, selector);
    }

    #[test]
    fn test_ingress_config_basic() {
        let config = IngressConfig::new("my-ingress", "example.com", "my-service", 80);
        assert_eq!(config.ingress_name, "my-ingress");
        assert_eq!(config.host, "example.com");
        assert_eq!(config.service_name, "my-service");
        assert_eq!(config.service_port, 80);
        assert_eq!(config.path, "/");
        assert!(config.tls_secret.is_none());
    }

    #[test]
    fn test_ingress_config_with_path() {
        let config =
            IngressConfig::new("my-ingress", "example.com", "my-service", 80).with_path("/api");
        assert_eq!(config.path, "/api");
    }

    #[test]
    fn test_ingress_config_with_tls() {
        let config = IngressConfig::new("my-ingress", "example.com", "my-service", 80)
            .with_tls_secret("tls-secret");
        assert_eq!(config.tls_secret, Some("tls-secret".to_string()));
    }

    #[test]
    fn test_ingress_config_with_annotations() {
        let mut annotations = BTreeMap::new();
        annotations.insert(
            "nginx.ingress.kubernetes.io/rewrite-target".to_string(),
            "/".to_string(),
        );
        let config = IngressConfig::new("my-ingress", "example.com", "my-service", 80)
            .with_annotations(annotations.clone());
        assert_eq!(config.annotations, Some(annotations));
    }
}
