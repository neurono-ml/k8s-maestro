use crate::steps::traits::{ResourceLimits, WorkFlowStep};
use anyhow::Result;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct WorkflowMetadata {
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub labels: std::collections::HashMap<String, String>,
    pub annotations: std::collections::HashMap<String, String>,
}

impl Default for WorkflowMetadata {
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            created_at: now,
            updated_at: now,
            labels: std::collections::HashMap::new(),
            annotations: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionMode {
    Sequential,
    Parallel(usize),
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::Sequential
    }
}

#[derive(Debug, Clone)]
pub struct CheckpointConfig {
    pub enabled: bool,
    pub checkpoint_interval_secs: u64,
    pub retention_count: usize,
    pub storage_path: Option<String>,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            checkpoint_interval_secs: 300,
            retention_count: 10,
            storage_path: None,
        }
    }
}

impl CheckpointConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_interval_secs(mut self, secs: u64) -> Self {
        self.checkpoint_interval_secs = secs;
        self
    }

    pub fn with_retention_count(mut self, count: usize) -> Self {
        self.retention_count = count;
        self
    }

    pub fn with_storage_path(mut self, path: impl Into<String>) -> Self {
        self.storage_path = Some(path.into());
        self
    }
}

pub struct Workflow {
    pub id: String,
    pub name: String,
    pub namespace: String,
    pub steps: Vec<Box<dyn WorkFlowStep>>,
    pub resource_limits: Option<ResourceLimits>,
    pub checkpoint_config: Option<CheckpointConfig>,
    pub metadata: WorkflowMetadata,
    pub parallelism: usize,
    pub execution_mode: ExecutionMode,
}

impl std::fmt::Debug for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Workflow")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("namespace", &self.namespace)
            .field(
                "steps",
                &self.steps.iter().map(|s| s.step_id()).collect::<Vec<_>>(),
            )
            .field("resource_limits", &self.resource_limits)
            .field("checkpoint_config", &self.checkpoint_config)
            .field("metadata", &self.metadata)
            .field("parallelism", &self.parallelism)
            .field("execution_mode", &self.execution_mode)
            .finish()
    }
}

impl Workflow {
    pub fn builder() -> crate::workflows::WorkflowBuilder {
        crate::workflows::WorkflowBuilder::new()
    }

    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("Workflow name cannot be empty");
        }

        if self.namespace.is_empty() {
            anyhow::bail!("Workflow namespace cannot be empty");
        }

        if self.steps.is_empty() {
            anyhow::bail!("Workflow must have at least one step");
        }

        if self.parallelism == 0 {
            anyhow::bail!("Workflow parallelism must be greater than 0");
        }

        Ok(())
    }

    pub fn resolve_resource_limits(&self, app_defaults: Option<&ResourceLimits>) -> ResourceLimits {
        let mut limits = app_defaults.cloned().unwrap_or_default();

        if let Some(workflow_limits) = &self.resource_limits {
            if workflow_limits.cpu.is_some() {
                limits.cpu = workflow_limits.cpu.clone();
            }
            if workflow_limits.memory.is_some() {
                limits.memory = workflow_limits.memory.clone();
            }
            if workflow_limits.cpu_request.is_some() {
                limits.cpu_request = workflow_limits.cpu_request.clone();
            }
            if workflow_limits.memory_request.is_some() {
                limits.memory_request = workflow_limits.memory_request.clone();
            }
        }

        limits
    }

    pub fn step_resource_limits(
        &self,
        _step: &dyn WorkFlowStep,
        app_defaults: Option<&ResourceLimits>,
    ) -> ResourceLimits {
        self.resolve_resource_limits(app_defaults)
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    pub fn is_parallel(&self) -> bool {
        matches!(self.execution_mode, ExecutionMode::Parallel(_))
    }

    pub fn actual_parallelism(&self) -> usize {
        match &self.execution_mode {
            ExecutionMode::Parallel(n) => *n.min(&self.parallelism),
            ExecutionMode::Sequential => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::steps::traits::ResourceLimitedStep;

    #[derive(Debug, Clone)]
    struct MockStep {
        id: String,
        limits: Option<ResourceLimits>,
    }

    impl MockStep {
        fn new(id: impl Into<String>) -> Self {
            Self {
                id: id.into(),
                limits: None,
            }
        }

        fn with_limits(mut self, limits: ResourceLimits) -> Self {
            self.limits = Some(limits);
            self
        }
    }

    impl WorkFlowStep for MockStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl ResourceLimitedStep for MockStep {
        fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
            self.limits = Some(limits);
            self
        }

        fn resource_limits(&self) -> Option<&ResourceLimits> {
            self.limits.as_ref()
        }
    }

    #[test]
    fn test_workflow_metadata_default() {
        let metadata = WorkflowMetadata::default();
        assert!(metadata.labels.is_empty());
        assert!(metadata.annotations.is_empty());
    }

    #[test]
    fn test_checkpoint_config_default() {
        let config = CheckpointConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.checkpoint_interval_secs, 300);
        assert_eq!(config.retention_count, 10);
        assert!(config.storage_path.is_none());
    }

    #[test]
    fn test_checkpoint_config_builder() {
        let config = CheckpointConfig::new()
            .enabled(true)
            .with_interval_secs(120)
            .with_retention_count(20)
            .with_storage_path("/tmp/checkpoints");

        assert!(config.enabled);
        assert_eq!(config.checkpoint_interval_secs, 120);
        assert_eq!(config.retention_count, 20);
        assert_eq!(config.storage_path, Some("/tmp/checkpoints".to_string()));
    }

    #[test]
    fn test_execution_mode_default() {
        let mode = ExecutionMode::default();
        assert_eq!(mode, ExecutionMode::Sequential);
    }

    #[test]
    fn test_workflow_validate_success() {
        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .add_step(step)
            .build()
            .unwrap();

        assert!(workflow.validate().is_ok());
    }

    #[test]
    fn test_workflow_validate_empty_name() {
        let step = MockStep::new("step-1");

        let workflow = Workflow {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            namespace: "default".to_string(),
            steps: vec![Box::new(step)],
            resource_limits: None,
            checkpoint_config: None,
            metadata: WorkflowMetadata::default(),
            parallelism: 1,
            execution_mode: ExecutionMode::default(),
        };

        assert!(workflow.validate().is_err());
    }

    #[test]
    fn test_workflow_validate_empty_namespace() {
        let step = MockStep::new("step-1");

        let workflow = Workflow {
            id: uuid::Uuid::new_v4().to_string(),
            name: "test-workflow".to_string(),
            namespace: "".to_string(),
            steps: vec![Box::new(step)],
            resource_limits: None,
            checkpoint_config: None,
            metadata: WorkflowMetadata::default(),
            parallelism: 1,
            execution_mode: ExecutionMode::default(),
        };

        assert!(workflow.validate().is_err());
    }

    #[test]
    fn test_workflow_resolve_resource_limits_no_defaults() {
        let workflow_limits = ResourceLimits::new().with_cpu("1000m").with_memory("1Gi");

        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_resource_limits(workflow_limits)
            .add_step(step)
            .build()
            .unwrap();

        let resolved = workflow.resolve_resource_limits(None);
        assert_eq!(resolved.cpu, Some("1000m".to_string()));
        assert_eq!(resolved.memory, Some("1Gi".to_string()));
    }

    #[test]
    fn test_workflow_resolve_resource_limits_with_app_defaults() {
        let app_limits = ResourceLimits::new().with_cpu("500m").with_memory("512Mi");

        let workflow_limits = ResourceLimits::new().with_cpu("1000m").with_memory("1Gi");

        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_resource_limits(workflow_limits)
            .add_step(step)
            .build()
            .unwrap();

        let resolved = workflow.resolve_resource_limits(Some(&app_limits));
        assert_eq!(resolved.cpu, Some("1000m".to_string()));
        assert_eq!(resolved.memory, Some("1Gi".to_string()));
    }

    #[test]
    fn test_workflow_step_resource_limits_no_step_limits() {
        let workflow_limits = ResourceLimits::new().with_cpu("1000m").with_memory("1Gi");

        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_resource_limits(workflow_limits)
            .add_step(step)
            .build()
            .unwrap();

        let step_limits = workflow.step_resource_limits(&*workflow.steps[0], None);
        assert_eq!(step_limits.cpu, Some("1000m".to_string()));
        assert_eq!(step_limits.memory, Some("1Gi".to_string()));
    }

    #[test]
    fn test_workflow_step_resource_limits_with_step_limits() {
        let workflow_limits = ResourceLimits::new().with_cpu("1000m").with_memory("1Gi");

        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_resource_limits(workflow_limits)
            .add_step(step)
            .build()
            .unwrap();

        let resolved_limits = workflow.step_resource_limits(&*workflow.steps[0], None);
        assert_eq!(resolved_limits.cpu, Some("1000m".to_string()));
        assert_eq!(resolved_limits.memory, Some("1Gi".to_string()));
    }

    #[test]
    fn test_workflow_step_resource_limits_priority() {
        let app_limits = ResourceLimits::new().with_cpu("500m").with_memory("512Mi");

        let workflow_limits = ResourceLimits::new().with_cpu("1000m").with_memory("1Gi");

        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_resource_limits(workflow_limits)
            .add_step(step)
            .build()
            .unwrap();

        let resolved_limits = workflow.step_resource_limits(&*workflow.steps[0], Some(&app_limits));
        assert_eq!(resolved_limits.cpu, Some("1000m".to_string()));
        assert_eq!(resolved_limits.memory, Some("1Gi".to_string()));
    }

    #[test]
    fn test_workflow_step_count() {
        let steps = vec![
            MockStep::new("step-1"),
            MockStep::new("step-2"),
            MockStep::new("step-3"),
        ];

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .add_steps(steps)
            .build()
            .unwrap();

        assert_eq!(workflow.step_count(), 3);
    }

    #[test]
    fn test_workflow_is_parallel_sequential() {
        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .add_step(step)
            .build()
            .unwrap();

        assert!(!workflow.is_parallel());
    }

    #[test]
    fn test_workflow_is_parallel() {
        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_execution_mode(ExecutionMode::Parallel(3))
            .add_step(step)
            .build()
            .unwrap();

        assert!(workflow.is_parallel());
    }

    #[test]
    fn test_workflow_actual_parallelism_sequential() {
        let step = MockStep::new("step-1");

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .add_step(step)
            .build()
            .unwrap();

        assert_eq!(workflow.actual_parallelism(), 1);
    }

    #[test]
    fn test_workflow_actual_parallelism_parallel() {
        let steps = vec![
            MockStep::new("step-1"),
            MockStep::new("step-2"),
            MockStep::new("step-3"),
        ];

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_parallelism(4)
            .with_execution_mode(ExecutionMode::Parallel(3))
            .add_steps(steps)
            .build()
            .unwrap();

        assert_eq!(workflow.actual_parallelism(), 3);
    }

    #[test]
    fn test_workflow_actual_parallelism_capped() {
        let steps = vec![
            MockStep::new("step-1"),
            MockStep::new("step-2"),
            MockStep::new("step-3"),
        ];

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_parallelism(2)
            .with_execution_mode(ExecutionMode::Parallel(5))
            .add_steps(steps)
            .build()
            .unwrap();

        assert_eq!(workflow.actual_parallelism(), 2);
    }

    #[test]
    fn test_multi_step_workflow() {
        let steps = vec![
            MockStep::new("build"),
            MockStep::new("test"),
            MockStep::new("deploy"),
        ];

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("ci-cd")
            .with_namespace("production")
            .add_steps(steps)
            .build()
            .unwrap();

        assert_eq!(workflow.step_count(), 3);
        assert_eq!(workflow.steps[0].step_id(), "build");
        assert_eq!(workflow.steps[1].step_id(), "test");
        assert_eq!(workflow.steps[2].step_id(), "deploy");
    }

    #[test]
    fn test_workflow_with_all_configurations() {
        let limits = ResourceLimits::new().with_cpu("2000m").with_memory("2Gi");

        let checkpoint = CheckpointConfig::new()
            .enabled(true)
            .with_interval_secs(60)
            .with_retention_count(5);

        let steps = vec![MockStep::new("step-1"), MockStep::new("step-2")];

        let workflow = crate::workflows::WorkflowBuilder::new()
            .with_name("full-featured")
            .with_namespace("production")
            .with_parallelism(4)
            .with_resource_limits(limits)
            .with_checkpointing(checkpoint)
            .with_label("env", "prod")
            .with_label("team", "engineering")
            .with_annotation("owner", "devops")
            .with_execution_mode(ExecutionMode::Parallel(2))
            .add_steps(steps)
            .build()
            .unwrap();

        assert_eq!(workflow.name, "full-featured");
        assert_eq!(workflow.namespace, "production");
        assert_eq!(workflow.parallelism, 4);
        assert!(workflow.resource_limits.is_some());
        assert!(workflow.checkpoint_config.is_some());
        assert!(workflow.checkpoint_config.as_ref().unwrap().enabled);
        assert_eq!(workflow.metadata.labels.len(), 2);
        assert_eq!(workflow.metadata.annotations.len(), 1);
        assert_eq!(workflow.actual_parallelism(), 2);
    }
}
