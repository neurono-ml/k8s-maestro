## 1. Project Setup

- [ ] 1.1 Add new dependencies to Cargo.toml (tokio, chrono, serde, serde_json, uuid, reqwest)
- [ ] 1.2 Create module structure at src/workflows/checkpointing/ with subdirectories
- [ ] 1.3 Create mod.rs files for checkpointing module hierarchy
- [ ] 1.4 Add checkpointing module to src/lib.rs or src/workflows/mod.rs

## 2. Core Data Structures

- [ ] 2.1 Create Checkpoint struct with workflow_id, checkpoint_time, steps, metadata, version
- [ ] 2.2 Create StepCheckpoint struct with status, last_execution, outputs, execution_count
- [ ] 2.3 Create CheckpointMetadata struct for listing operations
- [ ] 2.4 Implement serde serialization/deserialization for all structs
- [ ] 2.5 Add unit tests for data structures

## 3. Checkpointing Plugin System

- [ ] 3.1 Create CheckpointStorage trait in src/workflows/checkpointing/plugin/storage.rs
- [ ] 3.2 Define trait methods: connect, save_checkpoint, get_checkpoint, update_checkpoint, delete_checkpoint, list_checkpoints, cleanup
- [ ] 3.3 Create error types for storage operations (StorageError)
- [ ] 3.4 Create plugin/mod.rs to export CheckpointStorage trait
- [ ] 3.5 Add unit tests for trait behavior with mock implementation

## 4. SQLite Storage Implementation

- [ ] 4.1 Create SQLiteCheckpointStorage struct in src/workflows/checkpointing/plugin/sqlite.rs
- [ ] 4.2 Implement CheckpointStorage trait for SQLiteCheckpointStorage
- [ ] 4.3 Create HTTP client struct for communicating with SQLite pod
- [ ] 4.4 Implement connect() method to verify SQLite pod is accessible
- [ ] 4.5 Implement save_checkpoint() with HTTP POST
- [ ] 4.6 Implement get_checkpoint() with HTTP GET
- [ ] 4.7 Implement update_checkpoint() with HTTP PUT and version check
- [ ] 4.8 Implement delete_checkpoint() with HTTP DELETE
- [ ] 4.9 Implement list_checkpoints() with HTTP GET
- [ ] 4.10 Implement cleanup() method
- [ ] 4.11 Add unit tests for HTTP client operations

## 5. StatefulSet Management

- [ ] 5.1 create statefulset.rs module at src/workflows/checkpointing/statefulset.rs
- [ ] 5.2 Implement create_statefulset() function with Kubernetes client
- [ ] 5.3 Implement update_statefulset() function for rolling updates
- [ ] 5.4 Implement delete_statefulset() function with optional PVC deletion
- [ ] 5.5 Implement wait_for_statefulset_ready() with timeout
- [ ] 5.6 Configure StatefulSet with replicas=1, serviceName, podManagementPolicy
- [ ] 5.7 Create PVC template with configurable storage size
- [ ] 5.8 Configure SQLite container with image, port, resource limits
- [ ] 5.9 Add liveness and readiness probes for HTTP endpoint
- [ ] 5.10 Implement get_statefulset_status() function
- [ ] 5.11 Add integration tests for StatefulSet lifecycle with Kind cluster

## 6. SQLite HTTP API Server

- [ ] 6.1 Create HTTP server module for SQLite pod
- [ ] 6.2 Implement POST /checkpoints endpoint with validation
- [ ] 6.3 Implement GET /checkpoints/{workflow_id} endpoint
- [ ] 6.4 Implement PUT /checkpoints/{workflow_id} with optimistic locking
- [ ] 6.5 Implement DELETE /checkpoints/{workflow_id} endpoint
- [ ] 6.6 Implement GET /checkpoints endpoint for listing
- [ ] 6.7 Create SQLite database schema (checkpoints table)
- [ ] 6.8 Implement ACID-compliant transaction handling
- [ ] 6.9 Add error handling for HTTP responses (404, 409, 500)
- [ ] 6.10 Add unit tests for HTTP endpoints

## 7. Checkpoint Store Management

- [ ] 7.1 Create CheckpointStore struct in src/workflows/checkpointing/store.rs
- [ ] 7.2 Implement storage backend registration mechanism
- [ ] 7.3 Implement automatic backend selection based on config
- [ ] 7.4 Create RetryConfig struct (max_retries, initial_backoff, multiplier)
- [ ] 7.5 Implement exponential backoff retry logic for all operations
- [ ] 7.6 Implement save_checkpoint() with retry
- [ ] 7.7 Implement get_checkpoint() with retry
- [ ] 7.8 Implement update_checkpoint() with retry and version conflict handling
- [ ] 7.9 Implement delete_checkpoint() with retry
- [ ] 7.10 Implement list_checkpoints() with retry
- [ ] 7.11 Implement cleanup() method
- [ ] 7.12 Add unit tests for retry logic with mock failures

## 8. Checkpoint Configuration

- [ ] 8.1 Create CheckpointConfig struct in src/workflows/checkpointing/config.rs
- [ ] 8.2 Create CheckpointFrequency enum (OnStepCompletion, OnSuccess, Periodic)
- [ ] 8.3 Create CheckpointStorageConfig enum (Sqlite, Etcd, Redis, Postgres)
- [ ] 8.4 Add storage backend parameter structs for each type
- [ ] 8.5 Create RetentionPolicy struct with max_age and max_count
- [ ] 8.6 Implement default configuration values
- [ ] 8.7 Implement configuration validation
- [ ] 8.8 Add serde serialization for config structs
- [ ] 8.9 Implement environment variable loading
- [ ] 8.10 Add unit tests for configuration parsing and validation

## 9. Retention Policy Enforcement

- [ ] 9.1 Create cleanup module at src/workflows/checkpointing/cleanup.rs
- [ ] 9.2 Implement checkpoint age calculation based on checkpoint_time
- [ ] 9.3 Implement time-based retention enforcement (delete old checkpoints)
- [ ] 9.4 Implement count-based retention enforcement (keep N newest per workflow)
- [ ] 9.5 Implement combined policy enforcement
- [ ] 9.6 Create background cleanup job with scheduling
- [ ] 9.7 Implement manual cleanup trigger function
- [ ] 9.8 Add dry-run mode for cleanup operations
- [ ] 9.9 Track cleanup metrics (deleted count, policy violations)
- [ ] 9.10 Add unit tests for retention logic
- [ ] 9.11 Add integration tests for cleanup with SQLite

## 10. Workflow Integration

- [ ] 10.1 Integrate checkpoint hooks into workflow execution pipeline
- [ ] 10.2 Implement OnStepCompletion checkpoint trigger
- [ ] 10.3 Implement OnSuccess checkpoint trigger
- [ ] 10.4 Implement Periodic checkpoint trigger with timer
- [ ] 10.5 Add workflow resumption logic from checkpoint
- [ ] 10.6 Implement checkpoint version conflict detection and retry
- [ ] 10.7 Add error handling for checkpoint failures (non-blocking)
- [ ] 10.8 Create workflow example demonstrating checkpoint usage
- [ ] 10.9 Add integration tests for workflow checkpointing with Kind cluster

## 11. Documentation

- [ ] 11.1 Create README.md for checkpointing module
- [ ] 11.2 Document CheckpointStorage trait with examples
- [ ] 11.3 Document configuration options in config.rs
- [ ] 11.4 Document retention policy usage
- [ ] 11.5 Create example workflow with checkpointing
- [ ] 11.6 Update site-docs with checkpointing examples
- [ ] 11.7 Document StatefulSet deployment instructions
- [ ] 11.8 Document RBAC permissions required

## 12. Testing

- [ ] 12.1 Add unit tests for all storage trait operations
- [ ] 12.2 Add integration tests for SQLite StatefulSet with Kind
- [ ] 12.3 Test checkpoint save/load/update/delete operations
- [ ] 12.4 Test StatefulSet creation and lifecycle
- [ ] 12.5 Test optimistic locking with version conflicts
- [ ] 12.6 Test retention policy enforcement
- [ ] 12.7 Test retry logic with simulated network failures
- [ ] 12.8 Test background cleanup job
- [ ] 12.9 Add e2e tests for workflow resumption from checkpoint
- [ ] 12.10 Run cargo clippy and fix all warnings
- [ ] 12.11 Run cargo fmt to ensure consistent formatting
- [ ] 12.12 Run all tests with cargo test

## 13. Final Verification

- [ ] 13.1 Verify all specs are implemented (check against spec files)
- [ ] 13.2 Verify no mock/fake code exists (all real implementations)
- [ ] 13.3 Verify dependencies are up-to-date in Cargo.toml
- [ ] 13.4 Verify integration tests pass with Kind cluster
- [ ] 13.5 Create FDR for checkpointing feature in docs/fdrs/
- [ ] 13.6 Update CHANGELOG.md with feature summary
- [ ] 13.7 Run cargo build --release to ensure release build works
