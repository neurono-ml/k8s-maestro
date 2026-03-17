## Context

The k8s-maestro library orchestrates Kubernetes jobs but lacks a mechanism for observing and collecting output files from workflow steps. Currently, users must implement custom solutions to:
- Watch directories for file changes
- Cache outputs for later retrieval
- Expose files via HTTP for external access
- Propagate file events to other workflow steps

This design introduces a sidecar-based file observer system that integrates with Kubernetes jobs to provide these capabilities.

## Goals / Non-Goals

**Goals:**
- Provide real-time file change events via channels
- Implement tiered caching with memory (fast) and disk (persistent) tiers
- Expose files via HTTP REST API
- Support PVC persistence across pod restarts
- Offer fluent builder API for configuration
- Filter files by pattern, size, and type

**Non-Goals:**
- Distributed file observation across multiple pods (single-pod scope only)
- File content transformation or processing
- Binary diff for modification detection (full file replacement only)
- Authentication/authorization for HTTP service (future enhancement)

## Decisions

### D1: Sidecar Architecture
**Decision**: Implement as a sidecar container pattern with shared volume mount.

**Rationale**:
- Sidecar runs alongside main workload container in same pod
- Shared emptyDir or PVC volume enables file access
- Independent lifecycle management
- Standard Kubernetes pattern for helper services

**Alternatives considered**:
- *Init container*: Runs before main container, cannot observe during execution
- *External service*: Requires network transfer, adds latency and complexity

### D2: Tiered Cache with Spill
**Decision**: Two-tier cache (memory L1, disk L2) with configurable spill policy.

**Rationale**:
- Memory tier provides fast access for hot files
- Disk tier provides persistence and larger capacity
- Spill from memory to disk when L1 is full
- PVC-backed disk survives pod restarts

**Alternatives considered**:
- *Memory-only*: Lost on restart, limited capacity
- *Disk-only*: Higher latency for all access
- *Three-tier (memory/disk/remote)*: Overkill for single-pod scope

### D3: inotify for File Watching
**Decision**: Use Linux inotify via `notify` crate for filesystem events.

**Rationale**:
- Kernel-level efficiency vs polling
- Immediate event delivery
- Well-supported in Rust ecosystem
- Standard on all Kubernetes nodes

**Alternatives considered**:
- *Polling*: Higher CPU, delayed detection
- *FUSE*: Requires privileged container, complexity

### D4: axum for HTTP Service
**Decision**: Use axum framework for HTTP endpoints.

**Rationale**:
- Consistent with Rust ecosystem best practices
- Async-native with tokio integration
- Type-safe routing and extractors
- Lower overhead than alternatives

**Alternatives considered**:
- *actix-web*: More complex, heavier
- *warp*: Less intuitive API, similar performance

### D5: Broadcast Channel for Events
**Decision**: Use `tokio::sync::broadcast` for file event distribution.

**Rationale**:
- Multiple subscribers can receive events
- Non-blocking for slow consumers
- Configurable capacity for backpressure

**Alternatives considered**:
- *mpsc*: Single consumer only
- *watch*: Only latest value, no history

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| High file churn overwhelms channel | Configurable buffer size, backpressure handling |
| Memory exhaustion from large files | Max file size filter, streaming for large files |
| PVC unavailability | Graceful degradation to memory-only mode |
| inotify watch limits | Document limits, fail fast with clear error |
| Event ordering issues | Include timestamps, document eventual consistency |

## Migration Plan

1. **Phase 1**: Core types and builder API (no K8s deployment)
2. **Phase 2**: Cache implementation with memory tier
3. **Phase 3**: HTTP service and channel integration
4. **Phase 4**: Sidecar image and K8s deployment
5. **Phase 5**: PVC integration and persistence

No rollback needed - additive feature with no existing behavior changes.
