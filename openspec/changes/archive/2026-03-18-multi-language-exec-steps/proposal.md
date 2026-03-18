## Why

Workflow orchestration often requires executing custom code in multiple languages for data transformation, validation, and integration tasks. Currently, k8s-maestro only supports Kubernetes job orchestration without a way to run arbitrary code as workflow steps. Users must build and maintain separate container images for each code task, creating friction and reducing flexibility. This change enables code-as-step workflows where Python, Rust, Lua, and WASM code can be executed directly within pipeline definitions.

## What Changes

- **New module `src/steps/exec/`**: Multi-language code execution step implementations
- **Package loading system**: Support for loading packages from Git repositories, remote URLs, local paths, and registries
- **Python execution step**: Execute Python code with pip dependency management
- **Rust execution step**: Compile and execute Rust code with Cargo dependency management
- **Lua execution step**: Execute Lua scripts with LuaRocks package management
- **WASM execution step**: Execute WebAssembly modules in sandboxed environment
- **Sandboxed execution**: All code runs in isolated Kubernetes pods with configurable resource limits
- **Package caching**: Local cache for downloaded packages to improve performance
- **Comprehensive test coverage**: Unit tests for builders, package loading; integration tests with Kind

## Capabilities

### New Capabilities

- `package-loader`: Package source resolution and loading from Git, remote URLs, local paths, and registries with caching support
- `python-exec-step`: Python code execution step with pip requirements, volume mounts, and resource limits
- `rust-exec-step`: Rust code execution step with Cargo.toml generation, compilation, and execution
- `lua-exec-step`: Lua script execution step with LuaRocks package management
- `wasm-exec-step`: WebAssembly module execution step with wasmedge runtime sandboxing

### Modified Capabilities

(None - this is a new feature area)

## Impact

**Code Structure:**
- New `src/steps/exec/` module with submodules: `mod.rs`, `package_loader.rs`, `python.rs`, `rust.rs`, `lua.rs`, `wasm.rs`
- Updates to `src/steps/mod.rs` to export the new exec module

**Dependencies:**
- May require `git2` crate for Git operations
- May require `reqwest` for remote package fetching
- May require `sha2` for package integrity verification

**API Surface:**
- Public builders: `PythonStepBuilder`, `RustStepBuilder`, `LuaStepBuilder`, `WasmStepBuilder`
- Public enums: `PackageSource`
- Public structs: `PackageLoader`, `PackageCache`

**Kubernetes Resources:**
- Creates executor Pods with language-specific container images
- Uses ConfigMaps for code/scripts injection
- Uses InitContainers for package installation

**Testing:**
- Unit tests in each module file
- Integration tests in `tests/` using Kind cluster
