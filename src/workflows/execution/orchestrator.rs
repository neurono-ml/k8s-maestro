use crate::clients::MaestroK8sClient;
use crate::steps::result::StepResult;
use crate::steps::traits::WorkFlowStep;
use crate::workflows::dependency::{ConditionFn, DependencyGraph};
use crate::workflows::execution::executor::StepExecutor;
use crate::workflows::execution::scheduler::{Scheduler, FailureStrategy};
use crate::workflows::execution::workflow_execution::{WorkflowExecution, WorkflowStatus};
use crate::workflows::Workflow;
use anyhow::Result;
use std::sync::Arc;

pub struct WorkflowOrchestrator {
    workflow: Arc<Workflow>,
    executor: StepExecutor,
    scheduler: Scheduler,
    dependency_graph: DependencyGraph,
}

impl WorkflowOrchestrator {
    pub fn new(workflow: Workflow, client: MaestroK8sClient) -> Result<Self> {
        workflow.validate()?;

        let executor = StepExecutor::new(client);

        let parallelism = workflow.actual_parallelism();
        let scheduler = Scheduler::new(parallelism, FailureStrategy::Stop);

        let dependency_graph = Self::build_dependency_graph(&workflow);

        let workflow = Arc::new(workflow);

        Ok(Self {
            workflow,
            executor,
            scheduler,
            dependency_graph,
        })
    }

    fn build_dependency_graph(workflow: &Workflow) -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        for step in &workflow.steps {
            graph.add_node(step.step_id().to_string());
        }
        graph
    }

    pub async fn execute(&self) -> Result<WorkflowExecution> {
        let execution = WorkflowExecution::new(self.workflow.clone());

        self.dependency_graph.detect_cycles()?;

        execution.set_status(WorkflowStatus::Running).await;

        let execution_result = self.run_execution_loop(&execution).await;

        match execution_result {
            Ok(_) => {
                execution.set_status(WorkflowStatus::Succeeded).await;
            }
            Err(e) => {
                execution.set_error(e).await;
                execution.set_status(WorkflowStatus::Failed).await;
            }
        }

        Ok(execution)
    }

    async fn run_execution_loop(&self, execution: &WorkflowExecution) -> Result<()> {
        let mut completed_steps: Vec<String> = Vec::new();
        let steps = execution.get_steps();

        loop {
            let executable_steps = self.get_next_executable_steps(steps, &completed_steps).await?;

            if executable_steps.is_empty() {
                if completed_steps.len() == steps.len() {
                    break;
                }
                anyhow::bail!("Workflow stuck - no executable steps but not all completed");
            }

            let results = self.execute_steps(execution, executable_steps).await?;

            for result in results {
                completed_steps.push(result.step_id.clone());
                execution.set_step_result(result.step_id.clone(), result.clone()).await;
            }
        }

        Ok(())
    }

    async fn execute_steps(
        &self,
        execution: &WorkflowExecution,
        step_ids: Vec<String>,
    ) -> Result<Vec<StepResult>> {
        let steps = execution.get_steps();
        let mut results = Vec::new();

        for step_id in step_ids {
            if let Some(step) = steps.iter().find(|s| s.step_id() == step_id) {
                let result = self.execute_step_internal(step.as_ref()).await?;
                results.push(result);
            }
        }

        self.scheduler.handle_failure(&results)?;

        Ok(results)
    }

    async fn execute_step_internal(
        &self,
        step: &dyn WorkFlowStep,
    ) -> Result<StepResult> {
        let result = self.executor.execute_step(step).await?;
        Ok(result)
    }

    pub async fn execute_step(&self, execution: &WorkflowExecution, step_id: &str) -> Result<StepResult> {
        let steps = execution.get_steps();
        let step = steps
            .iter()
            .find(|s| s.step_id() == step_id)
            .ok_or_else(|| anyhow::anyhow!("Step not found: {}", step_id))?;

        let dependencies = self.dependency_graph.get_dependencies(step_id);
        let step_results = execution.get_all_step_results().await;

        for dep in dependencies {
            if !step_results.contains_key(&dep) {
                anyhow::bail!("Dependency {} not satisfied", dep);
            }
        }

        let result = self.executor.execute_step(step.as_ref()).await?;
        execution.set_step_result(step_id.to_string(), result.clone()).await;

        Ok(result)
    }

    pub fn evaluate_condition(&self, condition: &ConditionFn, step_results: &Vec<StepResult>) -> bool {
        condition(step_results)
    }

    pub async fn get_next_executable_steps(
        &self,
        steps: &Vec<Box<dyn WorkFlowStep>>,
        completed_steps: &Vec<String>,
    ) -> Result<Vec<String>> {
        let mut executable = Vec::new();

        for step in steps {
            let step_id = step.step_id();

            if completed_steps.contains(&step_id.to_string()) {
                continue;
            }

            let dependencies = self.dependency_graph.get_dependencies(step_id);
            let all_deps_satisfied = self.scheduler.check_dependencies_satisfied(
                step_id,
                completed_steps,
                &dependencies,
            );

            if all_deps_satisfied {
                executable.push(step_id.to_string());
            }
        }

        Ok(executable)
    }

    pub async fn mark_step_complete(
        &self,
        execution: &WorkflowExecution,
        step_id: String,
        result: StepResult,
    ) -> Result<()> {
        execution.set_step_result(step_id, result).await;
        Ok(())
    }

    pub fn detect_cycles(&self) -> Result<()> {
        self.dependency_graph.detect_cycles()
    }

    pub fn get_dependency_graph(&self) -> &DependencyGraph {
        &self.dependency_graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires cluster"]
    async fn test_orchestrator_creation() {
        let workflow = Workflow::builder()
            .with_name("test-workflow")
            .with_namespace("default")
            .build()
            .unwrap();

        let client = MaestroK8sClient::new().await.unwrap();
        let orchestrator = WorkflowOrchestrator::new(workflow, client);
        assert!(orchestrator.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requires cluster"]
    async fn test_execute_workflow() {
        let workflow = Workflow::builder()
            .with_name("test-workflow")
            .with_namespace("default")
            .build()
            .unwrap();

        let client = MaestroK8sClient::new().await.unwrap();
        let orchestrator = WorkflowOrchestrator::new(workflow, client).unwrap();

        let execution = orchestrator.execute().await;
        assert!(execution.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requires cluster"]
    async fn test_cycle_detection() {
        let workflow = Workflow::builder()
            .with_name("test-workflow")
            .with_namespace("default")
            .build()
            .unwrap();

        let client = MaestroK8sClient::new().await.unwrap();
        let orchestrator = WorkflowOrchestrator::new(workflow, client).unwrap();

        let result = orchestrator.detect_cycles();
        assert!(result.is_ok());
    }
}
