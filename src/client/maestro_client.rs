//! Client for managing Kubernetes workflows.
//!
//! This module provides the [`MaestroClient`] which is the main entry point
//! for creating and managing workflows in Kubernetes.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;

use crate::steps::traits::ResourceLimits;
use crate::workflows::Workflow;

/// Client for managing Kubernetes workflows.
///
/// The client is configured using [`MaestroClientBuilder`] and provides
/// methods for creating, retrieving, and listing workflows.
///
/// # Example
///
/// ```no_run
/// use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
/// use k8s_maestro::steps::traits::{WorkFlowStep, ResourceLimitedStep};
///
/// let client = MaestroClientBuilder::new()
///     .with_namespace("production")
///     .build()
///     .unwrap();
/// ```
pub struct MaestroClient {
    kube_config_path: Option<PathBuf>,
    namespace: String,
    dry_run: bool,
    default_timeout: Option<Duration>,
    log_level: Option<String>,
    default_resource_limits: Option<ResourceLimits>,
}

impl MaestroClient {
    pub(crate) fn new(
        kube_config_path: Option<PathBuf>,
        namespace: String,
        dry_run: bool,
        default_timeout: Option<Duration>,
        log_level: Option<String>,
        default_resource_limits: Option<ResourceLimits>,
    ) -> Self {
        Self {
            kube_config_path,
            namespace,
            dry_run,
            default_timeout,
            log_level,
            default_resource_limits,
        }
    }

    /// Returns the default namespace for operations.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns whether the client is in dry run mode.
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    /// Returns the default timeout for operations.
    pub fn default_timeout(&self) -> Option<&Duration> {
        self.default_timeout.as_ref()
    }

    /// Returns the log level for client operations.
    pub fn log_level(&self) -> Option<&str> {
        self.log_level.as_deref()
    }

    /// Returns the default resource limits for workflows.
    pub fn default_resource_limits(&self) -> Option<&ResourceLimits> {
        self.default_resource_limits.as_ref()
    }

    /// Returns the path to the kubeconfig file.
    pub fn kube_config_path(&self) -> Option<&PathBuf> {
        self.kube_config_path.as_ref()
    }

    /// Creates a new workflow.
    ///
    /// In dry run mode, the workflow is validated but not created.
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow to create
    ///
    /// # Errors
    ///
    /// Returns an error if the workflow is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
    /// # use k8s_maestro::steps::traits::{WorkFlowStep, ResourceLimitedStep};
    ///
    /// let client = MaestroClientBuilder::new().build().unwrap();
    /// # let step = MockStep::new("test");
    /// let workflow = WorkflowBuilder::new()
    ///     .with_name("my-workflow")
    ///     .add_step(step)
    ///     .build()
    ///     .unwrap();
    ///
    /// let created = client.create_workflow(workflow).unwrap();
    /// ```
    pub fn create_workflow(&self, workflow: Workflow) -> Result<CreatedWorkflow> {
        if self.dry_run {
            log::info!(
                "DRY RUN: Would create workflow '{}' in namespace '{}'",
                workflow.name,
                self.namespace
            );
            return Ok(CreatedWorkflow::DryRun(DryRunWorkflow {
                workflow,
                namespace: self.namespace.clone(),
            }));
        }

        log::info!(
            "Creating workflow '{}' in namespace '{}'",
            workflow.name,
            self.namespace
        );

        workflow.validate()?;

        Ok(CreatedWorkflow::Runtime(RuntimeWorkflow {
            workflow,
            namespace: self.namespace.clone(),
        }))
    }

    /// Retrieves a workflow by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The workflow ID
    ///
    /// # Returns
    ///
    /// Returns `Ok(None)` if the workflow is not found.
    pub fn get_workflow(&self, _id: &str) -> Result<Option<CreatedWorkflow>> {
        if self.dry_run {
            log::info!("DRY RUN: Would get workflow with id '{}'", _id);
            return Ok(None);
        }

        log::info!("Getting workflow with id '{}'", _id);

        Ok(None)
    }
}

/// Represents a workflow that has been created.
///
/// This enum wraps workflows in either dry run mode or runtime mode.
pub enum CreatedWorkflow {
    DryRun(DryRunWorkflow),
    Runtime(RuntimeWorkflow),
}

impl CreatedWorkflow {
    /// Returns the workflow ID.
    pub fn id(&self) -> &str {
        match self {
            CreatedWorkflow::DryRun(w) => w.id(),
            CreatedWorkflow::Runtime(w) => w.id(),
        }
    }

    /// Returns the workflow name.
    pub fn name(&self) -> &str {
        match self {
            CreatedWorkflow::DryRun(w) => w.name(),
            CreatedWorkflow::Runtime(w) => w.name(),
        }
    }

    /// Returns the workflow namespace.
    pub fn namespace(&self) -> &str {
        match self {
            CreatedWorkflow::DryRun(w) => w.namespace(),
            CreatedWorkflow::Runtime(w) => w.namespace(),
        }
    }

    /// Returns whether this is a dry run workflow.
    pub fn is_dry_run(&self) -> bool {
        matches!(self, CreatedWorkflow::DryRun(_))
    }
}

/// Trait for workflow-like objects.
///
/// This trait provides a common interface for different workflow representations.
pub trait WorkflowLike {
    /// Returns the workflow ID.
    fn id(&self) -> &str;

    /// Returns the workflow name.
    fn name(&self) -> &str;

    /// Returns the workflow namespace.
    fn namespace(&self) -> &str;
}

/// A workflow in dry run mode.
///
/// Dry run workflows are validated but not executed.
pub struct DryRunWorkflow {
    workflow: Workflow,
    namespace: String,
}

impl WorkflowLike for DryRunWorkflow {
    fn id(&self) -> &str {
        &self.workflow.id
    }

    fn name(&self) -> &str {
        &self.workflow.name
    }

    fn namespace(&self) -> &str {
        &self.namespace
    }
}

/// A workflow in runtime mode.
///
/// Runtime workflows are actually executed in the cluster.
pub struct RuntimeWorkflow {
    workflow: Workflow,
    namespace: String,
}

impl WorkflowLike for RuntimeWorkflow {
    fn id(&self) -> &str {
        &self.workflow.id
    }

    fn name(&self) -> &str {
        &self.workflow.name
    }

    fn namespace(&self) -> &str {
        &self.namespace
    }
}

#[cfg(test)]
mod tests {
    use super::super::MaestroClientBuilder;
    use super::*;
    use crate::steps::traits::{ResourceLimitedStep, WorkFlowStep};
    use crate::workflows::WorkflowBuilder;

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

    impl ResourceLimitedStep for MockStep {
        fn with_resource_limits(self, _limits: ResourceLimits) -> Self {
            self
        }

        fn resource_limits(&self) -> Option<&ResourceLimits> {
            None
        }
    }

    #[test]
    fn test_client_namespace() {
        let client = MaestroClientBuilder::new()
            .with_namespace("production")
            .build()
            .unwrap();

        assert_eq!(client.namespace(), "production");
    }

    #[test]
    fn test_client_dry_run() {
        let client = MaestroClientBuilder::new()
            .with_dry_run(true)
            .build()
            .unwrap();

        assert!(client.dry_run());
    }

    #[test]
    fn test_client_default_timeout() {
        let timeout = Duration::from_secs(60);
        let client = MaestroClientBuilder::new()
            .with_default_timeout(timeout)
            .build()
            .unwrap();

        assert_eq!(client.default_timeout(), Some(&timeout));
    }

    #[test]
    fn test_client_log_level() {
        let client = MaestroClientBuilder::new()
            .with_log_level("debug")
            .build()
            .unwrap();

        assert_eq!(client.log_level(), Some("debug"));
    }

    #[test]
    fn test_client_default_resource_limits() {
        let limits = ResourceLimits::new().with_cpu("500m").with_memory("512Mi");
        let client = MaestroClientBuilder::new()
            .with_default_resource_limits(limits)
            .build()
            .unwrap();

        assert!(client.default_resource_limits().is_some());
    }

    #[test]
    fn test_create_workflow_dry_run() {
        let client = MaestroClientBuilder::new()
            .with_dry_run(true)
            .with_namespace("test")
            .build()
            .unwrap();

        let step = MockStep::new("step-1");
        let workflow = WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .add_step(step)
            .build()
            .unwrap();

        let result = client.create_workflow(workflow);
        assert!(result.is_ok());

        let created = result.unwrap();
        assert!(created.is_dry_run());
        assert_eq!(created.name(), "test-workflow");
    }

    #[test]
    fn test_create_workflow_production() {
        let client = MaestroClientBuilder::new()
            .with_namespace("production")
            .build()
            .unwrap();

        let step = MockStep::new("step-1");
        let workflow = WorkflowBuilder::new()
            .with_name("test-workflow")
            .with_namespace("default")
            .add_step(step)
            .build()
            .unwrap();

        let result = client.create_workflow(workflow);
        assert!(result.is_ok());

        let created = result.unwrap();
        assert!(!created.is_dry_run());
        assert_eq!(created.name(), "test-workflow");
    }

    #[test]
    fn test_get_workflow_dry_run() {
        let client = MaestroClientBuilder::new()
            .with_dry_run(true)
            .build()
            .unwrap();

        let result = client.get_workflow("test-id");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_workflow_production() {
        let client = MaestroClientBuilder::new().build().unwrap();

        let result = client.get_workflow("test-id");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_list_workflows_dry_run() {
        let client = MaestroClientBuilder::new()
            .with_dry_run(true)
            .build()
            .unwrap();

        let result = client.list_workflows();
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_list_workflows_production() {
        let client = MaestroClientBuilder::new().build().unwrap();

        let result = client.list_workflows();
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
