## Context

k8s-maestro is a Kubernetes workflow orchestrator library currently lacking comprehensive test infrastructure. The project uses Rust with Cargo, and the existing k8s-maestro-k8s crate provides Kubernetes client functionality. Tests are currently ad-hoc without proper isolation or fixture organization. This creates technical debt for future development since tests must exist before implementation (TDD requirement).

**Constraints:**
- Must use testcontainers-rs with Kind for integration tests
- Unit tests must run fast (no cluster overhead)
- Integration tests need real cluster validation
- E2E tests should cover complete workflow scenarios
- Fixtures must support both YAML and programmatic creation

## Goals / Non-Goals

**Goals:**
- Establish reusable Kind cluster lifecycle with per-suite isolation
- Create organized fixture structure by resource type
- Provide test utilities for resource creation/cleanup
- Enable selective mocking for unit tests
- Ensure clear test organization patterns (unit/integration/E2E)
- Set up testcontainers-rs integration with existing crate structure

**Non-Goals:**
- Implementing actual test cases (only infrastructure)
- Mocking external services beyond K8s API
- Performance optimization of cluster provisioning
- Multi-cluster test scenarios

## Decisions

### 1. Use testcontainers-rs with Kind module
**Rationale:** testcontainers-rs provides Docker-based test isolation, and the Kind module offers pre-built integration. This eliminates manual Kind cluster setup and ensures clean test environments.

**Alternatives considered:**
- Manual Kind cluster setup: More control but requires external setup
- Real cluster connection: No isolation, requires external cluster
- K3s testcontainers: Lighter but less K8s API coverage

### 2. Per-test-file cluster isolation
**Rationale:** Creating a new Kind cluster for each test file provides good isolation without excessive overhead. Test files group related scenarios that can share a cluster.

**Alternatives considered:**
- Per-test isolation: Too slow (cluster creation is expensive)
- Single cluster for all integration tests: Risk of cross-test pollution
- Per-suite isolation (using Rust test attributes): More complex setup

### 3. Mixed fixture strategy (YAML + programmatic)
**Rationale:** YAML files are easy to maintain for static resources, while programmatic creation enables dynamic test scenarios. This provides flexibility for different test needs.

**Alternatives considered:**
- All YAML: Less flexible for dynamic scenarios
- All programmatic: Less readable, harder to maintain

### 4. Companion _test.rs modules for unit tests
**Rationale:** Keeping unit tests alongside source files improves discoverability and encourages testing during development. These tests should use selective mocking for speed.

**Alternatives considered:**
- Separate unit test directory: Better separation but harder to discover
- All tests in tests/ directory: Clearer but mixes concerns

### 5. Helper functions in tests/common/
**Rationale:** Centralizing test utilities reduces duplication and provides consistent patterns across integration and E2E tests. The common module can be shared across test files.

**Alternatives considered:**
- Utility crate: Overkill for test-only code
- Test macros: Good for repeated patterns but less flexible

## Risks / Trade-offs

**Risk: Slow integration tests** → Mitigation: Run unit tests in parallel with integration tests, provide clear test category targets

**Risk: Fixture management complexity** → Mitigation: Use clear directory structure, document fixture patterns, provide examples

**Risk: testcontainers compatibility issues** → Mitigation: Pin versions, test in CI early, provide fallback options

**Trade-off: Test execution time vs isolation** → Per-test-file isolation balances speed and isolation better than per-test isolation

**Trade-off: Fixture flexibility vs maintenance** → Mixed approach (YAML + programmatic) provides good balance
