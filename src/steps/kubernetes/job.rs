use crate::clients::MaestroK8sClient;
use crate::entities::ContainerLike;
use crate::entities::MaestroContainer;
use crate::steps::kubernetes::types::{IngressConfig, JobNameType, RestartPolicy, ServiceConfig};
use crate::steps::result::{StepResult, StepStatus};
use crate::steps::traits::{
    DeletableWorkFlowStep, KubeWorkFlowStep, LoggableWorkFlowStep, ServableWorkFlowStep,
    WaitableWorkFlowStep, WorkFlowStep,
};
use crate::steps::ResourceLimits;
use anyhow::Result;
use async_stream::try_stream;
use futures::Stream;
use k8s_openapi::api::batch::v1::Job;
use kube::Api;
use std::any::Any;
use std::collections::BTreeMap;
use std::pin::Pin;

pub struct KubeJobStep {
    step_id: String,
    namespace: String,
    name: JobNameType,
    // TODO: Implement proper usage of these fields to configure Kubernetes Job spec
    // These fields are pending implementation per specification
    #[allow(dead_code)]
    containers: Vec<Box<dyn ContainerLike + Send + Sync>>,
    #[allow(dead_code)]
    sidecars: Vec<Box<dyn ContainerLike + Send + Sync>>,
    #[allow(dead_code)]
    backoff_limit: Option<i32>,
    #[allow(dead_code)]
    restart_policy: RestartPolicy,
    #[allow(dead_code)]
    ttl_seconds: Option<i32>,
    #[allow(dead_code)]
    completions: Option<i32>,
    #[allow(dead_code)]
    parallelism: Option<i32>,
    #[allow(dead_code)]
    resource_limits: Option<ResourceLimits>,
    #[allow(dead_code)]
    service_config: Option<ServiceConfig>,
    #[allow(dead_code)]
    ingress_config: Option<IngressConfig>,
    client: MaestroK8sClient,
    dry_run: bool,
}

impl KubeJobStep {
    pub fn new(
        name: impl Into<String>,
        image: impl Into<String>,
        client: MaestroK8sClient,
    ) -> Self {
        let container = MaestroContainer::new(image.into(), "main");
        Self {
            step_id: uuid::Uuid::new_v4().to_string(),
            namespace: "default".to_string(),
            name: JobNameType::DefinedName(name.into()),
            containers: vec![Box::new(container)],
            sidecars: vec![],
            backoff_limit: None,
            restart_policy: RestartPolicy::Never,
            ttl_seconds: None,
            completions: None,
            parallelism: None,
            resource_limits: None,
            service_config: None,
            ingress_config: None,
            client,
            dry_run: false,
        }
    }

    pub fn builder() -> KubeJobStepBuilder {
        KubeJobStepBuilder::new()
    }
}

impl WorkFlowStep for KubeJobStep {
    fn step_id(&self) -> &str {
        &self.step_id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl KubeWorkFlowStep for KubeJobStep {
    fn namespace(&self) -> &str {
        &self.namespace
    }

    fn resource_name(&self) -> &str {
        match &self.name {
            JobNameType::DefinedName(name) => name,
            JobNameType::GenerateName(prefix) => prefix,
        }
    }
}

impl WaitableWorkFlowStep for KubeJobStep {
    fn wait(&self) -> impl std::future::Future<Output = Result<StepResult>> + Send {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let name = match &self.name {
            JobNameType::DefinedName(name) => name.clone(),
            JobNameType::GenerateName(prefix) => prefix.clone(),
        };
        let step_id = self.step_id.clone();

        async move {
            let jobs: Api<Job> = Api::namespaced(client, &namespace);
            let job = jobs.get(&name).await?;

            let status = job
                .status
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Job has no status"))?;
            let _succeeded = status.succeeded.unwrap_or(0);
            let failed = status.failed.unwrap_or(0);

            let result = StepResult::new(&step_id)
                .with_status(if failed > 0 {
                    StepStatus::Failure
                } else {
                    StepStatus::Success
                })
                .with_exit_code(if failed > 0 { 1 } else { 0 });

            Ok(result)
        }
    }
}

impl DeletableWorkFlowStep for KubeJobStep {
    fn delete_workflow(
        &self,
        dry_run: bool,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let name = match &self.name {
            JobNameType::DefinedName(name) => name.clone(),
            JobNameType::GenerateName(prefix) => prefix.clone(),
        };
        let self_dry_run = self.dry_run;

        async move {
            if dry_run || self_dry_run {
                log::info!(
                    "DRY RUN: Would delete job {} in namespace {}",
                    name,
                    namespace
                );
                return Ok(());
            }

            let jobs: Api<Job> = Api::namespaced(client, &namespace);
            jobs.delete(&name, &Default::default()).await?;
            Ok(())
        }
    }

    fn delete_associated_pods(
        &self,
        dry_run: bool,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let job_name = match &self.name {
            JobNameType::DefinedName(name) => name.clone(),
            JobNameType::GenerateName(prefix) => prefix.clone(),
        };
        let self_dry_run = self.dry_run;

        async move {
            if dry_run || self_dry_run {
                log::info!(
                    "DRY RUN: Would delete pods for job {} in namespace {}",
                    job_name,
                    namespace
                );
                return Ok(());
            }

            let pods: Api<k8s_openapi::api::core::v1::Pod> = Api::namespaced(client, &namespace);
            let pod_list = pods
                .list(&Default::default())
                .await?
                .into_iter()
                .filter(|pod| {
                    pod.metadata
                        .labels
                        .as_ref()
                        .and_then(|labels| labels.get("job-name"))
                        .map(|label| label == &job_name)
                        .unwrap_or(false)
                });

            for pod in pod_list {
                if let Some(name) = pod.metadata.name.as_ref() {
                    pods.delete(name, &Default::default()).await?;
                }
            }

            Ok(())
        }
    }
}

impl LoggableWorkFlowStep for KubeJobStep {
    fn stream_logs(
        &self,
        _options: crate::steps::traits::LogStreamOptions,
    ) -> Pin<Box<dyn Stream<Item = Result<String>> + Send + '_>> {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let job_name = match &self.name {
            JobNameType::DefinedName(name) => name.clone(),
            JobNameType::GenerateName(prefix) => prefix.clone(),
        };

        Box::pin(try_stream! {
            let pods: Api<k8s_openapi::api::core::v1::Pod> = Api::namespaced(client, &namespace);
            let pod_list = pods
                .list(&Default::default())
                .await?
                .into_iter()
                .filter(|pod| {
                    pod.metadata
                        .labels
                        .as_ref()
                        .and_then(|labels| labels.get("job-name"))
                        .map(|label| label == &job_name)
                        .unwrap_or(false)
                });

            for pod in pod_list {
                if let Some(pod_name) = pod.metadata.name.as_ref() {
                    let logs = pods.logs(pod_name, &Default::default()).await?;
                    yield format!("[{}]: {}", pod_name, logs);
                }
            }
        })
    }
}

impl ServableWorkFlowStep for KubeJobStep {
    fn expose_service(
        &self,
        service_name: &str,
        port: u16,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let job_name = match &self.name {
            JobNameType::DefinedName(name) => name.clone(),
            JobNameType::GenerateName(prefix) => prefix.clone(),
        };
        let service_name = service_name.to_string();
        let self_dry_run = self.dry_run;

        async move {
            if self_dry_run {
                return Ok(format!("DRY RUN: Would expose service {}", service_name));
            }

            let mut selector = BTreeMap::new();
            selector.insert("job-name".to_string(), job_name);

            let service = crate::networking::ServiceBuilder::new()
                .with_name(&service_name)
                .with_namespace(&namespace)
                .with_port(port as i32, port as i32, "TCP")
                .with_selector(selector)
                .build()?;

            let services: Api<k8s_openapi::api::core::v1::Service> =
                Api::namespaced(client, &namespace);
            services.create(&Default::default(), &service).await?;

            Ok(format!("Service {} exposed on port {}", service_name, port))
        }
    }

    fn expose_ingress(
        &self,
        ingress_name: &str,
        host: &str,
        service_port: u16,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let ingress_name = ingress_name.to_string();
        let host = host.to_string();
        let service_port = service_port as i32;
        let self_dry_run = self.dry_run;

        async move {
            if self_dry_run {
                return Ok(format!("DRY RUN: Would expose ingress {}", ingress_name));
            }

            let ingress = crate::networking::IngressBuilder::new()
                .with_name(&ingress_name)
                .with_namespace(&namespace)
                .with_host(&host)
                .with_path("/", "default-service", service_port)
                .build()?;

            let ingresses: Api<k8s_openapi::api::networking::v1::Ingress> =
                Api::namespaced(client, &namespace);
            ingresses.create(&Default::default(), &ingress).await?;

            Ok(format!("Ingress {} for host {}", ingress_name, host))
        }
    }
}

pub struct KubeJobStepBuilder {
    namespace: Option<String>,
    name: JobNameType,
    containers: Vec<Box<dyn ContainerLike + Send + Sync>>,
    sidecars: Vec<Box<dyn ContainerLike + Send + Sync>>,
    backoff_limit: Option<i32>,
    restart_policy: RestartPolicy,
    ttl_seconds: Option<i32>,
    completions: Option<i32>,
    parallelism: Option<i32>,
    resource_limits: Option<ResourceLimits>,
    service_config: Option<ServiceConfig>,
    ingress_config: Option<IngressConfig>,
    client: Option<MaestroK8sClient>,
    dry_run: bool,
}

impl KubeJobStepBuilder {
    pub fn new() -> Self {
        Self {
            namespace: None,
            name: JobNameType::DefinedName(String::new()),
            containers: Vec::new(),
            sidecars: Vec::new(),
            backoff_limit: None,
            restart_policy: RestartPolicy::Never,
            ttl_seconds: None,
            completions: None,
            parallelism: None,
            resource_limits: None,
            service_config: None,
            ingress_config: None,
            client: None,
            dry_run: false,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = JobNameType::DefinedName(name.into());
        self
    }

    pub fn with_name_type(mut self, name_type: JobNameType) -> Self {
        self.name = name_type;
        self
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn add_container(mut self, container: Box<dyn ContainerLike + Send + Sync>) -> Self {
        self.containers.push(container);
        self
    }

    pub fn add_sidecar(mut self, sidecar: Box<dyn ContainerLike + Send + Sync>) -> Self {
        self.sidecars.push(sidecar);
        self
    }

    pub fn with_backoff_limit(mut self, limit: i32) -> Self {
        self.backoff_limit = Some(limit);
        self
    }

    pub fn with_restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }

    pub fn with_ttl_seconds(mut self, ttl: i32) -> Self {
        self.ttl_seconds = Some(ttl);
        self
    }

    pub fn with_completions(mut self, completions: i32) -> Self {
        self.completions = Some(completions);
        self
    }

    pub fn with_parallelism(mut self, parallelism: i32) -> Self {
        self.parallelism = Some(parallelism);
        self
    }

    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = Some(limits);
        self
    }

    pub fn with_service_config(mut self, config: ServiceConfig) -> Self {
        self.service_config = Some(config);
        self
    }

    pub fn with_ingress_config(mut self, config: IngressConfig) -> Self {
        self.ingress_config = Some(config);
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

    pub fn build(self) -> Result<KubeJobStep> {
        let namespace = self
            .namespace
            .ok_or_else(|| anyhow::anyhow!("Namespace is required"))?;

        if self.containers.is_empty() {
            return Err(anyhow::anyhow!("At least one container is required"));
        }

        let name = match &self.name {
            JobNameType::DefinedName(s) if s.is_empty() => {
                return Err(anyhow::anyhow!("Job name is required"))
            }
            JobNameType::GenerateName(s) if s.is_empty() => {
                return Err(anyhow::anyhow!("Job name prefix is required"))
            }
            _ => self.name.clone(),
        };

        let step_id = match &name {
            JobNameType::DefinedName(name) => name.clone(),
            JobNameType::GenerateName(prefix) => prefix.clone(),
        };

        let client = self
            .client
            .ok_or_else(|| anyhow::anyhow!("Client is required"))?;

        Ok(KubeJobStep {
            step_id,
            namespace,
            name,
            containers: self.containers,
            sidecars: self.sidecars,
            backoff_limit: self.backoff_limit,
            restart_policy: self.restart_policy,
            ttl_seconds: self.ttl_seconds,
            completions: self.completions,
            parallelism: self.parallelism,
            resource_limits: self.resource_limits,
            service_config: self.service_config,
            ingress_config: self.ingress_config,
            client,
            dry_run: self.dry_run,
        })
    }
}

impl Default for KubeJobStepBuilder {
    fn default() -> Self {
        Self::new()
    }
}
