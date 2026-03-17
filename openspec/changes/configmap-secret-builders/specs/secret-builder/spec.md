## ADDED Requirements

### Requirement: SecretBuilder creates Secrets with fluent API
The system SHALL provide a `SecretBuilder` type that allows constructing Kubernetes Secrets using a fluent builder pattern.

#### Scenario: Build basic Secret with name
- **WHEN** user creates `SecretBuilder::new("my-secret")` and calls `build()`
- **THEN** system returns a `Secret` with metadata.name set to "my-secret"

#### Scenario: Build Secret with namespace
- **WHEN** user calls `with_namespace("production")` on the builder
- **THEN** system sets metadata.namespace to "production" in the resulting Secret

#### Scenario: Build Secret with type
- **WHEN** user calls `with_type(SecretType::Opaque)` on the builder
- **THEN** system sets type field to "Opaque" in the resulting Secret

#### Scenario: Build Secret with string data
- **WHEN** user calls `with_string_data("username", "admin")` on the builder
- **THEN** system adds the key-value pair to the Secret's stringData field

#### Scenario: Build Secret with binary data
- **WHEN** user calls `with_data("key.pem", vec![0x01, 0x02])` on the builder
- **THEN** system adds the key-value pair to the Secret's data field as base64

#### Scenario: Build Secret with labels
- **WHEN** user calls `with_labels(BTreeMap)` with labels on the builder
- **THEN** system sets metadata.labels to the provided map

#### Scenario: Build Secret with annotations
- **WHEN** user calls `with_annotations(BTreeMap)` with annotations on the builder
- **THEN** system sets metadata.annotations to the provided map

#### Scenario: Build immutable Secret
- **WHEN** user calls `with_immutable(true)` on the builder
- **THEN** system sets immutable field to true in the resulting Secret

### Requirement: SecretType enum provides type-safe secret types
The system SHALL provide a `SecretType` enum covering standard Kubernetes secret types.

#### Scenario: Opaque type
- **WHEN** user uses `SecretType::Opaque`
- **THEN** system produces type string "Opaque"

#### Scenario: ServiceAccountToken type
- **WHEN** user uses `SecretType::ServiceAccountToken`
- **THEN** system produces type string "kubernetes.io/service-account-token"

#### Scenario: Dockercfg type
- **WHEN** user uses `SecretType::Dockercfg`
- **THEN** system produces type string "kubernetes.io/dockercfg"

#### Scenario: DockerConfigJson type
- **WHEN** user uses `SecretType::DockerConfigJson`
- **THEN** system produces type string "kubernetes.io/dockerconfigjson"

#### Scenario: BasicAuth type
- **WHEN** user uses `SecretType::BasicAuth`
- **THEN** system produces type string "kubernetes.io/basic-auth"

#### Scenario: SshAuth type
- **WHEN** user uses `SecretType::SshAuth`
- **THEN** system produces type string "kubernetes.io/ssh-auth"

#### Scenario: Tls type
- **WHEN** user uses `SecretType::Tls`
- **THEN** system produces type string "kubernetes.io/tls"

#### Scenario: BootstrapToken type
- **WHEN** user uses `SecretType::BootstrapToken`
- **THEN** system produces type string "bootstrap.kubernetes.io/token"

### Requirement: Helper function creates Secret from file
The system SHALL provide `secret_from_file(path)` function to create a Secret from a file.

#### Scenario: Load Secret from file
- **WHEN** user calls `secret_from_file("/path/to/secret.key")`
- **THEN** system reads the file and creates a Secret with the file contents as data

### Requirement: Helper function creates Docker registry secret
The system SHALL provide `docker_secret_from_auth(auth)` function to create a docker-registry type Secret.

#### Scenario: Create Docker secret from auth string
- **WHEN** user calls `docker_secret_from_auth("registry.example.com", "username", "password", "email@example.com")`
- **THEN** system creates a Secret of type "kubernetes.io/dockerconfigjson" with proper .dockerconfigjson key

### Requirement: Helper function creates TLS secret
The system SHALL provide `tls_secret_from_certs(cert, key)` function to create a TLS type Secret.

#### Scenario: Create TLS secret from certificates
- **WHEN** user calls `tls_secret_from_certs("cert-content", "key-content")`
- **THEN** system creates a Secret of type "kubernetes.io/tls" with tls.crt and tls.key data entries

### Requirement: SecretBuilder converts to Kubernetes type
The system SHALL provide conversion from `SecretBuilder` to `k8s_openapi::api::core::v1::Secret`.

#### Scenario: Build returns valid Secret
- **WHEN** user calls `build()` on a configured builder
- **THEN** system returns a valid `k8s_openapi::api::core::v1::Secret` object
