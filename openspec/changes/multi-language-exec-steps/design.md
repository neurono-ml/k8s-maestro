## Context

k8s-maestro is a Rust-based Kubernetes workflow orchestrator that currently provides job orchestration through the `WorkFlowStep` trait hierarchy. The existing step traits (`ExecutableWorkFlowStep`, `WaitableWorkFlowStep`, `DeletableWorkFlowStep`, `LoggableWorkFlowStep`) provide a foundation for workflow steps that interact with Kubernetes resources.

This design extends the step system to support multi-language code execution directly within workflows. Each language step will create and manage Kubernetes Pods that execute the provided code in isolated containers with configurable resource limits, network policies, and volume mounts.

## Goals / Non-Goals

**Goals:**
- Support Python, Rust, Lua, and WASM code execution as workflow steps
- Enable package loading from Git repositories, remote URLs, local paths, and registries
- Provide sandboxed execution with configurable security boundaries
- Implement package caching to optimize repeated executions
- Follow existing trait patterns (`ExecutableWorkFlowStep`, `WaitableWorkFlowStep`, `DeletableWorkFlowStep`, `LoggableWorkFlowStep`)

**Non-Goals:**
- Custom container image building (uses pre-built language images)
- IDE integration or debugging support
- Hot-reloading of code during execution
- Distributed package caching across nodes

## Decisions

### D1: Package Source Abstraction

**Decision:** Use an enum `PackageSource` with variants for Git, RemotePath, LocalPath, and Registry.

**Rationale:** Provides a unified interface for different package sources while allowing source-specific handling. Git integration uses `git2` crate for cloning, remote paths use `reqwest` for HTTP downloads, local paths use filesystem operations, and registry support is extensible for future backends.

**Alternatives considered:**
- Trait objects: More flexible but adds complexity for serialization and error handling
- String-based URLs: Simpler but loses type safety and source-specific validation

### D2: Executor Pod Architecture

**Decision:** Each execution step creates a dedicated Pod with:
- InitContainer for package installation (if packages specified)
- Main container for code execution
- ConfigMap for code/scripts injection
- EmptyDir volume for package cache sharing between containers

**Rationale:** Separates package installation from execution, allowing the main container to remain minimal. ConfigMaps avoid building custom images for each code change.

**Alternatives considered:**
- Single container with entrypoint script: Simpler but makes package installation failures harder to diagnose
- Job resource instead of Pod: Adds unnecessary complexity for single-run code execution

### D3: Language-Specific Images

**Decision:** Use official/minimal images for each language:
- Python: `python:3.12-slim`
- Rust: `rust:1.75-slim`
- Lua: `abaez/lua:5.4` or custom minimal image
- WASM: `wasmedge/wasmedge:latest`

**Rationale:** Official images are well-maintained and security-patched. Slim variants reduce attack surface and image pull times.

**Alternatives considered:**
- Custom images with pre-installed common packages: Faster cold starts but requires maintenance
- Distroless images: Smaller but may lack tools needed for package installation

### D4: Package Caching Strategy

**Decision:** Implement `PackageCache` with:
- Local filesystem cache at `/tmp/k8s-maestro/cache/<source-hash>`
- SHA-256 hash of source URL for cache key
- Optional TTL for cache invalidation

**Rationale:** Simple implementation with predictable cache location. Hash-based keys prevent collisions and allow verification.

**Alternatives considered:**
- Redis/external cache: Adds deployment complexity
- Kubernetes Volume-based cache: Requires persistent volume provisioning

### D5: Security Sandbox Configuration

**Decision:** Apply the following security contexts by default:
- Run as non-root user
- Read-only root filesystem (with emptyDir for writable paths)
- No privilege escalation
- Configurable network policies (default: allow all for package installation)

**Rationale:** Defense in depth while maintaining usability for legitimate code execution.

**Alternatives considered:**
- gVisor/Kata Containers: Requires node-level configuration, not universally available
- seccomp profiles: More restrictive but may break legitimate package installations

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Large package downloads slow execution | Package caching; warn users about large dependencies |
| Malicious code in packages | Document trust requirements; support checksums |
| Resource exhaustion from parallel executions | Configurable resource limits; namespace quotas |
| Network access needed for packages | Configurable network policies; support air-gapped mode with local packages |
| WASM runtime compatibility | Document supported WASM features; version pinning |

## Migration Plan

1. **Phase 1:** Add `src/steps/exec/mod.rs` and `package_loader.rs` with unit tests
2. **Phase 2:** Implement `PythonStep` with builder pattern
3. **Phase 3:** Implement `RustStep`, `LuaStep`, `WasmStep`
4. **Phase 4:** Add integration tests with Kind cluster
5. **Phase 5:** Documentation and examples

**Rollback:** All changes are additive (new module). Rollback is simply removing the `exec` module and updating `mod.rs`.

## Open Questions

- Should we support custom container images per step? (Deferred: can be added via builder option)
- Should package cache be shared across namespaces? (Decision: No, per-namespace isolation)
- Timeout handling: Default timeout values? (Decision: 5 minutes default, configurable via builder)
