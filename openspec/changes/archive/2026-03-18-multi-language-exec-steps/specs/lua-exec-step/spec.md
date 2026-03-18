## ADDED Requirements

### Requirement: Lua step builder
The system SHALL provide a `LuaStepBuilder` for constructing LuaStep instances with fluent API.

#### Scenario: Build minimal Lua step
- **WHEN** LuaStepBuilder::new() is called followed by build()
- **THEN** the system SHALL create a LuaStep with default values

#### Scenario: Build Lua step with name
- **WHEN** LuaStepBuilder::new().with_name("script-runner").build() is called
- **THEN** the system SHALL create a LuaStep with step_id "script-runner"

#### Scenario: Build Lua step with code
- **WHEN** LuaStepBuilder::new().with_code("print('hello')").build() is called
- **THEN** the system SHALL create a LuaStep with the provided code as the script

#### Scenario: Build Lua step with LuaRocks packages
- **WHEN** LuaStepBuilder::new().with_luarocks(&["luasocket", "dkjson"]).build() is called
- **THEN** the system SHALL create a LuaStep that installs the specified packages before execution

### Requirement: Lua step implements workflow traits
The system SHALL implement ExecutableWorkFlowStep, WaitableWorkFlowStep, DeletableWorkFlowStep, and LoggableWorkFlowStep for LuaStep.

#### Scenario: Execute Lua step
- **WHEN** LuaStep::execute is called
- **THEN** the system SHALL create a Kubernetes Pod with Lua image, install LuaRocks packages, execute the script, and return StepResult

#### Scenario: Wait for Lua step completion
- **WHEN** LuaStep::wait is called
- **THEN** the system SHALL wait for the Pod to complete and return the final StepResult

#### Scenario: Delete Lua step resources
- **WHEN** LuaStep::delete_workflow is called with dry_run=false
- **THEN** the system SHALL delete the Pod and associated ConfigMaps

#### Scenario: Stream Lua step logs
- **WHEN** LuaStep::stream_logs is called with follow=true
- **THEN** the system SHALL stream stdout and stderr from the Pod in real-time

### Requirement: Lua step resource limits
The system SHALL support configurable resource limits for Lua steps.

#### Scenario: Set CPU and memory limits
- **WHEN** LuaStepBuilder::new().with_resource_limits(ResourceLimits::new().with_cpu("200m").with_memory("128Mi")).build() is called
- **THEN** the created Pod SHALL have the specified resource limits

### Requirement: Lua step volume mounts
The system SHALL support volume mounts for data access in Lua steps.

#### Scenario: Mount PVC for data access
- **WHEN** LuaStepBuilder::new().with_volume_mount("/data", "data-pvc").build() is called
- **THEN** the created Pod SHALL mount the PVC at /data

### Requirement: Lua step output capture
The system SHALL capture stdout, stderr, and exit code from Lua execution.

#### Scenario: Capture successful output
- **WHEN** a Lua step executes successfully with print("result")
- **THEN** the StepResult.stdout SHALL contain "result"

#### Scenario: Capture error output
- **WHEN** a Lua step fails with a runtime error
- **THEN** the StepResult.stderr SHALL contain the error message
- **AND** StepResult.status SHALL be StepStatus::Failure
