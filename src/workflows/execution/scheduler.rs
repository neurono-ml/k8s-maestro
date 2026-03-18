use crate::steps::result::StepResult;
use crate::steps::traits::WorkFlowStep;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureStrategy {
    Stop,
    Continue,
}

pub struct Scheduler {
    parallelism: usize,
    failure_strategy: FailureStrategy,
}

impl Scheduler {
    pub fn new(parallelism: usize, failure_strategy: FailureStrategy) -> Self {
        Self {
            parallelism,
            failure_strategy,
        }
    }

    pub async fn schedule_steps<F, Fut>(
        &self,
        step_ids: Vec<String>,
        execute_step: F,
    ) -> anyhow::Result<Vec<StepResult>>
    where
        F: Fn(String) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = anyhow::Result<StepResult>> + Send + 'static,
    {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.parallelism));
        let mut results = Vec::new();
        let mut handles = Vec::new();

        for step_id in step_ids {
            let semaphore = semaphore.clone();
            let execute_step = execute_step.clone();

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await?;
                execute_step(step_id).await
            });

            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await??;
            results.push(result);
        }

        Ok(results)
    }

    pub fn check_dependencies_satisfied(
        &self,
        step_id: &str,
        completed_steps: &Vec<String>,
        dependencies: &Vec<String>,
    ) -> bool {
        for dep in dependencies {
            if !completed_steps.contains(dep) {
                return false;
            }
        }
        true
    }

    pub fn handle_failure(&self, results: &Vec<StepResult>) -> anyhow::Result<()> {
        if self.failure_strategy == FailureStrategy::Stop {
            for result in results {
                if result.is_failure() {
                    anyhow::bail!("Step {} failed", result.step_id);
                }
            }
        }
        Ok(())
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new(1, FailureStrategy::Stop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = Scheduler::new(4, FailureStrategy::Continue);
        assert_eq!(scheduler.parallelism, 4);
        assert_eq!(scheduler.failure_strategy, FailureStrategy::Continue);
    }

    #[test]
    fn test_scheduler_default() {
        let scheduler = Scheduler::default();
        assert_eq!(scheduler.parallelism, 1);
        assert_eq!(scheduler.failure_strategy, FailureStrategy::Stop);
    }

    #[test]
    fn test_check_dependencies_satisfied() {
        let scheduler = Scheduler::default();
        let completed_steps = vec!["A".to_string(), "B".to_string()];
        let dependencies = vec!["A".to_string()];

        assert!(scheduler.check_dependencies_satisfied("C", &completed_steps, &dependencies));

        let dependencies = vec!["A".to_string(), "D".to_string()];
        assert!(!scheduler.check_dependencies_satisfied("C", &completed_steps, &dependencies));
    }

    #[test]
    fn test_handle_failure_stop() {
        let scheduler = Scheduler::new(1, FailureStrategy::Stop);
        let results = vec![
            StepResult::new("A").with_status(crate::steps::result::StepStatus::Success),
            StepResult::new("B").with_status(crate::steps::result::StepStatus::Failure),
        ];

        assert!(scheduler.handle_failure(&results).is_err());
    }

    #[test]
    fn test_handle_failure_continue() {
        let scheduler = Scheduler::new(1, FailureStrategy::Continue);
        let results = vec![
            StepResult::new("A").with_status(crate::steps::result::StepStatus::Success),
            StepResult::new("B").with_status(crate::steps::result::StepStatus::Failure),
        ];

        assert!(scheduler.handle_failure(&results).is_ok());
    }

    #[tokio::test]
    async fn test_schedule_steps() {
        let scheduler = Scheduler::new(2, FailureStrategy::Stop);

        let execute_step = |step_id: String| async move {
            Ok(StepResult::new(&step_id)
                .with_status(crate::steps::result::StepStatus::Success)
                .with_exit_code(0))
        };

        let step_ids = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let results = scheduler.schedule_steps(step_ids, execute_step).await.unwrap();

        assert_eq!(results.len(), 3);
        for result in results {
            assert!(result.is_success());
        }
    }
}
