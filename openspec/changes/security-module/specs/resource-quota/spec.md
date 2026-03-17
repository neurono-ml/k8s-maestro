## ADDED Requirements

### Requirement: ResourceQuota builder creates valid Kubernetes ResourceQuota resources

The system SHALL provide a `ResourceQuotaBuilder` that constructs valid `k8s_openapi::api::core::v1::ResourceQuota` resources.

#### Scenario: Basic ResourceQuota creation
- **WHEN** user creates a ResourceQuotaBuilder with name "compute-quota" and namespace "team-a"
- **THEN** builder SHALL produce a ResourceQuota with matching metadata name and namespace

#### Scenario: ResourceQuota with hard limits
- **WHEN** user sets hard limits for CPU ("10") and memory ("20Gi")
- **THEN** the resulting ResourceQuota SHALL have spec.hard with corresponding Quantity values

#### Scenario: ResourceQuota with scopes
- **WHEN** user adds scope "Terminating"
- **THEN** the resulting ResourceQuota SHALL include scope in spec.scopes array

### Requirement: Hard limits configuration with Quantity type

The system SHALL accept resource limits as `BTreeMap<String, Quantity>` for type-safe quantity specification.

#### Scenario: CPU quota limit
- **WHEN** user sets "requests.cpu" to "4"
- **THEN** spec.hard SHALL contain "requests.cpu" with Quantity "4"

#### Scenario: Memory quota limit
- **WHEN** user sets "limits.memory" to "16Gi"
- **THEN** spec.hard SHALL contain "limits.memory" with Quantity "16Gi"

#### Scenario: Pod count limit
- **WHEN** user sets "count/pods" to "10"
- **THEN** spec.hard SHALL contain "count/pods" with Quantity "10"

### Requirement: QuotaScope enumeration for quota scope selection

The system SHALL provide a `QuotaScope` enum for specifying quota scopes.

#### Scenario: Terminating scope
- **WHEN** user sets scope to Terminating
- **THEN** spec.scopes SHALL contain "Terminating"

#### Scenario: NotTerminating scope
- **WHEN** user sets scope to NotTerminating
- **THEN** spec.scopes SHALL contain "NotTerminating"

#### Scenario: BestEffort scope
- **WHEN** user sets scope to BestEffort
- **THEN** spec.scopes SHALL contain "BestEffort"

#### Scenario: NotBestEffort scope
- **WHEN** user sets scope to NotBestEffort
- **THEN** spec.scopes SHALL contain "NotBestEffort"

### Requirement: Preset resource quotas for workload tiers

The system SHALL provide preset resource quota configurations.

#### Scenario: Small workload preset
- **WHEN** user calls `ResourceQuotaBuilder::small_workload(name, namespace)`
- **THEN** builder SHALL produce a quota suitable for small workloads (limited CPU/memory/pods)

#### Scenario: Medium workload preset
- **WHEN** user calls `ResourceQuotaBuilder::medium_workload(name, namespace)`
- **THEN** builder SHALL produce a quota suitable for medium workloads (moderate CPU/memory/pods)

#### Scenario: Large workload preset
- **WHEN** user calls `ResourceQuotaBuilder::large_workload(name, namespace)`
- **THEN** builder SHALL produce a quota suitable for large workloads (generous CPU/memory/pods)
