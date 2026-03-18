use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct CleanupMetrics {
    pub deleted_count: usize,
    pub policy_violations: usize,
}

pub struct CleanupManager {
    storage: std::sync::Arc<dyn crate::workflows::checkpointing::plugin::CheckpointStorage>,
    policy: RetentionPolicy,
    dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    pub max_age: Option<Duration>,
    pub max_count: Option<usize>,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_age: Some(Duration::days(7)),
            max_count: Some(10),
        }
    }
}

impl CleanupManager {
    pub fn new(
        storage: std::sync::Arc<dyn crate::workflows::checkpointing::plugin::CheckpointStorage>,
        policy: RetentionPolicy,
    ) -> Self {
        Self {
            storage,
            policy,
            dry_run: false,
        }
    }

    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub async fn cleanup(
        &self,
    ) -> Result<CleanupMetrics, Box<dyn std::error::Error + Send + Sync>> {
        let mut metrics = CleanupMetrics::default();

        let checkpoints = self.storage.list_checkpoints().await?;
        let mut workflow_checkpoints: HashMap<String, Vec<(String, DateTime<Utc>)>> =
            HashMap::new();

        for metadata in checkpoints {
            workflow_checkpoints
                .entry(metadata.workflow_id.clone())
                .or_default()
                .push((metadata.workflow_id.clone(), metadata.checkpoint_time));
        }

        let now = Utc::now();
        let mut to_delete = Vec::new();

        for (_workflow_id, checkpoints_ref) in workflow_checkpoints.iter() {
            let mut checkpoints_by_age: Vec<_> = checkpoints_ref.clone();
            checkpoints_by_age.sort_by(|a, b| b.1.cmp(&a.1));

            if let Some(max_age) = self.policy.max_age {
                for (workflow_id, checkpoint_time) in checkpoints_ref.iter() {
                    let age = now.signed_duration_since(*checkpoint_time);
                    if age > max_age {
                        to_delete.push(workflow_id.clone());
                        metrics.policy_violations += 1;
                    }
                }
            }

            if let Some(max_count) = self.policy.max_count {
                if checkpoints_by_age.len() > max_count {
                    for (workflow_id, _) in checkpoints_by_age.iter().skip(max_count) {
                        to_delete.push(workflow_id.clone());
                        metrics.policy_violations += 1;
                    }
                }
            }
        }

        to_delete.sort();
        to_delete.dedup();

        if self.dry_run {
            return Ok(metrics);
        }

        for workflow_id in &to_delete {
            #[allow(clippy::redundant_pattern_matching)]
            if let Err(_) = self.storage.delete_checkpoint(workflow_id).await {
                log::warn!("Failed to delete checkpoint for workflow: {}", workflow_id);
            } else {
                metrics.deleted_count += 1;
            }
        }

        Ok(metrics)
    }

    pub async fn manual_cleanup(
        &self,
    ) -> Result<CleanupMetrics, Box<dyn std::error::Error + Send + Sync>> {
        self.cleanup().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflows::checkpointing::models::{Checkpoint, CheckpointMetadata};
    use crate::workflows::checkpointing::plugin::CheckpointStorage;
    use async_trait::async_trait;

    #[derive(Debug)]
    struct MockStorage {
        checkpoints: tokio::sync::RwLock<std::collections::HashMap<String, Checkpoint>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                checkpoints: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl CheckpointStorage for MockStorage {
        async fn connect(
            &self,
        ) -> Result<(), crate::workflows::checkpointing::plugin::StorageError> {
            Ok(())
        }

        async fn save_checkpoint(
            &self,
            workflow_id: &str,
            checkpoint: &Checkpoint,
        ) -> Result<(), crate::workflows::checkpointing::plugin::StorageError> {
            let mut checkpoints = self.checkpoints.write().await;
            checkpoints.insert(workflow_id.to_string(), checkpoint.clone());
            Ok(())
        }

        async fn get_checkpoint(
            &self,
            workflow_id: &str,
        ) -> Result<Option<Checkpoint>, crate::workflows::checkpointing::plugin::StorageError>
        {
            let checkpoints = self.checkpoints.read().await;
            Ok(checkpoints.get(workflow_id).cloned())
        }

        async fn update_checkpoint(
            &self,
            workflow_id: &str,
            checkpoint: &Checkpoint,
        ) -> Result<(), crate::workflows::checkpointing::plugin::StorageError> {
            let mut checkpoints = self.checkpoints.write().await;
            checkpoints.insert(workflow_id.to_string(), checkpoint.clone());
            Ok(())
        }

        async fn delete_checkpoint(
            &self,
            workflow_id: &str,
        ) -> Result<(), crate::workflows::checkpointing::plugin::StorageError> {
            let mut checkpoints = self.checkpoints.write().await;
            checkpoints.remove(workflow_id).ok_or_else(|| {
                crate::workflows::checkpointing::plugin::StorageError::NotFound(
                    workflow_id.to_string(),
                )
            })?;
            Ok(())
        }

        async fn list_checkpoints(
            &self,
        ) -> Result<Vec<CheckpointMetadata>, crate::workflows::checkpointing::plugin::StorageError>
        {
            let checkpoints = self.checkpoints.read().await;
            let metadata: Vec<CheckpointMetadata> = checkpoints
                .values()
                .map(CheckpointMetadata::from_checkpoint)
                .collect();
            Ok(metadata)
        }

        async fn cleanup(
            &self,
        ) -> Result<(), crate::workflows::checkpointing::plugin::StorageError> {
            self.checkpoints.write().await.clear();
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_retention_policy_default() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.max_age, Some(Duration::days(7)));
        assert_eq!(policy.max_count, Some(10));
    }

    #[tokio::test]
    async fn test_cleanup_manager_dry_run() {
        let storage = std::sync::Arc::new(MockStorage::new());
        let manager = CleanupManager::new(
            storage.clone(),
            RetentionPolicy {
                max_age: Some(Duration::days(1)),
                max_count: Some(5),
            },
        )
        .with_dry_run(true);

        let checkpoint = Checkpoint::new("workflow-1");
        storage
            .save_checkpoint("workflow-1", &checkpoint)
            .await
            .unwrap();

        let metrics = manager.cleanup().await.unwrap();
        assert_eq!(metrics.deleted_count, 0);
    }

    #[tokio::test]
    async fn test_cleanup_manager_age_policy() {
        let storage = std::sync::Arc::new(MockStorage::new());
        let mut old_checkpoint = Checkpoint::new("workflow-1");
        old_checkpoint.checkpoint_time = Utc::now() - Duration::days(10);

        storage
            .save_checkpoint("workflow-1", &old_checkpoint)
            .await
            .unwrap();

        let manager = CleanupManager::new(
            storage.clone(),
            RetentionPolicy {
                max_age: Some(Duration::days(7)),
                max_count: None,
            },
        );

        let metrics = manager.cleanup().await.unwrap();
        assert!(metrics.deleted_count > 0 || metrics.policy_violations > 0);
    }
}
