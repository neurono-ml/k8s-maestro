## Why

Current test infrastructure is insufficient - no integration tests, no fixture organization, and no automated cluster lifecycle management. Tests must exist before implementation changes (TDD requirement), making this a prerequisite for all future feature work.

## What Changes

- Set up testcontainers-rs with Kind for integration test isolation
- Create tests/common/ infrastructure with Kind cluster lifecycle management
- Organize test fixtures by resource type (workflows, secrets, configmaps, pvcs, failure scenarios)
- Implement test utilities for resource creation/cleanup and selective mocking
- Establish clear test organization patterns (unit, integration, E2E)

## Capabilities

### New Capabilities

- `kind-cluster-management`: Kind cluster provisioning, lifecycle management, and cleanup with per-suite isolation
- `test-fixtures-management`: Organized fixture structure supporting YAML files and programmatic creation by resource type
- `test-isolation-strategies`: Three-tier test organization - unit tests (no cluster), integration tests (per-file cluster), E2E tests (full scenarios)
- `test-utilities`: Helper functions for K8s resource creation, cleanup, and selective mocking

### Modified Capabilities

(None - this is foundational infrastructure, no existing specs changing)

## Impact

- **Code**: Adds tests/common/ directory structure, fixture files, and test utilities
- **Dependencies**: Adds testcontainers-rs with Kind provider, integration with existing k8s-maestro-k8s crate
- **Testing**: All future development must include tests before/during implementation
- **CI/CD**: Requires Kind setup for integration test execution
