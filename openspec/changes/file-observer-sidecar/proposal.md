## Why

Workflow steps in Kubernetes often produce output files that need to be collected, monitored, and made accessible to other steps or external systems. Currently, there's no standardized way to observe file changes in real-time, cache outputs for later retrieval, or expose them via HTTP endpoints. This creates friction when building data pipelines, ML workflows, or any system that needs to react to file outputs from jobs.

## What Changes

- Add `FileObserverSidecar` - a sidecar system that monitors filesystem changes in watched directories
- Implement three observation modes:
  - **Channel mode**: Real-time file events via tokio broadcast channels
  - **Cache mode**: Tiered caching with memory (L1) and PVC-backed disk (L2) storage
  - **HTTP service mode**: REST API for file retrieval and metadata access
- Create `TieredCache` with automatic spill from memory to disk using configurable eviction policies
- Add `FileFilter` for pattern-based include/exclude of files
- Build dedicated sidecar container image with inotify-based file watching
- Provide workflow integration API for programmatic file access

## Capabilities

### New Capabilities

- `file-observer`: Core file watching and event generation system with inotify integration, FileEvent types (Created/Modified/Deleted), and FileMetadata tracking
- `tiered-cache`: Two-tier caching system with LRU/FIFO eviction, memory tier (configurable size/files/TTL), and PVC-backed disk tier with spill policy
- `file-observer-http`: HTTP service exposing REST endpoints for file content retrieval, listing, metadata access, and HEAD requests for existence checks
- `file-observer-builder`: Fluent builder API for configuring FileObserverSidecar with watch paths, modes, cache config, and filters

### Modified Capabilities

None - this is a new feature area with no existing capability modifications.

## Impact

**New Modules**:
- `src/steps/observers/mod.rs` - module exports
- `src/steps/observers/file_observer.rs` - FileObserverSidecar, ObserverModes, FileObserverBuilder
- `src/steps/observers/observer_config.rs` - TieredCacheConfig, MemoryCacheConfig, DiskCacheConfig, FileFilter
- `src/steps/observers/file_event.rs` - FileEvent enum, FileMetadata, FileContent
- `src/steps/observers/cache.rs` - TieredCache, MemoryTier, DiskTier, CacheStats
- `src/steps/observers/http_service.rs` - FileHttpService with axum-based HTTP routes
- `src/steps/observers/integration.rs` - Workflow integration methods

**New Container**:
- `sidecars/file-observer/` - Dockerfile and Rust-based observer process

**Dependencies**:
- `notify` crate for inotify filesystem watching
- `axum` for HTTP service
- `lru` crate for memory tier cache
- Existing `k8s-openapi` and `kube` for PVC management
