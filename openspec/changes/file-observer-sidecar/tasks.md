## 1. Project Setup

- [ ] 1.1 Add dependencies to Cargo.toml (notify, axum, tower-http, lru, mime_guess, mime)
- [ ] 1.2 Create src/steps/observers/mod.rs with module exports

## 2. Core Types

- [ ] 2.1 Create src/steps/observers/file_event.rs with FileEvent enum (Created, Modified, Deleted variants)
- [ ] 2.2 Add FileMetadata struct with filename, path, mime_type, size, created_at, modified_at fields
- [ ] 2.3 Add FileContent struct combining metadata and Vec<u8> content

## 3. Configuration Types

- [ ] 3.1 Create src/steps/observers/observer_config.rs with MemoryCacheConfig (max_size_mb, max_files, ttl)
- [ ] 3.2 Add DiskCacheConfig (pvc_name, pvc_size, pvc_storage_class, max_size_mb, max_files, ttl)
- [ ] 3.3 Add SpillPolicy enum (LRU, FIFO)
- [ ] 3.4 Add TieredCacheConfig combining memory and disk configs with spill policy
- [ ] 3.5 Add FileFilter struct (include_patterns, exclude_patterns, max_file_size_mb)

## 4. File Observer Builder

- [ ] 4.1 Create src/steps/observers/file_observer.rs with ObserverModes struct (channel, cache, http_service)
- [ ] 4.2 Add FileObserverSidecar configuration struct (watch_path, modes, cache, filters)
- [ ] 4.3 Implement FileObserverBuilder with new() constructor
- [ ] 4.4 Add watch_path(path) fluent method
- [ ] 4.5 Add with_channel_mode(enabled) fluent method
- [ ] 4.6 Add with_cache_mode(enabled, config) fluent method
- [ ] 4.7 Add with_http_service(enabled, port) fluent method
- [ ] 4.8 Add with_filters(filters) fluent method
- [ ] 4.9 Implement build() with validation (required watch_path, at least one mode)
- [ ] 4.10 Add default configurations for cache and filter

## 5. Tiered Cache

- [ ] 5.1 Create src/steps/observers/cache.rs with CacheStats struct
- [ ] 5.2 Implement MemoryTier with LRU eviction using lru crate
- [ ] 5.3 Implement DiskTier with PVC-backed storage
- [ ] 5.4 Implement TieredCache combining memory and disk tiers
- [ ] 5.5 Add get(path) -> Option<FileContent> checking memory then disk
- [ ] 5.6 Add put(path, content) -> Result<()> with spill to disk on memory full
- [ ] 5.7 Add delete(path) -> Result<()> removing from both tiers
- [ ] 5.8 Add list() -> Vec<FileMetadata> from both tiers
- [ ] 5.9 Add evict_expired() -> Result<usize> for TTL cleanup
- [ ] 5.10 Add stats() -> CacheStats for metrics

## 6. HTTP Service

- [ ] 6.1 Create src/steps/observers/http_service.rs with FileHttpService struct
- [ ] 6.2 Implement GET /files/{path} endpoint for file content retrieval
- [ ] 6.3 Implement GET /files endpoint for listing all cached files
- [ ] 6.4 Implement GET /files/{path}/metadata endpoint for file metadata
- [ ] 6.5 Implement HEAD /files/{path} endpoint for existence check
- [ ] 6.6 Add start(port, cache: Arc<TieredCache>) -> Result<()> method
- [ ] 6.7 Add graceful shutdown handling

## 7. Workflow Integration

- [ ] 7.1 Create src/steps/observers/integration.rs
- [ ] 7.2 Implement get_file_content(path: &str) -> Result<FileContent>
- [ ] 7.3 Implement list_observed_files() -> Result<Vec<FileMetadata>>
- [ ] 7.4 Implement subscribe_to_events() -> Receiver<FileEvent>

## 8. Sidecar Image

- [ ] 8.1 Create sidecars/file-observer/ directory structure
- [ ] 8.2 Create Cargo.toml for sidecar binary with notify, tokio dependencies
- [ ] 8.3 Implement main.rs with inotify-based file watching using notify crate
- [ ] 8.4 Integrate with TieredCache for file storage
- [ ] 8.5 Integrate with HTTP service for endpoint exposure
- [ ] 8.6 Integrate with broadcast channel for event distribution
- [ ] 8.7 Create Dockerfile with minimal Rust runtime image
- [ ] 8.8 Add build script for container image

## 9. Unit Tests

- [ ] 9.1 Add tests for FileEvent creation variants
- [ ] 9.2 Add tests for FileObserverBuilder validation
- [ ] 9.3 Add tests for MemoryTier put/get/eviction
- [ ] 9.4 Add tests for DiskTier put/get/eviction
- [ ] 9.5 Add tests for TieredCache spill from memory to disk
- [ ] 9.6 Add tests for FileFilter include/exclude patterns
- [ ] 9.7 Add tests for cache TTL expiration
- [ ] 9.8 Add tests for LRU vs FIFO eviction policies

## 10. Integration Tests

- [ ] 10.1 Create integration test fixture with Kind cluster
- [ ] 10.2 Test deploying file observer sidecar with job
- [ ] 10.3 Test file creation events via channel
- [ ] 10.4 Test file modification events via channel
- [ ] 10.5 Test file deletion events via channel
- [ ] 10.6 Test HTTP endpoint GET /files/{path}
- [ ] 10.7 Test HTTP endpoint GET /files listing
- [ ] 10.8 Test cache persistence across pod restarts with PVC
- [ ] 10.9 Test large file handling with size limits
- [ ] 10.10 Test file eviction under memory pressure

## 11. Documentation

- [ ] 11.1 Update CHANGELOG.md with new feature
- [ ] 11.2 Add examples in site-docs/ for file observer usage
- [ ] 11.3 Update README.md with file observer capabilities
