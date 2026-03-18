use crate::clients::MaestroK8sClient;
use crate::steps::exec::PackageSource;
use crate::steps::result::{StepResult, StepStatus};
use crate::steps::traits::{
    DeletableWorkFlowStep, ExecutableWorkFlowStep, LoggableWorkFlowStep, WaitableWorkFlowStep,
    WorkFlowStep,
};
use crate::steps::ResourceLimits;
use anyhow::Result;
use async_stream::try_stream;
use futures::Stream;
use k8s_openapi::api::core::v1::{ConfigMap, Pod};
use kube::Api;
use std::any::Any;
use std::collections::BTreeMap;
use std::pin::Pin;
use std::time::Duration;

pub struct PythonStep {
    step_id: String,
    namespace: String,
    name: String,
    code: Option<String>,
    package_source: Option<PackageSource>,
    requirements: Vec<String>,
    entry_point: Option<String>,
    resource_limits: Option<ResourceLimits>,
    volume_mounts: BTreeMap<String, String>,
    environment_variables: BTreeMap<String, String>,
    timeout: Duration,
    client: MaestroK8sClient,
    dry_run: bool,
}

impl PythonStep {
    pub fn builder() -> PythonStepBuilder {
        PythonStepBuilder::new()
    }
}

impl WorkFlowStep for PythonStep {
    fn step_id(&self) -> &str {
        &self.step_id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExecutableWorkFlowStep for PythonStep {
    fn execute(&self) -> Result<StepResult> {
        tokio::runtime::Handle::current().block_on(self.execute_async())
    }

    fn cancel(&self) -> Result<()> {
        tokio::runtime::Handle::current().block_on(self.cancel_async())
    }
}

impl WaitableWorkFlowStep for PythonStep {
    fn wait(&self) -> impl std::future::Future<Output = Result<StepResult>> + Send {
        let step_id = self.step_id.clone();
        let namespace = self.namespace.clone();
        let name = self.name.clone();
        let client = self.client.inner().clone();

        async move {
            let pods: Api<Pod> = Api::namespaced(client, &namespace);
            let pod = pods.get(&name).await?;

            let status = pod
                .status
                .as_ref()
                .and_then(|s| s.phase.as_ref())
                .ok_or_else(|| anyhow::anyhow!("Pod has no phase"))?;

            let step_status = match status.as_str() {
                "Succeeded" => StepStatus::Success,
                "Failed" => StepStatus::Failure,
                _ => StepStatus::Failure,
            };

            Ok(StepResult::new(&step_id).with_status(step_status))
        }
    }
}

impl DeletableWorkFlowStep for PythonStep {
    fn delete_workflow(
        &self,
        dry_run: bool,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        let step_id = self.step_id.clone();
        let namespace = self.namespace.clone();
        let name = self.name.clone();
        let client = self.client.inner().clone();
        let self_dry_run = self.dry_run;

        async move {
            if dry_run || self_dry_run {
                log::info!(
                    "DRY RUN: Would delete resources for {}",
                    step_id
                );
                return Ok(());
            }

            let pods: Api<Pod> = Api::namespaced(client.clone(), &namespace);
            pods.delete(&name, &Default::default()).await?;

            let configmaps: Api<ConfigMap> = Api::namespaced(client, &namespace);
            let cm_name = format!("{}-code", name);
            let _ = configmaps.delete(&cm_name, &Default::default()).await;

            Ok(())
        }
    }

    fn delete_associated_pods(
        &self,
        _dry_run: bool,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        async move { Ok(()) }
    }
}

impl LoggableWorkFlowStep for PythonStep {
    fn stream_logs(
        &self,
        _options: crate::steps::traits::LogStreamOptions,
    ) -> Pin<Box<dyn Stream<Item = Result<String>> + Send + '_>> {
        let namespace = self.namespace.clone();
        let name = self.name.clone();
        let client = self.client.inner().clone();

        Box::pin(try_stream! {
            let pods: Api<Pod> = Api::namespaced(client, &namespace);
            let logs = pods.logs(&name, &Default::default()).await?;
            yield logs;
        })
    }
}

impl PythonStep {
    async fn execute_async(&self) -> Result<StepResult> {
        if self.dry_run {
            log::info!("DRY RUN: Would execute Python step {}", self.step_id);
            return Ok(StepResult::new(&self.step_id)
                .with_status(StepStatus::Success)
                .with_exit_code(0));
        }

        let pod = self.build_pod_spec()?;

        let pods: Api<Pod> = Api::namespaced(self.client.inner().clone(), &self.namespace);
        pods.create(&Default::default(), &pod).await?;

        Ok(StepResult::new(&self.step_id).with_status(StepStatus::Success))
    }

    async fn cancel_async(&self) -> Result<()> {
        let pods: Api<Pod> = Api::namespaced(self.client.inner().clone(), &self.namespace);
        pods.delete(&self.name, &Default::default()).await?;
        Ok(())
    }

    fn build_pod_spec(&self) -> Result<Pod> {
        use k8s_openapi::api::core::v1::{
            Container, PodSpec, PodStatus, Volume, VolumeMount,
        };
        use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

        let mut env_vars = Vec::new();
        for (key, value) in &self.environment_variables {
            env_vars.push(k8s_openapi::api::core::v1::EnvVar {
                name: key.clone(),
                value: Some(value.clone()),
                value_from: None,
            });
        }

        let mut volume_mounts = Vec::new();
        let mut volumes = Vec::new();

        if self.code.is_some() {
            let cm_name = format!("{}-code", self.name);
            volume_mounts.push(VolumeMount {
                name: "code".to_string(),
                mount_path: "/workspace".to_string(),
                ..Default::default()
            });
            volumes.push(Volume {
                name: "code".to_string(),
                config_map: Some(k8s_openapi::api::core::v1::ConfigMapVolumeSource {
                    name: cm_name,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        for (mount_path, volume_name) in &self.volume_mounts {
            volume_mounts.push(VolumeMount {
                name: volume_name.clone(),
                mount_path: mount_path.clone(),
                ..Default::default()
            });
            volumes.push(Volume {
                name: volume_name.clone(),
                persistent_volume_claim: Some(
                    k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource {
                        claim_name: volume_name.clone(),
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        }

        let mut command = vec!["python".to_string()];

        if let Some(entry) = &self.entry_point {
            command.push("/workspace/".to_string() + entry);
        } else if let Some(_) = &self.code {
            command.push("/workspace/script.py".to_string());
        }

        let mut container = Container {
            name: "python-executor".to_string(),
            image: Some("python:3.12-slim".to_string()),
            command: Some(command),
            env: Some(env_vars),
            volume_mounts: Some(volume_mounts),
            ..Default::default()
        };

        if let Some(limits) = &self.resource_limits {
            let mut resources = k8s_openapi::api::core::v1::ResourceRequirements::default();

            if let Some(cpu) = &limits.cpu {
                resources.limits = Some(BTreeMap::from([(
                    "cpu".to_string(),
                    Quantity(cpu.clone()),
                )]));
            }
            if let Some(memory) = &limits.memory {
                resources
                    .limits
                    .get_or_insert_with(BTreeMap::new)
                    .insert("memory".to_string(), Quantity(memory.clone()));
            }
            if let Some(cpu_req) = &limits.cpu_request {
                resources.requests = Some(BTreeMap::from([(
                    "cpu".to_string(),
                    Quantity(cpu_req.clone()),
                )]));
            }
            if let Some(memory_req) = &limits.memory_request {
                resources
                    .requests
                    .get_or_insert_with(BTreeMap::new)
                    .insert("memory".to_string(), Quantity(memory_req.clone()));
            }

            container.resources = Some(resources);
        }

        let pod = Pod {
            metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(self.name.clone()),
                namespace: Some(self.namespace.clone()),
                ..Default::default()
            },
            spec: Some(PodSpec {
                containers: vec![container],
                volumes: Some(volumes),
                restart_policy: Some("Never".to_string()),
                ..Default::default()
            }),
            status: Some(PodStatus::default()),
        };

        Ok(pod)
    }

    fn build_config_map(&self) -> Result<ConfigMap> {
        let mut data = BTreeMap::new();

        if let Some(code) = &self.code {
            data.insert("script.py".to_string(), code.clone());
        }

        if !self.requirements.is_empty() {
            data.insert(
                "requirements.txt".to_string(),
                self.requirements.join("\n"),
            );
        }

        Ok(ConfigMap {
            metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(format!("{}-code", self.name)),
                namespace: Some(self.namespace.clone()),
                ..Default::default()
            },
            data: Some(data),
            ..Default::default()
        })
    }
}

pub struct PythonStepBuilder {
    step_id: Option<String>,
    namespace: Option<String>,
    name: Option<String>,
    code: Option<String>,
    package_source: Option<PackageSource>,
    requirements: Vec<String>,
    entry_point: Option<String>,
    resource_limits: Option<ResourceLimits>,
    volume_mounts: BTreeMap<String, String>,
    environment_variables: BTreeMap<String, String>,
    timeout: Duration,
    client: Option<MaestroK8sClient>,
    dry_run: bool,
}

impl PythonStepBuilder {
    pub fn new() -> Self {
        Self {
            step_id: None,
            namespace: Some("default".to_string()),
            name: None,
            code: None,
            package_source: None,
            requirements: Vec::new(),
            entry_point: None,
            resource_limits: None,
            volume_mounts: BTreeMap::new(),
            environment_variables: BTreeMap::new(),
            timeout: Duration::from_secs(300),
            client: None,
            dry_run: false,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name_str = name.into();
        self.step_id = Some(name_str.clone());
        self.name = Some(name_str);
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_package(mut self, source: PackageSource) -> Self {
        self.package_source = Some(source);
        self
    }

    pub fn with_requirements(mut self, reqs: &[impl AsRef<str>]) -> Self {
        self.requirements = reqs.iter().map(|s| s.as_ref().to_string()).collect();
        self
    }

    pub fn with_entry_point(mut self, entry: impl Into<String>) -> Self {
        self.entry_point = Some(entry.into());
        self
    }

    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = Some(limits);
        self
    }

    pub fn with_volume_mount(mut self, mount_path: &str, volume_name: &str) -> Self {
        self.volume_mounts.insert(mount_path.to_string(), volume_name.to_string());
        self
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.environment_variables.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_client(mut self, client: MaestroK8sClient) -> Self {
        self.client = Some(client);
        self
    }

    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn build(self) -> Result<PythonStep> {
        let step_id = self
            .step_id
            .ok_or_else(|| anyhow::anyhow!("step_id is required"))?;
        let namespace = self.namespace.unwrap_or_else(|| "default".to_string());
        let name = self.name.unwrap_or_else(|| format!("{}-python", step_id));
        let client = self
            .client
            .ok_or_else(|| anyhow::anyhow!("client is required"))?;

        Ok(PythonStep {
            step_id,
            namespace,
            name,
            code: self.code,
            package_source: self.package_source,
            requirements: self.requirements,
            entry_point: self.entry_point,
            resource_limits: self.resource_limits,
            volume_mounts: self.volume_mounts,
            environment_variables: self.environment_variables,
            timeout: self.timeout,
            client,
            dry_run: self.dry_run,
        })
    }
}

impl Default for PythonStepBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_python_step_builder() {
        let client = MaestroK8sClient::new().await.unwrap();

        let builder = PythonStepBuilder::new()
            .with_name("test-step")
            .with_code("print('hello')")
            .with_client(client)
            .with_dry_run(true);

        let result = builder.build();
        assert!(result.is_ok());

        let step = result.unwrap();
        assert_eq!(step.step_id(), "test-step");
    }

    #[tokio::test]
    async fn test_python_step_builder_with_requirements() {
        let client = MaestroK8sClient::new().await.unwrap();

        let builder = PythonStepBuilder::new()
            .with_name("test-step")
            .with_code("import pandas\ndf = pandas.DataFrame()")
            .with_requirements(&["pandas", "numpy"])
            .with_client(client)
            .with_dry_run(true);

        let result = builder.build();
        assert!(result.is_ok());

        let step = result.unwrap();
        assert_eq!(step.requirements.len(), 2);
    }

    #[tokio::test]
    async fn test_python_step_builder_with_resource_limits() {
        let client = MaestroK8sClient::new().await.unwrap();

        let limits = ResourceLimits::new()
            .with_cpu("500m")
            .with_memory("256Mi");

        let builder = PythonStepBuilder::new()
            .with_name("test-step")
            .with_code("print('hello')")
            .with_resource_limits(limits)
            .with_client(client)
            .with_dry_run(true);

        let result = builder.build();
        assert!(result.is_ok());

        let step = result.unwrap();
        assert!(step.resource_limits.is_some());
    }
}
