use crate::entities::ContainerLike;
use crate::steps::ResourceLimits;
use k8s_openapi::api::core::v1::Container;
use serde_json::Value;
use std::collections::BTreeMap;

/// Configuration type for sidecar containers using a flexible key-value store.
///
/// This is a BTreeMap with JSON Values, allowing for arbitrary configuration
/// options to be passed to sidecar containers.
///
/// # Example
///
/// ```rust
/// use k8s_maestro::steps::SidecarConfig;
/// use serde_json::json;
///
/// let mut config = SidecarConfig::new();
/// config.insert("buffer_size".to_string(), json!(1024));
/// config.insert("log_level".to_string(), json!("debug"));
/// ```
pub type SidecarConfig = BTreeMap<String, Value>;

/// Represents a port configuration for a sidecar container.
///
/// This struct defines how a container port is exposed, including optional
/// host port mapping, protocol specification, and port naming.
///
/// # Example
///
/// ```ignore
/// use k8s_maestro::steps::SidecarBuilder;
///
/// // Builder uses port method internally
/// let builder = SidecarBuilder::new("nginx:latest")
///     .with_port(8080);
/// ```
#[derive(Debug, Clone)]
pub struct ContainerPort {
    /// The port number exposed inside the container.
    pub container_port: u16,
    /// Optional host port to map to the container port.
    pub host_port: Option<u16>,
    /// Network protocol (typically TCP or UDP).
    pub protocol: Option<String>,
    /// Optional name for the port (useful for service discovery).
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

/// A sidecar container that runs alongside the main container in a Kubernetes pod.
///
/// Sidecar containers extend the functionality of the main container by providing
/// supporting services like logging, monitoring, or proxying.
///
/// # Example
///
/// ```rust
/// use k8s_maestro::SidecarContainer;
/// use serde_json::json;
///
/// let sidecar = SidecarContainer::new("nginx:1.21", "nginx-sidecar")
///     .with_config("proxy_timeout", json!(60))
///     .with_env("LOG_LEVEL", "info");
/// ```
#[derive(Debug)]
pub struct SidecarContainer {
    /// The name of the sidecar container.
    pub name: String,
    /// The container image to use.
    pub image: String,
    /// Additional configuration options as key-value pairs.
    pub config: SidecarConfig,
    /// Ports to expose on the sidecar container.
    pub ports: Vec<ContainerPort>,
    /// Environment variables to set in the container.
    pub env: BTreeMap<String, String>,
    /// Volume mounts for the container.
    pub volume_mounts: Vec<String>,
    /// Optional resource limits (CPU, memory) for the container.
    pub resource_limits: Option<ResourceLimits>,
}

impl SidecarContainer {
    /// Creates a new sidecar container with the specified image and name.
    ///
    /// # Arguments
    ///
    /// * `image` - The container image (e.g., "nginx:1.21")
    /// * `name` - The name for the sidecar container
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::SidecarContainer;
    /// let sidecar = SidecarContainer::new("nginx:latest", "my-sidecar");
    /// ```
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

    /// Adds a configuration key-value pair to the sidecar.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key
    /// * `value` - The configuration value (JSON)
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::SidecarContainer;
    /// use serde_json::json;
    /// let sidecar = SidecarContainer::new("nginx:latest", "my-sidecar")
    ///     .with_config("timeout", json!(30));
    /// ```
    pub fn with_config(mut self, key: &str, value: Value) -> Self {
        self.config.insert(key.to_string(), value);
        self
    }

    /// Adds an environment variable to the sidecar container.
    ///
    /// # Arguments
    ///
    /// * `key` - The environment variable name
    /// * `value` - The environment variable value
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::SidecarContainer;
    /// let sidecar = SidecarContainer::new("nginx:latest", "my-sidecar")
    ///     .with_env("LOG_LEVEL", "debug");
    /// ```
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    /// Sets resource limits for the sidecar container.
    ///
    /// # Arguments
    ///
    /// * `limits` - The resource limits to apply
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::{SidecarContainer, ResourceLimits};
    /// let limits = ResourceLimits::new()
    ///     .with_cpu("500m")
    ///     .with_memory("256Mi");
    /// let sidecar = SidecarContainer::new("nginx:latest", "my-sidecar")
    ///     .with_resource_limits(limits);
    /// ```
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

/// Builder for creating [`SidecarContainer`] instances.
///
/// This builder provides a fluent interface for constructing sidecar containers
/// with validation at build time.
///
/// # Example
///
/// ```rust
/// use k8s_maestro::steps::SidecarBuilder;
/// use serde_json::json;
///
/// let sidecar = SidecarBuilder::new("nginx:1.21")
///     .with_name("proxy")
///     .with_port(8080)
///     .with_env("PROXY_MODE", "reverse")
///     .with_config("timeout", json!(60))
///     .build()
///     .expect("Failed to build sidecar");
/// ```
pub struct SidecarBuilder {
    image: Option<String>,
    name: Option<String>,
    config: SidecarConfig,
    ports: Vec<ContainerPort>,
    env: BTreeMap<String, String>,
    resource_limits: Option<ResourceLimits>,
}

impl SidecarBuilder {
    /// Creates a new builder with the required image.
    ///
    /// # Arguments
    ///
    /// * `image` - The container image (e.g., "nginx:1.21")
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::steps::SidecarBuilder;
    /// let builder = SidecarBuilder::new("nginx:latest");
    /// ```
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

    /// Sets the name of the sidecar container.
    ///
    /// # Arguments
    ///
    /// * `name` - The container name
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::steps::SidecarBuilder;
    /// let builder = SidecarBuilder::new("nginx:latest")
    ///     .with_name("my-proxy");
    /// ```
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Adds a port to expose on the sidecar container.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number inside the container
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::steps::SidecarBuilder;
    /// let builder = SidecarBuilder::new("nginx:latest")
    ///     .with_port(8080);
    /// ```
    pub fn with_port(mut self, port: u16) -> Self {
        self.ports.push(ContainerPort::new(port));
        self
    }

    /// Adds a configuration key-value pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key
    /// * `value` - The configuration value (JSON)
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::steps::SidecarBuilder;
    /// use serde_json::json;
    /// let builder = SidecarBuilder::new("nginx:latest")
    ///     .with_config("timeout", json!(30));
    /// ```
    pub fn with_config(mut self, key: &str, value: Value) -> Self {
        self.config.insert(key.to_string(), value);
        self
    }

    /// Adds an environment variable.
    ///
    /// # Arguments
    ///
    /// * `key` - The environment variable name
    /// * `value` - The environment variable value
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::steps::SidecarBuilder;
    /// let builder = SidecarBuilder::new("nginx:latest")
    ///     .with_env("LOG_LEVEL", "debug");
    /// ```
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    /// Sets resource limits for the sidecar.
    ///
    /// # Arguments
    ///
    /// * `limits` - The resource limits to apply
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::steps::{SidecarBuilder, ResourceLimits};
    /// let limits = ResourceLimits::new().with_cpu("500m");
    /// let builder = SidecarBuilder::new("nginx:latest")
    ///     .with_resource_limits(limits);
    /// ```
    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = Some(limits);
        self
    }

    /// Builds the [`SidecarContainer`].
    ///
    /// Returns an error if image or name is not set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::steps::SidecarBuilder;
    ///
    /// let sidecar = SidecarBuilder::new("nginx:latest")
    ///     .with_name("proxy")
    ///     .build()
    ///     .expect("Failed to build sidecar");
    /// ```
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
