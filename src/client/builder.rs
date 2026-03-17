//! Builder for constructing [`MaestroClient`] instances with fluent API.
//!
//! # Example
//!
//! ```no_run
//! use k8s_maestro::MaestroClientBuilder;
//!
//! let client = MaestroClientBuilder::new()
//!     .with_namespace("production")
//!     .with_dry_run(true)
//!     .build()
//!     .unwrap();
//! ```

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;

use super::maestro_client::MaestroClient;
use crate::steps::traits::ResourceLimits;

/// Builder for constructing [`MaestroClient`] instances.
///
/// The builder provides a fluent API for configuring the client with
/// all available options before construction.
pub struct MaestroClientBuilder {
    kube_config_path: Option<PathBuf>,
    namespace: Option<String>,
    dry_run: bool,
    default_timeout: Option<Duration>,
    log_level: Option<String>,
    default_resource_limits: Option<ResourceLimits>,
}

impl Default for MaestroClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MaestroClientBuilder {
    /// Creates a new builder with default values.
    ///
    /// # Defaults
    ///
    /// - namespace: `"default"`
    /// - dry_run: `false`
    /// - All other fields: `None`
    pub fn new() -> Self {
        Self {
            kube_config_path: None,
            namespace: None,
            dry_run: false,
            default_timeout: None,
            log_level: None,
            default_resource_limits: None,
        }
    }

    /// Sets the path to the kubeconfig file.
    ///
    /// If not set, the default Kubernetes configuration locations are used.
    pub fn with_kube_config(mut self, path: impl Into<PathBuf>) -> Self {
        self.kube_config_path = Some(path.into());
        self
    }

    /// Sets the default namespace for operations.
    ///
    /// If not set, defaults to `"default"`.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Enables or disables dry run mode.
    ///
    /// In dry run mode, operations are validated but not executed.
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Sets the default timeout for operations.
    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = Some(timeout);
        self
    }

    /// Sets the log level for client operations.
    ///
    /// Valid values include: `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`.
    pub fn with_log_level(mut self, level: impl Into<String>) -> Self {
        self.log_level = Some(level.into());
        self
    }

    /// Sets default resource limits for workflows.
    pub fn with_default_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.default_resource_limits = Some(limits);
        self
    }

    /// Builds and returns a configured [`MaestroClient`].
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn build(self) -> Result<MaestroClient> {
        let namespace = self.namespace.unwrap_or_else(|| "default".to_string());

        if self.dry_run {
            log::info!("Creating MaestroClient in dry_run mode");
        }

        Ok(MaestroClient::new(
            self.kube_config_path,
            namespace,
            self.dry_run,
            self.default_timeout,
            self.log_level,
            self.default_resource_limits,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new() {
        let builder = MaestroClientBuilder::new();
        assert!(builder.kube_config_path.is_none());
        assert!(builder.namespace.is_none());
        assert!(!builder.dry_run);
        assert!(builder.default_timeout.is_none());
        assert!(builder.log_level.is_none());
        assert!(builder.default_resource_limits.is_none());
    }

    #[test]
    fn test_builder_default() {
        let builder = MaestroClientBuilder::default();
        assert!(builder.kube_config_path.is_none());
        assert!(!builder.dry_run);
    }

    #[test]
    fn test_builder_with_kube_config() {
        let builder = MaestroClientBuilder::new().with_kube_config("/path/to/config");
        assert_eq!(
            builder.kube_config_path,
            Some(PathBuf::from("/path/to/config"))
        );
    }

    #[test]
    fn test_builder_with_namespace() {
        let builder = MaestroClientBuilder::new().with_namespace("production");
        assert_eq!(builder.namespace, Some("production".to_string()));
    }

    #[test]
    fn test_builder_with_dry_run() {
        let builder = MaestroClientBuilder::new().with_dry_run(true);
        assert!(builder.dry_run);
    }

    #[test]
    fn test_builder_with_default_timeout() {
        let timeout = Duration::from_secs(60);
        let builder = MaestroClientBuilder::new().with_default_timeout(timeout);
        assert_eq!(builder.default_timeout, Some(timeout));
    }

    #[test]
    fn test_builder_with_log_level() {
        let builder = MaestroClientBuilder::new().with_log_level("debug");
        assert_eq!(builder.log_level, Some("debug".to_string()));
    }

    #[test]
    fn test_builder_with_default_resource_limits() {
        let limits = ResourceLimits::new().with_cpu("500m").with_memory("512Mi");
        let builder = MaestroClientBuilder::new().with_default_resource_limits(limits.clone());
        assert!(builder.default_resource_limits.is_some());
        let stored_limits = builder.default_resource_limits.unwrap();
        assert_eq!(stored_limits.cpu, Some("500m".to_string()));
        assert_eq!(stored_limits.memory, Some("512Mi".to_string()));
    }

    #[test]
    fn test_builder_build_default_namespace() {
        let client = MaestroClientBuilder::new().build().unwrap();
        assert_eq!(client.namespace(), "default");
        assert!(!client.dry_run());
    }

    #[test]
    fn test_builder_build_custom_namespace() {
        let client = MaestroClientBuilder::new()
            .with_namespace("staging")
            .build()
            .unwrap();
        assert_eq!(client.namespace(), "staging");
    }

    #[test]
    fn test_builder_build_dry_run_mode() {
        let client = MaestroClientBuilder::new()
            .with_dry_run(true)
            .build()
            .unwrap();
        assert!(client.dry_run());
    }

    #[test]
    fn test_builder_build_all_options() {
        let limits = ResourceLimits::new().with_cpu("1000m").with_memory("1Gi");
        let timeout = Duration::from_secs(120);

        let client = MaestroClientBuilder::new()
            .with_kube_config("/custom/config")
            .with_namespace("production")
            .with_dry_run(true)
            .with_default_timeout(timeout)
            .with_log_level("info")
            .with_default_resource_limits(limits)
            .build()
            .unwrap();

        assert_eq!(client.namespace(), "production");
        assert!(client.dry_run());
    }

    #[test]
    fn test_builder_fluent_api() {
        let client = MaestroClientBuilder::new()
            .with_namespace("test")
            .with_dry_run(true)
            .with_log_level("debug")
            .build()
            .unwrap();

        assert_eq!(client.namespace(), "test");
        assert!(client.dry_run());
    }
}
