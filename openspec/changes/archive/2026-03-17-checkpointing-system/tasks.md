## 1. Project Setup

- [x] 1.1 Add new dependencies to Cargo.toml (tokio, chrono, serde, serde_json, uuid, reqwest)
- [x] 1.2 Create module structure at src/workflows/checkpointing/ with subdirectories
- [x] 1.3 Create mod.rs files for checkpointing module hierarchy
- [x] 1.4 Add checkpointing module to src/lib.rs or src/workflows/mod.rs

## 2. Core Data Structures

- [x] 2.1 Create Checkpoint struct with workflow_id, checkpoint_time, steps, metadata, version
- [x] 2.2 Create StepCheckpoint struct with status, last_execution, outputs, execution_count
- [x] 2.3 Create CheckpointMetadata struct for listing operations
- [x] 2.4 Implement serde serialization/deserialization for all structs
- [x] 2.5 Add unit tests for data structures

## 3. Checkpointing Plugin System

- [x] 3.1 Create CheckpointStorage trait in src/workflows/checkpointing/plugin/storage.rs
- [x] 3.2 Define trait methods: connect, save_checkpoint, get_checkpoint, update_checkpoint, delete_checkpoint, list_checkpoints, cleanup
- [x] 3.3 Create error types for storage operations (StorageError)
- [x] 3.4 Create plugin/mod.rs to export CheckpointStorage trait
- [x] 3.5 Add unit tests for trait behavior with mock implementation

## 4. SQLite Storage Implementation

- [x] 4.1 Create SQLiteCheckpointStorage struct in src/workflows/checkpointing/plugin/sqlite.rs
- [x] 4.2 Implement CheckpointStorage trait for SQLiteCheckpointStorage
- [x] 4.3 Create HTTP client struct for communicating with SQLite pod
- [x] 4.4 Implement connect() method to verify SQLite pod is accessible
- [x] 4.5 Implement save_checkpoint() with HTTP POST
- [x] 4.6 Implement get_checkpoint() with HTTP GET
- [x] 4.7 Implement update_checkpoint() with HTTP PUT and version check
- [x] 4.8 Implement delete_checkpoint() with HTTP DELETE
- [x] 4.9 Implement list_checkpoints() with HTTP GET
- [x] 4.10 Implement cleanup() method
- [x] 4.11 Add unit tests for HTTP client operations

## 5. StatefulSet Management

- [x] 5.1 create statefulset.rs module at src/workflows/checkpointing/statefulset.rs
- [x] 5.2 Implement create_statefulset() function with Kubernetes client
- [x] 5.3 Implement update_statefulset() function for rolling updates
- [x] 5.4 Implement delete_statefulset() function with optional PVC deletion
- [x] 5.5 Implement wait_for_statefulset_ready() with timeout
- [x] 5.6 Configure StatefulSet with replicas=1, serviceName, podManagementPolicy
- [x] 5.7 Create PVC template with configurable storage size
- [x] 5.8 Configure SQLite container with image, port, resource limits
- [x] 5.9 Add liveness and readiness probes for HTTP endpoint
- [x] 5.10 Implement get_statefulset_status() function
- [x] 5.11 Add integration tests for StatefulSet lifecycle with Kind cluster

## 6. SQLite HTTP API Server

- [x] 6.1 Create HTTP server module for SQLite pod
- [x] 6.2 Implement POST /checkpoints endpoint with validation
- [x] 6.3 Implement GET /checkpoints/{workflow_id} endpoint
- [x] 6.4 Implement PUT /checkpoints/{workflow_id} with optimistic locking
- [x] 6.5 Implement DELETE /checkpoints/{workflow_id} endpoint
- [x] 6.6 Implement GET /checkpoints endpoint for listing
- [x] 6.7 Create SQLite database schema (checkpoints table)
- [x] 6.8 Implement ACID-compliant transaction handling
- [x] 6.9 Add error handling for HTTP responses (404, 409, 500)
- [x] 6.10 Add unit tests for HTTP endpoints

## 7. Checkpoint Store Management

- [x] 7.1 Create CheckpointStore struct in src/workflows/checkpointing/store.rs
- [x] 7.2 Implement storage backend registration mechanism
- [x] 7.3 Implement automatic backend selection based on config
- [x] 7.4 Create RetryConfig struct (max_retries, initial_backoff, multiplier)
- [x] 7.5 Implement exponential backoff retry logic for all operations
- [x] 7.6 Implement save_checkpoint() with retry
- [x] 7.7 Implement get_checkpoint() with retry
- [x] 7.8 Implement update_checkpoint() with retry and version conflict handling
- [x] 7.9 Implement delete_checkpoint() with retry
- [x] 7.10 Implement list_checkpoints() with retry
- [x] 7.11 Implement cleanup() method
- [x] 7.12 Add unit tests for retry logic with mock failures

## 8. Checkpoint Configuration

- [x] 8.1 Create CheckpointConfig struct in src/workflows/checkpointing/config.rs
- [x] 8.2 Create CheckpointFrequency enum (OnStepCompletion, OnSuccess, Periodic)
- [x] 8.3 Create CheckpointStorageConfig enum (Sqlite, Etcd, Redis, Postgres)
- [x] 8.4 Add storage backend parameter structs for each type
- [x] 8.5 Create RetentionPolicy struct with max_age and max_count
- [x] 8.6 Implement default configuration values
- [x] 8.7 Implement configuration validation
- [x] 8.8 Add serde serialization for config structs
- [x] 8.9 Implement environment variable loading
- [x] 8.10 Add unit tests for configuration parsing and validation

## 9. Retention Policy Enforcement

- [x] 9.1 Create cleanup module at src/workflows/checkpointing/cleanup.rs
- [x] 9.2 Implement checkpoint age calculation based on checkpoint_time
- [x] 9.3 Implement time-based retention enforcement (delete old checkpoints)
- [x] 9.4 Implement count-based retention enforcement (keep N newest per workflow)
- [x] 9.5 Implement combined policy enforcement
- [x] 9.6 Create background cleanup job with scheduling
- [x] 9.7 Implement manual cleanup trigger function
- [x] 9.8 Add dry-run mode for cleanup operations
- [x] 9.9 Track cleanup metrics (deleted count, policy violations)
- [x] 9.10 Add unit tests for retention logic
- [x] 9.11 Add integration tests for cleanup with SQLite

## 10. Workflow Integration

- [x] 10.1 Integrate checkpoint hooks into workflow execution pipeline
- [x] 10.2 Implement OnStepCompletion checkpoint trigger
- [x] 10.3 Implement OnSuccess checkpoint trigger
- [x] 10.4 Implement Periodic checkpoint trigger with timer
- [x] 10.5 Add workflow resumption logic from checkpoint
- [x] 10.6 Implement checkpoint version conflict detection and retry
- [x] 10.7 Add error handling for checkpoint failures (non-blocking)
- [x] 10.8 Create workflow example demonstrating checkpoint usage
- [x] 10.9 Add integration tests for workflow checkpointing with Kind cluster

## 11. Documentation

- [x] 11.1 Create README.md for checkpointing module
- [x] 11.2 Document CheckpointStorage trait with examples
- [x] 11.3 Document configuration options in config.rs
- [x] 11.4 Document retention policy usage
- [x] 11.5 Create example workflow with checkpointing
- [x] 11.6 Update site-docs with checkpointing examples
- [x] 11.7 Document StatefulSet deployment instructions
- [x] 11.8 Document RBAC permissions required

## 12. Testing

- [x] 12.1 Add unit tests for all storage trait operations
- [x] 12.2 Add integration tests for SQLite StatefulSet with Kind
- [x] 12.3 Test checkpoint save/load/update/delete operations
- [x] 12.4 Test StatefulSet creation and lifecycle
- [x] 12.5 Test optimistic locking with version conflicts
- [x] 12.6 Test retention policy enforcement
- [x] 12.7 Test retry logic with simulated network failures
- [x] 12.8 Test background cleanup job
- [x] 12.9 Add e2e tests for workflow resumption from checkpoint
- [x] 12.10 Run cargo clippy and fix all warnings
- [x] 12.11 Run cargo fmt to ensure consistent formatting
- [x] 12.12 Run all tests with cargo test

## 13. Final Verification

- [x] 13.1 Verify all specs are implemented (check against spec files)
- [x] 13.2 Verify no mock/fake code exists (all real implementations)
- [x] 13.3 Verify dependencies are up-to-date in Cargo.toml
- [x] 13.4 Verify integration tests pass with Kind cluster
- [x] 13.5 Create FDR for checkpointing feature in docs/fdrs/
- [x] 13.6 Update CHANGELOG.md with feature summary
- [x] 13.7 Run cargo build --release to ensure release build works
