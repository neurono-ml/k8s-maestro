## ADDED Requirements

### Requirement: Organize fixtures by resource type
The test infrastructure SHALL organize test fixtures in tests/common/fixtures/ with subdirectories for each Kubernetes resource type (workflows, secrets, configmaps, pvcs, failure_scenarios).

#### Scenario: Fixtures directory structure
- **WHEN** test fixtures are organized
- **THEN** tests/common/fixtures/ contains subdirectories: workflows/, secrets/, configmaps/, pvcs/, failure_scenarios/
- **AND** each subdirectory contains relevant test fixtures for that resource type

#### Scenario: Workflow fixtures available
- **WHEN** tests need workflow resources
- **THEN** workflow fixtures are available in tests/common/fixtures/workflows/
- **AND** fixtures represent valid and invalid workflow configurations

#### Scenario: Failure scenario fixtures
- **WHEN** tests need to validate error handling
- **THEN** failure scenario fixtures are available in tests/common/fixtures/failure_scenarios/
- **AND** fixtures include malformed or problematic resource configurations

### Requirement: Support YAML and programmatic fixtures
The test infrastructure SHALL support both YAML file fixtures and programmatic fixture creation to accommodate static and dynamic test scenarios.

#### Scenario: YAML fixture for static resources
- **WHEN** test needs a static Kubernetes resource
- **THEN** YAML file fixture is available in appropriate fixtures/ subdirectory
- **AND** fixture can be loaded and applied to cluster

#### Scenario: Programmatic fixture for dynamic resources
- **WHEN** test needs a dynamically configured Kubernetes resource
- **THEN** helper function creates the resource programmatically
- **AND** resource parameters can be customized per test

#### Scenario: Mixed fixture approach
- **WHEN** test needs both static and dynamic resources
- **THEN** YAML fixtures provide base resources
- **AND** programmatic creation modifies or extends base resources

### Requirement: Provide fixture loading helpers
The test infrastructure SHALL provide helper functions for loading fixtures from files and creating resources programmatically.

#### Scenario: Load YAML fixture
- **WHEN** test needs to load a YAML fixture
- **THEN** helper function reads YAML file from fixtures/ directory
- **AND** returns parsed Kubernetes resource object

#### Scenario: Create resource programmatically
- **WHEN** test needs to create a resource dynamically
- **THEN** helper function accepts parameters and constructs valid Kubernetes resource
- **AND** resource is returned ready for cluster application

#### Scenario: Apply fixture to cluster
- **WHEN** test needs to apply a fixture to cluster
- **THEN** helper function applies loaded resource to test cluster
- **AND** operation waits for resource to be ready (when applicable)
