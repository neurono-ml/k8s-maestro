use crate::steps::traits::{ResourceLimits, WorkFlowStep};
use anyhow::Result;

use super::{ExecutionMode, LegacyCheckpointConfig, Workflow, WorkflowMetadata};

pub struct WorkflowBuilder {
    name: Option<String>,
    namespace: Option<String>,
    steps: Vec<Box<dyn WorkFlowStep>>,
    resource_limits: Option<ResourceLimits>,
    checkpoint_config: Option<LegacyCheckpointConfig>,
    metadata: WorkflowMetadata,
    parallelism: usize,
    execution_mode: ExecutionMode,
}

impl Default for WorkflowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            namespace: Some("default".to_string()),
            steps: Vec::new(),
            resource_limits: None,
            checkpoint_config: None,
            metadata: WorkflowMetadata::default(),
            parallelism: 1,
            execution_mode: ExecutionMode::default(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_parallelism(mut self, parallelism: usize) -> Self {
        self.parallelism = parallelism;
        self
    }

    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = Some(limits);
        self
    }

    pub fn add_step(mut self, step: impl WorkFlowStep + 'static) -> Self {
        self.steps.push(Box::new(step));
        self
    }

    pub fn add_steps(mut self, steps: Vec<impl WorkFlowStep + 'static>) -> Self {
        for step in steps {
            self.steps.push(Box::new(step));
        }
        self
    }

    pub fn with_checkpointing(mut self, config: LegacyCheckpointConfig) -> Self {
        self.checkpoint_config = Some(config);
        self
    }

    pub fn with_execution_mode(mut self, mode: ExecutionMode) -> Self {
        self.execution_mode = mode;
        self
    }

    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.labels.insert(key.into(), value.into());
        self
    }

    pub fn with_annotation(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.annotations.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<Workflow> {
        let name = self
            .name
            .ok_or_else(|| anyhow::anyhow!("Workflow name is required"))?;
        let namespace = self.namespace.unwrap_or_else(|| "default".to_string());

        if self.steps.is_empty() {
            anyhow::bail!("Workflow must have at least one step");
        }

        if self.parallelism == 0 {
            anyhow::bail!("Workflow parallelism must be greater than 0");
        }

        let workflow = Workflow {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            namespace,
            steps: self.steps,
            resource_limits: self.resource_limits,
            checkpoint_config: self.checkpoint_config,
            metadata: self.metadata,
            parallelism: self.parallelism,
            execution_mode: self.execution_mode,
        };

        workflow.validate()?;
        Ok(workflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct MockStep {
        id: String,
    }

    impl MockStep {
        fn new(id: impl Into<String>) -> Self {
            Self { id: id.into() }
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

    #[test]
    fn test_builder_new() {
        let builder = WorkflowBuilder::new();
        assert_eq!(builder.namespace, Some("default".to_string()));
        assert_eq!(builder.parallelism, 1);
    }

    #[test]
    fn test_builder_default() {
        let builder = WorkflowBuilder::default();
        assert_eq!(builder.namespace, Some("default".to_string()));
        assert_eq!(builder.parallelism, 1);
    }

    #[test]
    fn test_builder_with_name() {
        let builder = WorkflowBuilder::new().with_name("test-workflow");
        assert_eq!(builder.name, Some("test-workflow".to_string()));
    }

    #[test]
    fn test_builder_with_namespace() {
        let builder = WorkflowBuilder::new().with_namespace("production");
        assert_eq!(builder.namespace, Some("production".to_string()));
    }

    #[test]
    fn test_builder_with_parallelism() {
        let builder = WorkflowBuilder::new().with_parallelism(4);
        assert_eq!(builder.parallelism, 4);
    }

    #[test]
    fn test_builder_with_resource_limits() {
        let limits = ResourceLimits::new().with_cpu("500m").with_memory("512Mi");

        let builder = WorkflowBuilder::new().with_resource_limits(limits);
        assert!(builder.resource_limits.is_some());
        assert_eq!(
            builder.resource_limits.as_ref().unwrap().cpu,
            Some("500m".to_string())
        );
    }

    #[test]
    fn test_builder_add_step() {
        let step = MockStep::new("step-1");
        let builder = WorkflowBuilder::new().add_step(step);
        assert_eq!(builder.steps.len(), 1);
    }

    #[test]
    fn test_builder_add_steps() {
        let steps = vec![
            MockStep::new("step-1"),
            MockStep::new("step-2"),
            MockStep::new("step-3"),
        ];

        let builder = WorkflowBuilder::new().add_steps(steps);
        assert_eq!(builder.steps.len(), 3);
    }

    #[test]
    fn test_builder_with_checkpointing() {
        use super::super::LegacyCheckpointConfig;

        let config = LegacyCheckpointConfig::new()
            .enabled(true)
            .with_interval_secs(60);

        let builder = WorkflowBuilder::new().with_checkpointing(config);
        assert!(builder.checkpoint_config.is_some());
        assert!(builder.checkpoint_config.unwrap().enabled);
    }

    #[test]
    fn test_builder_with_execution_mode() {
        use super::super::ExecutionMode;

        let builder = WorkflowBuilder::new().with_execution_mode(ExecutionMode::Parallel(3));
        assert_eq!(builder.execution_mode, ExecutionMode::Parallel(3));
    }

    #[test]
    fn test_builder_with_labels() {
        let builder = WorkflowBuilder::new()
            .with_label("env", "production")
            .with_label("team", "platform");

        assert_eq!(builder.metadata.labels.len(), 2);
        assert_eq!(
            builder.metadata.labels.get("env"),
            Some(&"production".to_string())
        );
    }

    #[test]
    fn test_builder_with_annotations() {
        let builder = WorkflowBuilder::new()
            .with_annotation("description", "Test workflow")
            .with_annotation("owner", "devops");

        assert_eq!(builder.metadata.annotations.len(), 2);
        assert_eq!(
            builder.metadata.annotations.get("description"),
            Some(&"Test workflow".to_string())
        );
    }

    #[test]
    fn test_builder_build_success() {
        let step = MockStep::new("step-1");

        let workflow = WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .add_step(step)
            .build()
            .unwrap();

        assert_eq!(workflow.name, "test-workflow");
        assert_eq!(workflow.namespace, "default");
        assert_eq!(workflow.step_count(), 1);
        assert!(!workflow.id.is_empty());
    }

    #[test]
    fn test_builder_build_no_name() {
        let step = MockStep::new("step-1");

        let result = WorkflowBuilder::new()
            .with_namespace("default")
            .add_step(step)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name is required"));
    }

    #[test]
    fn test_builder_build_no_steps() {
        let result = WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must have at least one step"));
    }

    #[test]
    fn test_builder_build_zero_parallelism() {
        let step = MockStep::new("step-1");

        let result = WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .with_parallelism(0)
            .add_step(step)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("parallelism must be greater than 0"));
    }

    #[test]
    fn test_builder_fluent_api() {
        let limits = ResourceLimits::new().with_cpu("1000m").with_memory("1Gi");

        let checkpoint = super::super::LegacyCheckpointConfig::new()
            .enabled(true)
            .with_interval_secs(60);

        let steps = vec![MockStep::new("step-1"), MockStep::new("step-2")];

        let workflow = WorkflowBuilder::new()
            .with_name("fluent-test")
            .with_namespace("production")
            .with_parallelism(2)
            .with_resource_limits(limits)
            .with_checkpointing(checkpoint)
            .with_label("env", "prod")
            .with_annotation("owner", "devops")
            .with_execution_mode(super::super::ExecutionMode::Parallel(2))
            .add_steps(steps)
            .build()
            .unwrap();

        assert_eq!(workflow.name, "fluent-test");
        assert_eq!(workflow.parallelism, 2);
        assert!(workflow.resource_limits.is_some());
        assert!(workflow.checkpoint_config.is_some());
        assert_eq!(workflow.metadata.labels.len(), 1);
        assert_eq!(workflow.metadata.annotations.len(), 1);
        assert_eq!(workflow.step_count(), 2);
    }
}
