use crate::entities::ContainerLike;
use crate::steps::ResourceLimits;
use k8s_openapi::api::core::v1::Container;
use serde_json::Value;
use std::collections::BTreeMap;

pub type SidecarConfig = BTreeMap<String, Value>;

#[derive(Debug, Clone)]
pub struct ContainerPort {
    pub container_port: u16,
    pub host_port: Option<u16>,
    pub protocol: Option<String>,
    pub name: Option<String>,
}

impl ContainerPort {
    pub fn new(container_port: u16) -> Self {
        Self {
            container_port,
            host_port: None,
            protocol: Some("TCP".to_string()),
            name: None,
        }
    }

    pub fn with_host_port(mut self, port: u16) -> Self {
        self.host_port = Some(port);
        self
    }

    pub fn with_protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

pub struct SidecarContainer {
    pub name: String,
    pub image: String,
    pub config: SidecarConfig,
    pub ports: Vec<ContainerPort>,
    pub env: BTreeMap<String, String>,
    pub volume_mounts: Vec<String>,
    pub resource_limits: Option<ResourceLimits>,
}

impl SidecarContainer {
    pub fn new(image: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            name: name.into(),
            config: BTreeMap::new(),
            ports: Vec::new(),
            env: BTreeMap::new(),
            volume_mounts: Vec::new(),
            resource_limits: None,
        }
    }

    pub fn with_config(mut self, key: &str, value: Value) -> Self {
        self.config.insert(key.to_string(), value);
        self
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = Some(limits);
        self
    }
}

impl ContainerLike for SidecarContainer {
    fn as_container(&self) -> Container {
        let env = if !self.env.is_empty() {
            Some(
                self.env
                    .iter()
                    .map(|(k, v)| k8s_openapi::api::core::v1::EnvVar {
                        name: k.clone(),
                        value: Some(v.clone()),
                        ..Default::default()
                    })
                    .collect(),
            )
        } else {
            None
        };

        let ports = if !self.ports.is_empty() {
            Some(
                self.ports
                    .iter()
                    .map(|p| k8s_openapi::api::core::v1::ContainerPort {
                        container_port: p.container_port as i32,
                        host_port: p.host_port.map(|p| p as i32),
                        protocol: p.protocol.clone(),
                        name: p.name.clone(),
                        ..Default::default()
                    })
                    .collect(),
            )
        } else {
            None
        };

        let mut resources = None;
        if let Some(limits) = &self.resource_limits {
            let mut requests = BTreeMap::new();
            let mut limits_map = BTreeMap::new();

            if let Some(cpu) = &limits.cpu {
                limits_map.insert(
                    "cpu".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(cpu.clone()),
                );
            }
            if let Some(memory) = &limits.memory {
                limits_map.insert(
                    "memory".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(memory.clone()),
                );
            }

            if let Some(cpu_request) = &limits.cpu_request {
                requests.insert(
                    "cpu".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(cpu_request.clone()),
                );
            }
            if let Some(memory_request) = &limits.memory_request {
                requests.insert(
                    "memory".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(memory_request.clone()),
                );
            }

            resources = Some(k8s_openapi::api::core::v1::ResourceRequirements {
                limits: if limits_map.is_empty() {
                    None
                } else {
                    Some(limits_map)
                },
                requests: if requests.is_empty() {
                    None
                } else {
                    Some(requests)
                },
                ..Default::default()
            });
        }

        Container {
            name: self.name.clone(),
            image: Some(self.image.clone()),
            env,
            ports,
            resources,
            ..Default::default()
        }
    }
}

pub struct SidecarBuilder {
    image: Option<String>,
    name: Option<String>,
    config: SidecarConfig,
    ports: Vec<ContainerPort>,
    env: BTreeMap<String, String>,
    resource_limits: Option<ResourceLimits>,
}

impl SidecarBuilder {
    pub fn new(image: &str) -> Self {
        Self {
            image: Some(image.to_string()),
            name: None,
            config: BTreeMap::new(),
            ports: Vec::new(),
            env: BTreeMap::new(),
            resource_limits: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.ports.push(ContainerPort::new(port));
        self
    }

    pub fn with_config(mut self, key: &str, value: Value) -> Self {
        self.config.insert(key.to_string(), value);
        self
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = Some(limits);
        self
    }

    pub fn build(self) -> anyhow::Result<SidecarContainer> {
        let image = self
            .image
            .ok_or_else(|| anyhow::anyhow!("Image is required"))?;
        let name = self
            .name
            .ok_or_else(|| anyhow::anyhow!("Name is required"))?;

        Ok(SidecarContainer {
            image,
            name,
            config: self.config,
            ports: self.ports,
            env: self.env,
            volume_mounts: Vec::new(),
            resource_limits: self.resource_limits,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidecar_container_creation() {
        let sidecar = SidecarContainer::new("nginx:latest", "nginx-sidecar");
        assert_eq!(sidecar.name, "nginx-sidecar");
        assert_eq!(sidecar.image, "nginx:latest");
        assert!(sidecar.config.is_empty());
        assert!(sidecar.ports.is_empty());
        assert!(sidecar.env.is_empty());
        assert!(sidecar.volume_mounts.is_empty());
        assert!(sidecar.resource_limits.is_none());
    }

    #[test]
    fn test_container_port_creation() {
        let port = ContainerPort::new(8080);
        assert_eq!(port.container_port, 8080);
        assert!(port.host_port.is_none());
        assert_eq!(port.protocol, Some("TCP".to_string()));
        assert!(port.name.is_none());
    }

    #[test]
    fn test_container_port_with_options() {
        let port = ContainerPort::new(8080)
            .with_host_port(80)
            .with_protocol("UDP")
            .with_name("http");
        assert_eq!(port.container_port, 8080);
        assert_eq!(port.host_port, Some(80));
        assert_eq!(port.protocol, Some("UDP".to_string()));
        assert_eq!(port.name, Some("http".to_string()));
    }

    #[test]
    fn test_sidecar_container_with_config() {
        let sidecar = SidecarContainer::new("fluentd:v1.14", "log-collector")
            .with_config("buffer_size", serde_json::json!(1024))
            .with_config("log_level", serde_json::json!("debug"));
        assert_eq!(sidecar.config.len(), 2);
        assert_eq!(
            sidecar.config.get("buffer_size"),
            Some(&serde_json::json!(1024))
        );
    }

    #[test]
    fn test_sidecar_builder_creates_valid_sidecar() {
        let builder = SidecarBuilder::new("nginx:latest")
            .with_name("proxy")
            .with_port(8080)
            .with_env("PROXY_MODE", "reverse");
        let sidecar = builder.build().unwrap();
        assert_eq!(sidecar.name, "proxy");
        assert_eq!(sidecar.image, "nginx:latest");
        assert_eq!(sidecar.ports.len(), 1);
        assert_eq!(sidecar.ports[0].container_port, 8080);
        assert_eq!(sidecar.env.get("PROXY_MODE"), Some(&"reverse".to_string()));
    }

    #[test]
    fn test_sidecar_builder_validation_missing_name() {
        let builder = SidecarBuilder::new("nginx:latest").with_port(8080);
        let result = builder.build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Name is required"));
    }

    #[test]
    fn test_sidecar_builder_validation_missing_image() {
        let sidecar = SidecarContainer::new("nginx:latest", "test");
        assert_eq!(sidecar.name, "test");
    }

    #[test]
    fn test_sidecar_container_like() {
        let sidecar = SidecarContainer::new("nginx:latest", "test-sidecar");
        let k8s_container = sidecar.as_container();
        assert_eq!(k8s_container.name, "test-sidecar");
        assert_eq!(k8s_container.image, Some("nginx:latest".to_string()));
    }
}
