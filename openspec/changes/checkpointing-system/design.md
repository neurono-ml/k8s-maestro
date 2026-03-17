## Context

Kubernetes workflows in k8s-maestro currently lack state persistence capabilities. When a workflow fails or is interrupted, users must restart from the beginning, wasting compute resources and time. The checkpointing system needs to provide persistent, editable storage without requiring external services (Etcd, Redis, etc.). The system must support multiple storage backends via a plugin architecture, with SQLite as the default implementation running in a StatefulSet with PVC for persistence.

## Goals / Non-Goals

**Goals:**
- Enable workflow resumption from any checkpoint point
- Provide plugin-based storage architecture for extensibility
- Implement persistent, transactional checkpoint storage (ACID compliance)
- Support optimistic locking to prevent concurrent modification conflicts
- Offer flexible checkpoint frequency options (OnStepCompletion, OnSuccess, Periodic)
- Provide REST API for checkpoint CRUD operations
- Implement automatic backend selection and retry logic
- Support retention policies for checkpoint lifecycle management

**Non-Goals:**
- Distributed checkpoint storage (single-pod SQLite is sufficient)
- Checkpoint compression or encryption (deferred to plugins)
- Automatic checkpoint recovery (requires manual resumption)
- Checkpoint versioning beyond optimistic locking
- Multi-cluster checkpoint sharing

## Decisions

### Plugin Architecture for Storage

**Decision**: Use trait-based plugin architecture (`CheckpointStorage`) rather than configuration-driven abstraction.

**Rationale**: Traits provide compile-time type safety and zero-cost abstraction in Rust. Plugins can be dynamically registered and swapped without runtime overhead. This matches Rust's idiomatic design patterns and allows for easy testing with mock implementations.

**Alternatives Considered**:
- Configuration-driven (enum-based): Less flexible, requires modifying core code for new backends
- Dynamic library loading: Adds complexity, requires unsafe code, not necessary at this stage

### SQLite with StatefulSet

**Decision**: Use SQLite in a StatefulSet with PVC for default storage.

**Rationale**: SQLite provides ACID compliance, zero external dependencies, and file-based storage that's easily inspectable/editable. StatefulSet ensures stable network identity and ordered pod startup. PVC ensures data persists across pod rescheduling. This meets the requirement for ephemeral yet persistent, editable storage without external services.

**Alternatives Considered**:
- Etcd: Requires external cluster, overkill for single-namespace use
- Redis: External dependency, additional operational overhead
- In-memory: Not persistent, fails across pod restarts

### REST API for SQLite Communication

**Decision**: Use HTTP REST API to communicate with SQLite StatefulSet pod.

**Rationale**: REST APIs are simple, language-agnostic, and easy to debug. HTTP is widely supported and works well within Kubernetes clusters. This allows the storage backend to be swapped without changing client code.

**Alternatives Considered**:
- gRPC: Better performance but adds protobuf complexity
- Direct SQLite file access: Requires shared volumes, complex coordination

### Optimistic Locking with Version Field

**Decision**: Use optimistic locking with version field in Checkpoint struct.

**Rationale**: Optimistic locking is suitable for low-contention checkpoint operations. Version field provides simple conflict detection without distributed locking overhead. Clients can detect conflicts and retry with appropriate backoff.

**Alternatives Considered**:
- Pessimistic locking: Requires distributed lock manager, adds complexity
- Last-write-wins: Loses data, not acceptable for stateful workflows

### Checkpoint Frequency Enum

**Decision**: Use enum with variants `OnStepCompletion`, `OnSuccess`, `Periodic(Duration)`.

**Rationale**: Provides clear, type-safe options for common checkpoint scenarios. Duration-based periodic checkpointing is flexible for custom intervals.

**Alternatives Considered**:
- Config string: No compile-time safety, error-prone
- Bitmask: Less readable, harder to extend

### CheckpointStore with Retry Logic

**Decision**: Implement CheckpointStore with exponential backoff retry for network failures.

**Rationale**: Network communication with StatefulSet pod can fail temporarily. Exponential backoff reduces retry load while ensuring eventual consistency. This isolates retry logic from storage implementation.

**Alternatives Considered**:
- Client-side retry: Duplicates logic across storage implementations
- Circuit breaker: Useful but may block checkpoint saves prematurely

## Risks / Trade-offs

**Risk**: SQLite StatefulSet pod failure could block checkpoint operations
→ Mitigation: Implement retry logic with exponential backoff in CheckpointStore. Consider adding health checks and automatic pod restart policies.

**Risk**: Concurrent checkpoint updates could lead to version conflicts
→ Mitigation: Implement optimistic locking with version field. Return clear error messages on conflict and provide retry guidance.

**Risk**: Checkpoint storage may grow unbounded without retention policies
→ Mitigation: Implement retention policies with configurable time limits and max checkpoint counts per workflow. Background cleanup job will enforce policies.

**Trade-off**: Single-pod SQLite limits horizontal scaling
→ Acceptance: Checkpointing is not high-throughput (typically one save per step). Single pod is sufficient for most use cases. Plugin architecture allows horizontal scaling backends for high-demand scenarios.

**Trade-off**: REST API adds latency compared to direct SQLite access
→ Acceptance: Latency is acceptable for checkpoint operations (not in hot path). REST API provides simpler architecture and debugging capabilities.

**Risk**: PVC storage limits may be exceeded
→ Mitigation: Monitor storage usage, implement alerting, provide documentation on PVC sizing. Consider checkpoint compression in future iterations.

## Migration Plan

**Deployment Steps**:
1. Deploy StatefulSet and PVC for checkpoint storage (kubectl apply)
2. Update k8s-maestro deployment with new checkpointing module
3. Configure checkpoint behavior in workflow definitions (optional defaults)
4. Enable checkpointing in production workflow (gradual rollout)

**Rollback Strategy**:
1. Disable checkpointing in workflow configuration (fallback to no checkpointing)
2. Delete StatefulSet and PVC if needed (kubectl delete)
3. Revert k8s-maestro deployment to previous version
4. No data loss: existing checkpoints are preserved until manual cleanup

**Data Migration**: Not applicable (new feature, no existing checkpoint data to migrate)

## Open Questions

- Should checkpoint compression be implemented in SQLite plugin or deferred to future plugins? (Deferred: add compression as plugin option later)
- What is the default retention policy? (Decision: 7 days, 10 checkpoints per workflow)
- Should checkpointing be enabled by default for all workflows? (Decision: Disabled by default, opt-in via config)
- How should checkpoint size limits be enforced? (Decision: Document best practices, enforce via PVC quotas)
