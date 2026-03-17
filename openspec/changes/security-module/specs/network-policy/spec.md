## ADDED Requirements

### Requirement: NetworkPolicy builder creates valid Kubernetes NetworkPolicy resources

The system SHALL provide a `NetworkPolicyBuilder` that constructs valid `k8s_openapi::api::networking::v1::NetworkPolicy` resources.

#### Scenario: Basic NetworkPolicy creation
- **WHEN** user creates a NetworkPolicyBuilder with name "test-policy" and namespace "default"
- **THEN** builder SHALL produce a NetworkPolicy with matching metadata name and namespace

#### Scenario: NetworkPolicy with pod selector
- **WHEN** user sets pod selector with labels `{"app": "web"}`
- **THEN** the resulting NetworkPolicy SHALL have spec.podSelector.matchLabels set accordingly

#### Scenario: NetworkPolicy with ingress rules
- **WHEN** user adds an ingress rule with from selector and port
- **THEN** the resulting NetworkPolicy SHALL include the rule in spec.ingress array

#### Scenario: NetworkPolicy with egress rules
- **WHEN** user adds an egress rule with to selector and port
- **THEN** the resulting NetworkPolicy SHALL include the rule in spec.egress array

### Requirement: NetworkPolicyRule configures traffic sources and destinations

The system SHALL provide a `NetworkPolicyRule` type for configuring ingress from and egress to selectors.

#### Scenario: Rule with namespace selector
- **WHEN** user creates a rule with namespace selector `{"kubernetes.io/metadata.name": "production"}`
- **THEN** the rule SHALL translate to appropriate NetworkPolicyPeer with namespaceSelector

#### Scenario: Rule with pod selector
- **WHEN** user creates a rule with pod selector `{"app": "api"}`
- **THEN** the rule SHALL translate to appropriate NetworkPolicyPeer with podSelector

#### Scenario: Rule with port configuration
- **WHEN** user specifies port 8080 with TCP protocol
- **THEN** the rule SHALL include Port with port number and protocol

### Requirement: Preset network policies for common scenarios

The system SHALL provide preset network policy configurations.

#### Scenario: Deny all traffic preset
- **WHEN** user calls `NetworkPolicyBuilder::deny_all(name, namespace)`
- **THEN** builder SHALL produce a policy that denies all ingress and egress traffic

#### Scenario: Allow all traffic preset
- **WHEN** user calls `NetworkPolicyBuilder::allow_all(name, namespace)`
- **THEN** builder SHALL produce a policy that allows all ingress and egress traffic

#### Scenario: Allow within namespace preset
- **WHEN** user calls `NetworkPolicyBuilder::allow_within_namespace(name, namespace)`
- **THEN** builder SHALL produce a policy that allows traffic only within the same namespace

### Requirement: PolicyType enumeration for policy scope

The system SHALL provide a `PolicyType` enum for specifying policy types.

#### Scenario: Ingress-only policy
- **WHEN** user sets policy type to Ingress
- **THEN** only spec.policyTypes SHALL contain "Ingress"

#### Scenario: Egress-only policy
- **WHEN** user sets policy type to Egress
- **THEN** only spec.policyTypes SHALL contain "Egress"

#### Scenario: Both ingress and egress policy
- **WHEN** user sets policy type to Both
- **THEN** spec.policyTypes SHALL contain both "Ingress" and "Egress"
