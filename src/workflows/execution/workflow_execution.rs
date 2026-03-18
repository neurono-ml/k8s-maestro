use crate::steps::result::StepResult;
use crate::steps::traits::WorkFlowStep;
use crate::workflows::Workflow;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub workflow_id: String,
    pub status: WorkflowStatus,
    pub step_results: BTreeMap<String, StepResult>,
    pub started_at: Option<SystemTime>,
    pub completed_at: Option<SystemTime>,
}

pub struct WorkflowExecution {
    pub workflow_id: String,
    status: Arc<tokio::sync::RwLock<WorkflowStatus>>,
    step_results: Arc<tokio::sync::RwLock<BTreeMap<String, StepResult>>>,
    started_at: SystemTime,
    completed_at: Arc<tokio::sync::RwLock<Option<SystemTime>>>,
    error: Arc<tokio::sync::RwLock<Option<anyhow::Error>>>,
    workflow: Arc<Workflow>,
}

impl WorkflowExecution {
    pub fn new(workflow: Arc<Workflow>) -> Self {
        let workflow_id = workflow.id.clone();
        Self {
            workflow_id,
            status: Arc::new(tokio::sync::RwLock::new(WorkflowStatus::Pending)),
            step_results: Arc::new(tokio::sync::RwLock::new(BTreeMap::new())),
            started_at: SystemTime::now(),
            completed_at: Arc::new(tokio::sync::RwLock::new(None)),
            error: Arc::new(tokio::sync::RwLock::new(None)),
            workflow,
        }
    }

    pub async fn wait(&self) -> anyhow::Result<()> {
        loop {
            let status = *self.status.read().await;
            match status {
                WorkflowStatus::Succeeded => return Ok(()),
                WorkflowStatus::Failed => {
                    if let Some(err) = self.error.read().await.as_ref() {
                        anyhow::bail!("{}", err);
                    }
                    anyhow::bail!("Workflow failed");
                }
                WorkflowStatus::Cancelled => anyhow::bail!("Workflow cancelled"),
                _ => tokio::time::sleep(tokio::time::Duration::from_millis(100)).await,
            }
        }
    }

    pub async fn cancel(&self) -> anyhow::Result<()> {
        let mut status = self.status.write().await;
        if *status != WorkflowStatus::Running {
            anyhow::bail!("Cannot cancel workflow in {:?} state", *status);
        }
        *status = WorkflowStatus::Cancelled;

        let mut completed_at = self.completed_at.write().await;
        *completed_at = Some(SystemTime::now());

        Ok(())
    }

    pub async fn get_status(&self) -> WorkflowStatus {
        *self.status.read().await
    }

    pub async fn get_step_result(&self, step_id: &str) -> Option<StepResult> {
        self.step_results.read().await.get(step_id).cloned()
    }

    pub async fn delete(&self, dry_run: bool) -> anyhow::Result<()> {
        for step in &self.workflow.steps {
            if let Some(kube_step) = step.as_any().downcast_ref::<crate::steps::kubernetes::KubeJobStep>() {
                kube_step.delete_workflow(dry_run).await?;
            } else if let Some(kube_step) = step.as_any().downcast_ref::<crate::steps::kubernetes::KubePodStep>() {
                kube_step.delete_workflow(dry_run).await?;
            }
        }
        Ok(())
    }

    pub async fn get_checkpoint(&self) -> Option<Checkpoint> {
        Some(Checkpoint {
            workflow_id: self.workflow_id.clone(),
            status: *self.status.read().await,
            step_results: self.step_results.read().await.clone(),
            started_at: Some(self.started_at),
            completed_at: *self.completed_at.read().await,
        })
    }

    pub(crate) async fn set_status(&self, status: WorkflowStatus) {
        let mut s = self.status.write().await;
        *s = status;
        if status == WorkflowStatus::Succeeded || status == WorkflowStatus::Failed {
            let mut completed_at = self.completed_at.write().await;
            *completed_at = Some(SystemTime::now());
        }
    }

    pub(crate) async fn set_error(&self, error: anyhow::Error) {
        let mut err = self.error.write().await;
        *err = Some(error);
    }

    pub(crate) async fn set_step_result(&self, step_id: String, result: StepResult) {
        let mut results = self.step_results.write().await;
        results.insert(step_id, result);
    }

    pub(crate) async fn get_all_step_results(&self) -> BTreeMap<String, StepResult> {
        self.step_results.read().await.clone()
    }

    pub(crate) fn get_steps(&self) -> &Vec<Box<dyn WorkFlowStep>> {
        &self.workflow.steps
    }

    pub(crate) fn get_namespace(&self) -> &str {
        &self.workflow.namespace
    }
}
