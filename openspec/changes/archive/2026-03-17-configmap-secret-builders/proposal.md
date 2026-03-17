## Why

Kubernetes workflows require configuration and sensitive data management through ConfigMaps and Secrets. Currently, k8s-maestro lacks builders for these core Kubernetes resources, forcing users to manually construct raw Kubernetes objects. This change introduces type-safe, fluent builders for ConfigMaps, Secrets, and specialized secrets like ImagePullSecrets, enabling workflow steps to create and manage configuration resources declaratively.

## What Changes

- New `ConfigMapBuilder` with fluent API for building Kubernetes ConfigMaps
- New `SecretBuilder` with fluent API for building Kubernetes Secrets
- New `SecretType` enum for type-safe secret types (Opaque, ServiceAccountToken, Dockercfg, etc.)
- New `ImagePullSecretBuilder` for creating docker-registry type secrets
- Helper functions for loading configuration from files and directories
- Helper functions for creating specialized secrets (TLS, Docker auth)
- Comprehensive unit tests for all builders and helpers
- Integration tests with Kind for real Kubernetes validation

## Capabilities

### New Capabilities

- `configmap-builder`: ConfigMapBuilder with fluent API, file/directory loading helpers, immutable support
- `secret-builder`: SecretBuilder with fluent API, SecretType enum, specialized secret helpers (TLS, Docker)
- `image-pull-secret-builder`: ImagePullSecretBuilder for docker-registry secrets with auth configuration

### Modified Capabilities

- None

## Impact

- **New modules**: `src/entities/config/mod.rs`, `src/entities/config/configmap.rs`, `src/entities/config/secret.rs`, `src/entities/config/image_pull_secret.rs`
- **Dependencies**: Uses existing `k8s-openapi` types for ConfigMap and Secret
- **API surface**: New public builders and helper functions in `k8s_maestro::entities::config`
- **Tests**: Unit tests in each module, integration tests in `tests/` with Kind cluster
