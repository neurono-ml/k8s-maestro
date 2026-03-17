## ADDED Requirements

### Requirement: README must display project badges
The README SHALL display GitHub badges at the top including version, license, build status, documentation link, and crates.io link.

#### Scenario: User views README on GitHub
- **WHEN** user opens README.md on GitHub
- **THEN** badges for version, license, build status, docs.rs, and crates.io are visible at the top

### Requirement: README must have project description
The README SHALL include the project description "A Kubernetes workflow orchestrator with minimal requirements and full power".

#### Scenario: User reads project description
- **WHEN** user reads the README
- **THEN** the description clearly states "A Kubernetes workflow orchestrator with minimal requirements and full power"

### Requirement: README must list all features
The README SHALL include a features section documenting: multi-step workflows, conditional execution, multiple step types, services/ingress support, sidecar plugins, file observer, checkpointing, multi-tenant security, builder pattern, and TDD support.

#### Scenario: User evaluates library capabilities
- **WHEN** user reads the features section
- **THEN** all 10 features are listed with brief descriptions

### Requirement: README must include quick start guide
The README SHALL include a quick start guide with installation and basic usage.

#### Scenario: New user gets started
- **WHEN** user follows quick start guide
- **THEN** they can install and run a basic workflow

### Requirement: README must include usage examples
The README SHALL include both basic and advanced usage examples demonstrating the workflow builder API.

#### Scenario: User learns API patterns
- **WHEN** user reads usage examples
- **THEN** they see working code for basic workflow creation and advanced dependency chains

### Requirement: README must link to API documentation
The README SHALL include links to API documentation on docs.rs.

#### Scenario: User needs detailed API reference
- **WHEN** user clicks docs.rs link
- **THEN** they are taken to the API documentation

### Requirement: README must include contributing guidelines
The README SHALL include or link to contributing guidelines.

#### Scenario: Developer wants to contribute
- **WHEN** developer reads contributing section
- **THEN** they understand how to contribute to the project

### Requirement: README must include license information
The README SHALL include license information (MIT OR Apache-2.0).

#### Scenario: User checks license compatibility
- **WHEN** user reads license section
- **THEN** they see "MIT OR Apache-2.0" license
