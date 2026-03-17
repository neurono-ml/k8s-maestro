## ADDED Requirements

### Requirement: ServiceBuilder creates Kubernetes Services
The system SHALL provide a ServiceBuilder with fluent API for creating Kubernetes Service resources.

#### Scenario: Create ClusterIP service with all options
- **WHEN** user creates a ServiceBuilder with name, namespace, port, selector, and type ClusterIP
- **THEN** system generates a valid Kubernetes Service with specified configuration

#### Scenario: Create headless service
- **WHEN** user creates a ServiceBuilder with type Headless or cluster_ip "None"
- **THEN** system generates a Service with clusterIP set to "None"

#### Scenario: Create NodePort service
- **WHEN** user creates a ServiceBuilder with type NodePort
- **THEN** system generates a Service with type NodePort and allocates node port

#### Scenario: Create LoadBalancer service
- **WHEN** user creates a ServiceBuilder with type LoadBalancer
- **THEN** system generates a Service with type LoadBalancer

### Requirement: ServiceBuilder supports port configuration
The system SHALL allow configuration of single or multiple ports with protocol specification.

#### Scenario: Configure single port
- **WHEN** user calls with_port(80, 8080, "TCP")
- **THEN** system creates a port mapping from 80 to 8080 with TCP protocol

#### Scenario: Configure multiple ports
- **WHEN** user calls with_ports with a vector of ServicePort structs
- **THEN** system creates all specified port mappings with names and protocols

### Requirement: ServiceBuilder supports advanced options
The system SHALL support session affinity, external traffic policy, and cluster IP configuration.

#### Scenario: Configure session affinity
- **WHEN** user calls with_session_affinity("ClientIP")
- **THEN** system sets sessionAffinity to ClientIP in the Service

#### Scenario: Configure external traffic policy
- **WHEN** user calls with_external_traffic_policy("Local")
- **THEN** system sets externalTrafficPolicy to Local in the Service

### Requirement: ServiceBuilder validates required fields
The system SHALL return an error when required fields are missing.

#### Scenario: Missing name
- **WHEN** user builds a service without setting name
- **THEN** system returns error indicating name is required

#### Scenario: Missing namespace
- **WHEN** user builds a service without setting namespace
- **THEN** system returns error indicating namespace is required
