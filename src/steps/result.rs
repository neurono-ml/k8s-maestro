use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    Success,
    Failure,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub status: StepStatus,
    pub exit_code: i32,
    pub outputs: BTreeMap<String, serde_json::Value>,
    pub stdout: String,
    pub stderr: String,
    pub metadata: BTreeMap<String, String>,
    pub execution_time: Duration,
}

impl StepResult {
    pub fn new(step_id: impl Into<String>) -> Self {
        Self {
            step_id: step_id.into(),
            status: StepStatus::Skipped,
            exit_code: -1,
            outputs: BTreeMap::new(),
            stdout: String::new(),
            stderr: String::new(),
            metadata: BTreeMap::new(),
            execution_time: Duration::ZERO,
        }
    }

    pub fn with_status(mut self, status: StepStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_exit_code(mut self, exit_code: i32) -> Self {
        self.exit_code = exit_code;
        self
    }

    pub fn with_output(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.outputs.insert(key.into(), value);
        self
    }

    pub fn with_stdout(mut self, stdout: impl Into<String>) -> Self {
        self.stdout = stdout.into();
        self
    }

    pub fn with_stderr(mut self, stderr: impl Into<String>) -> Self {
        self.stderr = stderr.into();
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn with_execution_time(mut self, execution_time: Duration) -> Self {
        self.execution_time = execution_time;
        self
    }

    pub fn is_success(&self) -> bool {
        self.status == StepStatus::Success
    }

    pub fn is_failure(&self) -> bool {
        self.status == StepStatus::Failure
    }

    pub fn is_skipped(&self) -> bool {
        self.status == StepStatus::Skipped
    }
}

impl Default for StepResult {
    fn default() -> Self {
        Self::new("default")
    }
}
