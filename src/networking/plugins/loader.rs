use crate::networking::plugins::{PluginInfo, SidecarPlugin};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const PLUGIN_FACTORY_SYMBOL: &str = "maestro_plugin_factory";

#[allow(dead_code, improper_ctypes_definitions)]
type PluginFactory = extern "C" fn() -> *mut dyn SidecarPlugin;

struct LoadedLibrary {
    #[allow(dead_code)]
    library: Library,
    plugin_name: String,
}

/// A loader for dynamically loading sidecar plugins at runtime.
///
/// This struct manages the lifecycle of dynamically loaded plugin libraries.
/// It can load plugins from shared libraries (`.so`, `.dylib`, `.dll`) and
/// track which plugins are currently loaded.
///
/// # Example
///
/// ```rust
/// use k8s_maestro::networking::plugins::DynamicPluginLoader;
///
/// let loader = DynamicPluginLoader::new();
/// // Load a plugin from a shared library
/// match loader.load_plugin("/path/to/plugin.so") {
///     Ok(plugin) => println!("Loaded plugin: {}", plugin.name()),
///     Err(e) => eprintln!("Failed to load plugin: {}", e),
/// }
/// ```
pub struct DynamicPluginLoader {
    loaded_plugins: Mutex<HashMap<String, LoadedLibrary>>,
}

impl DynamicPluginLoader {
    /// Creates a new plugin loader.
    pub fn new() -> Self {
        Self {
            loaded_plugins: Mutex::new(HashMap::new()),
        }
    }

    /// Loads a plugin from the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the shared library (`.so`, `.dylib`, or `.dll`)
    ///
    /// # Example
    ///
    /// ```rust
    /// use k8s_maestro::networking::plugins::DynamicPluginLoader;
    ///
    /// let loader = DynamicPluginLoader::new();
    /// let plugin = loader.load_plugin("/usr/lib/maestro/plugins/my_plugin.so");
    /// ```
    pub fn load_plugin(&self, path: &str) -> anyhow::Result<Box<dyn SidecarPlugin>> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(anyhow::anyhow!("Plugin file not found: {}", path.display()));
        }

        let library = unsafe { Library::new(path) }
            .map_err(|e| anyhow::anyhow!("Failed to load library '{}': {}", path.display(), e))?;

        let factory: Symbol<PluginFactory> =
            unsafe { library.get(PLUGIN_FACTORY_SYMBOL.as_bytes()) }.map_err(|e| {
                anyhow::anyhow!(
                    "Failed to find plugin factory symbol in '{}': {}",
                    path.display(),
                    e
                )
            })?;

        let plugin_ptr = factory();

        if plugin_ptr.is_null() {
            return Err(anyhow::anyhow!("Plugin factory returned null pointer"));
        }

        let plugin: Box<dyn SidecarPlugin> = unsafe { std::mem::transmute(plugin_ptr) };
        let plugin_name = plugin.name().to_string();

        {
            let mut loaded = self.loaded_plugins.lock().unwrap();
            if loaded.contains_key(&plugin_name) {
                return Err(anyhow::anyhow!(
                    "Plugin '{}' is already loaded",
                    plugin_name
                ));
            }
            loaded.insert(
                plugin_name.clone(),
                LoadedLibrary {
                    library,
                    plugin_name,
                },
            );
        }

        Ok(plugin)
    }

    /// Unloads a previously loaded plugin by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the plugin to unload
    pub fn unload_plugin(&self, name: &str) -> anyhow::Result<()> {
        let mut loaded = self.loaded_plugins.lock().unwrap();
        loaded
            .remove(name)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", name))?;
        Ok(())
    }

    /// Lists all currently loaded plugins.
    ///
    /// Returns a vector of [`PluginInfo`] for each loaded plugin.
    pub fn list_loaded_plugins(&self) -> Vec<PluginInfo> {
        let loaded = self.loaded_plugins.lock().unwrap();
        loaded
            .values()
            .map(|lib| PluginInfo {
                name: lib.plugin_name.clone(),
                version: "1.0.0".to_string(),
                description: String::new(),
                author: String::new(),
            })
            .collect()
    }
}

impl Default for DynamicPluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DynamicPluginLoader {
    fn drop(&mut self) {
        let mut loaded = self.loaded_plugins.lock().unwrap();
        loaded.clear();
    }
}

/// Returns the default plugin directory path.
///
/// The default directory is `~/.maestro/plugins` on Unix-like systems
/// or `%USERPROFILE%\.maestro\plugins` on Windows.
///
/// # Example
///
/// ```rust
/// use k8s_maestro::networking::plugins::get_default_plugin_dir;
///
/// let plugin_dir = get_default_plugin_dir();
/// println!("Default plugin directory: {}", plugin_dir.display());
/// ```
pub fn get_default_plugin_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".maestro").join("plugins")
}

/// Discovers plugin files in the specified directory.
///
/// Scans the directory for valid plugin shared libraries (`.so`, `.dylib`, `.dll`)
/// and returns their paths. Both direct library files and directories containing
/// a library named according to the platform are supported.
///
/// # Arguments
///
/// * `dir` - The directory to scan for plugins
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use k8s_maestro::networking::plugins::discover_plugins;
///
/// let plugins = discover_plugins(Path::new("/opt/maestro/plugins")).unwrap();
/// for plugin in plugins {
///     println!("Found plugin: {}", plugin.display());
/// }
/// ```
pub fn discover_plugins(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    if !dir.is_dir() {
        return Err(anyhow::anyhow!(
            "Plugin directory is not a directory: {}",
            dir.display()
        ));
    }

    let mut plugins = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if is_dynamic_library_extension(ext) {
                    plugins.push(path);
                }
            }
        } else if path.is_dir() {
            let library_path = path.join(get_library_filename());
            if library_path.exists() {
                plugins.push(library_path);
            }
        }
    }

    Ok(plugins)
}

fn is_dynamic_library_extension(ext: &OsStr) -> bool {
    ext == "so" || ext == "dylib" || ext == "dll"
}

#[cfg(unix)]
fn get_library_filename() -> &'static str {
    "libmaestro_plugin.so"
}

#[cfg(target_os = "macos")]
fn get_library_filename() -> &'static str {
    "libmaestro_plugin.dylib"
}

#[cfg(target_os = "windows")]
fn get_library_filename() -> &'static str {
    "maestro_plugin.dll"
}

/// Metadata for a dynamically loaded plugin.
///
/// This struct contains information about a plugin, including its name,
/// version, description, and author. It is typically read from a
/// `plugin.toml` file in the plugin directory.
///
/// # Example
///
/// ```toml
/// name = "my-plugin"
/// version = "1.0.0"
/// description = "A sample plugin"
/// author = "John Doe"
/// ```
#[derive(Debug, serde::Deserialize)]
pub struct PluginMetadata {
    /// The unique name of the plugin.
    pub name: String,
    /// The version string of the plugin.
    pub version: String,
    /// An optional description of what the plugin does.
    pub description: Option<String>,
    /// The author of the plugin.
    pub author: Option<String>,
}

/// Parses plugin metadata from a plugin directory.
///
/// Reads the `plugin.toml` file from the plugin directory and parses it
/// into a [`PluginMetadata`] struct. If the file doesn't exist, returns
/// default metadata based on the directory name.
///
/// # Arguments
///
/// * `plugin_dir` - Path to the plugin directory
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use k8s_maestro::networking::plugins::parse_plugin_metadata;
///
/// let metadata = parse_plugin_metadata(Path::new("/path/to/plugin")).unwrap();
/// println!("Plugin: {} v{}", metadata.name, metadata.version);
/// ```
pub fn parse_plugin_metadata(plugin_dir: &Path) -> anyhow::Result<PluginMetadata> {
    let metadata_file = plugin_dir.join("plugin.toml");

    if !metadata_file.exists() {
        let library_name = plugin_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        return Ok(PluginMetadata {
            name: library_name.to_string(),
            version: "1.0.0".to_string(),
            description: None,
            author: None,
        });
    }

    let content = std::fs::read_to_string(&metadata_file)?;
    let metadata: PluginMetadata = toml::from_str(&content).map_err(|e| {
        anyhow::anyhow!(
            "Failed to parse plugin metadata at '{}': {}",
            metadata_file.display(),
            e
        )
    })?;

    validate_plugin_metadata(&metadata)?;

    Ok(metadata)
}

fn validate_plugin_metadata(metadata: &PluginMetadata) -> anyhow::Result<()> {
    if metadata.name.is_empty() {
        return Err(anyhow::anyhow!("Plugin name cannot be empty"));
    }

    if metadata.version.is_empty() {
        return Err(anyhow::anyhow!("Plugin version cannot be empty"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_plugin_loader_creation() {
        let loader = DynamicPluginLoader::new();
        let plugins = loader.list_loaded_plugins();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_plugin_loader_default() {
        let loader = DynamicPluginLoader::default();
        let plugins = loader.list_loaded_plugins();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_default_plugin_dir() {
        let dir = get_default_plugin_dir();
        assert!(dir.to_string_lossy().contains(".maestro"));
        assert!(dir.to_string_lossy().contains("plugins"));
    }

    #[test]
    fn test_discover_plugins_nonexistent_dir() {
        let dir = PathBuf::from("/nonexistent/path/to/plugins");
        let plugins = discover_plugins(&dir).unwrap();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_library_extension_check() {
        assert!(is_dynamic_library_extension(OsStr::new("so")));
        assert!(is_dynamic_library_extension(OsStr::new("dylib")));
        assert!(is_dynamic_library_extension(OsStr::new("dll")));
        assert!(!is_dynamic_library_extension(OsStr::new("txt")));
        assert!(!is_dynamic_library_extension(OsStr::new("rs")));
    }

    #[test]
    fn test_parse_metadata_invalid_empty_name() {
        let metadata = PluginMetadata {
            name: String::new(),
            version: "1.0.0".to_string(),
            description: None,
            author: None,
        };
        assert!(validate_plugin_metadata(&metadata).is_err());
    }

    #[test]
    fn test_parse_metadata_invalid_empty_version() {
        let metadata = PluginMetadata {
            name: "test-plugin".to_string(),
            version: String::new(),
            description: None,
            author: None,
        };
        assert!(validate_plugin_metadata(&metadata).is_err());
    }

    #[test]
    fn test_parse_metadata_valid() {
        let metadata = PluginMetadata {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test plugin".to_string()),
            author: Some("Test Author".to_string()),
        };
        assert!(validate_plugin_metadata(&metadata).is_ok());
    }

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

        fn create_sidecar(&self) -> anyhow::Result<crate::entities::SidecarContainer> {
            Ok(crate::entities::SidecarContainer::new(
                self.image(),
                self.name(),
            ))
        }
    }

    #[test]
    fn test_load_nonexistent_plugin() {
        let loader = DynamicPluginLoader::new();
        let result = loader.load_plugin("/nonexistent/plugin.so");
        assert!(result.is_err());
    }

    #[test]
    fn test_unload_nonexistent_plugin() {
        let loader = DynamicPluginLoader::new();
        let result = loader.unload_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_unload_loaded_plugin() {
        let loader = DynamicPluginLoader::new();
        let result = loader.unload_plugin("nonexistent");
        assert!(result.is_err());
        assert!(loader.list_loaded_plugins().is_empty());
    }
}
