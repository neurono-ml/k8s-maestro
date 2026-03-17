## 1. Module Structure

- [ ] 1.1 Create `src/entities/config/mod.rs` with public re-exports
- [ ] 1.2 Create `src/entities/config/configmap.rs` with ConfigMapBuilder struct
- [ ] 1.3 Create `src/entities/config/secret.rs` with SecretBuilder and SecretType
- [ ] 1.4 Create `src/entities/config/image_pull_secret.rs` with ImagePullSecretBuilder
- [ ] 1.5 Update `src/entities/mod.rs` to export config module

## 2. ConfigMapBuilder Implementation

- [ ] 2.1 Implement `ConfigMapBuilder::new(name)` constructor
- [ ] 2.2 Implement `with_namespace(namespace)` method
- [ ] 2.3 Implement `with_data(key, value)` method for string data
- [ ] 2.4 Implement `with_binary_data(key, bytes)` method for binary data
- [ ] 2.5 Implement `with_labels(labels)` method
- [ ] 2.6 Implement `with_annotations(annotations)` method
- [ ] 2.7 Implement `with_immutable(immutable)` method
- [ ] 2.8 Implement `build()` method returning `Result<ConfigMap>`

## 3. ConfigMap Helper Functions

- [ ] 3.1 Implement `configmap_from_file(path)` function
- [ ] 3.2 Implement `configmap_from_directory(dir)` function

## 4. SecretBuilder Implementation

- [ ] 4.1 Implement `SecretType` enum with all Kubernetes secret types
- [ ] 4.2 Implement `Display` trait for `SecretType`
- [ ] 4.3 Implement `SecretBuilder::new(name)` constructor
- [ ] 4.4 Implement `with_namespace(namespace)` method
- [ ] 4.5 Implement `with_type(type)` method
- [ ] 4.6 Implement `with_string_data(key, value)` method
- [ ] 4.7 Implement `with_data(key, bytes)` method for binary data
- [ ] 4.8 Implement `with_labels(labels)` method
- [ ] 4.9 Implement `with_annotations(annotations)` method
- [ ] 4.10 Implement `with_immutable(immutable)` method
- [ ] 4.11 Implement `build()` method returning `Result<Secret>`

## 5. Secret Helper Functions

- [ ] 5.1 Implement `secret_from_file(path)` function
- [ ] 5.2 Implement `docker_secret_from_auth(registry, username, password, email)` function
- [ ] 5.3 Implement `tls_secret_from_certs(cert, key)` function

## 6. ImagePullSecretBuilder Implementation

- [ ] 6.1 Implement `ImagePullSecretBuilder::new(name)` constructor
- [ ] 6.2 Implement `with_registry(registry)` method
- [ ] 6.3 Implement `with_username(username)` method
- [ ] 6.4 Implement `with_password(password)` method
- [ ] 6.5 Implement `with_email(email)` method
- [ ] 6.6 Implement `build()` method with validation and `Result<Secret>`

## 7. Unit Tests

- [ ] 7.1 Add unit tests for ConfigMapBuilder basic construction
- [ ] 7.2 Add unit tests for ConfigMapBuilder with all options
- [ ] 7.3 Add unit tests for ConfigMap file helpers
- [ ] 7.4 Add unit tests for SecretBuilder basic construction
- [ ] 7.5 Add unit tests for SecretType enum conversions
- [ ] 7.6 Add unit tests for SecretBuilder with all options
- [ ] 7.7 Add unit tests for Secret helper functions
- [ ] 7.8 Add unit tests for ImagePullSecretBuilder construction
- [ ] 7.9 Add unit tests for ImagePullSecretBuilder validation
- [ ] 7.10 Add unit tests for ImagePullSecret dockerconfigjson format

## 8. Integration Tests

- [ ] 8.1 Create integration test for ConfigMap creation and pod reading
- [ ] 8.2 Create integration test for Secret creation and pod reading
- [ ] 8.3 Create integration test for ImagePullSecret creation
- [ ] 8.4 Create integration test for immutable ConfigMap
- [ ] 8.5 Create integration test for immutable Secret
- [ ] 8.6 Create integration test for ConfigMap update

## 9. Documentation and Examples

- [ ] 9.1 Add rustdoc comments to all public types and methods
- [ ] 9.2 Create example for ConfigMapBuilder usage
- [ ] 9.3 Create example for SecretBuilder usage
- [ ] 9.4 Create example for ImagePullSecretBuilder usage
- [ ] 9.5 Update CHANGELOG.md with new features
