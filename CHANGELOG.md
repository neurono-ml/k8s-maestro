# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Container Infrastructure (MVP)
- `ComputeResource` type for container resource specifications
- Support for cpu, memory, and ephemeral_storage resources
- `AsRef<str>` trait implementation for string conversion
- Foundation for container builder pattern (full implementation in future PR)
- Export from entities module
- Simplified MVP implementation

#### Checkpointing System
- SQLite-based checkpoint storage with StatefulSet persistence
- `CheckpointStorage` plugin trait for extensible storage backends
- `Checkpoint` and `StepCheckpoint` data structures with serde support
- `CheckpointConfig` with multiple frequency options (OnStepCompletion, OnSuccess, Periodic)
- `CheckpointStore` with automatic retry logic and exponential backoff
- StatefulSet lifecycle management (create, update, delete)
- Cleanup manager with retention policies (age-based and count-based)
- Configuration support for multiple storage backends (SQLite, Etcd, Redis, Postgres)
- 143 comprehensive unit tests for all checkpointing operations
- Complete documentation with usage examples

#### Dependency Chain System
- `DependencyChain` builder for constructing dependency graphs with fluent API
- `DependencyGraph` for managing workflow step dependencies
- `ConditionBuilder` with predefined conditions: all_success, any_success, all_failure, any_failure
- Conditional execution based on step results using closures
- `depends_on_any` for any-of dependency semantics
- Topological sort using Kahn's algorithm
- Cycle detection during DAG construction
- Parallel execution level computation
- Diamond pattern, parallel starts, and disconnected graphs support
- Complete unit tests for all dependency scenarios

#### MaestroClient with Builder Pattern
- `MaestroClient` with builder pattern for configuring Kubernetes workflow client
- `MaestroClientBuilder` with fluent API for client construction
- Centralized configuration including `dry_run`, namespace, timeouts, logging, and resource limits
- `CreatedWorkflow` enum wrapping workflows in dry run or runtime mode
- Workflow management API: `create_workflow()`, `get_workflow()`, `list_workflows()`
- `WorkflowLike` trait for common workflow interface
- Complete documentation with examples for all public APIs
- Unit tests covering builder pattern, client configuration, and workflow operations

#### Test Infrastructure
- Comprehensive test infrastructure with three-tier organization (unit, integration, E2E)
- Kind cluster lifecycle management using testcontainers
- YAML fixture loading and parsing utilities
- Resource creation helpers for ConfigMaps, Secrets, PVCs, and Namespaces
- Resource cleanup helpers with timeout and retry logic
- Resource validation helpers with configurable timeouts
- Selective mocking utilities for unit tests without cluster dependencies
- Sample integration and E2E test files demonstrating test patterns

#### Project Structure
- Module organization with `client`, `workflows`, `steps`, `entities`, `networking`, `security`, and `images` modules
- Basic crate documentation describing k8s-maestro as a Kubernetes job orchestrator

### Documentation
- Test infrastructure documentation in `tests/common/README.md`
- Fixture creation patterns in `tests/common/fixtures/README.md`
- Testing guide in `site-docs/testing.md`
- Updated `AGENTS.md` with test infrastructure usage instructions

### Dependencies
- Added testcontainers for Docker-based test isolation
- Added mockall for mocking in unit tests
- Added base64 for kubeconfig encoding

## [0.3.0] - Previous Release

### Added
- Initial workflow and step management
- Workflow builder pattern
- Step result and status types
- Dependency management with DAG support
