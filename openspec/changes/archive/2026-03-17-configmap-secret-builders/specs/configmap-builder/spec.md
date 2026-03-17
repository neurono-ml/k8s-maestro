## ADDED Requirements

### Requirement: ConfigMapBuilder creates ConfigMaps with fluent API
The system SHALL provide a `ConfigMapBuilder` type that allows constructing Kubernetes ConfigMaps using a fluent builder pattern.

#### Scenario: Build basic ConfigMap with name
- **WHEN** user creates `ConfigMapBuilder::new("my-config")` and calls `build()`
- **THEN** system returns a `ConfigMap` with metadata.name set to "my-config"

#### Scenario: Build ConfigMap with namespace
- **WHEN** user calls `with_namespace("production")` on the builder
- **THEN** system sets metadata.namespace to "production" in the resulting ConfigMap

#### Scenario: Build ConfigMap with string data
- **WHEN** user calls `with_data("config.yaml", "key: value")` on the builder
- **THEN** system adds the key-value pair to the ConfigMap's data field

#### Scenario: Build ConfigMap with binary data
- **WHEN** user calls `with_binary_data("binary.bin", vec![0x01, 0x02])` on the builder
- **THEN** system adds the key-value pair to the ConfigMap's binaryData field as base64

#### Scenario: Build ConfigMap with labels
- **WHEN** user calls `with_labels(BTreeMap)` with labels on the builder
- **THEN** system sets metadata.labels to the provided map

#### Scenario: Build ConfigMap with annotations
- **WHEN** user calls `with_annotations(BTreeMap)` with annotations on the builder
- **THEN** system sets metadata.annotations to the provided map

#### Scenario: Build immutable ConfigMap
- **WHEN** user calls `with_immutable(true)` on the builder
- **THEN** system sets immutable field to true in the resulting ConfigMap

### Requirement: ConfigMapBuilder supports multiple data entries
The system SHALL allow adding multiple data entries to a ConfigMap.

#### Scenario: Add multiple string data entries
- **WHEN** user calls `with_data("key1", "value1")` and `with_data("key2", "value2")`
- **THEN** system includes both entries in the ConfigMap's data field

#### Scenario: Mix string and binary data
- **WHEN** user calls both `with_data()` and `with_binary_data()` methods
- **THEN** system populates both data and binaryData fields in the ConfigMap

### Requirement: Helper function creates ConfigMap from file
The system SHALL provide `configmap_from_file(path)` function to create a ConfigMap from a file.

#### Scenario: Load ConfigMap from file
- **WHEN** user calls `configmap_from_file("/path/to/config.yaml")`
- **THEN** system reads the file and creates a ConfigMap with the file contents as data

#### Scenario: File not found error
- **WHEN** user calls `configmap_from_file()` with non-existent path
- **THEN** system returns an error indicating file not found

### Requirement: Helper function creates ConfigMap from directory
The system SHALL provide `configmap_from_directory(dir)` function to create a ConfigMap from all files in a directory.

#### Scenario: Load ConfigMap from directory
- **WHEN** user calls `configmap_from_directory("/path/to/configs/")`
- **THEN** system reads all files in the directory and creates a ConfigMap with each file as a key

#### Scenario: Empty directory handling
- **WHEN** user calls `configmap_from_directory()` on empty directory
- **THEN** system creates a ConfigMap with empty data field

### Requirement: ConfigMapBuilder converts to Kubernetes type
The system SHALL provide conversion from `ConfigMapBuilder` to `k8s_openapi::api::core::v1::ConfigMap`.

#### Scenario: Build returns valid ConfigMap
- **WHEN** user calls `build()` on a configured builder
- **THEN** system returns a valid `k8s_openapi::api::core::v1::ConfigMap` object
