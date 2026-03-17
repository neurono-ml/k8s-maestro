# Implementation Tasks: Sidecar Plugin System

## Overview

Implement sidecar container and plugin system for workflow step extensibility.

---

## Phase 1: Setup & Types

### 1.1 Project Setup
- [ ] Create `src/steps/kubernetes/sidecar.rs` module
- [ ] Create `src/networking/plugins/mod.rs` module
- [ ] Add `libloading` dependency to `Cargo.toml`
- [ ] Add module exports to `src/steps/mod.rs`
- [ ] Add module exports to `src/networking/mod.rs`

### 1.2 SidecarContainer Types
- [ ] Define `SidecarContainer` struct with fields: name, image, config, ports, env, volume_mounts, resource_limits
- [ ] Define `SidecarConfig` type alias (BTreeMap<String, serde_json::Value>)
- [ ] Define `ContainerPort` struct: container_port, host_port, protocol, name
- [ ] Add unit tests for SidecarContainer struct creation

---

## Phase 2: SidecarBuilder

### 2.1 Builder Implementation
- [ ] Create `SidecarBuilder` with fluent API
- [ ] Implement `new(image: &str)` constructor
- [ ] Implement `with_name(name: &str) -> Self`
- [ ] Implement `with_port(port: u16) -> Self`
- [ ] Implement `with_config(key: &str, value: Value) -> Self`
- [ ] Implement `with_env(key: &str, value: &str) -> Self`
- [ ] Implement `with_volume_mount(mount: VolumeMount) -> Self`
- [ ] Implement `with_resource_limits(limits: ResourceLimits) -> Self`
- [ ] Implement `build() -> Result<SidecarContainer>`
- [ ] Add validation for required fields (image, name)

### 2.2 Builder Tests
- [ ] Test builder creates valid SidecarContainer
- [ ] Test builder validation catches missing required fields
- [ ] Test builder with all optional fields
- [ ] Test builder with multiple ports
- [ ] Test builder with complex config

---

## Phase 3: Plugin Trait System

### 3.1 Core Plugin Types
- [ ] Define `SidecarPlugin` trait with methods:
  - `name() -> &str`
  - `image() -> &str`
  - `default_config() -> BTreeMap<String, Value>`
  - `install(&self, step: &mut impl KubeWorkFlowStep) -> Result<()>`
  - `validate_config(&self, config: &Map) -> Result<()>`
- [ ] Define `PluginInfo` struct: name, version, description, author
- [ ] Add unit tests for trait definition

### 3.2 Plugin Tests
- [ ] Create test plugin implementing SidecarPlugin
- [ ] Test plugin installation to mock step
- [ ] Test plugin config validation
- [ ] Test plugin with invalid config

---

## Phase 4: Dynamic Plugin Loading

### 4.1 DynamicLoader Implementation
- [ ] Create `DynamicPluginLoader` struct
- [ ] Implement `new()` constructor
- [ ] Implement `load_plugin(path: &str) -> Result<Box<dyn SidecarPlugin>>`
- [ ] Implement `unload_plugin(name: &str) -> Result<()>`
- [ ] Implement `list_loaded_plugins() -> Vec<PluginInfo>`
- [ ] Handle libloading errors gracefully
- [ ] Implement Drop for proper cleanup

### 4.2 Plugin Discovery
- [ ] Implement plugin directory resolution (`~/.maestro/plugins/`)
- [ ] Implement `discover_plugins(dir: &Path) -> Result<Vec<PathBuf>>`
- [ ] Parse `plugin.toml` metadata files
- [ ] Validate plugin metadata

### 4.3 Loading Tests
- [ ] Test plugin loading from valid library
- [ ] Test error handling for invalid library
- [ ] Test plugin unloading
- [ ] Test plugin discovery from directory
- [ ] Test metadata parsing

---

## Phase 5: Plugin Registry

### 5.1 Registry Implementation
- [ ] Create `PluginRegistry` struct
- [ ] Implement `new()` constructor
- [ ] Implement `register_plugin(plugin: Box<dyn SidecarPlugin>)`
- [ ] Implement `get_plugin(name: &str) -> Option<&dyn SidecarPlugin>`
- [ ] Implement `list_plugins() -> Vec<PluginInfo>`
- [ ] Implement `install_plugin_to_step(plugin_name: &str, step: &mut impl KubeWorkFlowStep) -> Result<()>`
- [ ] Implement `unregister_plugin(name: &str) -> Result<()>`

### 5.2 Registry Tests
- [ ] Test plugin registration
- [ ] Test plugin retrieval by name
- [ ] Test plugin listing
- [ ] Test plugin installation to step
- [ ] Test duplicate registration handling
- [ ] Test unregistration

---

## Phase 6: Integration

### 6.1 Step Integration
- [ ] Add `add_sidecar` method to `KubeJobStepBuilder`
- [ ] Add `add_sidecar` method to `KubePodStepBuilder`
- [ ] Ensure sidecar containers are added to pod spec
- [ ] Handle sidecar lifecycle (starts with main, terminates with main)

### 6.2 Integration Tests (Kind)
- [ ] Test sidecar container in pod with Kind cluster
- [ ] Test sidecar communication with main container
- [ ] Test plugin installation to real step
- [ ] Test sidecar resource limits enforcement
- [ ] Test sidecar with volume mounts

---

## Phase 7: Documentation

### 7.1 Code Documentation
- [ ] Add rustdoc comments to SidecarContainer
- [ ] Add rustdoc comments to SidecarBuilder
- [ ] Add rustdoc comments to SidecarPlugin trait
- [ ] Add rustdoc comments to DynamicPluginLoader
- [ ] Add rustdoc comments to PluginRegistry

### 7.2 User Documentation
- [ ] Create plugin development guide
- [ ] Create example native plugin
- [ ] Document plugin.toml format
- [ ] Document plugin directory structure
- [ ] Add usage examples to module docs

---

## Phase 8: Verification

### 8.1 Quality Checks
- [ ] Run `cargo test --lib` - all tests pass
- [ ] Run `cargo clippy` - no warnings
- [ ] Run `cargo fmt --check` - formatted
- [ ] Run `cargo doc` - no warnings

### 8.2 Integration Verification
- [ ] Run integration tests with Kind cluster
- [ ] Verify sidecar containers work in real pods
- [ ] Verify plugin loading works end-to-end
- [ ] Test on Linux platform
- [ ] Document platform-specific notes (macOS, Windows)
