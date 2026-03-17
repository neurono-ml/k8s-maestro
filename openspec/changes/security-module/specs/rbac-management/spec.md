## ADDED Requirements

### Requirement: ServiceAccountBuilder creates valid ServiceAccount resources

The system SHALL provide a `ServiceAccountBuilder` that constructs valid `k8s_openapi::api::core::v1::ServiceAccount` resources.

#### Scenario: Basic ServiceAccount creation
- **WHEN** user creates a ServiceAccountBuilder with name "workflow-sa" and namespace "production"
- **THEN** builder SHALL produce a ServiceAccount with matching metadata name and namespace

#### Scenario: ServiceAccount with annotations
- **WHEN** user adds annotation "eks.amazonaws.com/role-arn" with role ARN value
- **THEN** the resulting ServiceAccount SHALL include the annotation in metadata.annotations

### Requirement: RoleBuilder creates valid Role resources

The system SHALL provide a `RoleBuilder` that constructs valid `k8s_openapi::api::rbac::v1::Role` resources.

#### Scenario: Basic Role creation
- **WHEN** user creates a RoleBuilder with name "pod-reader" and namespace "default"
- **THEN** builder SHALL produce a Role with matching metadata name and namespace

#### Scenario: Role with policy rules
- **WHEN** user adds a rule with apiGroups ["core"], resources ["pods"], verbs ["get", "list"]
- **THEN** the resulting Role SHALL include the rule in rules array

### Requirement: RoleBindingBuilder creates valid RoleBinding resources

The system SHALL provide a `RoleBindingBuilder` that constructs valid `k8s_openapi::api::rbac::v1::RoleBinding` resources.

#### Scenario: Basic RoleBinding creation
- **WHEN** user creates a RoleBindingBuilder with name "pod-reader-binding" and namespace "default"
- **THEN** builder SHALL produce a RoleBinding with matching metadata name and namespace

#### Scenario: RoleBinding with subject
- **WHEN** user sets subject to ServiceAccount "workflow-sa" in namespace "production"
- **THEN** the resulting RoleBinding SHALL include the subject in subjects array

#### Scenario: RoleBinding with role reference
- **WHEN** user sets roleRef to Role "pod-reader"
- **THEN** the resulting RoleBinding SHALL reference the role in roleRef

### Requirement: ClusterRoleBuilder and ClusterRoleBindingBuilder for cluster-wide RBAC

The system SHALL provide builders for cluster-scoped RBAC resources.

#### Scenario: ClusterRole creation
- **WHEN** user creates a ClusterRoleBuilder with name "node-reader"
- **THEN** builder SHALL produce a ClusterRole with cluster-wide scope (no namespace)

#### Scenario: ClusterRoleBinding creation
- **WHEN** user creates a ClusterRoleBindingBuilder binding ServiceAccount to ClusterRole
- **THEN** builder SHALL produce a ClusterRoleBinding with appropriate roleRef and subjects

### Requirement: PolicyRule configures RBAC permissions

The system SHALL provide a `PolicyRule` type for configuring RBAC rules.

#### Scenario: Rule with API groups
- **WHEN** user sets api_groups to ["apps", "batch"]
- **THEN** the rule SHALL include ["apps", "batch"] in apiGroups

#### Scenario: Rule with resources
- **WHEN** user sets resources to ["deployments", "jobs"]
- **THEN** the rule SHALL include ["deployments", "jobs"] in resources

#### Scenario: Rule with verbs
- **WHEN** user sets verbs to ["get", "list", "watch"]
- **THEN** the rule SHALL include ["get", "list", "watch"] in verbs

#### Scenario: Rule with resource names
- **WHEN** user sets resource_names to ["specific-config"]
- **THEN** the rule SHALL restrict access to named resource in resourceNames

### Requirement: Preset roles for common workflow scenarios

The system SHALL provide preset role configurations.

#### Scenario: Workflow executor preset
- **WHEN** user calls `RoleBuilder::workflow_executor(name, namespace)`
- **THEN** builder SHALL produce a role with permissions to create/manage jobs and pods

#### Scenario: Workflow viewer preset
- **WHEN** user calls `RoleBuilder::workflow_viewer(name, namespace)`
- **THEN** builder SHALL produce a role with read-only permissions for jobs and pods

#### Scenario: Admin preset
- **WHEN** user calls `RoleBuilder::admin(name, namespace)`
- **THEN** builder SHALL produce a role with full permissions for all resources in namespace
