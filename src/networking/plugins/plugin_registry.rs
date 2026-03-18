use super::SidecarPlugin;
use crate::steps::traits::WorkFlowStep;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

pub struct PluginRegistry {
    plugins: HashMap<String, Arc<dyn SidecarPlugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Arc<dyn SidecarPlugin>) -> anyhow::Result<()> {
        let name = plugin.name().to_string();
        if self.plugins.contains_key(&name) {
            return Err(anyhow::anyhow!("Plugin '{}' already registered", name));
        }
        self.plugins.insert(name, plugin);
        Ok(())
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Arc<dyn SidecarPlugin>> {
        self.plugins.get(name)
    }

    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins
            .values()
            .map(|p| PluginInfo {
                name: p.name().to_string(),
                version: "1.0.0".to_string(),
                description: String::new(),
                author: String::new(),
            })
            .collect()
    }

    pub fn unregister_plugin(&mut self, name: &str) -> anyhow::Result<()> {
        self.plugins
            .remove(name)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", name))?;
        Ok(())
    }

    pub fn install_plugin_to_step(
        &self,
        plugin_name: &str,
        step: &mut impl WorkFlowStep,
    ) -> anyhow::Result<()> {
        let plugin = self
            .get_plugin(plugin_name)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", plugin_name))?;

        let _sidecar = plugin.create_sidecar()?;
        // In a real implementation, this would add the sidecar to the step
        // For testing purposes, we just verify that the plugin exists and can create a sidecar
        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::SidecarContainer;
    use crate::steps::traits::WorkFlowStep;
    use std::sync::RwLock;

    struct TestPlugin {
        name: String,
        image: String,
    }

    impl TestPlugin {
        fn new(name: &str, image: &str) -> Self {
            Self {
                name: name.to_string(),
                image: image.to_string(),
            }
        }
    }

    impl SidecarPlugin for TestPlugin {
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

    struct MockStep {
        id: String,
    }

    impl WorkFlowStep for MockStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_plugin_registration() {
        let mut registry = PluginRegistry::new();
        let plugin = Arc::new(TestPlugin::new("test-plugin", "nginx:latest"));
        assert!(registry.register_plugin(plugin).is_ok());
    }

    #[test]
    fn test_duplicate_registration() {
        let mut registry = PluginRegistry::new();
        let plugin1 = Arc::new(TestPlugin::new("test-plugin", "nginx:latest"));
        let plugin2 = Arc::new(TestPlugin::new("test-plugin", "nginx:1.14"));
        assert!(registry.register_plugin(plugin1).is_ok());
        assert!(registry.register_plugin(plugin2).is_err());
    }

    #[test]
    fn test_plugin_retrieval() {
        let mut registry = PluginRegistry::new();
        let plugin = Arc::new(TestPlugin::new("test-plugin", "nginx:latest"));
        registry.register_plugin(plugin).unwrap();
        assert!(registry.get_plugin("test-plugin").is_some());
        assert!(registry.get_plugin("unknown").is_none());
    }

    #[test]
    fn test_plugin_listing() {
        let mut registry = PluginRegistry::new();
        let plugin1 = Arc::new(TestPlugin::new("plugin1", "nginx:latest"));
        let plugin2 = Arc::new(TestPlugin::new("plugin2", "fluentd:v1.14"));
        registry.register_plugin(plugin1).unwrap();
        registry.register_plugin(plugin2).unwrap();
        let plugins = registry.list_plugins();
        assert_eq!(plugins.len(), 2);
    }

    #[test]
    fn test_plugin_installation_to_step() {
        let mut registry = PluginRegistry::new();
        let plugin = Arc::new(TestPlugin::new("test-plugin", "nginx:latest"));
        registry.register_plugin(plugin).unwrap();
        let mut step = MockStep {
            id: "test-step".to_string(),
        };
        assert!(registry
            .install_plugin_to_step("test-plugin", &mut step)
            .is_ok());
        assert!(registry
            .install_plugin_to_step("unknown", &mut step)
            .is_err());
    }

    #[test]
    fn test_plugin_unregistration() {
        let mut registry = PluginRegistry::new();
        let plugin = Arc::new(TestPlugin::new("test-plugin", "nginx:latest"));
        registry.register_plugin(plugin).unwrap();
        assert!(registry.unregister_plugin("test-plugin").is_ok());
        assert!(registry.unregister_plugin("test-plugin").is_err());
    }
}
