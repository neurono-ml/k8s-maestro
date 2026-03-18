# container-env-config Specification

## Purpose
TBD - created by archiving change implement-maestro-container-builder. Update Purpose after archive.
## Requirements
### Requirement: Set environment variables from map
The system SHALL allow setting environment variables from a `BTreeMap<String, EnvironmentVariableSource>`.

#### Scenario: Set environment variables
- **WHEN** user calls `set_environment_variables(env_map)` with `{"PORT": Value("8080"), "DEBUG": Value("true")}`
- **THEN** resulting container has environment variables PORT=8080 and DEBUG=true

### Requirement: Add single environment variable
The system SHALL allow adding individual environment variables.

#### Scenario: Add environment variable
- **WHEN** user calls `add_environment_variable("API_KEY", "secret123")` on builder
- **THEN** resulting container has environment variable API_KEY=secret123

### Requirement: Environment variable from field reference
The system SHALL support environment variables from pod field references via `EnvironmentVariableSource::FieldRef`.

#### Scenario: Set env from pod field
- **WHEN** user creates `EnvironmentVariableSource::FieldRef(FieldRef { field_path: "metadata.name".to_string() })`
- **THEN** resulting k8s container has env var with `value_from.field_ref.field_path` set to "metadata.name"

### Requirement: Environment variable from resource field reference
The system SHALL support environment variables from resource field references via `EnvironmentVariableSource::ResourceFieldRef`.

#### Scenario: Set env from resource field
- **WHEN** user creates `EnvironmentVariableSource::ResourceFieldRef(ResourceFieldRef { resource: "limits.cpu".to_string() })`
- **THEN** resulting k8s container has env var with `value_from.resource_field_ref.resource` set to "limits.cpu"

### Requirement: Set environment variables from ConfigMap
The system SHALL support loading environment variables from ConfigMap via `EnvironmentVariableFromObject`.

#### Scenario: Add env from ConfigMap
- **WHEN** user calls `add_environment_variables_from_object(&EnvironmentVariableFromObject::ConfigMap("my-config".to_string()))`
- **THEN** resulting k8s container has `env_from.config_map_ref.name` set to "my-config"

### Requirement: Set environment variables from Secret
The system SHALL support loading environment variables from Secret via `EnvironmentVariableFromObject`.

#### Scenario: Add env from Secret
- **WHEN** user calls `add_environment_variables_from_object(&EnvironmentVariableFromObject::Secret("my-secret".to_string()))`
- **THEN** resulting k8s container has `env_from.secret_ref.name` set to "my-secret"

### Requirement: Set multiple env_from sources
The system SHALL allow setting multiple `env_from` sources at once.

#### Scenario: Set env from multiple sources
- **WHEN** user calls `set_environment_variables_from_objects(&[ConfigMap("config1"), Secret("secret1")])`
- **THEN** resulting k8s container has both ConfigMap and Secret in `env_from` list

