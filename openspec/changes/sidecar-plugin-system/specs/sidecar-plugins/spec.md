## ADDED Requirements

### Requirement: SidecarPlugin trait definition
The system SHALL provide a `SidecarPlugin` trait with methods for name, image, default_config, install, and validate_config.

#### Scenario: Implement SidecarPlugin trait
- **WHEN** a plugin developer implements `SidecarPlugin` for a struct
- **THEN** the struct SHALL provide implementations for all trait methods

### Requirement: SidecarPlugin name method
The system SHALL require plugins to provide a unique identifier via `name() -> &str`.

#### Scenario: Get plugin name
- **WHEN** `name()` is called on a plugin
- **THEN** it SHALL return a non-empty string identifier unique within the registry

### Requirement: SidecarPlugin image method
The system SHALL require plugins to specify their container image via `image() -> &str`.

#### Scenario: Get plugin image
- **WHEN** `image()` is called on a plugin
- **THEN** it SHALL return a valid container image reference (e.g., "nginx:latest", "registry.io/plugin:v1")

### Requirement: SidecarPlugin default_config method
The system SHALL allow plugins to provide default configuration via `default_config() -> BTreeMap<String, Value>`.

#### Scenario: Get default configuration
- **WHEN** `default_config()` is called on a plugin
- **THEN** it SHALL return a map of configuration keys to JSON values with sensible defaults

### Requirement: SidecarPlugin install method
The system SHALL allow plugins to add themselves to workflow steps via `install(&self, step: &mut impl KubeWorkFlowStep) -> Result<()>`.

#### Scenario: Install plugin to step
- **WHEN** `install()` is called with a mutable reference to a workflow step
- **THEN** the plugin SHALL add its sidecar container to the step and return Ok(())

#### Scenario: Install plugin fails gracefully
- **WHEN** `install()` encounters an error (e.g., step already has sidecar with same name)
- **THEN** it SHALL return an error describing the failure

### Requirement: SidecarPlugin validate_config method
The system SHALL allow plugins to validate their configuration via `validate_config(&self, config: &Map) -> Result<()>`.

#### Scenario: Validate valid configuration
- **WHEN** `validate_config()` is called with configuration matching the plugin's schema
- **THEN** it SHALL return Ok(())

#### Scenario: Validate invalid configuration
- **WHEN** `validate_config()` is called with invalid configuration
- **THEN** it SHALL return an error describing which fields are invalid

### Requirement: PluginInfo struct
The system SHALL provide a `PluginInfo` struct containing name, version, description, and author fields.

#### Scenario: Create PluginInfo
- **WHEN** plugin metadata is loaded or queried
- **THEN** a `PluginInfo` instance SHALL be available with all metadata fields populated

### Requirement: PluginRegistry registration
The system SHALL provide `PluginRegistry::register_plugin(plugin: Box<dyn SidecarPlugin>)` for adding plugins.

#### Scenario: Register plugin
- **WHEN** a plugin is registered with the registry
- **THEN** the plugin SHALL be available for lookup by name

#### Scenario: Register duplicate plugin
- **WHEN** a plugin is registered with a name that already exists
- **THEN** the registry SHALL return an error or replace the existing plugin based on configuration

### Requirement: PluginRegistry lookup
The system SHALL provide `PluginRegistry::get_plugin(name: &str) -> Option<&dyn SidecarPlugin>`.

#### Scenario: Lookup existing plugin
- **WHEN** `get_plugin("my-plugin")` is called for a registered plugin
- **THEN** it SHALL return Some with a reference to the plugin

#### Scenario: Lookup non-existent plugin
- **WHEN** `get_plugin("unknown")` is called
- **THEN** it SHALL return None

### Requirement: PluginRegistry list plugins
The system SHALL provide `PluginRegistry::list_plugins() -> Vec<PluginInfo>`.

#### Scenario: List all plugins
- **WHEN** `list_plugins()` is called on a registry with 3 registered plugins
- **THEN** it SHALL return a vector of 3 `PluginInfo` instances

### Requirement: PluginRegistry install to step
The system SHALL provide `PluginRegistry::install_plugin_to_step(plugin_name: &str, step: &mut impl KubeWorkFlowStep) -> Result<()>`.

#### Scenario: Install registered plugin to step
- **WHEN** `install_plugin_to_step("logging-plugin", step)` is called
- **THEN** the registry SHALL look up the plugin and call its `install()` method

#### Scenario: Install unregistered plugin to step
- **WHEN** `install_plugin_to_step("unknown", step)` is called
- **THEN** it SHALL return an error indicating the plugin is not registered
