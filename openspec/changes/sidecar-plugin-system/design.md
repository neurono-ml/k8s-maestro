## Context

K8s Maestro currently provides workflow step orchestration but lacks a standardized mechanism for extending steps with sidecar containers. Users manually configure additional containers for common concerns like logging, monitoring, or service mesh injection. This design introduces a plugin-based sidecar system that enables:

1. **Declarative sidecar configuration** via a fluent builder API
2. **Reusable plugins** that can be shared across projects and teams
3. **Extensibility** for future Python, Lua, and WebAssembly plugins

The system integrates with existing `KubeWorkFlowStep` trait implementations and follows Rust's type-safe patterns.

## Goals / Non-Goals

**Goals:**
- Define `SidecarContainer` with builder pattern matching existing container builders
- Create `SidecarPlugin` trait for native Rust plugins with install/validate lifecycle
- Implement `DynamicPluginLoader` using libloading for .so/.dylib/.dll plugins
- Implement `PluginRegistry` for plugin management and step installation
- Design API extensible for future Python/Lua/WASM plugins
- Support plugin discovery from configurable directory (`~/.maestro/plugins/`)

**Non-Goals:**
- Python plugin support (future iteration via pyo3)
- Lua plugin support (future iteration via mlua)
- WebAssembly plugin support (future iteration via wasmtime)
- Plugin marketplace or remote plugin installation
- Hot-reloading of plugins at runtime

## Decisions

### D1: SidecarContainer as separate type from MaestroContainer

**Rationale**: Sidecars have different concerns than main containers (plugin config, lifecycle binding). A dedicated type allows plugin-specific metadata and validation without complicating the main container API.

**Alternatives considered**:
- Reusing `MaestroContainer`: Would require adding plugin-specific fields, violating SRP
- Trait-based abstraction: Over-engineering for current needs

### D2: Plugin trait with install method accepting `&mut impl KubeWorkFlowStep`

**Rationale**: Using generics allows any type implementing `KubeWorkFlowStep` to receive plugin sidecars without dynamic dispatch overhead.

**Alternatives considered**:
- `&mut dyn KubeWorkFlowStep`: Runtime overhead, less type-safe
- Builder-only approach: Less flexible, harder to compose plugins

### D3: libloading for native plugins

**Rationale**: libloading is the de-facto standard for Rust dynamic library loading. It supports Linux (.so), macOS (.dylib), and Windows (.dll).

**Alternatives considered**:
- `dlopen`: Less maintained, similar functionality
- Static linking only: No runtime extensibility

### D4: Plugin directory at `~/.maestro/plugins/` with plugin.toml metadata

**Rationale**: Consistent with tool conventions (cargo, npm). TOML is human-friendly and already used in Rust ecosystem.

**Alternatives considered**:
- JSON metadata: Less readable
- Embedded metadata: Harder to inspect without loading

### D5: Future plugin types via separate traits

**Rationale**: Python, Lua, and WASM plugins have different FFI boundaries. Separate traits (`PythonSidecarPlugin`, `LuaSidecarPlugin`, `WasmSidecarPlugin`) allow each to have idiomatic interfaces while sharing registration infrastructure.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Native plugins are platform-specific | Document clearly, provide cross-compilation guidance |
| Plugin ABI stability | Version the plugin interface, use semantic versioning |
| Security: untrusted plugins | Document sandboxing recommendations, future: sign plugins |
| libloading safety (unsafe) | Wrap in safe API, extensive testing, document plugin contract |
| Memory leaks in plugin lifecycle | Implement Drop properly, test load/unload cycles |
