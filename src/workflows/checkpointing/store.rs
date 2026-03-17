use crate::workflows::checkpointing::config::CheckpointStorageConfig;
use crate::workflows::checkpointing::models::{Checkpoint, CheckpointMetadata};
use crate::workflows::checkpointing::plugin::{
    CheckpointStorage, SQLiteCheckpointStorage, StorageError, StorageResult,
};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_backoff: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(100),
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_initial_backoff(mut self, backoff: Duration) -> Self {
        self.initial_backoff = backoff;
        self
    }

    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }
}

pub struct CheckpointStore {
    storage: Arc<dyn CheckpointStorage>,
    retry_config: RetryConfig,
}

impl CheckpointStore {
    pub fn new(storage_config: &CheckpointStorageConfig) -> Result<Self, StorageError> {
        let storage: Arc<dyn CheckpointStorage> = match storage_config {
            CheckpointStorageConfig::Sqlite { namespace, .. } => Arc::new(
                SQLiteCheckpointStorage::new(namespace, "maestro-checkpoint-storage"),
            ),
            CheckpointStorageConfig::Etcd { .. } => {
                return Err(StorageError::InternalError(
                    "Etcd storage not yet implemented".to_string(),
                ))
            }
            CheckpointStorageConfig::Redis { .. } => {
                return Err(StorageError::InternalError(
                    "Redis storage not yet implemented".to_string(),
                ))
            }
            CheckpointStorageConfig::Postgres { .. } => {
                return Err(StorageError::InternalError(
                    "Postgres storage not yet implemented".to_string(),
                ))
            }
        };

        Ok(Self {
            storage,
            retry_config: RetryConfig::default(),
        })
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    pub async fn initialize(&self) -> StorageResult<()> {
        self.storage.connect().await
    }

    async fn retry_operation<F, Fut, T>(&self, operation: F) -> StorageResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = StorageResult<T>>,
    {
        let mut backoff = self.retry_config.initial_backoff;

        for attempt in 0..self.retry_config.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < self.retry_config.max_retries - 1 => {
                    if matches!(
                        e,
                        StorageError::NetworkError(_) | StorageError::ConnectionError(_)
                    ) {
                        tokio::time::sleep(backoff).await;
                        backoff = Duration::from_millis(
                            (backoff.as_millis() as f64 * self.retry_config.backoff_multiplier)
                                as u64,
                        );
                    } else {
                        return Err(e);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Err(StorageError::InternalError(
            "Max retries exceeded".to_string(),
        ))
    }

    pub async fn save_checkpoint(
        &self,
        workflow_id: &str,
        checkpoint: &Checkpoint,
    ) -> StorageResult<()> {
        self.retry_operation(|| self.storage.save_checkpoint(workflow_id, checkpoint))
            .await
    }

    pub async fn get_checkpoint(&self, workflow_id: &str) -> StorageResult<Option<Checkpoint>> {
        self.retry_operation(|| self.storage.get_checkpoint(workflow_id))
            .await
    }

    pub async fn update_checkpoint(
        &self,
        workflow_id: &str,
        checkpoint: &Checkpoint,
    ) -> StorageResult<()> {
        self.retry_operation(|| self.storage.update_checkpoint(workflow_id, checkpoint))
            .await
    }

    pub async fn delete_checkpoint(&self, workflow_id: &str) -> StorageResult<()> {
        self.retry_operation(|| self.storage.delete_checkpoint(workflow_id))
            .await
    }

    pub async fn list_checkpoints(&self) -> StorageResult<Vec<CheckpointMetadata>> {
        self.retry_operation(|| self.storage.list_checkpoints())
            .await
    }

    pub async fn cleanup(&self) -> StorageResult<()> {
        self.retry_operation(|| self.storage.cleanup()).await
    }

    pub async fn close(&self) -> StorageResult<()> {
        self.storage.cleanup().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflows::checkpointing::plugin::CheckpointStorage;
    use async_trait::async_trait;

    #[derive(Debug)]
    struct MockStorage {
        checkpoints: tokio::sync::RwLock<std::collections::HashMap<String, Checkpoint>>,
        call_count: tokio::sync::RwLock<usize>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                checkpoints: tokio::sync::RwLock::new(std::collections::HashMap::new()),
                call_count: tokio::sync::RwLock::new(0),
            }
        }
    }

    #[async_trait]
    impl CheckpointStorage for MockStorage {
        async fn connect(&self) -> StorageResult<()> {
            Ok(())
        }

        async fn save_checkpoint(
            &self,
            workflow_id: &str,
            checkpoint: &Checkpoint,
        ) -> StorageResult<()> {
            *self.call_count.write().await += 1;
            let mut checkpoints = self.checkpoints.write().await;
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
    async fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_backoff, Duration::from_millis(100));
        assert_eq!(config.backoff_multiplier, 2.0);
    }

    #[tokio::test]
    async fn test_retry_config_builder() {
        let config = RetryConfig::new()
            .with_max_retries(5)
            .with_initial_backoff(Duration::from_millis(200))
            .with_backoff_multiplier(3.0);

        assert_eq!(config.max_retries, 5);
        assert_eq!(config.initial_backoff, Duration::from_millis(200));
        assert_eq!(config.backoff_multiplier, 3.0);
    }

    #[tokio::test]
    async fn test_checkpoint_store_save_and_get() {
        let storage = Arc::new(MockStorage::new());
        let store = CheckpointStore {
            storage: storage.clone(),
            retry_config: RetryConfig::default(),
        };

        store.initialize().await.unwrap();

        let checkpoint = Checkpoint::new("workflow-1");
        store
            .save_checkpoint("workflow-1", &checkpoint)
            .await
            .unwrap();

        let retrieved = store.get_checkpoint("workflow-1").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().workflow_id, "workflow-1");
    }

    #[tokio::test]
    async fn test_checkpoint_store_list() {
        let storage = Arc::new(MockStorage::new());
        let store = CheckpointStore {
            storage: storage.clone(),
            retry_config: RetryConfig::default(),
        };

        store.initialize().await.unwrap();

        let checkpoint1 = Checkpoint::new("workflow-1");
        let checkpoint2 = Checkpoint::new("workflow-2");

        store
            .save_checkpoint("workflow-1", &checkpoint1)
            .await
            .unwrap();
        store
            .save_checkpoint("workflow-2", &checkpoint2)
            .await
            .unwrap();

        let list = store.list_checkpoints().await.unwrap();
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn test_checkpoint_store_delete() {
        let storage = Arc::new(MockStorage::new());
        let store = CheckpointStore {
            storage: storage.clone(),
            retry_config: RetryConfig::default(),
        };

        store.initialize().await.unwrap();

        let checkpoint = Checkpoint::new("workflow-1");
        store
            .save_checkpoint("workflow-1", &checkpoint)
            .await
            .unwrap();

        store.delete_checkpoint("workflow-1").await.unwrap();
        let result = store.get_checkpoint("workflow-1").await.unwrap();
        assert!(result.is_none());
    }
}
