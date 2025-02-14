use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::{Container, Volume}, apimachinery::{self, pkg::apis::meta::v1::LabelSelectorRequirement}};

use super::{container_like::ContainerLike, job_name_type::JobNameType, restart_policy::RestartPolicy};


pub struct JobBuilder {
    pub name: JobNameType,
    pub namespace: String,
    pub backoff_limit: usize,

    pub restart_policy: RestartPolicy,
    pub containers: Vec<Box<dyn ContainerLike>>,
    pub init_containers: Vec<Box<dyn ContainerLike>>,
    pub image_pull_secret_names: Vec<String>,
    pub volumes: Vec<Volume>,
    pub num_replicas: Option<i32>,
    pub match_expression: Vec<apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>,
    pub match_labels: std::collections::BTreeMap<String, String>
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
            num_replicas: Some(1),
            match_expression: Vec::new(),
            match_labels: BTreeMap::new()
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
    
    pub fn set_num_replicas(mut self, num_replicas: i32) -> JobBuilder {
        self.num_replicas = Some(num_replicas);
        self
    }

    pub fn add_label_selector_expression(self, ) -> LabelSelectorRequirement{
        todo!()
    }
}

pub fn extract_container_list(containers: &Vec<Box<dyn ContainerLike>>) -> Vec<Container>{
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