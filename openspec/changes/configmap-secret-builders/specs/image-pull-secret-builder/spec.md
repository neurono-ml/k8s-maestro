## ADDED Requirements

### Requirement: ImagePullSecretBuilder creates docker-registry secrets
The system SHALL provide an `ImagePullSecretBuilder` type for creating Kubernetes docker-registry type secrets for image pulling.

#### Scenario: Build basic ImagePullSecret with name
- **WHEN** user creates `ImagePullSecretBuilder::new("my-registry-secret")` and calls `build()`
- **THEN** system returns a `Secret` with metadata.name set to "my-registry-secret" and type "kubernetes.io/dockerconfigjson"

#### Scenario: Build ImagePullSecret with registry
- **WHEN** user calls `with_registry("https://index.docker.io/v1/")` on the builder
- **THEN** system sets the registry server in the docker config

#### Scenario: Build ImagePullSecret with username
- **WHEN** user calls `with_username("myuser")` on the builder
- **THEN** system sets the username in the docker config auth

#### Scenario: Build ImagePullSecret with password
- **WHEN** user calls `with_password("mypassword")` on the builder
- **THEN** system sets the password in the docker config auth

#### Scenario: Build ImagePullSecret with email
- **WHEN** user calls `with_email("user@example.com")` on the builder
- **THEN** system sets the email in the docker config auth

### Requirement: ImagePullSecretBuilder generates correct dockerconfigjson format
The system SHALL generate the correct `.dockerconfigjson` structure for Kubernetes.

#### Scenario: Docker config JSON structure
- **WHEN** user builds an ImagePullSecret with registry, username, password, and email
- **THEN** system creates a Secret with data key ".dockerconfigjson" containing base64-encoded JSON with auths structure

#### Scenario: Auth is base64 encoded
- **WHEN** user builds an ImagePullSecret
- **THEN** system base64-encodes the username:password combination for the auth field

### Requirement: ImagePullSecretBuilder returns Kubernetes Secret type
The system SHALL return `k8s_openapi::api::core::v1::Secret` from the build method.

#### Scenario: Build returns valid Secret
- **WHEN** user calls `build()` on a fully configured ImagePullSecretBuilder
- **THEN** system returns a valid `k8s_openapi::api::core::v1::Secret` object with type "kubernetes.io/dockerconfigjson"

### Requirement: ImagePullSecretBuilder validates required fields
The system SHALL require registry, username, and password for a valid ImagePullSecret.

#### Scenario: Missing registry returns error
- **WHEN** user calls `build()` without setting registry
- **THEN** system returns an error indicating registry is required

#### Scenario: Missing username returns error
- **WHEN** user calls `build()` without setting username
- **THEN** system returns an error indicating username is required

#### Scenario: Missing password returns error
- **WHEN** user calls `build()` without setting password
- **THEN** system returns an error indicating password is required

#### Scenario: Email is optional
- **WHEN** user calls `build()` without setting email
- **THEN** system successfully creates the Secret with email field omitted or empty
