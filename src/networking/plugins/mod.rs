mod loader;
mod plugin_registry;

use crate::entities::SidecarContainer;
use serde_json::Value;
use std::collections::BTreeMap;

pub use loader::{
    discover_plugins, get_default_plugin_dir, parse_plugin_metadata, DynamicPluginLoader,
    PluginMetadata,
};
pub use plugin_registry::{PluginInfo, PluginRegistry};

/// A plugin trait for creating sidecar containers.
///
/// Implement this trait to create custom sidecar plugins that can be
/// dynamically loaded and used to extend job functionality.
///
/// # Example
///
/// ```ignore
/// // Implementation requires returning a concrete SidecarContainer from steps module
/// use k8s_maestro::{SidecarPlugin, SidecarContainer};
///
/// struct MyPlugin;
///
/// impl SidecarPlugin for MyPlugin {
///     fn name(&self) -> &str { "my-plugin" }
///     fn image(&self) -> &str { "nginx:latest" }
///     fn create_sidecar(&self) -> anyhow::Result<SidecarContainer> {
///         Ok(SidecarContainer::new(self.image(), self.name()))
///     }
/// }
/// ```
pub trait SidecarPlugin: Send + Sync {
    /// Returns the unique name of the plugin.
    fn name(&self) -> &str;

    /// Returns the container image used by this plugin.
    fn image(&self) -> &str;

    /// Returns the default configuration for the plugin.
    ///
    /// Override this to provide default configuration values.
    fn default_config(&self) -> BTreeMap<String, Value> {
        BTreeMap::new()
    }

    /// Creates a [`SidecarContainer`] instance from this plugin.
    ///
    /// This method should construct a sidecar container with the appropriate
    /// configuration based on the plugin's settings.
    fn create_sidecar(&self) -> anyhow::Result<SidecarContainer>;

    /// Validates the given configuration.
    ///
    /// Override this to implement custom validation logic.
    /// Returns `Ok(())` if valid, or an error describing the validation failure.
    fn validate_config(&self, config: &BTreeMap<String, Value>) -> anyhow::Result<()> {
        let _ = config;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPlugin {
        name: String,
        image: String,
    }

    impl MockPlugin {
        fn new(name: &str, image: &str) -> Self {
            Self {
                name: name.to_string(),
                image: image.to_string(),
            }
        }
    }

    impl SidecarPlugin for MockPlugin {
        fn name(&self) -> &str {
            &self.name
        }

        fn image(&self) -> &str {
            &self.image
        }

        fn create_sidecar(&self) -> anyhow::Result<SidecarContainer> {
            Ok(SidecarContainer::new(self.image(), self.name()))
        }
    }

    #[test]
    fn test_plugin_trait() {
        let plugin = MockPlugin::new("test-plugin", "nginx:latest");
        assert_eq!(plugin.name(), "test-plugin");
        assert_eq!(plugin.image(), "nginx:latest");
    }

    #[test]
    fn test_default_config() {
        let plugin = MockPlugin::new("test-plugin", "nginx:latest");
        let config = plugin.default_config();
        assert!(config.is_empty());
    }

    #[test]
    fn test_create_sidecar() {
        let plugin = MockPlugin::new("test-plugin", "nginx:latest");
        let _sidecar = plugin.create_sidecar().unwrap();
        // SidecarContainer was created successfully
        // In a real test, we would verify the container configuration
    }

    #[test]
    fn test_validate_config() {
        let plugin = MockPlugin::new("test-plugin", "nginx:latest");
        let config = BTreeMap::new();
        assert!(plugin.validate_config(&config).is_ok());
    }
}
