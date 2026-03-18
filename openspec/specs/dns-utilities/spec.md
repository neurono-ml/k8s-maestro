## ADDED Requirements

### Requirement: Generate service DNS names
The system SHALL provide a function to generate standard Kubernetes service DNS names.

#### Scenario: Generate service FQDN
- **WHEN** user calls service_dns_name("my-service", "default")
- **THEN** system returns "my-service.default.svc.cluster.local"

#### Scenario: Generate service FQDN with custom namespace
- **WHEN** user calls service_dns_name("api-server", "production")
- **THEN** system returns "api-server.production.svc.cluster.local"

### Requirement: Generate pod DNS names
The system SHALL provide a function to generate standard Kubernetes pod DNS names.

#### Scenario: Generate pod FQDN
- **WHEN** user calls pod_dns_name("my-pod", "default")
- **THEN** system returns "my-pod.default.pod.cluster.local"

### Requirement: Generate headless service DNS patterns
The system SHALL provide a function to generate wildcard DNS patterns for headless services.

#### Scenario: Generate headless service wildcard pattern
- **WHEN** user calls headless_service_dns_pattern("stateful-set", "default")
- **THEN** system returns "*.stateful-set.default.svc.cluster.local"
