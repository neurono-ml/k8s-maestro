use super::storage::{CheckpointStorage, StorageError, StorageResult};
use crate::workflows::checkpointing::models::{Checkpoint, CheckpointMetadata};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_PORT: u16 = 8080;

pub struct SQLiteCheckpointStorage {
    namespace: String,
    service_name: String,
    port: u16,
    client: Client,
}

impl SQLiteCheckpointStorage {
    pub fn new(namespace: impl Into<String>, service_name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            service_name: service_name.into(),
            port: DEFAULT_PORT,
            client: Client::builder()
                .timeout(DEFAULT_TIMEOUT)
                .build()
                .unwrap_or_default(),
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_default();
        self
    }

    fn base_url(&self) -> String {
        format!("http://{}.{}/api/v1", self.service_name, self.namespace)
    }

    async fn handle_http_response(&self, response: reqwest::Response) -> StorageResult<String> {
        let status = response.status();

        if status.is_success() {
            Ok(response.text().await.unwrap_or_default())
        } else {
            let error_msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            match status.as_u16() {
                404 => Err(StorageError::NotFound(error_msg)),
                409 => Err(StorageError::AlreadyExists(error_msg)),
                500 => Err(StorageError::InternalError(error_msg)),
                _ => Err(StorageError::NetworkError(format!(
                    "HTTP {}: {}",
                    status, error_msg
                ))),
            }
        }
    }
}

#[async_trait]
impl CheckpointStorage for SQLiteCheckpointStorage {
    async fn connect(&self) -> StorageResult<()> {
        let url = format!("{}/health", self.base_url());

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(StorageError::ConnectionError(format!(
                "Health check failed: {}",
                response.status()
            )))
        }
    }

    async fn save_checkpoint(
        &self,
        workflow_id: &str,
        checkpoint: &Checkpoint,
    ) -> StorageResult<()> {
        let url = format!("{}/checkpoints", self.base_url());

        let body = json!({
            "workflow_id": workflow_id,
            "checkpoint": checkpoint,
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        self.handle_http_response(response).await?;
        Ok(())
    }

    async fn get_checkpoint(&self, workflow_id: &str) -> StorageResult<Option<Checkpoint>> {
        let url = format!("{}/checkpoints/{}", self.base_url(), workflow_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();

        if status == 404 {
            return Ok(None);
        }

        if !status.is_success() {
            return Err(StorageError::NetworkError(format!("HTTP {}", status)));
        }

        let text = response
            .text()
            .await
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        let checkpoint: Checkpoint = serde_json::from_str(&text)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        Ok(Some(checkpoint))
    }

    async fn update_checkpoint(
        &self,
        workflow_id: &str,
        checkpoint: &Checkpoint,
    ) -> StorageResult<()> {
        let url = format!("{}/checkpoints/{}", self.base_url(), workflow_id);

        let body = json!({
            "checkpoint": checkpoint,
            "expected_version": checkpoint.version.saturating_sub(1),
        });

        let response = self
            .client
            .put(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();

        if status == 409 {
            return Err(StorageError::VersionConflict {
                expected: checkpoint.version.saturating_sub(1),
                actual: checkpoint.version,
            });
        }

        self.handle_http_response(response).await?;
        Ok(())
    }

    async fn delete_checkpoint(&self, workflow_id: &str) -> StorageResult<()> {
        let url = format!("{}/checkpoints/{}", self.base_url(), workflow_id);

        let response = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        self.handle_http_response(response).await?;
        Ok(())
    }

    async fn list_checkpoints(&self) -> StorageResult<Vec<CheckpointMetadata>> {
        let url = format!("{}/checkpoints", self.base_url());

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let text = self.handle_http_response(response).await?;

        let metadata: Vec<CheckpointMetadata> = serde_json::from_str(&text)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        Ok(metadata)
    }

    async fn cleanup(&self) -> StorageResult<()> {
        let url = format!("{}/checkpoints/cleanup", self.base_url());

        let response = self
            .client
            .post(&url)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        self.handle_http_response(response).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflows::checkpointing::models::StepCheckpoint;
    use std::collections::BTreeMap;

    #[allow(dead_code)]
    fn create_test_checkpoint(workflow_id: &str) -> Checkpoint {
        let mut outputs = BTreeMap::new();
        outputs.insert("result".to_string(), serde_json::json!("success"));

        Checkpoint::new(workflow_id).with_step(
            "step-1",
            StepCheckpoint::new("completed").with_outputs(outputs),
        )
    }

    #[test]
    fn test_sqlite_checkpoint_storage_new() {
        let storage = SQLiteCheckpointStorage::new("default", "maestro-checkpoint-storage");
        assert_eq!(storage.namespace, "default");
        assert_eq!(storage.service_name, "maestro-checkpoint-storage");
        assert_eq!(storage.port, 8080);
    }

    #[test]
    fn test_sqlite_checkpoint_storage_with_port() {
        let storage =
            SQLiteCheckpointStorage::new("default", "maestro-checkpoint-storage").with_port(9090);
        assert_eq!(storage.port, 9090);
    }

    #[test]
    fn test_sqlite_checkpoint_storage_base_url() {
        let storage = SQLiteCheckpointStorage::new("default", "maestro-checkpoint-storage");
        assert_eq!(
            storage.base_url(),
            "http://maestro-checkpoint-storage.default/api/v1"
        );
    }

    #[test]
    fn test_sqlite_checkpoint_storage_custom_namespace() {
        let storage = SQLiteCheckpointStorage::new("production", "checkpoint-service");
        assert_eq!(
            storage.base_url(),
            "http://checkpoint-service.production/api/v1"
        );
    }
}
