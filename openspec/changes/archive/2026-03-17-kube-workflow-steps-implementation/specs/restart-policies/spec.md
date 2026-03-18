## ADDED Requirements

### Requirement: Define RestartPolicy enum
The system SHALL provide a RestartPolicy enum with standard Kubernetes restart policy options.

#### Scenario: Use Never policy
- **WHEN** RestartPolicy::Never is configured
- **THEN** pods do not restart after termination
- **AND** this is the default for Jobs

#### Scenario: Use OnFailure policy
- **WHEN** RestartPolicy::OnFailure is configured
- **THEN** pods restart only when they fail
- **AND** failed containers are restarted within the pod

#### Scenario: Use Always policy
- **WHEN** RestartPolicy::Always is configured
- **THEN** pods always restart regardless of exit status
- **AND** this is more common for Deployments

### Requirement: Configure restart policy via builder
The system SHALL support configuring restart policy in both KubeJobStep and KubePodStep builders.

#### Scenario: Set restart policy on job
- **WHEN** job_builder.with_restart_policy(RestartPolicy::OnFailure) is called
- **THEN** the Job pod template has restartPolicy: OnFailure

#### Scenario: Set restart policy on pod
- **WHEN** pod_builder.with_restart_policy(RestartPolicy::Never) is called
- **THEN** the Pod has restartPolicy: Never

### Requirement: Map to Kubernetes restart policy
The system SHALL correctly map RestartPolicy enum values to Kubernetes API values.

#### Scenario: Map Never to K8s
- **WHEN** RestartPolicy::Never is used
- **THEN** the Kubernetes resource has restartPolicy: "Never"

#### Scenario: Map OnFailure to K8s
- **WHEN** RestartPolicy::OnFailure is used
- **THEN** the Kubernetes resource has restartPolicy: "OnFailure"

#### Scenario: Map Always to K8s
- **WHEN** RestartPolicy::Always is used
- **THEN** the Kubernetes resource has restartPolicy: "Always"
