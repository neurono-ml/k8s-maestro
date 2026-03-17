## ADDED Requirements

### Requirement: Python step builder
The system SHALL provide a `PythonStepBuilder` for constructing PythonStep instances with fluent API.

#### Scenario: Build minimal Python step
- **WHEN** PythonStepBuilder::new() is called followed by build()
- **THEN** the system SHALL create a PythonStep with default values

#### Scenario: Build Python step with name
- **WHEN** PythonStepBuilder::new().with_name("data-processor").build() is called
- **THEN** the system SHALL create a PythonStep with step_id "data-processor"

#### Scenario: Build Python step with package
- **WHEN** PythonStepBuilder::new().with_package(PackageSource::Git { ... }).build() is called
- **THEN** the system SHALL create a PythonStep that loads the package before execution

#### Scenario: Build Python step with inline code
- **WHEN** PythonStepBuilder::new().with_code("print('hello')").build() is called
- **THEN** the system SHALL create a PythonStep with the provided code as the script

#### Scenario: Build Python step with requirements
- **WHEN** PythonStepBuilder::new().with_requirements(&["pandas", "numpy"]).build() is called
- **THEN** the system SHALL create a PythonStep that installs the specified packages before execution

#### Scenario: Build Python step with entry point
- **WHEN** PythonStepBuilder::new().with_entry_point("main.py").build() is called
- **THEN** the system SHALL create a PythonStep that executes the specified script from loaded packages

### Requirement: Python step implements workflow traits
The system SHALL implement ExecutableWorkFlowStep, WaitableWorkFlowStep, DeletableWorkFlowStep, and LoggableWorkFlowStep for PythonStep.

#### Scenario: Execute Python step
- **WHEN** PythonStep::execute is called
- **THEN** the system SHALL create a Kubernetes Pod with Python image, install requirements, execute the code, and return StepResult

#### Scenario: Wait for Python step completion
- **WHEN** PythonStep::wait is called
- **THEN** the system SHALL wait for the Pod to complete and return the final StepResult

#### Scenario: Delete Python step resources
- **WHEN** PythonStep::delete_workflow is called with dry_run=false
- **THEN** the system SHALL delete the Pod and associated ConfigMaps

#### Scenario: Stream Python step logs
- **WHEN** PythonStep::stream_logs is called with follow=true
- **THEN** the system SHALL stream stdout and stderr from the Pod in real-time

### Requirement: Python step resource limits
The system SHALL support configurable resource limits for Python steps.

#### Scenario: Set CPU and memory limits
- **WHEN** PythonStepBuilder::new().with_resource_limits(ResourceLimits::new().with_cpu("500m").with_memory("256Mi")).build() is called
- **THEN** the created Pod SHALL have the specified resource limits

#### Scenario: Set timeout
- **WHEN** PythonStepBuilder::new().with_timeout(Duration::from_secs(300)).build() is called
- **THEN** the step SHALL fail if execution exceeds 300 seconds

### Requirement: Python step volume mounts
The system SHALL support volume mounts for data access in Python steps.

#### Scenario: Mount PVC for data access
- **WHEN** PythonStepBuilder::new().with_volume_mount("/data", "data-pvc").build() is called
- **THEN** the created Pod SHALL mount the PVC at /data

### Requirement: Python step environment variables
The system SHALL support custom environment variables for Python steps.

#### Scenario: Set environment variables
- **WHEN** PythonStepBuilder::new().with_env("API_KEY", "secret").build() is called
- **THEN** the created Pod SHALL have API_KEY environment variable set

### Requirement: Python step output capture
The system SHALL capture stdout, stderr, and exit code from Python execution.

#### Scenario: Capture successful output
- **WHEN** a Python step executes successfully with print("result")
- **THEN** the StepResult.stdout SHALL contain "result"

#### Scenario: Capture error output
- **WHEN** a Python step fails with an exception
- **THEN** the StepResult.stderr SHALL contain the exception traceback
- **AND** StepResult.status SHALL be StepStatus::Failure
