use k8s_openapi::api::{batch::v1::Job, core::v1::{ConfigMap, Secret}};
use kube::{api::{DeleteParams, PostParams}, Api};
use crate::entities::job::MaestroWorkflow;

pub struct MaestroK8sClientBuilder {
    namespace: Option<String>,
    k8s_client: kube::Client,
}

impl MaestroK8sClientBuilder {
    pub async fn default() -> anyhow::Result<Self> {
        rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
        let builder = Self{
            namespace: None,
            k8s_client: kube::Client::try_default().await?,
        };

        Ok(builder)
    }

    pub fn with_namespace<S>(mut self, namespace: S) -> MaestroK8sClientBuilder
    where
        S: Into<String>,
    {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_kube_client<K>(mut self, kube_client: K) -> MaestroK8sClientBuilder
    where
        K: Into<kube::Client>
    {
        self.k8s_client = kube_client.into();
        self
    }

    pub fn build(self) -> anyhow::Result<MaestroK8sClient> {
        let k8s_client= self.k8s_client;
        
        let maestro_client = MaestroK8sClient {
            k8s_client,
            namespace: self.namespace,
        };

        Ok(maestro_client)
    }
}

pub struct MaestroK8sClient {
    k8s_client: kube::Client,
    namespace: Option<String>,
}

impl MaestroK8sClient {
    pub async fn builder() -> anyhow::Result<MaestroK8sClientBuilder> {
        MaestroK8sClientBuilder::default().await
    }

    pub async fn new() -> anyhow::Result<MaestroK8sClient> {
        MaestroK8sClientBuilder::default().await?.build()
    }

    pub async fn create_job(&self, job: &Job) -> anyhow::Result<MaestroWorkflow> {
        let api = 
            if let Some(namespace) = &self.namespace {
                Api::<Job>::namespaced(self.k8s_client.clone(), namespace)
            } else {
                Api::<Job>::default_namespaced(self.k8s_client.clone())
            };
        
        let post_parameters = PostParams::default();
        
        let created_job = api.create(&post_parameters, &job).await?;
        let maestro_job = MaestroWorkflow::new(&created_job, self.k8s_client.clone());

        Ok(maestro_job)
    }

    pub async fn create_configmap(&self, config_map: &ConfigMap) -> anyhow::Result<ConfigMap> {
        let api = 
            if let Some(namespace) = &self.namespace {
                Api::<ConfigMap>::namespaced(self.k8s_client.clone(), namespace)
            } else {
                Api::<ConfigMap>::default_namespaced(self.k8s_client.clone())
            };

        let post_parameters = PostParams::default();
        let created = api.create(&post_parameters, &config_map).await?;
        Ok(created)
    }

    pub async fn delete_configmap(&self, name: &str) -> anyhow::Result<()> {
        let api = 
            if let Some(namespace) = &self.namespace {
                Api::<ConfigMap>::namespaced(self.k8s_client.clone(), namespace)
            } else {
                Api::<ConfigMap>::default_namespaced(self.k8s_client.clone())
            };
        let delete_parameters = DeleteParams::default();
        api.delete(name, &delete_parameters).await?;
        
        Ok(())
    }

    pub async fn create_secret(&self, name: &str) -> anyhow::Result<()> {
        let api = 
            if let Some(namespace) = &self.namespace {
                Api::<Secret>::namespaced(self.k8s_client.clone(), namespace)
            } else {
                Api::<Secret>::default_namespaced(self.k8s_client.clone())
            };

        let delete_parameters = DeleteParams::default();
        api.delete(name, &delete_parameters).await?;
        
        Ok(())
    }

    pub async fn delete_secret(&self, name: &str) -> anyhow::Result<()> {
        let api = 
            if let Some(namespace) = &self.namespace {
                Api::<Secret>::namespaced(self.k8s_client.clone(), namespace)
            } else {
                Api::<Secret>::default_namespaced(self.k8s_client.clone())
            };
        let delete_parameters = DeleteParams::default();
        api.delete(name, &delete_parameters).await?;
        
        Ok(())
    }

}