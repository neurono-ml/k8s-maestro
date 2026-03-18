## ADDED Requirements

### Requirement: Define JobNameType enum
The system SHALL provide a JobNameType enum with variants for defined and generated names.

#### Scenario: Use defined name
- **WHEN** JobNameType::DefinedName("my-job") is used
- **THEN** the Kubernetes resource is created with the exact name "my-job"

#### Scenario: Use generated name
- **WHEN** JobNameType::GenerateName("job-prefix-") is used
- **THEN** the Kubernetes resource is created with a generated name starting with "job-prefix-"
- **AND** the exact name is determined by the Kubernetes API

### Requirement: Configure job name via builder
The system SHALL support configuring job name using either variant of JobNameType.

#### Scenario: Set defined name in builder
- **WHEN** builder.with_name("my-job") is called
- **THEN** the builder internally uses JobNameType::DefinedName("my-job")

#### Scenario: Set generate name in builder
- **WHEN** builder.with_name_type(JobNameType::GenerateName("job-")) is called
- **THEN** the builder uses the generate name pattern

### Requirement: Generate step_id from job name
The system SHALL derive the step_id from the job name or the generated name.

#### Scenario: Step_id from defined name
- **WHEN** a job is created with JobNameType::DefinedName("my-job")
- **THEN** the step_id is "my-job"

#### Scenario: Step_id from generated name
- **WHEN** a job is created with JobNameType::GenerateName("job-") and K8s assigns "job-abc123"
- **THEN** the step_id is "job-abc123" (the actual assigned name)
