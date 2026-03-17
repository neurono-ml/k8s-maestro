use crate::steps::result::StepResult;
use futures::Stream;
use serde_json::Value;
use std::any::Any;
use std::pin::Pin;

pub trait WorkFlowStep: Send + Sync + Any {
    fn step_id(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub trait KubeWorkFlowStep: WorkFlowStep {
    fn namespace(&self) -> &str;
    fn resource_name(&self) -> &str;
}

pub trait ExecutableWorkFlowStep: WorkFlowStep {
    fn execute(&self) -> anyhow::Result<StepResult>;
    fn cancel(&self) -> anyhow::Result<()>;
}

pub trait WaitableWorkFlowStep: WorkFlowStep {
    fn wait(&self) -> impl std::future::Future<Output = anyhow::Result<StepResult>> + Send;
}

pub trait DeletableWorkFlowStep: WorkFlowStep {
    fn delete_workflow(
        &self,
        dry_run: bool,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn delete_associated_pods(
        &self,
        dry_run: bool,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

#[derive(Debug, Clone, Default)]
pub struct LogStreamOptions {
    pub follow: bool,
    pub tail_lines: Option<i64>,
    pub since_seconds: Option<i64>,
    pub timestamps: bool,
    pub limit_bytes: Option<i64>,
}

pub trait LoggableWorkFlowStep: WorkFlowStep {
    fn stream_logs(
        &self,
        options: LogStreamOptions,
    ) -> Pin<Box<dyn Stream<Item = anyhow::Result<String>> + Send + '_>>;
}

pub trait ServableWorkFlowStep: WorkFlowStep {
    fn expose_service(
        &self,
        service_name: &str,
        port: u16,
    ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
    fn expose_ingress(
        &self,
        ingress_name: &str,
        host: &str,
        service_port: u16,
    ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
}

pub type ExecutionCondition = dyn Fn(&dyn WorkFlowStep) -> bool + Send + Sync;

pub trait ConditionalStep: WorkFlowStep {
    fn with_execution_condition<F>(self, condition: F) -> Self
    where
        F: Fn(&dyn WorkFlowStep) -> bool + Send + Sync + 'static;

    fn should_execute(&self) -> bool;
}

pub trait StepWithOutputs: WorkFlowStep {
    fn get_step_result(&self) -> StepResult;
    fn get_output(&self, key: &str) -> Option<Value>;
}

#[derive(Debug, Clone, Default)]
pub struct ResourceLimits {
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub cpu_request: Option<String>,
    pub memory_request: Option<String>,
    pub ephemeral_storage: Option<String>,
    pub ephemeral_storage_request: Option<String>,
}

impl ResourceLimits {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cpu(mut self, cpu: impl Into<String>) -> Self {
        self.cpu = Some(cpu.into());
        self
    }

    pub fn with_memory(mut self, memory: impl Into<String>) -> Self {
        self.memory = Some(memory.into());
        self
    }

    pub fn with_cpu_request(mut self, cpu_request: impl Into<String>) -> Self {
        self.cpu_request = Some(cpu_request.into());
        self
    }

    pub fn with_memory_request(mut self, memory_request: impl Into<String>) -> Self {
        self.memory_request = Some(memory_request.into());
        self
    }
}

pub trait ResourceLimitedStep: WorkFlowStep {
    fn with_resource_limits(self, limits: ResourceLimits) -> Self;
    fn resource_limits(&self) -> Option<&ResourceLimits>;
}

pub trait HasResourceLimits: WorkFlowStep {
    fn resource_limits(&self) -> Option<&ResourceLimits>;
}

impl<T: ResourceLimitedStep> HasResourceLimits for T {
    fn resource_limits(&self) -> Option<&ResourceLimits> {
        self.resource_limits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::steps::result::StepStatus;
    use async_stream::try_stream;
    use std::time::Duration;

    #[derive(Debug, Clone)]
    struct MockStep {
        id: String,
    }

    impl WorkFlowStep for MockStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[derive(Debug, Clone)]
    struct MockKubeStep {
        id: String,
        namespace: String,
        resource_name: String,
    }

    impl WorkFlowStep for MockKubeStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl KubeWorkFlowStep for MockKubeStep {
        fn namespace(&self) -> &str {
            &self.namespace
        }

        fn resource_name(&self) -> &str {
            &self.resource_name
        }
    }

    #[derive(Debug, Clone)]
    struct MockExecutableStep {
        id: String,
    }

    impl WorkFlowStep for MockExecutableStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ExecutableWorkFlowStep for MockExecutableStep {
        fn execute(&self) -> anyhow::Result<StepResult> {
            Ok(StepResult::new(&self.id)
                .with_status(StepStatus::Success)
                .with_exit_code(0))
        }

        fn cancel(&self) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    struct MockWaitableStep {
        id: String,
    }

    impl WorkFlowStep for MockWaitableStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl WaitableWorkFlowStep for MockWaitableStep {
        fn wait(&self) -> impl std::future::Future<Output = anyhow::Result<StepResult>> + Send {
            let id = self.id.clone();
            async move {
                Ok(StepResult::new(&id)
                    .with_status(StepStatus::Success)
                    .with_execution_time(Duration::from_secs(1)))
            }
        }
    }

    #[derive(Debug, Clone)]
    struct MockDeletableStep {
        id: String,
    }

    impl WorkFlowStep for MockDeletableStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl DeletableWorkFlowStep for MockDeletableStep {
        fn delete_workflow(
            &self,
            _dry_run: bool,
        ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
            async move { Ok(()) }
        }

        fn delete_associated_pods(
            &self,
            _dry_run: bool,
        ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
            async move { Ok(()) }
        }
    }

    #[derive(Debug, Clone)]
    struct MockLoggableStep {
        id: String,
    }

    impl WorkFlowStep for MockLoggableStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl LoggableWorkFlowStep for MockLoggableStep {
        fn stream_logs(
            &self,
            _options: LogStreamOptions,
        ) -> Pin<Box<dyn Stream<Item = anyhow::Result<String>> + Send + '_>> {
            let step_id = self.id.clone();
            Box::pin(try_stream! {
                yield format!("Log line from {}", step_id);
            })
        }
    }

    #[derive(Debug, Clone)]
    struct MockServableStep {
        id: String,
    }

    impl WorkFlowStep for MockServableStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ServableWorkFlowStep for MockServableStep {
        fn expose_service(
            &self,
            service_name: &str,
            port: u16,
        ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send {
            let service_name = service_name.to_string();
            async move { Ok(format!("Service {} exposed on port {}", service_name, port)) }
        }

        fn expose_ingress(
            &self,
            ingress_name: &str,
            host: &str,
            service_port: u16,
        ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send {
            let ingress_name = ingress_name.to_string();
            let host = host.to_string();
            async move {
                Ok(format!(
                    "Ingress {} for host {} on port {}",
                    ingress_name, host, service_port
                ))
            }
        }
    }

    struct MockConditionalStep {
        id: String,
        condition: Option<Box<dyn Fn(&dyn WorkFlowStep) -> bool + Send + Sync>>,
    }

    impl MockConditionalStep {
        fn new(id: impl Into<String>) -> Self {
            Self {
                id: id.into(),
                condition: None,
            }
        }
    }

    impl WorkFlowStep for MockConditionalStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ConditionalStep for MockConditionalStep {
        fn with_execution_condition<F>(mut self, condition: F) -> Self
        where
            F: Fn(&dyn WorkFlowStep) -> bool + Send + Sync + 'static,
        {
            self.condition = Some(Box::new(condition));
            self
        }

        fn should_execute(&self) -> bool {
            self.condition.as_ref().map(|c| c(self)).unwrap_or(true)
        }
    }

    #[derive(Debug, Clone)]
    struct MockStepWithOutputs {
        id: String,
        result: StepResult,
    }

    impl MockStepWithOutputs {
        fn new(id: impl Into<String>) -> Self {
            let id_str = id.into();
            Self {
                id: id_str.clone(),
                result: StepResult::new(id_str),
            }
        }

        fn with_result(mut self, result: StepResult) -> Self {
            self.result = result;
            self
        }
    }

    impl WorkFlowStep for MockStepWithOutputs {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl StepWithOutputs for MockStepWithOutputs {
        fn get_step_result(&self) -> StepResult {
            self.result.clone()
        }

        fn get_output(&self, key: &str) -> Option<Value> {
            self.result.outputs.get(key).cloned()
        }
    }

    #[derive(Debug, Clone)]
    struct MockResourceLimitedStep {
        id: String,
        limits: Option<ResourceLimits>,
    }

    impl MockResourceLimitedStep {
        fn new(id: impl Into<String>) -> Self {
            Self {
                id: id.into(),
                limits: None,
            }
        }
    }

    impl WorkFlowStep for MockResourceLimitedStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ResourceLimitedStep for MockResourceLimitedStep {
        fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
            self.limits = Some(limits);
            self
        }

        fn resource_limits(&self) -> Option<&ResourceLimits> {
            self.limits.as_ref()
        }
    }

    #[derive(Debug, Clone)]
    struct CompositeStep {
        id: String,
    }

    impl WorkFlowStep for CompositeStep {
        fn step_id(&self) -> &str {
            &self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl KubeWorkFlowStep for CompositeStep {
        fn namespace(&self) -> &str {
            "default"
        }

        fn resource_name(&self) -> &str {
            &self.id
        }
    }

    impl WaitableWorkFlowStep for CompositeStep {
        fn wait(&self) -> impl std::future::Future<Output = anyhow::Result<StepResult>> + Send {
            let id = self.id.clone();
            async move { Ok(StepResult::new(&id).with_status(StepStatus::Success)) }
        }
    }

    impl DeletableWorkFlowStep for CompositeStep {
        fn delete_workflow(
            &self,
            _dry_run: bool,
        ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
            async move { Ok(()) }
        }

        fn delete_associated_pods(
            &self,
            _dry_run: bool,
        ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
            async move { Ok(()) }
        }
    }

    #[test]
    fn test_workflow_step() {
        let step = MockStep {
            id: "test-step".to_string(),
        };
        assert_eq!(step.step_id(), "test-step");
    }

    #[test]
    fn test_kube_workflow_step() {
        let step = MockKubeStep {
            id: "kube-step".to_string(),
            namespace: "staging".to_string(),
            resource_name: "job-123".to_string(),
        };
        assert_eq!(step.step_id(), "kube-step");
        assert_eq!(step.namespace(), "staging");
        assert_eq!(step.resource_name(), "job-123");
    }

    #[test]
    fn test_executable_workflow_step() {
        let step = MockExecutableStep {
            id: "exec-step".to_string(),
        };
        let result = step.execute().unwrap();
        assert_eq!(result.step_id, "exec-step");
        assert!(result.is_success());
        assert_eq!(result.exit_code, 0);
    }

    #[tokio::test]
    async fn test_waitable_workflow_step() {
        let step = MockWaitableStep {
            id: "wait-step".to_string(),
        };
        let result = step.wait().await.unwrap();
        assert_eq!(result.step_id, "wait-step");
        assert!(result.is_success());
    }

    #[tokio::test]
    async fn test_deletable_workflow_step() {
        let step = MockDeletableStep {
            id: "delete-step".to_string(),
        };
        step.delete_workflow(false).await.unwrap();
        step.delete_associated_pods(false).await.unwrap();
    }

    #[tokio::test]
    async fn test_loggable_workflow_step() {
        let step = MockLoggableStep {
            id: "log-step".to_string(),
        };
        let options = LogStreamOptions::default();
        let mut stream = step.stream_logs(options);

        use futures::StreamExt;
        if let Some(item) = stream.next().await {
            let log = item.unwrap();
            assert!(log.contains("log-step"));
        }
    }

    #[tokio::test]
    async fn test_servable_workflow_step() {
        let step = MockServableStep {
            id: "service-step".to_string(),
        };
        let service_url = step.expose_service("my-service", 8080).await.unwrap();
        assert!(service_url.contains("my-service"));
        assert!(service_url.contains("8080"));

        let ingress_url = step
            .expose_ingress("my-ingress", "example.com", 8080)
            .await
            .unwrap();
        assert!(ingress_url.contains("my-ingress"));
        assert!(ingress_url.contains("example.com"));
    }

    #[test]
    fn test_conditional_step() {
        let step = MockConditionalStep::new("cond-step");
        assert!(step.should_execute());

        let step_with_condition = MockConditionalStep::new("cond-step")
            .with_execution_condition(|s| s.step_id() == "cond-step");
        assert!(step_with_condition.should_execute());

        let step_with_false_condition = MockConditionalStep::new("cond-step")
            .with_execution_condition(|s| s.step_id() == "other-step");
        assert!(!step_with_false_condition.should_execute());
    }

    #[test]
    fn test_step_with_outputs() {
        let result = StepResult::new("output-step")
            .with_output("key1", Value::String("value1".to_string()))
            .with_output("key2", Value::Number(42.into()));

        let step = MockStepWithOutputs::new("output-step").with_result(result.clone());

        assert_eq!(step.get_step_result().step_id, "output-step");
        assert_eq!(
            step.get_output("key1"),
            Some(Value::String("value1".to_string()))
        );
        assert_eq!(step.get_output("key2"), Some(Value::Number(42.into())));
        assert_eq!(step.get_output("key3"), None);
    }

    #[test]
    fn test_resource_limited_step() {
        let limits = ResourceLimits::new()
            .with_cpu("500m")
            .with_memory("256Mi")
            .with_cpu_request("100m")
            .with_memory_request("128Mi");

        let step = MockResourceLimitedStep::new("resource-step").with_resource_limits(limits);

        assert!(ResourceLimitedStep::resource_limits(&step).is_some());
        let limits = ResourceLimitedStep::resource_limits(&step).unwrap();
        assert_eq!(limits.cpu, Some("500m".to_string()));
        assert_eq!(limits.memory, Some("256Mi".to_string()));
        assert_eq!(limits.cpu_request, Some("100m".to_string()));
        assert_eq!(limits.memory_request, Some("128Mi".to_string()));
    }

    #[test]
    fn test_trait_combinations() {
        let step = CompositeStep {
            id: "composite-step".to_string(),
        };

        assert_eq!(step.step_id(), "composite-step");
        assert_eq!(step.namespace(), "default");
        assert_eq!(step.resource_name(), "composite-step");
    }

    #[tokio::test]
    async fn test_composite_step_multiple_traits() {
        let step = CompositeStep {
            id: "composite-step".to_string(),
        };

        let result = step.wait().await.unwrap();
        assert!(result.is_success());

        step.delete_workflow(false).await.unwrap();
        step.delete_associated_pods(false).await.unwrap();
    }

    #[test]
    fn test_trait_object_usage() {
        let step: Box<dyn WorkFlowStep> = Box::new(MockStep {
            id: "boxed-step".to_string(),
        });
        assert_eq!(step.step_id(), "boxed-step");
    }

    #[test]
    fn test_kube_trait_object() {
        let step: Box<dyn KubeWorkFlowStep> = Box::new(MockKubeStep {
            id: "boxed-kube-step".to_string(),
            namespace: "production".to_string(),
            resource_name: "job-456".to_string(),
        });

        assert_eq!(step.step_id(), "boxed-kube-step");
        assert_eq!(step.namespace(), "production");
        assert_eq!(step.resource_name(), "job-456");
    }

    #[test]
    fn test_log_stream_options() {
        let options = LogStreamOptions::default();
        assert!(!options.follow);
        assert!(options.tail_lines.is_none());
        assert!(options.since_seconds.is_none());
        assert!(!options.timestamps);
        assert!(options.limit_bytes.is_none());
    }

    #[test]
    fn test_log_stream_options_custom() {
        let options = LogStreamOptions {
            follow: true,
            tail_lines: Some(100),
            since_seconds: Some(60),
            timestamps: true,
            limit_bytes: Some(1024),
        };

        assert!(options.follow);
        assert_eq!(options.tail_lines, Some(100));
        assert_eq!(options.since_seconds, Some(60));
        assert!(options.timestamps);
        assert_eq!(options.limit_bytes, Some(1024));
    }
}
