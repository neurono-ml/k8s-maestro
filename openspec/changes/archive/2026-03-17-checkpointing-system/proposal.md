## Why

Kubernetes workflows lack native checkpoint/resume capabilities. When workflows fail or are interrupted, users must restart from the beginning, wasting time and resources. A plugin-based checkpointing system enables state persistence across failures, workflow resumption, and auditability without external dependencies.

## What Changes

- Add checkpointing module with plugin-based storage architecture
- Create CheckpointStorage trait for extensible storage backends
- Implement SQLiteCheckpointStorage with StatefulSet for persistent, editable storage
- Add checkpoint configuration with flexible frequency options (OnStepCompletion, OnSuccess, Periodic)
- Create Checkpoint and StepCheckpoint structs with optimistic locking (version field)
- Add REST API in StatefulSet for CRUD operations on checkpoints
- Implement automatic backend selection and retry logic in CheckpointStore
- Support retention policies for checkpoint lifecycle management

## Capabilities

### New Capabilities

- `checkpointing-plugin`: Plugin-based storage trait and implementations
- `checkpoint-sqlite`: SQLite StatefulSet storage with REST API
- `checkpoint-store`: Central checkpoint management with retry logic
- `checkpoint-config`: Configuration for checkpoint behavior and storage backends
- `checkpoint-statefulset`: StatefulSet lifecycle management for SQLite storage
- `checkpoint-retention`: Policies for checkpoint cleanup and expiration

### Modified Capabilities

- None (new functionality)

## Impact

- New module: `src/workflows/checkpointing/` directory structure
- Adds dependencies: tokio, chrono, serde, serde_json, uuid
- Requires Kubernetes RBAC permissions for StatefulSet and PVC creation
- Integration with existing workflow execution pipeline for checkpoint hooks
- No breaking changes to existing APIs
