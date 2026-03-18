## 1. Module Structure and Dependencies

- [x] 1.1 Create `src/steps/exec/mod.rs` with module exports
- [x] 1.2 Create `src/steps/exec/package_loader.rs` with PackageSource enum
- [x] 1.3 Add dependencies to Cargo.toml (git2, reqwest, sha2, tempfile)
- [x] 1.4 Update `src/steps/mod.rs` to export exec module

## 2. Package Loader Implementation

- [x] 2.1 Implement `PackageSource` enum with Git, RemotePath, LocalPath, Registry variants
- [x] 2.2 Implement `PackageLoader` struct with load() method
- [x] 2.3 Implement `fetch_git()` method for Git repository cloning
- [x] 2.4 Implement `fetch_remote()` method for HTTP downloads
- [x] 2.5 Implement `validate_local()` method for local path validation
- [x] 2.6 Implement `PackageCache` struct with cache key generation (SHA-256)
- [x] 2.7 Add unit tests for PackageSource parsing
- [ ] 2.8 Add unit tests for PackageLoader (with mocked network calls)
- [x] 2.9 Add unit tests for PackageCache operations

## 3. Python Execution Step

- [x] 3.1 Create `src/steps/exec/python.rs` module
- [x] 3.2 Implement `PythonStep` struct with all required fields
- [x] 3.3 Implement `PythonStepBuilder` with fluent API
- [x] 3.4 Implement `WorkFlowStep` trait for PythonStep
- [x] 3.5 Implement `ExecutableWorkFlowStep` trait for PythonStep
- [x] 3.6 Implement `WaitableWorkFlowStep` trait for PythonStep
- [x] 3.7 Implement `DeletableWorkFlowStep` trait for PythonStep
- [x] 3.8 Implement `LoggableWorkFlowStep` trait for PythonStep
- [x] 3.9 Implement Pod spec generation with Python image (python:3.12-slim)
- [x] 3.10 Implement requirements.txt generation and installation
- [x] 3.11 Implement ConfigMap creation for code/scripts
- [x] 3.12 Add unit tests for PythonStepBuilder
- [ ] 3.13 Add unit tests for Pod spec generation

## 4. Rust Execution Step

- [ ] 4.1 Create `src/steps/exec/rust.rs` module
- [ ] 4.2 Implement `RustStep` struct with all required fields
- [ ] 4.3 Implement `RustStepBuilder` with fluent API
- [ ] 4.4 Implement `WorkFlowStep` trait for RustStep
- [ ] 4.5 Implement `ExecutableWorkFlowStep` trait for RustStep
- [ ] 4.6 Implement `WaitableWorkFlowStep` trait for RustStep
- [ ] 4.7 Implement `DeletableWorkFlowStep` trait for RustStep
- [ ] 4.8 Implement `LoggableWorkFlowStep` trait for RustStep
- [ ] 4.9 Implement Pod spec generation with Rust image (rust:1.75-slim)
- [ ] 4.10 Implement Cargo.toml generation with dependencies
- [ ] 4.11 Implement main.rs generation from code string
- [ ] 4.12 Add unit tests for RustStepBuilder
- [ ] 4.13 Add unit tests for Cargo.toml generation

## 5. Lua Execution Step

- [ ] 5.1 Create `src/steps/exec/lua.rs` module
- [ ] 5.2 Implement `LuaStep` struct with all required fields
- [ ] 5.3 Implement `LuaStepBuilder` with fluent API
- [ ] 5.4 Implement `WorkFlowStep` trait for LuaStep
- [ ] 5.5 Implement `ExecutableWorkFlowStep` trait for LuaStep
- [ ] 5.6 Implement `WaitableWorkFlowStep` trait for LuaStep
- [ ] 5.7 Implement `DeletableWorkFlowStep` trait for LuaStep
- [ ] 5.8 Implement `LoggableWorkFlowStep` trait for LuaStep
- [ ] 5.9 Implement Pod spec generation with Lua image
- [ ] 5.10 Implement LuaRocks package installation
- [ ] 5.11 Add unit tests for LuaStepBuilder
- [ ] 5.12 Add unit tests for script generation

## 6. WASM Execution Step

- [ ] 6.1 Create `src/steps/exec/wasm.rs` module
- [ ] 6.2 Implement `WasmStep` struct with all required fields
- [ ] 6.3 Implement `WasmStepBuilder` with fluent API
- [ ] 6.4 Implement `WorkFlowStep` trait for WasmStep
- [ ] 6.5 Implement `ExecutableWorkFlowStep` trait for WasmStep
- [ ] 6.6 Implement `WaitableWorkFlowStep` trait for WasmStep
- [ ] 6.7 Implement `DeletableWorkFlowStep` trait for WasmStep
- [ ] 6.8 Implement `LoggableWorkFlowStep` trait for WasmStep
- [ ] 6.9 Implement Pod spec generation with WasmEdge image
- [ ] 6.10 Implement WASM module loading and execution
- [ ] 6.11 Implement WASI environment configuration
- [ ] 6.12 Add unit tests for WasmStepBuilder
- [ ] 6.13 Add unit tests for WASM command generation

## 7. Common Execution Features

- [ ] 7.1 Implement resource limits support (CPU, memory, timeout) for all steps
- [ ] 7.2 Implement volume mount support for all steps
- [ ] 7.3 Implement environment variables support for all steps
- [ ] 7.4 Implement network policy configuration for all steps
- [ ] 7.5 Implement working directory configuration for all steps
- [ ] 7.6 Implement stdout/stderr capture and StepResult population
- [ ] 7.7 Implement timeout handling with Pod termination

## 8. Integration Tests

- [ ] 8.1 Create integration test infrastructure with Kind cluster
- [ ] 8.2 Add integration test for Python code execution with dependencies
- [ ] 8.3 Add integration test for Rust code compilation and execution
- [ ] 8.4 Add integration test for Lua script execution with LuaRocks
- [ ] 8.5 Add integration test for WASM module execution
- [ ] 8.6 Add integration test for package loading from Git
- [ ] 8.7 Add integration test for package loading from remote URL
- [ ] 8.8 Add integration test for resource limit enforcement
- [ ] 8.9 Add integration test for timeout handling
- [ ] 8.10 Add integration test for output capture

## 9. Documentation

- [ ] 9.1 Add inline documentation for all public APIs
- [ ] 9.2 Create usage examples in examples/ directory
- [ ] 9.3 Update README.md with multi-language execution features
- [ ] 9.4 Update CHANGELOG.md with new feature
