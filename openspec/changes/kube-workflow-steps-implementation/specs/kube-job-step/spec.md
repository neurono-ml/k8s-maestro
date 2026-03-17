## ADDED Requirements

### Requirement: Create Kubernetes Job step
The system SHALL provide a KubeJobStep implementation that creates and manages Kubernetes Job resources through the kube client.

#### Scenario: Successful job creation
- **WHEN** a KubeJobStep is built with valid name, namespace, and at least one container
- **THEN** the system creates a Kubernetes Job in the specified namespace
- **AND** the job name matches the configured name or follows generateName pattern
- **AND** the job contains the specified containers and configuration
- **AND** the step_id is derived from the job name

### Requirement: Configure job with builder pattern
The system SHALL provide a fluent builder API for configuring KubeJobStep with all Kubernetes Job properties.

#### Scenario: Build job with minimal configuration
- **WHEN** builder is called with new(), with_name(), with_namespace(), add_container(), and build()
- **THEN** the system creates a KubeJobStep with the specified name, namespace, and container
- **AND** all other properties have sensible defaults

#### Scenario: Build job with full configuration
- **WHEN** builder methods are called for all optional properties (backoff_limit, restart_policy, ttl_seconds, completions, parallelism, resource_limits, service_config, ingress_config)
- **THEN** the system creates a KubeJobStep with all specified properties configured
- **AND** each property maps to the corresponding Kubernetes Job field

#### Scenario: Build job with sidecar containers
- **WHEN** builder is called with add_sidecar() for additional containers
- **THEN** the system creates a KubeJobStep with main container plus sidecar containers
- **AND** all containers are included in the Job pod template

### Requirement: Wait for job completion
The system SHALL support waiting for a Kubernetes Job to complete or fail through the WaitableWorkFlowStep trait.

#### Scenario: Wait for successful job completion
- **WHEN** wait() is called on a running job that completes successfully
- **THEN** the system watches the job status until completion
- **AND** returns StepResult with Success status and exit code 0

#### Scenario: Wait for failed job
- **WHEN** wait() is called on a job that fails
- **THEN** the system detects the failure from job status
- **AND** returns StepResult with Failure status and appropriate exit code

#### Scenario: Wait with timeout
- **WHEN** wait() is called and job takes longer than configured TTL or timeout
- **THEN** the system returns a timeout error
- **AND** includes the job status in the error context

### Requirement: Delete job and associated resources
The system SHALL support deleting the Kubernetes Job and its associated pods through the DeletableWorkFlowStep trait.

#### Scenario: Delete job successfully
- **WHEN** delete_workflow() is called with dry_run=false on an existing job
- **THEN** the system deletes the Kubernetes Job from the cluster
- **AND** cascading deletion removes associated pods

#### Scenario: Delete associated pods
- **WHEN** delete_associated_pods() is called with dry_run=false
- **THEN** the system deletes all pods created by the job
- **AND** returns success even if some pods were already deleted

#### Scenario: Dry run deletion
- **WHEN** delete_workflow() or delete_associated_pods() is called with dry_run=true
- **THEN** the system validates the operation but does not delete resources
- **AND** returns success without modifying the cluster

### Requirement: Stream logs from job pods
The system SHALL support streaming logs from job pods through the LoggableWorkFlowStep trait.

#### Scenario: Stream logs from running job
- **WHEN** stream_logs() is called with LogStreamOptions on a running job
- **THEN** the system streams log lines from all job pods
- **AND** follows the pod logs until the job completes

#### Scenario: Stream logs with tail lines limit
- **WHEN** stream_logs() is called with tail_lines=Some(100)
- **THEN** the system returns only the last 100 log lines from each pod

#### Scenario: Stream logs with timestamps
- **WHEN** stream_logs() is called with timestamps=true
- **THEN** each log line includes the timestamp

### Requirement: Expose job as service
The system SHALL support creating a Kubernetes Service for the job through the ServableWorkFlowStep trait.

#### Scenario: Expose service on port
- **WHEN** expose_service() is called with service_name and port
- **THEN** the system creates a Service resource targeting the job pods
- **AND** the service exposes the specified port
- **AND** returns the service access URL

### Requirement: Expose job via ingress
The system SHALL support creating Kubernetes Ingress for the job through the ServableWorkFlowStep trait.

#### Scenario: Expose ingress for service
- **WHEN** expose_ingress() is called with ingress_name, host, and service_port
- **THEN** the system creates an Ingress resource routing to the service
- **AND** the ingress uses the specified host
- **AND** returns the ingress URL

### Requirement: Configure job restart policy
The system SHALL support configuring the restart policy for job pods.

#### Scenario: Set restart policy to Never
- **WHEN** with_restart_policy(RestartPolicy::Never) is called
- **THEN** the job pod template has restartPolicy: Never

#### Scenario: Set restart policy to OnFailure
- **WHEN** with_restart_policy(RestartPolicy::OnFailure) is called
- **THEN** the job pod template has restartPolicy: OnFailure

### Requirement: Configure job resource limits
The system SHALL support applying resource limits and requests to job containers.

#### Scenario: Set CPU and memory limits
- **WHEN** with_resource_limits() is called with CPU and memory values
- **THEN** all job containers have the specified resource limits
- **AND** resource requests are set if provided

### Requirement: Validate builder configuration
The system SHALL validate all required fields are set before building the step.

#### Scenario: Build without name
- **WHEN** build() is called without calling with_name() or with_name_type()
- **THEN** the system returns an error indicating name is required

#### Scenario: Build without namespace
- **WHEN** build() is called without calling with_namespace()
- **THEN** the system returns an error indicating namespace is required

#### Scenario: Build without containers
- **WHEN** build() is called without calling add_container()
- **THEN** the system returns an error indicating at least one container is required

### Requirement: Handle external job deletion
The system SHALL handle cases where the job is deleted externally while waiting.

#### Scenario: Job deleted during wait
- **WHEN** wait() is called and the job is deleted by an external process
- **THEN** the system detects the deletion
- **AND** returns an error indicating the job no longer exists
