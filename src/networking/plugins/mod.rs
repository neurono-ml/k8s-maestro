mod plugin_registry;

use crate::entities::SidecarContainer;
use serde_json::Value;
use std::collections::BTreeMap;

pub use plugin_registry::{PluginInfo, PluginRegistry};

pub trait SidecarPlugin: Send + Sync {
    fn name(&self) -> &str;

    fn image(&self) -> &str;

    fn default_config(&self) -> BTreeMap<String, Value> {
        BTreeMap::new()
    }

    fn create_sidecar(&self) -> anyhow::Result<SidecarContainer>;

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
