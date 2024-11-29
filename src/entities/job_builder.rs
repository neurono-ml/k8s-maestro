use k8s_openapi::api::{batch::v1::{Job, JobSpec}, core::v1::{Container, LocalObjectReference, PodSpec, PodTemplateSpec, Volume}};
use kube::api::ObjectMeta;

use super::{container_like::ContainerLike, job_name_type::JobNameType, restart_policy::RestartPolicy};


pub struct JobBuilder {
    pub name: JobNameType,
    pub namespace: String,
    pub backoff_limit: usize,

    pub restart_policy: RestartPolicy,
    pub containers: Vec<Box<dyn ContainerLike>>,
    pub init_containers: Vec<Box<dyn ContainerLike>>,
    pub image_pull_secret_names: Vec<String>,
    pub volumes: Vec<Volume>
}

impl JobBuilder{
    pub fn new(name: &JobNameType, namespace: &str) -> JobBuilder {
        JobBuilder {
            name: name.clone(),
            namespace: namespace.to_owned(),
            
            backoff_limit: 6,
            restart_policy: RestartPolicy::default(),
            image_pull_secret_names: Vec::new(),
            containers: Vec::new(),
            init_containers: Vec::new(),
            volumes: Vec::new(),
        }
    }

    pub fn set_defined_name(mut self, defined_name: &str) -> JobBuilder {
        self.name = JobNameType::DefinedName(defined_name.to_owned());
        self
    }

    pub fn set_generate_name(mut self, defined_name: &str) -> JobBuilder {
        self.name = JobNameType::GenerateName(defined_name.to_owned());
        self
    }

    pub fn set_backoff_limit(mut self, backoff_limit: usize) -> JobBuilder {
        self.backoff_limit = backoff_limit;
        self
    }

    pub fn set_restart_policy(mut self, restart_policy: &RestartPolicy) -> JobBuilder {
        self.restart_policy = restart_policy.to_owned();
        self
    }

    pub fn add_container(mut self, container_like: Box<dyn ContainerLike>) -> anyhow::Result<JobBuilder> {
        self.add_container_volumes(&container_like)?;
        self.containers.push(container_like);
        
        Ok(self)
    }

    pub fn add_init_container(mut self, container_like: Box<dyn ContainerLike>) -> anyhow::Result<JobBuilder> {
        self.add_container_volumes(&container_like)?;
        self.init_containers.push(container_like);

        Ok(self)
    }

    pub fn add_image_pull_secret_name(mut self, image_pull_secret_name: &str) -> JobBuilder {
        self.image_pull_secret_names.push(image_pull_secret_name.to_owned());
        self
    }

    pub fn set_image_pull_secret_names(mut self, image_pull_secret_names: Vec<String>)  -> JobBuilder {
        self.image_pull_secret_names = image_pull_secret_names;
        self
    }

    pub fn build(self) -> anyhow::Result<Job> {
        let image_pull_secret_local_object_references =
            self.image_pull_secret_names.iter().map(|name| LocalObjectReference{
                name: name.to_owned(),
            }).collect();

        let pod_spec = PodSpec {
            restart_policy: self.restart_policy.into(),
            containers: extract_container_list(&self.containers),
            init_containers: Some(extract_container_list(&self.init_containers)),
            volumes: Some(self.volumes),
            image_pull_secrets: Some(image_pull_secret_local_object_references),
            ..PodSpec::default()
        };

        let pod_template_spec = PodTemplateSpec{
            spec: Some(pod_spec),
            ..PodTemplateSpec::default()
        };
                
        let job_spec = JobSpec{
            template: pod_template_spec,
            backoff_limit: Some(self.backoff_limit as i32),
            ..JobSpec::default()
        };
        
        let job_meta = match self.name {
            JobNameType::DefinedName(define_name) => ObjectMeta{
                name: Some(define_name.to_string()),
                namespace: Some(self.namespace.to_owned()),
                ..ObjectMeta::default()
            },
            JobNameType::GenerateName(generate_name) => ObjectMeta{
                generate_name: Some(generate_name.to_string()),
                namespace: Some(self.namespace.to_owned()),
                ..ObjectMeta::default()
            },
        };

        let job = Job{ 
            spec: Some(job_spec),
            metadata: job_meta,
            ..Job::default()
        };
        
        Ok(job)
    }

    fn add_container_volumes(&mut self, container_like: &Box<dyn ContainerLike>) -> Result<(), anyhow::Error> {
        let container_volumes = container_like.get_volumes()?;

        for container_volume in container_volumes.iter() {
            if self.volumes.contains(container_volume) {
                continue
            } else {
                self.volumes.push(container_volume.clone());
            }
        }

        Ok(())
    }
    
}

fn extract_container_list(containers: &Vec<Box<dyn ContainerLike>>) -> Vec<Container>{
    containers.iter().map(|container_line|{
        let container = container_line.into_container()?;
        anyhow::Ok(container.to_owned())
    }).filter_map(|container_result| {
        if let Ok(container) = container_result {
            Some(container)
        } else {
            None
        }
    })
    .collect()
}