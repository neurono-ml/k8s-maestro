## ADDED Requirements

### Requirement: SecurityContextConfig defines security settings

The system SHALL provide a `SecurityContextConfig` struct for configuring security settings.

#### Scenario: Run as specific user
- **WHEN** user sets run_as_user to 1000
- **THEN** the security context SHALL configure runAsUser to 1000

#### Scenario: Run as specific group
- **WHEN** user sets run_as_group to 3000
- **THEN** the security context SHALL configure runAsGroup to 3000

#### Scenario: Run as non-root
- **WHEN** user sets run_as_non_root to true
- **THEN** the security context SHALL set runAsNonRoot to true

#### Scenario: Read-only root filesystem
- **WHEN** user sets read_only_root_filesystem to true
- **THEN** the security context SHALL set readOnlyRootFilesystem to true

#### Scenario: Prevent privilege escalation
- **WHEN** user sets allow_privilege_escalation to false
- **THEN** the security context SHALL set allowPrivilegeEscalation to false

### Requirement: PodSecurityContext for pod-level security

The system SHALL provide a `PodSecurityContext` type for pod-level security settings.

#### Scenario: Pod-level security context
- **WHEN** user creates a PodSecurityContext with fsGroup 2000
- **THEN** the resulting context SHALL set securityContext.fsGroup to 2000

#### Scenario: Pod with supplemental groups
- **WHEN** user sets supplemental_groups to [1000, 2000]
- **THEN** the resulting context SHALL include supplementalGroups array

### Requirement: ContainerSecurityContext for container-level security

The system SHALL provide a `ContainerSecurityContext` type for container-level security settings.

#### Scenario: Container with capabilities
- **WHEN** user adds capability "NET_ADMIN" to add list
- **THEN** the resulting context SHALL include capabilities.add with "NET_ADMIN"

#### Scenario: Container with dropped capabilities
- **WHEN** user adds capability "ALL" to drop list
- **THEN** the resulting context SHALL include capabilities.drop with "ALL"

#### Scenario: Container with seccomp profile
- **WHEN** user sets seccomp_profile to RuntimeDefault
- **THEN** the resulting context SHALL set seccompProfile.type to RuntimeDefault

### Requirement: Preset security contexts for security tiers

The system SHALL provide preset security context configurations.

#### Scenario: Restricted preset
- **WHEN** user calls `SecurityContextConfig::restricted()`
- **THEN** config SHALL enforce: non-root, no privilege escalation, read-only filesystem, dropped capabilities

#### Scenario: Baseline preset
- **WHEN** user calls `SecurityContextConfig::baseline()`
- **THEN** config SHALL enforce: non-root, no privilege escalation with moderate restrictions

#### Scenario: Privileged preset
- **WHEN** user calls `SecurityContextConfig::privileged()`
- **THEN** config SHALL allow: root, privilege escalation, all capabilities
