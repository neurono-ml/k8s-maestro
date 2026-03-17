## ADDED Requirements

### Requirement: LimitRangeBuilder creates valid LimitRange resources

The system SHALL provide a `LimitRangeBuilder` that constructs valid `k8s_openapi::api::core::v1::LimitRange` resources.

#### Scenario: Basic LimitRange creation
- **WHEN** user creates a LimitRangeBuilder with name "container-limits" and namespace "default"
- **THEN** builder SHALL produce a LimitRange with matching metadata name and namespace

#### Scenario: LimitRange with default limits
- **WHEN** user sets default limit for CPU to "500m"
- **THEN** the resulting LimitRange SHALL include default.cpu in spec.limits

#### Scenario: LimitRange with max limits
- **WHEN** user sets max limit for memory to "4Gi"
- **THEN** the resulting LimitRange SHALL include max.memory in spec.limits

### Requirement: Limit range types for different resource kinds

The system SHALL support limit ranges for Container, Pod, and PersistentVolumeClaim types.

#### Scenario: Container type limits
- **WHEN** user sets limit type to Container
- **THEN** the limit SHALL apply to individual containers with type "Container"

#### Scenario: Pod type limits
- **WHEN** user sets limit type to Pod
- **THEN** the limit SHALL apply to entire pods with type "Pod"

#### Scenario: PersistentVolumeClaim type limits
- **WHEN** user sets limit type to PersistentVolumeClaim
- **THEN** the limit SHALL apply to PVC storage with type "PersistentVolumeClaim"

### Requirement: Default and min/max resource constraints

The system SHALL support default, defaultRequest, min, and max constraint types.

#### Scenario: Default resource limit
- **WHEN** user sets default for memory to "512Mi"
- **THEN** containers without explicit limits SHALL use 512Mi as default

#### Scenario: Default request limit
- **WHEN** user sets default_request for CPU to "100m"
- **THEN** containers without explicit requests SHALL use 100m as default request

#### Scenario: Min resource limit
- **WHEN** user sets min for CPU to "50m"
- **THEN** containers MUST request at least 50m CPU

#### Scenario: Max resource limit
- **WHEN** user sets max for memory to "2Gi"
- **THEN** containers SHALL NOT exceed 2Gi memory

### Requirement: LimitRangeItem configuration

The system SHALL provide methods to configure complete LimitRangeItem entries.

#### Scenario: Configure complete container limits
- **WHEN** user configures Container type with default, min, and max for CPU and memory
- **THEN** the LimitRange SHALL contain a complete LimitRangeItem for Container type

#### Scenario: Configure PVC storage limits
- **WHEN** user configures PersistentVolumeClaim type with min and max storage
- **THEN** the LimitRange SHALL contain a LimitRangeItem restricting PVC sizes
