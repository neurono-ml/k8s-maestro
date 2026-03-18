use crate::clients::MaestroK8sClient;
use crate::steps::kubernetes::{KubeJobStep, KubePodStep};
use crate::steps::result::StepResult;
use crate::steps::traits::{WorkFlowStep, KubeWorkFlowStep};
use anyhow::Result;
use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::core::v1::Pod;
use kube::Api;

pub struct StepExecutor {
    client: MaestroK8sClient,
}

impl StepExecutor {
    pub fn new(client: MaestroK8sClient) -> Self {
        Self { client }
    }

    pub async fn execute_step(&self, step: &dyn WorkFlowStep) -> Result<StepResult> {
        if let Some(kube_job) = step.as_any().downcast_ref::<KubeJobStep>() {
            self.execute_kube_step(kube_job).await
        } else if let Some(kube_pod) = step.as_any().downcast_ref::<KubePodStep>() {
            self.execute_pod_step(kube_pod).await
        } else {
            anyhow::bail!("Unsupported step type: {}", step.step_id())
        }
    }

    pub async fn execute_kube_step(&self, step: &KubeJobStep) -> Result<StepResult> {
        let step_id = step.step_id().to_string();
        let namespace = step.namespace().to_string();
        let resource_name = step.resource_name().to_string();

        let client = self.client.inner().clone();
        let jobs: Api<Job> = Api::namespaced(client, &namespace);

        let job = jobs.get(&resource_name).await.map_err(|e| {
            anyhow::anyhow!("Failed to get job {}: {}", resource_name, e)
        })?;

        let status = job
            .status
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Job has no status"))?;

        let succeeded = status.succeeded.unwrap_or(0);
        let failed = status.failed.unwrap_or(0);

        let step_result = StepResult::new(&step_id)
            .with_status(if failed > 0 {
                crate::steps::result::StepStatus::Failure
            } else {
                crate::steps::result::StepStatus::Success
            })
            .with_exit_code(if failed > 0 { 1 } else { 0 })
            .with_output("succeeded", serde_json::json!(succeeded))
            .with_output("failed", serde_json::json!(failed));

        Ok(step_result)
    }

    pub async fn execute_pod_step(&self, step: &KubePodStep) -> Result<StepResult> {
        let step_id = step.step_id().to_string();
        let namespace = step.namespace().to_string();
        let resource_name = step.resource_name().to_string();

        let client = self.client.inner().clone();
        let pods: Api<Pod> = Api::namespaced(client, &namespace);

        let pod = pods.get(&resource_name).await.map_err(|e| {
            anyhow::anyhow!("Failed to get pod {}: {}", resource_name, e)
        })?;

        let phase = pod
            .status
            .as_ref()
            .and_then(|s| s.phase.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Pod has no phase"))?;

        let step_result = StepResult::new(&step_id)
            .with_status(match phase.as_str() {
                "Succeeded" => crate::steps::result::StepStatus::Success,
                "Failed" => crate::steps::result::StepStatus::Failure,
                _ => crate::steps::result::StepStatus::Failure,
            })
            .with_exit_code(if phase == "Succeeded" { 0 } else { 1 })
            .with_output("phase", serde_json::json!(phase));

        Ok(step_result)
    }
}
