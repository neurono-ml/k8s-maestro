## ADDED Requirements

### Requirement: Rust step builder
The system SHALL provide a `RustStepBuilder` for constructing RustStep instances with fluent API.

#### Scenario: Build minimal Rust step
- **WHEN** RustStepBuilder::new() is called followed by build()
- **THEN** the system SHALL create a RustStep with default values

#### Scenario: Build Rust step with name
- **WHEN** RustStepBuilder::new().with_name("compute-task").build() is called
- **THEN** the system SHALL create a RustStep with step_id "compute-task"

#### Scenario: Build Rust step with code
- **WHEN** RustStepBuilder::new().with_code("fn main() { println!(\"hello\"); }").build() is called
- **THEN** the system SHALL create a RustStep with the provided code as main.rs

#### Scenario: Build Rust step with Cargo dependencies
- **WHEN** RustStepBuilder::new().with_dependencies(&["serde", "tokio"]).build() is called
- **THEN** the system SHALL create a RustStep with Cargo.toml including the specified dependencies

### Requirement: Rust step implements workflow traits
The system SHALL implement ExecutableWorkFlowStep, WaitableWorkFlowStep, DeletableWorkFlowStep, and LoggableWorkFlowStep for RustStep.

#### Scenario: Execute Rust step
- **WHEN** RustStep::execute is called
- **THEN** the system SHALL create a Kubernetes Pod with Rust image, compile the code, execute the binary, and return StepResult

#### Scenario: Wait for Rust step completion
- **WHEN** RustStep::wait is called
- **THEN** the system SHALL wait for the Pod to complete and return the final StepResult

#### Scenario: Delete Rust step resources
- **WHEN** RustStep::delete_workflow is called with dry_run=false
- **THEN** the system SHALL delete the Pod and associated ConfigMaps

#### Scenario: Stream Rust step logs
- **WHEN** RustStep::stream_logs is called with follow=true
- **THEN** the system SHALL stream stdout and stderr from the Pod in real-time

### Requirement: Rust step Cargo.toml generation
The system SHALL automatically generate Cargo.toml for Rust steps with dependencies.

#### Scenario: Generate Cargo.toml with dependencies
- **WHEN** a RustStep is created with dependencies ["serde", "tokio"]
- **THEN** the generated Cargo.toml SHALL include serde and tokio in [dependencies]

#### Scenario: Generate Cargo.toml with edition
- **WHEN** a RustStep is created
- **THEN** the generated Cargo.toml SHALL specify edition = "2021"

### Requirement: Rust step resource limits
The system SHALL support configurable resource limits for Rust steps.

#### Scenario: Set CPU and memory limits
- **WHEN** RustStepBuilder::new().with_resource_limits(ResourceLimits::new().with_cpu("1000m").with_memory("512Mi")).build() is called
- **THEN** the created Pod SHALL have the specified resource limits

#### Scenario: Higher default limits for compilation
- **WHEN** a RustStep is created without explicit limits
- **THEN** the system SHALL apply higher default limits to accommodate compilation overhead

### Requirement: Rust step compilation handling
The system SHALL handle compilation failures gracefully.

#### Scenario: Compilation error
- **WHEN** a Rust step has invalid code that fails to compile
- **THEN** StepResult.status SHALL be StepStatus::Failure
- **AND** StepResult.stderr SHALL contain the compiler error message

#### Scenario: Compilation timeout
- **WHEN** Rust compilation exceeds the timeout
- **THEN** the system SHALL terminate the Pod and return StepResult with status Failure
