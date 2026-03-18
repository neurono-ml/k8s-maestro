use crate::workflows::checkpointing::models::{Checkpoint, CheckpointMetadata};
use thiserror::Error;

#[async_trait::async_trait]
pub trait CheckpointStorage: Send + Sync {
    async fn connect(&self) -> StorageResult<()>;

    async fn save_checkpoint(
        &self,
        workflow_id: &str,
        checkpoint: &Checkpoint,
    ) -> StorageResult<()>;

    async fn get_checkpoint(&self, workflow_id: &str) -> StorageResult<Option<Checkpoint>>;

    async fn update_checkpoint(
        &self,
        workflow_id: &str,
        checkpoint: &Checkpoint,
    ) -> StorageResult<()>;

    async fn delete_checkpoint(&self, workflow_id: &str) -> StorageResult<()>;

    async fn list_checkpoints(&self) -> StorageResult<Vec<CheckpointMetadata>>;

    async fn cleanup(&self) -> StorageResult<()>;
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Checkpoint not found: {0}")]
    NotFound(String),

    #[error("Checkpoint already exists: {0}")]
    AlreadyExists(String),

    #[error("Version conflict: expected {expected}, got {actual}")]
    VersionConflict { expected: u32, actual: u32 },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

#[cfg(test)]
#[derive(Debug)]
struct MockStorage {
    checkpoints: tokio::sync::RwLock<std::collections::HashMap<String, Checkpoint>>,
}

#[cfg(test)]
impl MockStorage {
    fn new() -> Self {
        Self {
            checkpoints: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

#[cfg(test)]
#[async_trait::async_trait]
impl CheckpointStorage for MockStorage {
    async fn connect(&self) -> StorageResult<()> {
        Ok(())
    }

    async fn save_checkpoint(
        &self,
        workflow_id: &str,
        checkpoint: &Checkpoint,
    ) -> StorageResult<()> {
        let mut checkpoints = self.checkpoints.write().await;
        if checkpoints.contains_key(workflow_id) {
            return Err(StorageError::AlreadyExists(workflow_id.to_string()));
        }
        checkpoints.insert(workflow_id.to_string(), checkpoint.clone());
        Ok(())
    }

    async fn get_checkpoint(&self, workflow_id: &str) -> StorageResult<Option<Checkpoint>> {
        let checkpoints = self.checkpoints.read().await;
        Ok(checkpoints.get(workflow_id).cloned())
    }

    async fn update_checkpoint(
        &self,
        workflow_id: &str,
        checkpoint: &Checkpoint,
    ) -> StorageResult<()> {
        let mut checkpoints = self.checkpoints.write().await;
        if !checkpoints.contains_key(workflow_id) {
            return Err(StorageError::NotFound(workflow_id.to_string()));
        }
        checkpoints.insert(workflow_id.to_string(), checkpoint.clone());
        Ok(())
    }

    async fn delete_checkpoint(&self, workflow_id: &str) -> StorageResult<()> {
        let mut checkpoints = self.checkpoints.write().await;
        checkpoints
            .remove(workflow_id)
            .ok_or_else(|| StorageError::NotFound(workflow_id.to_string()))?;
        Ok(())
    }

    async fn list_checkpoints(&self) -> StorageResult<Vec<CheckpointMetadata>> {
        let checkpoints = self.checkpoints.read().await;
        let metadata: Vec<CheckpointMetadata> = checkpoints
            .values()
            .map(CheckpointMetadata::from_checkpoint)
            .collect();
        Ok(metadata)
    }

    async fn cleanup(&self) -> StorageResult<()> {
        self.checkpoints.write().await.clear();
        Ok(())
    }
}

#[tokio::test]
async fn test_mock_storage_save_and_get() {
    let storage = MockStorage::new();
    storage.connect().await.unwrap();

    let checkpoint = Checkpoint::new("workflow-1");
    storage
        .save_checkpoint("workflow-1", &checkpoint)
        .await
        .unwrap();

    let retrieved = storage.get_checkpoint("workflow-1").await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().workflow_id, "workflow-1");
}

#[tokio::test]
async fn test_mock_storage_save_duplicate() {
    let storage = MockStorage::new();
    storage.connect().await.unwrap();

    let checkpoint = Checkpoint::new("workflow-1");
    storage
        .save_checkpoint("workflow-1", &checkpoint)
        .await
        .unwrap();

    let result = storage.save_checkpoint("workflow-1", &checkpoint).await;
    assert!(matches!(result, Err(StorageError::AlreadyExists(_))));
}

#[tokio::test]
async fn test_mock_storage_get_nonexistent() {
    let storage = MockStorage::new();
    storage.connect().await.unwrap();

    let result = storage.get_checkpoint("workflow-1").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_mock_storage_update_nonexistent() {
    let storage = MockStorage::new();
    storage.connect().await.unwrap();

    let checkpoint = Checkpoint::new("workflow-1");
    let result = storage.update_checkpoint("workflow-1", &checkpoint).await;
    assert!(matches!(result, Err(StorageError::NotFound(_))));
}

#[tokio::test]
async fn test_mock_storage_delete() {
    let storage = MockStorage::new();
    storage.connect().await.unwrap();

    let checkpoint = Checkpoint::new("workflow-1");
    storage
        .save_checkpoint("workflow-1", &checkpoint)
        .await
        .unwrap();

    storage.delete_checkpoint("workflow-1").await.unwrap();
    let result = storage.get_checkpoint("workflow-1").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_mock_storage_delete_nonexistent() {
    let storage = MockStorage::new();
    storage.connect().await.unwrap();

    let result = storage.delete_checkpoint("workflow-1").await;
    assert!(matches!(result, Err(StorageError::NotFound(_))));
}

#[tokio::test]
async fn test_mock_storage_list() {
    let storage = MockStorage::new();
    storage.connect().await.unwrap();

    let checkpoint1 = Checkpoint::new("workflow-1");
    let checkpoint2 = Checkpoint::new("workflow-2");

    storage
        .save_checkpoint("workflow-1", &checkpoint1)
        .await
        .unwrap();
    storage
        .save_checkpoint("workflow-2", &checkpoint2)
        .await
        .unwrap();

    let list = storage.list_checkpoints().await.unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn test_mock_storage_cleanup() {
    let storage = MockStorage::new();
    storage.connect().await.unwrap();

    let checkpoint = Checkpoint::new("workflow-1");
    storage
        .save_checkpoint("workflow-1", &checkpoint)
        .await
        .unwrap();

    storage.cleanup().await.unwrap();
    let list = storage.list_checkpoints().await.unwrap();
    assert_eq!(list.len(), 0);
}
