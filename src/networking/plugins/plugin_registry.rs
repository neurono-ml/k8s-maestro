use super::SidecarPlugin;
use crate::steps::traits::WorkFlowStep;
use std::collections::HashMap;
use std::sync::Arc;

/// Information about a registered plugin.
///
/// This struct provides basic metadata about a plugin, including its name,
/// version, description, and author.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// The unique name of the plugin.
    pub name: String,
    /// The version string of the plugin.
    pub version: String,
    /// A description of what the plugin does.
    pub description: String,
    /// The author of the plugin.
    pub author: String,
}

/// A registry for managing sidecar plugins.
///
/// The registry provides methods to register, retrieve, list, and unregister
/// plugins. It acts as a central repository for all available sidecar plugins.
///
/// # Example
///
/// ```ignore
/// // Full example requires SidecarContainer from steps module
/// use k8s_maestro::{PluginRegistry, SidecarPlugin, SidecarContainer};
/// use std::sync::Arc;
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
///
/// let mut registry = PluginRegistry::new();
/// registry.register_plugin(Arc::new(MyPlugin)).unwrap();
/// ```
pub struct PluginRegistry {
    plugins: HashMap<String, Arc<dyn SidecarPlugin>>,
}

impl PluginRegistry {
    /// Creates a new empty plugin registry.
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Registers a plugin with the registry.
    ///
    /// # Arguments
    ///
    /// * `plugin` - The plugin to register
    ///
    /// # Errors
    ///
    /// Returns an error if a plugin with the same name is already registered.
    pub fn register_plugin(&mut self, plugin: Arc<dyn SidecarPlugin>) -> anyhow::Result<()> {
        let name = plugin.name().to_string();
        if self.plugins.contains_key(&name) {
            return Err(anyhow::anyhow!("Plugin '{}' already registered", name));
        }
        self.plugins.insert(name, plugin);
        Ok(())
    }

    /// Retrieves a registered plugin by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the plugin to retrieve
    ///
    /// # Returns
    ///
    /// The plugin if found, otherwise [`None`].
    pub fn get_plugin(&self, name: &str) -> Option<&Arc<dyn SidecarPlugin>> {
        self.plugins.get(name)
    }

    /// Lists all registered plugins.
    ///
    /// # Returns
    ///
    /// A vector of [`PluginInfo`] for each registered plugin.
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

    /// Unregisters a plugin from the registry.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the plugin to unregister
    ///
    /// # Errors
    ///
    /// Returns an error if the plugin is not found.
    pub fn unregister_plugin(&mut self, name: &str) -> anyhow::Result<()> {
        self.plugins
            .remove(name)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", name))?;
        Ok(())
    }

    /// Installs a plugin to a workflow step by creating a sidecar container.
    ///
    /// Installs a registered plugin to a workflow step by creating a sidecar container.
    ///
    /// This method retrieves the plugin, creates a sidecar container from it,
    /// and associates it with the workflow step.
    ///
    /// **Note**: This is a stub implementation. It verifies the plugin exists and
    /// can create a sidecar, but does NOT actually add the sidecar to the step.
    /// Full integration requires implementing `add_sidecar` on the step builder.
    ///
    /// # Arguments
    ///
    /// * `plugin_name` - The name of the plugin to install
    /// * `step` - The workflow step to install the plugin to (unused in stub)
    ///
    /// # Errors
    ///
    /// Returns an error if the plugin is not found or sidecar creation fails.
    #[allow(unused_variables)]
    pub fn install_plugin_to_step(
        &self,
        plugin_name: &str,
        step: &mut impl WorkFlowStep,
    ) -> anyhow::Result<()> {
        let plugin = self
            .get_plugin(plugin_name)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", plugin_name))?;

        let sidecar = plugin.create_sidecar()?;
        // Stub: Verify we can create the sidecar, but don't add to step
        // In full implementation: step.add_sidecar(Box::new(sidecar));
        let _ = sidecar;
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
