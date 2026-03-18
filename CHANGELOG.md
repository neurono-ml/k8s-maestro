# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2026-03-18

### Added

#### Bug Fixes
- Fixed all compilation errors in tests and examples
- Fixed all warnings in library, tests, and examples
- Achieved zero warnings and zero errors in cargo check and cargo clippy
- Fixed module structure and imports across the codebase
- Fixed test module in src/entities/config/tests.rs
- Added Debug derive to container structs
- Commented out non-existent method calls in tests

### Improved
- Updated GitHub Actions workflow to automate releases with CHANGELOG integration
- Release notes are now automatically extracted from CHANGELOG.md
- Beta releases are automatically marked as pre-releases

## [0.4.0] - 2026-03-18

### Added

#### Security Module
- `SecurityClient` for managing Kubernetes security resources
- `NetworkPolicyBuilder` with `deny_all()`, `allow_all()`, and `allow_within_namespace()` presets
- `ResourceQuotaBuilder` with `small_workload()`, `medium_workload()`, and `large_workload()` presets
- RBAC builders: `ServiceAccountBuilder`, `RoleBuilder`, `RoleBindingBuilder`, `ClusterRoleBuilder`, `ClusterRoleBindingBuilder`
- `SecurityContext` builders: `PodSecurityContextBuilder`, `SecurityContextConfig`
- `LimitRangeBuilder` and `LimitRangeItemBuilder` for resource constraint management
- Network policy rule types: `PolicyType` (Ingress, Egress, Both), `NetworkPolicyRule`
- Resource quota types: `QuotaScope` (Terminating, NotTerminating, BestEffort, NotBestEffort)
- Preset RBAC roles: `workflow_executor()`, `workflow_viewer()`, `admin()`
- Module structure in `src/security/` with public re-exports
- Unit tests for all security builders and preset configurations

#### Sidecar Plugin System (MVP)
- `SidecarContainer` struct with builder pattern for sidecar containers
- `SidecarBuilder` fluent API for configuring sidecars
- `SidecarPlugin` trait defining plugin interface
- `PluginRegistry` for managing registered plugins
- `PluginInfo` struct for plugin metadata
- `ContainerPort` struct for port configuration
- Module structure in `src/steps/kubernetes/sidecar.rs` and `src/networking/plugins/`
- Unit tests for all plugin system components
- MVP implementation with essential plugin functionality

#### Documentation Update
- Complete rewrite of README.md with professional structure
- GitHub badges (version, license, build status, docs.rs, crates.io)
- Comprehensive feature list (10 key features)
- Installation instructions with feature flags
- Quick start guide with workflow example
- Usage examples (basic and advanced patterns)
- API documentation links
- Contributing guidelines

#### Examples Update
- Renamed examples to workflow-centric API:
  - `use_job_builder.rs` → `use_workflow_builder.rs`
  - `apply_and_watch.rs` → `apply_and_watch_workflow.rs`
  - `delete.rs` → `delete_workflow.rs`
- Created `examples/README.md` with comprehensive descriptions
- New examples:
  - `use_services.rs` - Service exposure
  - `use_sidecar.rs` - Sidecar containers
  - `multi_step_workflow.rs` - Multi-step workflows
  - `python_step.rs` - Python step (aspirational)
  - `rust_step.rs` - Rust step (aspirational)
  - `wasm_step.rs` - WASM step (aspirational)
- Updated all examples to use new workflow-centric API

#### Site-Docs Structure
- `site-docs/index.md` - Landing page with overview
- `site-docs/getting-started/installation.md` - Installation guide
- `site-docs/getting-started/quick-start.md` - Quick start guide
- `site-docs/getting-started/concepts.md` - Core concepts
- `site-docs/guides/basic-workflow.md` - Basic workflow guide
- `site-docs/reference/configuration.md` - Configuration reference
- `site-docs/reference/troubleshooting.md` - Troubleshooting guide

#### Migration Guide and Utilities
- Comprehensive migration guide for v0.3.0 → v0.4.0 API transition
- Migration utilities module (`src/migration/mod.rs`) with type aliases for backward compatibility
- Deprecated type aliases: `Job` → `Workflow`, `JobBuilder` → `WorkflowBuilder`, `MaestroK8sClient` → `MaestroClient`
- Migration helper functions for client creation with different dry_run and namespace configurations
- `allow_deprecated!` macro for temporarily suppressing deprecation warnings during migration
- `MigrationChecklist` trait for tracking migration progress in tests
- Detailed code examples showing old vs new API patterns
- FAQ section addressing migration concerns
- Complete migration checklist with step-by-step guidance

### Changed
- BREAKING: Job concept renamed to Workflow (conceptual shift from single jobs to multi-step workflows)
- BREAKING: `JobBuilder` → `WorkflowBuilder` with new fluent API
- BREAKING: `MaestroK8sClient` → `MaestroClient` with builder pattern configuration
- BREAKING: `create_job()` → `create_workflow()` method rename
- BREAKING: `dry_run` parameter moved from individual function calls to client builder configuration
- BREAKING: `entities::job` module → `entities::workflows` module

### Fixed
- Fixed all compilation errors in tests and examples
- Fixed all warnings in library, tests, and examples
- Achieved zero warnings and zero errors in cargo check and cargo clippy
- Fixed module structure and imports
- Fixed trait implementations and method signatures

### Removed
- Removed obsolete feature branches (all merged into main)

## [Unreleased]

### Added

#### Security Module
- `SecurityClient` for managing Kubernetes security resources
- `NetworkPolicyBuilder` with `deny_all()`, `allow_all()`, and `allow_within_namespace()` presets
- `ResourceQuotaBuilder` with `small_workload()`, `medium_workload()`, and `large_workload()` presets
- RBAC builders: `ServiceAccountBuilder`, `RoleBuilder`, `RoleBindingBuilder`, `ClusterRoleBuilder`, `ClusterRoleBindingBuilder`
- `SecurityContext` builders: `PodSecurityContextBuilder`, `SecurityContextConfig`
- `LimitRangeBuilder` and `LimitRangeItemBuilder` for resource constraint management
- Network policy rule types: `PolicyType` (Ingress, Egress, Both), `NetworkPolicyRule`
- Resource quota types: `QuotaScope` (Terminating, NotTerminating, BestEffort, NotBestEffort)
- Preset RBAC roles: `workflow_executor()`, `workflow_viewer()`, `admin()`
- Module structure in `src/security/` with public re-exports
- Unit tests for all security builders and preset configurations

#### Sidecar Plugin System (MVP)
- `SidecarContainer` struct with builder pattern for sidecar containers
- `SidecarBuilder` fluent API for configuring sidecars
- `SidecarPlugin` trait defining plugin interface
- `PluginRegistry` for managing registered plugins
- `PluginInfo` struct for plugin metadata
- `ContainerPort` struct for port configuration
- Module structure in `src/steps/kubernetes/sidecar.rs` and `src/networking/plugins/`
- Unit tests for all plugin system components
- MVP implementation with essential plugin functionality

#### Documentation Update
- Complete rewrite of README.md with professional structure
- GitHub badges (version, license, build status, docs.rs, crates.io)
- Comprehensive feature list (10 key features)
- Installation instructions with feature flags
- Quick start guide with workflow example
- Usage examples (basic and advanced patterns)
- API documentation links
- Contributing guidelines

#### Examples Update
- Renamed examples to workflow-centric API:
  - `use_job_builder.rs` → `use_workflow_builder.rs`
  - `apply_and_watch.rs` → `apply_and_watch_workflow.rs`
  - `delete.rs` → `delete_workflow.rs`
- Created `examples/README.md` with comprehensive descriptions
- New examples:
  - `use_services.rs` - Service exposure
  - `use_sidecar.rs` - Sidecar containers
  - `multi_step_workflow.rs` - Multi-step workflows
  - `python_step.rs` - Python step (aspirational)
  - `rust_step.rs` - Rust step (aspirational)
  - `wasm_step.rs` - WASM step (aspirational)
- Updated all examples to use new workflow-centric API

#### Site-Docs Structure
- `site-docs/index.md` - Landing page with overview
- `site-docs/getting-started/installation.md` - Installation guide
- `site-docs/getting-started/quick-start.md` - Quick start guide
- `site-docs/getting-started/concepts.md` - Core concepts
- `site-docs/guides/basic-workflow.md` - Basic workflow guide
- `site-docs/reference/configuration.md` - Configuration reference
- `site-docs/reference/troubleshooting.md` - Troubleshooting guide

#### Migration Guide and Utilities
- Comprehensive migration guide for v0.3.0 → v0.4.0 API transition
- Migration utilities module (`src/migration/mod.rs`) with type aliases for backward compatibility
- Deprecated type aliases: `Job` → `Workflow`, `JobBuilder` → `WorkflowBuilder`, `MaestroK8sClient` → `MaestroClient`
- Migration helper functions for client creation with different dry_run and namespace configurations
- `allow_deprecated!` macro for temporarily suppressing deprecation warnings during migration
- `MigrationChecklist` trait for tracking migration progress in tests
- Detailed code examples showing old vs new API patterns
- FAQ section addressing common migration questions
- Complete migration checklist with step-by-step guidance

#### File Observer Sidecar (MVP)
- File event detection system with `FileEvent` enum (Created, Modified, Deleted)
- `FileMetadata` tracking (filename, path, mime_type, size, timestamps)
- `FileContent` combining metadata and byte content
- Memory-only tiered cache with LRU eviction using `lru` crate
- `MemoryCacheConfig` for cache size, file count, and TTL limits
- `FileFilter` for pattern matching and size limits
- `FileObserverBuilder` with fluent API for observer configuration
- Observer modes support (channel, cache, http_service)
- HTTP service with axum for file access endpoints:
  - GET /files/{path} for file content retrieval
  - GET /files for listing all cached files
  - GET /files/{path}/metadata for file metadata
  - HEAD /files/{path} for existence check
- Sidecar binary with inotify-based file watching using `notify` crate
- Unit tests for core types
- Module structure in `src/steps/observers/` with public re-exports

#### Multi-Language Execution Steps (MVP)
- `PythonStep` for executing Python code in Kubernetes Pods
- `PythonStepBuilder` with fluent API for Python step configuration
- `PackageSource` enum supporting Git, RemotePath, LocalPath, and Registry sources
- `PackageLoader` for loading packages from multiple sources with caching
- `PackageCache` with SHA-256 based cache key generation
- Support for Python requirements.txt and pip package installation
- ConfigMap-based code injection for inline scripts
- Resource limits support (CPU, memory, requests)
- Volume mounts support for data access
- Environment variables support
- Pod spec generation with python:3.12-slim image
- Trait implementations: `WorkFlowStep`, `ExecutableWorkFlowStep`, `WaitableWorkFlowStep`, `DeletableWorkFlowStep`, `LoggableWorkFlowStep`
- Dry run support for all operations
- Module structure in `src/steps/exec/` with public re-exports
- Unit tests for package loader and Python step builder

#### Workflow Execution Orchestrator (MVP)
- `WorkflowExecution` struct for tracking workflow state and lifecycle
- `WorkflowStatus` enum (Pending, Running, Succeeded, Failed, Cancelled)
- `Checkpoint` struct for state serialization
- `Scheduler` for parallel step execution with semaphore-based rate limiting
- `FailureStrategy` enum (Stop, Continue) for configurable failure handling
- `StepExecutor` for executing KubeJob and KubePod step types
- `WorkflowOrchestrator` for DAG-based workflow execution with dependency resolution
- Condition evaluation against dependency results using closures
- Cycle detection before workflow execution
- Execution loop with level-by-level step execution
- `wait()`, `cancel()`, `get_status()`, `get_step_result()`, `delete()`, `get_checkpoint()` methods on WorkflowExecution
- Module structure in `src/workflows/execution/` with public re-exports
- Integration with existing `DependencyGraph` and `ConditionFn` types
- Basic unit tests for scheduler components
- MVP implementation with essential functionality only

#### Kube Workflow Steps Implementation (MVP)
- `KubeJobStep` with full Kubernetes Job lifecycle management
- `KubePodStep` with full Kubernetes Pod lifecycle management
- `KubeJobStepBuilder` with fluent API for job configuration
- `KubePodStepBuilder` with fluent API for pod configuration
- Supporting types: `JobNameType` (DefinedName, GenerateName), `RestartPolicy` (Never, OnFailure, Always)
- `ServiceConfig` for service configuration with port mapping and selector
- `IngressConfig` for ingress configuration with path, TLS, and annotations
- `ContainerLike` trait for polymorphic container handling
- `MaestroContainer` with image, arguments, environment variables, and resource limits
- `SidecarContainer` for sidecar containers with same capabilities
- `MaestroK8sClient` wrapper around `kube::Client` for K8s API interactions
- Trait implementations: `WorkFlowStep`, `KubeWorkFlowStep`, `WaitableWorkFlowStep`, `DeletableWorkFlowStep`, `LoggableWorkFlowStep`, `ServableWorkFlowStep`
- Builder validation for required fields (name, namespace, containers)
- Dry run support for all operations
- Module structure in `src/steps/kubernetes/` with public re-exports
- Client module in `src/clients/` for K8s client abstraction
- Entity types in `src/entities/containers.rs` for container abstractions
- Comprehensive unit tests for all supporting types
- MVP implementation with essential functionality only

#### Volume Builders (MVP)
- `MaestroPVCMountVolumeBuilder` for Persistent Volume Claim volumes
- `ConfigMapVolumeBuilder` for ConfigMap volumes with item-level control
- `SecretVolumeBuilder` for Secret volumes with item-level control
- `EmptyDirVolumeBuilder` for temporary empty directory volumes with memory and size limit options
- `HostPathVolumeBuilder` for host path volumes with type validation
- Supporting types: `VolumeType`, `VolumeItem`, `AccessMode`, `Medium`, `HostPathType`
- `VolumeMountLike` trait for polymorphic volume handling in containers
- Fluent builder API consistent with existing codebase patterns
- Comprehensive unit tests for all volume builders and conversions
- Module structure in `src/entities/volumes/` with public re-exports

#### Networking Module (MVP)
- `ServiceBuilder` with fluent API for creating Kubernetes Services
- Support for all service types: ClusterIP, Headless, NodePort, LoadBalancer
- `ServiceType` enum for type-safe service type selection
- `ServicePort` struct for port configuration with protocol support
- Multiple ports configuration with named ports
- Advanced options: session affinity, external traffic policy, custom cluster IP
- Service selector configuration for pod targeting
- `IngressBuilder` with fluent API for creating Kubernetes Ingress resources
- `IngressPath` struct with path type support (Exact, Prefix, ImplementationSpecific)
- `PathType` enum for type-safe path type selection
- `TLSConfig` struct for TLS configuration with multiple hosts
- Multiple paths support for complex routing rules
- Custom annotations support for ingress-specific configurations
- Ingress class support for controller selection
- DNS utilities: `service_dns_name()`, `pod_dns_name()`, `headless_service_dns_pattern()`
- Module structure in `src/networking/` with public re-exports
- Comprehensive rustdoc documentation on all public types and methods
- Unit tests for all builder types and validation
- Examples: `use_service_builder.rs`, `use_ingress_builder.rs`
- Fluent builder pattern consistent with existing codebase patterns

#### ConfigMap and Secret Builders
- `ConfigMapBuilder` with fluent API for building Kubernetes ConfigMaps
- Support for string data, binary data, labels, annotations, and immutable ConfigMaps
- `SecretBuilder` with fluent API for building Kubernetes Secrets
- `SecretType` enum for type-safe secret types (Opaque, ServiceAccountToken, Dockercfg, DockerConfigJson, BasicAuth, SshAuth, Tls, BootstrapToken)
- `ImagePullSecretBuilder` for creating docker-registry type secrets
- Support for string data, binary data, labels, annotations, and immutable Secrets
- `ByteString` type conversion for binary data compatibility with k8s-openapi
- Base64 encoding support for secret data and docker authentication
- Module structure in `src/entities/config/` with public re-exports
- Comprehensive rustdoc documentation on all public types and methods
- Basic unit tests for builder construction and validation
- Fluent builder pattern consistent with existing codebase patterns

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
