## ADDED Requirements

### Requirement: Define ServiceConfig type
The system SHALL provide a ServiceConfig type for configuring Kubernetes Service creation.

#### Scenario: Create service config with port
- **WHEN** ServiceConfig is created with a port number
- **THEN** the config specifies the service port
- **AND** can be used to expose a workflow step

#### Scenario: Create service config with target port
- **WHEN** ServiceConfig is created with port and target_port
- **THEN** the config specifies both the service port and the pod target port
- **AND** allows port mapping

### Requirement: Configure service selector
The system SHALL support configuring pod selector labels for the service.

#### Scenario: Set selector labels
- **WHEN** ServiceConfig is configured with selector labels
- **THEN** the service routes to pods with matching labels

### Requirement: Configure service type
The system SHALL support configuring the Kubernetes Service type.

#### Scenario: Set service type to ClusterIP
- **WHEN** ServiceConfig is configured with type: ClusterIP
- **THEN** the service is accessible within the cluster only

#### Scenario: Set service type to NodePort
- **WHEN** ServiceConfig is configured with type: NodePort
- **THEN** the service is accessible on cluster nodes

#### Scenario: Set service type to LoadBalancer
- **WHEN** ServiceConfig is configured with type: LoadBalancer
- **THEN** the service gets an external IP from cloud provider

### Requirement: Configure service session affinity
The system SHALL support configuring session affinity for the service.

#### Scenario: Set session affinity to ClientIP
- **WHEN** ServiceConfig is configured with session_affinity: ClientIP
- **THEN** client connections are routed to the same pod

### Requirement: Expose step with service config
The system SHALL support using ServiceConfig to expose workflow steps.

#### Scenario: Expose job with service config
- **WHEN** KubeJobStep is built with service_config
- **AND** expose_service() is called
- **THEN** the system creates a service using the provided configuration
