## ADDED Requirements

### Requirement: Organize tests by isolation level
The test infrastructure SHALL organize tests into three categories: unit tests, integration tests, and E2E tests, each with appropriate isolation strategies.

#### Scenario: Unit tests use companion modules
- **WHEN** unit tests are written
- **THEN** tests are placed in _test.rs companion modules alongside source files
- **AND** tests use selective mocking, no Kind cluster required

#### Scenario: Integration tests use per-file cluster
- **WHEN** integration tests are written
- **THEN** tests are placed in tests/integration/ directory
- **AND** each test file has its own Kind cluster isolated from other files

#### Scenario: E2E tests use cluster scenarios
- **WHEN** E2E tests are written
- **THEN** tests are placed in tests/e2e/ directory
- **AND** tests cover complete workflow scenarios with real cluster interaction

### Requirement: Unit tests execute without cluster
The test infrastructure SHALL ensure unit tests execute without Kind cluster, using selective mocking for speed.

#### Scenario: Unit test mocks K8s client
- **WHEN** unit test runs
- **THEN** K8s client operations are mocked (no actual cluster calls)
- **AND** test validates logic without cluster dependencies

#### Scenario: Unit test execution is fast
- **WHEN** unit tests run
- **THEN** tests complete in seconds (not minutes)
- **AND** no Docker or Kind setup is required

### Requirement: Integration tests use per-file cluster
The test infrastructure SHALL provision one Kind cluster per integration test file, isolated from other test files.

#### Scenario: Integration test file has dedicated cluster
- **WHEN** integration test file executes
- **THEN** single Kind cluster is provisioned for all tests in the file
- **AND** cluster is deleted after all tests in the file complete

#### Scenario: Integration tests validate real behavior
- **WHEN** integration test runs
- **THEN** test interacts with real Kind cluster
- **AND** K8s resources are created/managed on actual cluster

### Requirement: E2E tests cover full scenarios
The test infrastructure SHALL support E2E tests that validate complete workflow scenarios from start to finish.

#### Scenario: E2E test runs complete workflow
- **WHEN** E2E test executes
- **THEN** test creates resources, executes workflow, validates results, cleans up
- **AND** all steps interact with real Kind cluster

#### Scenario: E2E test validates cross-component behavior
- **WHEN** E2E test runs
- **THEN** multiple components interact in the test scenario
- **AND** end-to-end behavior is validated

### Requirement: Provide test category selectors
The test infrastructure SHALL allow running tests by category (unit, integration, E2E) separately.

#### Scenario: Run only unit tests
- **WHEN** developer wants fast feedback
- **THEN** cargo test --lib executes unit tests only
- **AND** no Kind cluster is created

#### Scenario: Run only integration tests
- **WHEN** developer validates cluster interactions
- **THEN** cargo test --test integration executes integration tests only
- **AND** Kind clusters are provisioned per test file

#### Scenario: Run only E2E tests
- **WHEN** developer validates complete scenarios
- **THEN** cargo test --test e2e executes E2E tests only
- **AND** full workflow scenarios are validated
