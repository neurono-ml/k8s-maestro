use k8s_openapi::api::{batch::v1::Job, core::v1::{ConfigMap, Secret}};
use kube::{api::{DeleteParams, PostParams}, runtime::reflector::Lookup, Api};
use crate::entities::job::MaestroJob;

pub struct MaestroK8sClient {
    client: kube::Client
}

impl MaestroK8sClient {
    /// Create a Maestro client using the default Kuberntes environment
    pub async fn new() -> anyhow::Result<MaestroK8sClient> {
        let client = kube::Client::try_default().await?;
        let k8s_client = MaestroK8sClient{ client };

        Ok(k8s_client)
    }

    pub async fn create_job(&self, job: &Job, namespace: &str, dry_run: bool) -> anyhow::Result<MaestroJob> {
        let api = Api::<Job>::namespaced(self.client.clone(), namespace);
        let post_parameters = PostParams{ dry_run, ..PostParams::default()};
        
        let created_job = api.create(&post_parameters, &job).await?;
        let maestro_job = MaestroJob::new(&created_job, self.client.clone());

        Ok(maestro_job)
    }

    pub async fn create_configmap(&self, config_map: &ConfigMap, namespace: &str, dry_run: bool) -> anyhow::Result<ConfigMap> {
        let api = Api::<ConfigMap>::namespaced(self.client.clone(), namespace);
        let post_parameters = PostParams{ dry_run, ..PostParams::default()};
        let created = api.create(&post_parameters, &config_map).await?;
        Ok(created)
    }

    pub async fn delete_configmap(&self, name: &str, namespace: &str, dry_run: bool) -> anyhow::Result<()> {
        let api = Api::<ConfigMap>::namespaced(self.client.clone(), namespace);
        let delete_parameters = DeleteParams{ dry_run, ..DeleteParams::default()};
        api.delete(name, &delete_parameters).await?;
        
        Ok(())
    }

    pub async fn create_secret(&self, name: &str, namespace: &str, dry_run: bool) -> anyhow::Result<()> {
        let api = Api::<Secret>::namespaced(self.client.clone(), namespace);
        let delete_parameters = DeleteParams{ dry_run, ..DeleteParams::default()};
        api.delete(name, &delete_parameters).await?;
        
        Ok(())
    }
}