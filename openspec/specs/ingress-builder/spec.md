## ADDED Requirements

### Requirement: IngressBuilder creates Kubernetes Ingress
The system SHALL provide an IngressBuilder with fluent API for creating Kubernetes Ingress resources.

#### Scenario: Create basic ingress
- **WHEN** user creates an IngressBuilder with name, namespace, host, and path mapping
- **THEN** system generates a valid Kubernetes Ingress with specified routing rules

#### Scenario: Create ingress with TLS
- **WHEN** user creates an IngressBuilder with TLS secret configured
- **THEN** system generates an Ingress with TLS configuration pointing to the secret

#### Scenario: Create ingress with multiple paths
- **WHEN** user calls with_paths with multiple IngressPath structs
- **THEN** system creates routing rules for all specified paths

### Requirement: IngressBuilder supports annotations
The system SHALL allow custom annotations for ingress-specific configurations.

#### Scenario: Add annotations
- **WHEN** user calls with_annotations with a map of key-value pairs
- **THEN** system adds all annotations to the Ingress metadata

### Requirement: IngressBuilder supports ingress class
The system SHALL allow specification of ingress class for controller selection.

#### Scenario: Set ingress class
- **WHEN** user calls with_ingress_class("nginx")
- **THEN** system sets ingressClassName to "nginx" in the Ingress spec

### Requirement: IngressBuilder validates required fields
The system SHALL return an error when required fields are missing.

#### Scenario: Missing name
- **WHEN** user builds an ingress without setting name
- **THEN** system returns error indicating name is required

#### Scenario: Missing namespace
- **WHEN** user builds an ingress without setting namespace
- **THEN** system returns error indicating namespace is required
