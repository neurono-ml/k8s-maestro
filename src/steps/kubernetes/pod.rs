use crate::clients::MaestroK8sClient;
use crate::entities::ContainerLike;
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
use k8s_openapi::api::core::v1::Pod;
use kube::Api;
use std::any::Any;
use std::collections::BTreeMap;
use std::pin::Pin;

pub struct KubePodStep {
    step_id: String,
    namespace: String,
    name: JobNameType,
    containers: Vec<Box<dyn ContainerLike + Send + Sync>>,
    sidecars: Vec<Box<dyn ContainerLike + Send + Sync>>,
    restart_policy: RestartPolicy,
    resource_limits: Option<ResourceLimits>,
    service_config: Option<ServiceConfig>,
    ingress_config: Option<IngressConfig>,
    client: MaestroK8sClient,
    dry_run: bool,
}

impl KubePodStep {
    pub fn builder() -> KubePodStepBuilder {
        KubePodStepBuilder::new()
    }
}

impl WorkFlowStep for KubePodStep {
    fn step_id(&self) -> &str {
        &self.step_id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl KubeWorkFlowStep for KubePodStep {
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

impl WaitableWorkFlowStep for KubePodStep {
    fn wait(&self) -> impl std::future::Future<Output = Result<StepResult>> + Send {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let name = match &self.name {
            JobNameType::DefinedName(name) => name.clone(),
            JobNameType::GenerateName(prefix) => prefix.clone(),
        };
        let step_id = self.step_id.clone();

        async move {
            let pods: Api<Pod> = Api::namespaced(client, &namespace);
            let pod = pods.get(&name).await?;

            let phase = pod
                .status
                .as_ref()
                .and_then(|s| s.phase.as_ref())
                .ok_or_else(|| anyhow::anyhow!("Pod has no phase"))?;

            let result = StepResult::new(&step_id)
                .with_status(match phase.as_str() {
                    "Succeeded" => StepStatus::Success,
                    "Failed" => StepStatus::Failure,
                    _ => StepStatus::Failure,
                })
                .with_exit_code(if phase == "Succeeded" { 0 } else { 1 });

            Ok(result)
        }
    }
}

impl DeletableWorkFlowStep for KubePodStep {
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
                    "DRY RUN: Would delete pod {} in namespace {}",
                    name,
                    namespace
                );
                return Ok(());
            }

            let pods: Api<Pod> = Api::namespaced(client, &namespace);
            pods.delete(&name, &Default::default()).await?;
            Ok(())
        }
    }

    fn delete_associated_pods(
        &self,
        dry_run: bool,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            if dry_run {
                log::info!("DRY RUN: delete_associated_pods is a no-op for pods");
                return Ok(());
            }
            Ok(())
        }
    }
}

impl LoggableWorkFlowStep for KubePodStep {
    fn stream_logs(
        &self,
        _options: crate::steps::traits::LogStreamOptions,
    ) -> Pin<Box<dyn Stream<Item = Result<String>> + Send + '_>> {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let name = match &self.name {
            JobNameType::DefinedName(name) => name.clone(),
            JobNameType::GenerateName(prefix) => prefix.clone(),
        };

        Box::pin(try_stream! {
            let pods: Api<Pod> = Api::namespaced(client, &namespace);
            let logs = pods.logs(&name, &Default::default()).await?;
            yield logs;
        })
    }
}

impl ServableWorkFlowStep for KubePodStep {
    fn expose_service(
        &self,
        service_name: &str,
        port: u16,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        let client = self.client.inner().clone();
        let namespace = self.namespace.clone();
        let pod_name = match &self.name {
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
            selector.insert("pod-name".to_string(), pod_name);

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

pub struct KubePodStepBuilder {
    namespace: Option<String>,
    name: JobNameType,
    containers: Vec<Box<dyn ContainerLike + Send + Sync>>,
    sidecars: Vec<Box<dyn ContainerLike + Send + Sync>>,
    restart_policy: RestartPolicy,
    resource_limits: Option<ResourceLimits>,
    service_config: Option<ServiceConfig>,
    ingress_config: Option<IngressConfig>,
    client: Option<MaestroK8sClient>,
    dry_run: bool,
}

impl KubePodStepBuilder {
    pub fn new() -> Self {
        Self {
            namespace: None,
            name: JobNameType::DefinedName(String::new()),
            containers: Vec::new(),
            sidecars: Vec::new(),
            restart_policy: RestartPolicy::Never,
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

    pub fn with_restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
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

    pub fn build(self) -> Result<KubePodStep> {
        let namespace = self
            .namespace
            .ok_or_else(|| anyhow::anyhow!("Namespace is required"))?;

        if self.containers.is_empty() {
            return Err(anyhow::anyhow!("At least one container is required"));
        }

        let name = match &self.name {
            JobNameType::DefinedName(s) if s.is_empty() => {
                return Err(anyhow::anyhow!("Pod name is required"))
            }
            JobNameType::GenerateName(s) if s.is_empty() => {
                return Err(anyhow::anyhow!("Pod name prefix is required"))
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

        Ok(KubePodStep {
            step_id,
            namespace,
            name,
            containers: self.containers,
            sidecars: self.sidecars,
            restart_policy: self.restart_policy,
            resource_limits: self.resource_limits,
            service_config: self.service_config,
            ingress_config: self.ingress_config,
            client,
            dry_run: self.dry_run,
        })
    }
}

impl Default for KubePodStepBuilder {
    fn default() -> Self {
        Self::new()
    }
}
