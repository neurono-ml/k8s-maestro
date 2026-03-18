## ADDED Requirements

### Requirement: DynamicPluginLoader struct
The system SHALL provide a `DynamicPluginLoader` for loading native plugins from the filesystem.

#### Scenario: Create plugin loader
- **WHEN** a `DynamicPluginLoader` is instantiated
- **THEN** it SHALL be ready to load plugins from the configured directory

### Requirement: DynamicPluginLoader load_plugin method
The system SHALL provide `load_plugin(path: &str) -> Result<Box<dyn SidecarPlugin>>` for loading native libraries.

#### Scenario: Load valid native plugin
- **WHEN** `load_plugin("/path/to/plugin.so")` is called on a valid plugin library
- **THEN** it SHALL load the library and return a boxed `SidecarPlugin` trait object

#### Scenario: Load invalid plugin path
- **WHEN** `load_plugin("/nonexistent/plugin.so")` is called
- **THEN** it SHALL return an error indicating the file was not found

#### Scenario: Load corrupted plugin
- **WHEN** `load_plugin()` is called on a file that is not a valid plugin library
- **THEN** it SHALL return an error describing the load failure

### Requirement: DynamicPluginLoader unload_plugin method
The system SHALL provide `unload_plugin(name: &str) -> Result<()>` for unloading plugins.

#### Scenario: Unload loaded plugin
- **WHEN** `unload_plugin("my-plugin")` is called on a loaded plugin
- **THEN** the plugin SHALL be unloaded and its resources released

#### Scenario: Unload non-existent plugin
- **WHEN** `unload_plugin("unknown")` is called
- **THEN** it SHALL return an error indicating the plugin is not loaded

### Requirement: DynamicPluginLoader list_loaded_plugins method
The system SHALL provide `list_loaded_plugins() -> Vec<PluginInfo>`.

#### Scenario: List loaded plugins
- **WHEN** `list_loaded_plugins()` is called with 2 plugins loaded
- **THEN** it SHALL return a vector of 2 `PluginInfo` instances

### Requirement: Plugin discovery from directory
The system SHALL discover plugins by scanning the plugin directory for library files (.so, .dylib, .dll) with corresponding plugin.toml metadata.

#### Scenario: Discover plugins in directory
- **WHEN** the plugin directory contains `plugin-a.so` with `plugin-a.toml` and `plugin-b.so` with `plugin-b.toml`
- **THEN** discovery SHALL identify both plugins and their metadata

#### Scenario: Skip plugins without metadata
- **WHEN** the plugin directory contains `orphan.so` without a corresponding `orphan.toml`
- **THEN** discovery SHALL skip the orphan library and log a warning

### Requirement: Plugin metadata format
The system SHALL parse plugin.toml files containing name, version, description, author, and library path.

#### Scenario: Parse valid plugin.toml
- **WHEN** a plugin.toml contains `[plugin]\nname = "my-plugin"\nversion = "1.0.0"\nlibrary = "libplugin.so"`
- **THEN** the metadata SHALL be parsed into a `PluginInfo` struct

#### Scenario: Parse invalid plugin.toml
- **WHEN** a plugin.toml is malformed or missing required fields
- **THEN** the loader SHALL return an error indicating the parse failure

### Requirement: Default plugin directory
The system SHALL use `~/.maestro/plugins/` as the default plugin directory, overridable via configuration.

#### Scenario: Use default plugin directory
- **WHEN** no plugin directory is explicitly configured
- **THEN** the loader SHALL use `$HOME/.maestro/plugins/`

#### Scenario: Use custom plugin directory
- **WHEN** a custom directory is configured
- **THEN** the loader SHALL use the specified path for plugin discovery

### Requirement: Platform-specific library extensions
The system SHALL support platform-specific library extensions: .so (Linux), .dylib (macOS), .dll (Windows).

#### Scenario: Linux plugin loading
- **WHEN** running on Linux and loading "plugin.so"
- **THEN** the loader SHALL successfully load the shared object

#### Scenario: macOS plugin loading
- **WHEN** running on macOS and loading "plugin.dylib"
- **THEN** the loader SHALL successfully load the dynamic library

#### Scenario: Windows plugin loading
- **WHEN** running on Windows and loading "plugin.dll"
- **THEN** the loader SHALL successfully load the DLL
