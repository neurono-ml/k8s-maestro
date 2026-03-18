## ADDED Requirements

### Requirement: Create Kubernetes Pod step
The system SHALL provide a KubePodStep implementation that creates and manages Kubernetes Pod resources through the kube client.

#### Scenario: Successful pod creation
- **WHEN** a KubePodStep is built with valid name, namespace, and at least one container
- **THEN** the system creates a Kubernetes Pod in the specified namespace
- **AND** the pod name matches the configured name or follows generateName pattern
- **AND** the pod contains the specified containers and configuration
- **AND** the step_id is derived from the pod name

### Requirement: Configure pod with builder pattern
The system SHALL provide a fluent builder API for configuring KubePodStep with all Kubernetes Pod properties.

#### Scenario: Build pod with minimal configuration
- **WHEN** builder is called with new(), with_name(), with_namespace(), add_container(), and build()
- **THEN** the system creates a KubePodStep with the specified name, namespace, and container
- **AND** all other properties have sensible defaults

#### Scenario: Build pod with restart policy
- **WHEN** builder is called with with_restart_policy(RestartPolicy::Never)
- **THEN** the system creates a KubePodStep with restartPolicy: Never

#### Scenario: Build pod with resource limits
- **WHEN** builder is called with with_resource_limits()
- **THEN** the system creates a KubePodStep with the specified resource limits

#### Scenario: Build pod with sidecar containers
- **WHEN** builder is called with add_sidecar() for additional containers
- **THEN** the system creates a KubePodStep with main container plus sidecar containers

### Requirement: Wait for pod completion
The system SHALL support waiting for a Kubernetes Pod to complete or fail through the WaitableWorkFlowStep trait.

#### Scenario: Wait for successful pod completion
- **WHEN** wait() is called on a running pod that completes successfully
- **THEN** the system watches the pod status until completion
- **AND** returns StepResult with Success status and exit code 0

#### Scenario: Wait for failed pod
- **WHEN** wait() is called on a pod that fails
- **THEN** the system detects the failure from pod status
- **AND** returns StepResult with Failure status and appropriate exit code

### Requirement: Delete pod
The system SHALL support deleting the Kubernetes Pod through the DeletableWorkFlowStep trait.

#### Scenario: Delete pod successfully
- **WHEN** delete_workflow() is called with dry_run=false on an existing pod
- **THEN** the system deletes the Kubernetes Pod from the cluster

#### Scenario: Dry run deletion
- **WHEN** delete_workflow() is called with dry_run=true
- **THEN** the system validates the operation but does not delete the pod

### Requirement: Stream logs from pod
The system SHALL support streaming logs from the pod through the LoggableWorkFlowStep trait.

#### Scenario: Stream logs from running pod
- **WHEN** stream_logs() is called with LogStreamOptions on a running pod
- **THEN** the system streams log lines from the pod
- **AND** follows the pod logs until completion

### Requirement: Expose pod as service
The system SHALL support creating a Kubernetes Service for the pod through the ServableWorkFlowStep trait.

#### Scenario: Expose service on port
- **WHEN** expose_service() is called with service_name and port
- **THEN** the system creates a Service resource targeting the pod
- **AND** returns the service access URL

### Requirement: Expose pod via ingress
The system SHALL support creating Kubernetes Ingress for the pod through the ServableWorkFlowStep trait.

#### Scenario: Expose ingress for service
- **WHEN** expose_ingress() is called with ingress_name, host, and service_port
- **THEN** the system creates an Ingress resource routing to the service
- **AND** returns the ingress URL

### Requirement: Validate pod builder configuration
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

### Requirement: Handle external pod deletion
The system SHALL handle cases where the pod is deleted externally while waiting.

#### Scenario: Pod deleted during wait
- **WHEN** wait() is called and the pod is deleted by an external process
- **THEN** the system detects the deletion
- **AND** returns an error indicating the pod no longer exists
