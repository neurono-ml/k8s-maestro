## ADDED Requirements

### Requirement: WASM step builder
The system SHALL provide a `WasmStepBuilder` for constructing WasmStep instances with fluent API.

#### Scenario: Build minimal WASM step
- **WHEN** WasmStepBuilder::new() is called followed by build()
- **THEN** the system SHALL create a WasmStep with default values

#### Scenario: Build WASM step with name
- **WHEN** WasmStepBuilder::new().with_name("wasm-processor").build() is called
- **THEN** the system SHALL create a WasmStep with step_id "wasm-processor"

#### Scenario: Build WASM step with WASM module
- **WHEN** WasmStepBuilder::new().with_module(PackageSource::RemotePath { url: "https://example.com/module.wasm" }).build() is called
- **THEN** the system SHALL create a WasmStep that loads and executes the WASM module

#### Scenario: Build WASM step with local WASM file
- **WHEN** WasmStepBuilder::new().with_module(PackageSource::LocalPath { path: "/local/module.wasm" }).build() is called
- **THEN** the system SHALL create a WasmStep that uses the local WASM module

### Requirement: WASM step implements workflow traits
The system SHALL implement ExecutableWorkFlowStep, WaitableWorkFlowStep, DeletableWorkFlowStep, and LoggableWorkFlowStep for WasmStep.

#### Scenario: Execute WASM step
- **WHEN** WasmStep::execute is called
- **THEN** the system SHALL create a Kubernetes Pod with WasmEdge image, load the WASM module, execute it, and return StepResult

#### Scenario: Wait for WASM step completion
- **WHEN** WasmStep::wait is called
- **THEN** the system SHALL wait for the Pod to complete and return the final StepResult

#### Scenario: Delete WASM step resources
- **WHEN** WasmStep::delete_workflow is called with dry_run=false
- **THEN** the system SHALL delete the Pod and associated ConfigMaps

#### Scenario: Stream WASM step logs
- **WHEN** WasmStep::stream_logs is called with follow=true
- **THEN** the system SHALL stream stdout and stderr from the Pod in real-time

### Requirement: WASM sandbox execution
The system SHALL execute WASM modules in a sandboxed environment.

#### Scenario: WASM module isolation
- **WHEN** a WASM step is executed
- **THEN** the module SHALL run in WasmEdge sandbox with restricted system access

#### Scenario: WASM resource limits
- **WHEN** WasmStepBuilder::new().with_resource_limits(ResourceLimits::new().with_cpu("500m").with_memory("256Mi")).build() is called
- **THEN** the WASM runtime SHALL enforce the specified memory and CPU limits

### Requirement: WASM step function invocation
The system SHALL support invoking specific WASM functions with arguments.

#### Scenario: Invoke named function
- **WHEN** WasmStepBuilder::new().with_function("process_data").with_args(&["arg1", "arg2"]).build() is called
- **THEN** the system SHALL execute the specified function with the provided arguments

#### Scenario: Default function invocation
- **WHEN** a WasmStep is created without specifying a function
- **THEN** the system SHALL invoke the default entry point (typically _start or main)

### Requirement: WASM step WASI support
The system SHALL support WASI (WebAssembly System Interface) for WASM modules.

#### Scenario: WASI filesystem access
- **WHEN** WasmStepBuilder::new().with_volume_mount("/data", "data-pvc").build() is called
- **THEN** the WASM module SHALL have access to the mounted volume via WASI filesystem APIs

#### Scenario: WASI environment variables
- **WHEN** WasmStepBuilder::new().with_env("CONFIG_PATH", "/config").build() is called
- **THEN** the WASM module SHALL have access to the environment variable via WASI

### Requirement: WASM step output capture
The system SHALL capture stdout, stderr, and exit code from WASM execution.

#### Scenario: Capture successful output
- **WHEN** a WASM step executes successfully with stdout output
- **THEN** the StepResult.stdout SHALL contain the output

#### Scenario: Capture WASM trap
- **WHEN** a WASM module traps (crashes)
- **THEN** the StepResult.stderr SHALL contain the trap message
- **AND** StepResult.status SHALL be StepStatus::Failure
