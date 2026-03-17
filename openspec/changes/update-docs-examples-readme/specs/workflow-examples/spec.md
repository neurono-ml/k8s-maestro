## ADDED Requirements

### Requirement: Examples must have README
The examples directory SHALL include a README.md describing all examples with their purposes.

#### Scenario: User browses examples
- **WHEN** user opens examples/README.md
- **THEN** they see descriptions for all available examples

### Requirement: Examples must demonstrate workflow builder
The examples SHALL include use_workflow_builder.rs demonstrating basic workflow creation with WorkflowBuilder.

#### Scenario: User learns basic workflow creation
- **WHEN** user runs use_workflow_builder.rs
- **THEN** they see how to create a simple workflow with steps

### Requirement: Examples must demonstrate volume usage
The examples SHALL include use_volumes.rs updated to use current API with workflow patterns.

#### Scenario: User learns volume mounting
- **WHEN** user runs use_volumes.rs
- **THEN** they see how to mount volumes in workflow steps

### Requirement: Examples must demonstrate watching workflows
The examples SHALL include apply_and_watch_workflow.rs demonstrating workflow application and status watching.

#### Scenario: User learns to monitor workflows
- **WHEN** user runs apply_and_watch_workflow.rs
- **THEN** they see how to apply and watch workflow execution

### Requirement: Examples must demonstrate workflow deletion
The examples SHALL include delete_workflow.rs demonstrating proper workflow cleanup.

#### Scenario: User learns cleanup patterns
- **WHEN** user runs delete_workflow.rs
- **THEN** they see how to properly delete workflows and associated resources

### Requirement: Examples must demonstrate services
The examples SHALL include use_services.rs demonstrating service exposure.

#### Scenario: User learns service creation
- **WHEN** user runs use_services.rs
- **THEN** they see how to expose services from workflow steps

### Requirement: Examples must demonstrate sidecars
The examples SHALL include use_sidecar.rs demonstrating sidecar container usage.

#### Scenario: User learns sidecar patterns
- **WHEN** user runs use_sidecar.rs
- **THEN** they see how to add sidecar containers to steps

### Requirement: Examples must demonstrate multi-step workflows
The examples SHALL include multi_step_workflow.rs demonstrating multiple steps with dependencies.

#### Scenario: User learns dependency chains
- **WHEN** user runs multi_step_workflow.rs
- **THEN** they see how to create workflows with multiple dependent steps

### Requirement: Examples must demonstrate Python steps
The examples SHALL include python_step.rs showing Python step API (aspirational example).

#### Scenario: User learns Python integration
- **WHEN** user reads python_step.rs
- **THEN** they understand planned Python step API

### Requirement: Examples must demonstrate Rust steps
The examples SHALL include rust_step.rs showing Rust step API (aspirational example).

#### Scenario: User learns Rust step integration
- **WHEN** user reads rust_step.rs
- **THEN** they understand planned Rust step API

### Requirement: Examples must demonstrate WASM steps
The examples SHALL include wasm_step.rs showing WASM step API (aspirational example).

#### Scenario: User learns WASM integration
- **WHEN** user reads wasm_step.rs
- **THEN** they understand planned WASM step API

### Requirement: All examples must have comments
Each example file SHALL include comments explaining the code's purpose and key concepts.

#### Scenario: User reads example code
- **WHEN** user opens any example file
- **THEN** they find explanatory comments throughout
