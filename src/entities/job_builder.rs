use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::{Container, PersistentVolumeClaim, Volume}, apimachinery::{self, pkg::apis::meta::v1::LabelSelectorRequirement}};

use super::{container_like::ContainerLike, workflow_name_type::WorkflowNameType, restart_policy::RestartPolicy, utils::LabelOperator};


pub struct WorkflowStepBuilder {
    pub name: WorkflowNameType,
    pub namespace: String,
    pub backoff_limit: usize,

    pub restart_policy: RestartPolicy,
    pub containers: Vec<Box<dyn ContainerLike>>,
    pub init_containers: Vec<Box<dyn ContainerLike>>,
    pub image_pull_secret_names: Vec<String>,
    pub volumes: Vec<Volume>,
    pub pvcs: Vec<PersistentVolumeClaim>,
    pub num_replicas: Option<i32>,
    pub match_expression: Vec<apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>,
    pub match_labels: std::collections::BTreeMap<String, String>
}

impl WorkflowStepBuilder{
    pub fn new(name: &WorkflowNameType, namespace: &str) -> WorkflowStepBuilder {
        WorkflowStepBuilder {
            name: name.clone(),
            namespace: namespace.to_owned(),
            
            backoff_limit: 6,
            restart_policy: RestartPolicy::default(),
            image_pull_secret_names: Vec::new(),
            containers: Vec::new(),
            init_containers: Vec::new(),
            volumes: Vec::new(),
            pvcs: Vec::new(),
            num_replicas: Some(1),
            match_expression: Vec::new(),
            match_labels: BTreeMap::new()
        }
    }

    pub fn set_defined_name(mut self, defined_name: &str) -> WorkflowStepBuilder {
        self.name = WorkflowNameType::DefinedName(defined_name.to_owned());
        self
    }

    pub fn set_generate_name(mut self, defined_name: &str) -> WorkflowStepBuilder {
        self.name = WorkflowNameType::GenerateName(defined_name.to_owned());
        self
    }

    pub fn set_backoff_limit(mut self, backoff_limit: usize) -> WorkflowStepBuilder {
        self.backoff_limit = backoff_limit;
        self
    }

    pub fn set_restart_policy(mut self, restart_policy: &RestartPolicy) -> WorkflowStepBuilder {
        self.restart_policy = restart_policy.to_owned();
        self
    }

    pub fn add_container(mut self, container_like: Box<dyn ContainerLike>) -> anyhow::Result<WorkflowStepBuilder> {
        self.add_container_volumes(&container_like)?;
        self.containers.push(container_like);
        
        Ok(self)
    }

    pub fn add_init_container(mut self, container_like: Box<dyn ContainerLike>) -> anyhow::Result<WorkflowStepBuilder> {
        self.add_container_volumes(&container_like)?;
        self.init_containers.push(container_like);

        Ok(self)
    }

    pub fn add_image_pull_secret_name(mut self, image_pull_secret_name: &str) -> WorkflowStepBuilder {
        self.image_pull_secret_names.push(image_pull_secret_name.to_owned());
        self
    }

    pub fn set_image_pull_secret_names(mut self, image_pull_secret_names: Vec<String>)  -> WorkflowStepBuilder {
        self.image_pull_secret_names = image_pull_secret_names;
        self
    }
    
    fn add_container_volumes(&mut self, container_like: &Box<dyn ContainerLike>) -> Result<(), anyhow::Error> {
        self.add_container_volume_likes(container_like)?;

        let pvc_templates =
            container_like
                .get_pvcs()?;

        self.add_container_pvc_templates(pvc_templates);

        Ok(())
    }

    fn add_container_pvc_templates(&mut self, pvc_templates: Vec<PersistentVolumeClaim>) {
        for pvc_template in pvc_templates.into_iter() {
            if self.pvcs.contains(&pvc_template) {
                continue;
            } else {
                self.pvcs.push(pvc_template);
            }
        }
    }
    
    fn add_container_volume_likes(&mut self, container_like: &Box<dyn ContainerLike>) -> Result<(), anyhow::Error> {
        let container_volumes = container_like.get_volumes()?;

        Ok(for container_volume in container_volumes.iter() {
            if self.volumes.contains(container_volume) {
                continue
            } else {
                self.volumes.push(container_volume.clone());
            }
        })
    }
    
    pub fn set_num_replicas(mut self, num_replicas: i32) -> WorkflowStepBuilder {
        self.num_replicas = Some(num_replicas);
        self
    }

    pub fn add_label_selector_expression<K, V, D>(self, key: K, operator: LabelOperator, values: V) -> LabelSelectorRequirement
    where
        K: Into<String>, D: Into<String>, V: Into<Vec<D>>
    {
        let values_string = values.into().into_iter().map(|v| v.into()).collect();

        let label_selector = 
            LabelSelectorRequirement {
                operator: operator.to_string(),
                key: key.into(),
                values: Some(values_string)
            };

        label_selector
    }
}

pub fn extract_container_list(containers: &Vec<Box<dyn ContainerLike>>) -> Vec<Container>{
    containers.iter().rev().map(|container_line|{
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