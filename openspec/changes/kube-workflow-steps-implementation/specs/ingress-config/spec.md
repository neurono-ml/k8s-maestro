## ADDED Requirements

### Requirement: Define IngressConfig type
The system SHALL provide an IngressConfig type for configuring Kubernetes Ingress creation.

#### Scenario: Create ingress config with host
- **WHEN** IngressConfig is created with a host name
- **THEN** the config specifies the ingress host
- **AND** can be used to expose a workflow step

#### Scenario: Create ingress config with path
- **WHEN** IngressConfig is created with a path prefix
- **THEN** the config specifies the path routing rule
- **AND** traffic matching the path is routed to the service

### Requirement: Configure ingress service backend
The system SHALL support configuring the service backend for the ingress.

#### Scenario: Set service name and port
- **WHEN** IngressConfig is configured with service_name and service_port
- **THEN** the ingress routes to the specified service port

### Requirement: Configure ingress TLS
The system SHALL support configuring TLS for the ingress.

#### Scenario: Set TLS certificate
- **WHEN** IngressConfig is configured with TLS certificate secret name
- **THEN** the ingress is configured for HTTPS traffic
- **AND** uses the specified secret for TLS termination

### Requirement: Configure ingress annotations
The system SHALL support configuring Kubernetes annotations for the ingress.

#### Scenario: Set nginx ingress annotations
- **WHEN** IngressConfig is configured with nginx-specific annotations
- **THEN** the ingress controller applies the nginx configuration

### Requirement: Expose step with ingress config
The system SHALL support using IngressConfig to expose workflow steps.

#### Scenario: Expose job with ingress config
- **WHEN** KubeJobStep is built with ingress_config
- **AND** expose_ingress() is called
- **THEN** the system creates an ingress using the provided configuration
