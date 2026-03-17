use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StepCheckpoint {
    pub status: String,
    pub last_execution: DateTime<Utc>,
    pub outputs: BTreeMap<String, serde_json::Value>,
    pub execution_count: u32,
}

impl StepCheckpoint {
    pub fn new(status: impl Into<String>) -> Self {
        Self {
            status: status.into(),
            last_execution: Utc::now(),
            outputs: BTreeMap::new(),
            execution_count: 0,
        }
    }

    pub fn with_outputs(mut self, outputs: BTreeMap<String, serde_json::Value>) -> Self {
        self.outputs = outputs;
        self
    }

    pub fn with_execution_count(mut self, count: u32) -> Self {
        self.execution_count = count;
        self
    }

    pub fn increment_execution_count(&mut self) {
        self.execution_count += 1;
    }

    pub fn update_status(&mut self, status: impl Into<String>) {
        self.status = status.into();
        self.last_execution = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Checkpoint {
    pub workflow_id: String,
    pub checkpoint_time: DateTime<Utc>,
    pub steps: BTreeMap<String, StepCheckpoint>,
    pub metadata: BTreeMap<String, String>,
    pub version: u32,
}

impl Checkpoint {
    pub fn new(workflow_id: impl Into<String>) -> Self {
        Self {
            workflow_id: workflow_id.into(),
            checkpoint_time: Utc::now(),
            steps: BTreeMap::new(),
            metadata: BTreeMap::new(),
            version: 0,
        }
    }

    pub fn with_step(mut self, step_id: impl Into<String>, step: StepCheckpoint) -> Self {
        self.steps.insert(step_id.into(), step);
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn increment_version(&mut self) {
        self.version += 1;
    }

    pub fn update_checkpoint_time(&mut self) {
        self.checkpoint_time = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckpointMetadata {
    pub workflow_id: String,
    pub checkpoint_time: DateTime<Utc>,
    pub version: u32,
    pub step_count: usize,
}

impl CheckpointMetadata {
    pub fn from_checkpoint(checkpoint: &Checkpoint) -> Self {
        Self {
            workflow_id: checkpoint.workflow_id.clone(),
            checkpoint_time: checkpoint.checkpoint_time,
            version: checkpoint.version,
            step_count: checkpoint.steps.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_checkpoint_new() {
        let step = StepCheckpoint::new("completed");
        assert_eq!(step.status, "completed");
        assert_eq!(step.execution_count, 0);
        assert!(step.outputs.is_empty());
    }

    #[test]
    fn test_step_checkpoint_with_outputs() {
        let mut outputs = BTreeMap::new();
        outputs.insert("key".to_string(), serde_json::json!("value"));

        let step = StepCheckpoint::new("completed").with_outputs(outputs);
        assert_eq!(step.outputs.get("key"), Some(&serde_json::json!("value")));
    }

    #[test]
    fn test_step_checkpoint_increment_execution_count() {
        let mut step = StepCheckpoint::new("completed");
        assert_eq!(step.execution_count, 0);
        step.increment_execution_count();
        assert_eq!(step.execution_count, 1);
    }

    #[test]
    fn test_step_checkpoint_update_status() {
        let mut step = StepCheckpoint::new("running");
        step.update_status("completed");
        assert_eq!(step.status, "completed");
    }

    #[test]
    fn test_checkpoint_new() {
        let checkpoint = Checkpoint::new("workflow-1");
        assert_eq!(checkpoint.workflow_id, "workflow-1");
        assert_eq!(checkpoint.version, 0);
        assert!(checkpoint.steps.is_empty());
        assert!(checkpoint.metadata.is_empty());
    }

    #[test]
    fn test_checkpoint_with_step() {
        let step = StepCheckpoint::new("completed");
        let checkpoint = Checkpoint::new("workflow-1").with_step("step-1", step);
        assert!(checkpoint.steps.contains_key("step-1"));
    }

    #[test]
    fn test_checkpoint_with_metadata() {
        let checkpoint = Checkpoint::new("workflow-1").with_metadata("key", "value");
        assert_eq!(checkpoint.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_checkpoint_increment_version() {
        let mut checkpoint = Checkpoint::new("workflow-1");
        assert_eq!(checkpoint.version, 0);
        checkpoint.increment_version();
        assert_eq!(checkpoint.version, 1);
    }

    #[test]
    fn test_checkpoint_metadata_from_checkpoint() {
        let step = StepCheckpoint::new("completed");
        let checkpoint = Checkpoint::new("workflow-1")
            .with_step("step-1", step)
            .with_metadata("key", "value");

        let metadata = CheckpointMetadata::from_checkpoint(&checkpoint);
        assert_eq!(metadata.workflow_id, "workflow-1");
        assert_eq!(metadata.step_count, 1);
        assert_eq!(metadata.version, 0);
    }

    #[test]
    fn test_checkpoint_serialization() {
        let checkpoint = Checkpoint::new("workflow-1");
        let serialized = serde_json::to_string(&checkpoint).unwrap();
        let deserialized: Checkpoint = serde_json::from_str(&serialized).unwrap();
        assert_eq!(checkpoint, deserialized);
    }

    #[test]
    fn test_step_checkpoint_serialization() {
        let step = StepCheckpoint::new("completed");
        let serialized = serde_json::to_string(&step).unwrap();
        let deserialized: StepCheckpoint = serde_json::from_str(&serialized).unwrap();
        assert_eq!(step, deserialized);
    }

    #[test]
    fn test_checkpoint_metadata_serialization() {
        let checkpoint = Checkpoint::new("workflow-1");
        let metadata = CheckpointMetadata::from_checkpoint(&checkpoint);
        let serialized = serde_json::to_string(&metadata).unwrap();
        let deserialized: CheckpointMetadata = serde_json::from_str(&serialized).unwrap();
        assert_eq!(metadata, deserialized);
    }
}
