## ADDED Requirements

### Requirement: Set resource bounds from map
The system SHALL allow setting resource limits and requests from `BTreeMap<ComputeResource, Quantity>`.

#### Scenario: Set CPU and memory limits
- **WHEN** user calls `set_resource_bounds(bounds)` with `{Cpu: "500m", Memory: "256Mi"}`
- **THEN** resulting container has `resources.limits.cpu`="500m" and `resources.limits.memory`="256Mi"

### Requirement: ComputeResource enum supports standard resources
The system SHALL provide `ComputeResource` enum with variants: `Cpu`, `Memory`, `EphemeralStorage`, `Storage`, `Custom(String)`.

#### Scenario: Use standard compute resources
- **WHEN** user uses `ComputeResource::Cpu` or `ComputeResource::Memory`
- **THEN** system correctly maps to Kubernetes resource names "cpu" and "memory"

#### Scenario: Use custom compute resource
- **WHEN** user uses `ComputeResource::Custom("nvidia.com/gpu".to_string())`
- **THEN** system correctly maps to Kubernetes resource name "nvidia.com/gpu"

### Requirement: Resource limits differentiate limits and requests
The system SHALL support both resource limits and requests using a `ResourceLimits` type.

#### Scenario: Set limits and requests separately
- **WHEN** user sets `ResourceLimits { cpu: Some("500m"), cpu_request: Some("100m"), memory: Some("256Mi"), memory_request: Some("128Mi") }`
- **THEN** resulting container has `resources.limits.cpu`="500m", `resources.requests.cpu`="100m", `resources.limits.memory`="256Mi", `resources.requests.memory`="128Mi"

### Requirement: Resource bounds with Quantity type
The system SHALL use `k8s_openapi::apimachinery::pkg::api::resource::Quantity` for resource values.

#### Scenario: Use Quantity for resources
- **WHEN** user provides resource values as strings like "500m" or "1Gi"
- **THEN** system creates valid `Quantity` objects for Kubernetes API

### Requirement: Ephemeral storage resource support
The system SHALL support ephemeral storage resource configuration.

#### Scenario: Set ephemeral storage
- **WHEN** user includes `ComputeResource::EphemeralStorage` with value "1Gi" in resource bounds
- **THEN** resulting container has `resources.limits.ephemeral-storage`="1Gi"
