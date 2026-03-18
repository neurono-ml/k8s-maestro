## Why

Workflow steps currently lack extensibility for third-party integrations. Users need to manually configure sidecar containers for common patterns like logging agents, service meshes, or monitoring tools. A plugin-based sidecar system enables reusable, shareable extensions that can be installed and configured declaratively, reducing boilerplate and enabling community-driven extensions.

## What Changes

- **New**: `SidecarContainer` struct with builder pattern for defining sidecar containers in workflow steps
- **New**: `SidecarBuilder` fluent API for configuring sidecars (ports, env, volumes, resources, config)
- **New**: `SidecarPlugin` trait defining the plugin interface for native Rust plugins
- **New**: `DynamicPluginLoader` for runtime loading of native plugins (.so/.dylib/.dll) via libloading
- **New**: `PluginRegistry` for managing registered plugins and installing them to workflow steps
- **New**: Plugin directory structure at `~/.maestro/plugins/` (configurable)
- **New**: Plugin metadata format (plugin.toml) for discovery and validation
- **Extensible**: API designed for future Python (pyo3), Lua (mlua), and WebAssembly (wasmtime) plugins

## Capabilities

### New Capabilities

- `sidecar-containers`: Core sidecar container definition, builder pattern, and integration with workflow steps
- `sidecar-plugins`: Plugin trait system, registry, and lifecycle management for extending sidecar functionality
- `dynamic-plugin-loading`: Runtime discovery and loading of native plugins from filesystem using libloading

### Modified Capabilities

None - this is a new feature with no existing capability modifications.

## Impact

- **New modules**: 
  - `src/steps/kubernetes/sidecar.rs` - Sidecar container types and builder
  - `src/networking/plugins/mod.rs` - Plugin trait and registry
  - `src/networking/plugins/dynamic_loader.rs` - Dynamic library loading
  - `src/networking/plugins/plugin_registry.rs` - Plugin management
- **New dependencies**: `libloading` for dynamic library loading, `serde_json` for config
- **Tests**: Unit tests for builders, plugin loading, and registration; integration tests with Kind cluster
- **Documentation**: Plugin development guide, example plugins
