# Implementation Tasks: Sidecar Plugin System

## Overview

Implement sidecar container and plugin system for workflow step extensibility.

---

## Phase 1: Setup & Types

### 1.1 Project Setup
- [x] Create `src/steps/kubernetes/sidecar.rs` module
- [x] Create `src/networking/plugins/mod.rs` module
- [x] Add `libloading` dependency to `Cargo.toml`
- [x] Add module exports to `src/steps/mod.rs`
- [x] Add module exports to `src/networking/mod.rs`

### 1.2 SidecarContainer Types
- [x] Define `SidecarContainer` struct with fields: name, image, config, ports, env, volume_mounts, resource_limits
- [x] Define `SidecarConfig` type alias (BTreeMap<String, serde_json::Value>)
- [x] Define `ContainerPort` struct: container_port, host_port, protocol, name
- [x] Add unit tests for SidecarContainer struct creation

---

## Phase 2: SidecarBuilder

### 2.1 Builder Implementation
- [x] Create `SidecarBuilder` with fluent API
- [x] Implement `new(image: &str)` constructor
- [x] Implement `with_name(name: &str) -> Self`
- [x] Implement `with_port(port: u16) -> Self`
- [x] Implement `with_config(key: &str, value: Value) -> Self`
- [x] Implement `with_env(key: &str, value: &str) -> Self`
- [x] Implement `with_volume_mount(mount: VolumeMount) -> Self`
- [x] Implement `with_resource_limits(limits: ResourceLimits) -> Self`
- [x] Implement `build() -> Result<SidecarContainer>`
- [x] Add validation for required fields (image, name)

### 2.2 Builder Tests
- [x] Test builder creates valid SidecarContainer
- [x] Test builder validation catches missing required fields
- [x] Test builder with all optional fields
- [x] Test builder with multiple ports
- [x] Test builder with complex config

---

## Phase 3: Plugin Trait System

### 3.1 Core Plugin Types
- [x] Define `SidecarPlugin` trait with methods:
  - `name() -> &str`
  - `image() -> &str`
  - `default_config() -> BTreeMap<String, Value>`
  - `create_sidecar(&self) -> Result<SidecarContainer>`
  - `validate_config(&self, config: &Map) -> Result<()>`
- [x] Define `PluginInfo` struct: name, version, description, author
- [x] Add unit tests for trait definition

### 3.2 Plugin Tests
- [x] Create test plugin implementing SidecarPlugin
- [x] Test plugin installation to mock step
- [x] Test plugin config validation
- [x] Test plugin with invalid config

---

## Phase 4: Dynamic Plugin Loading (MVP: Out of Scope)

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
- [x] Create `PluginRegistry` struct
- [x] Implement `new()` constructor
- [x] Implement `register_plugin(plugin: Box<dyn SidecarPlugin>)`
- [x] Implement `get_plugin(name: &str) -> Option<&dyn SidecarPlugin>`
- [x] Implement `list_plugins() -> Vec<PluginInfo>`
- [x] Implement `install_plugin_to_step(plugin_name: &str, step: &mut impl KubeWorkFlowStep) -> Result<()>`
- [x] Implement `unregister_plugin(name: &str) -> Result<()>`

### 5.2 Registry Tests
- [x] Test plugin registration
- [x] Test plugin retrieval by name
- [x] Test plugin listing
- [x] Test plugin installation to step
- [x] Test duplicate registration handling
- [x] Test unregistration

---

## Phase 6: Integration (MVP: Out of Scope)

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

## Phase 7: Documentation (MVP: Out of Scope)

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

## Phase 8: Verification (MVP: Out of Scope)

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

---

## MVP Summary

**Completed Tasks:** 38/88
**Scope:** Phases 1, 2, 3, and 5 (essential plugin system functionality)

The MVP implements:
- SidecarContainer with builder pattern
- SidecarPlugin trait with trait-safe interface
- PluginRegistry for plugin management
- Comprehensive unit tests

**Not Implemented (MVP scope):**
- Dynamic plugin loading (Phase 4)
- Integration with KubeJobStep/KubePodStep (Phase 6)
- Documentation (Phase 7)
- Verification/Integration tests (Phase 8)
