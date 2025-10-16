use k8s_openapi::api::{batch::v1::{Job, JobSpec}, core::v1::{LocalObjectReference, PodSpec, PodTemplateSpec}};
use kube::api::ObjectMeta;

use crate::entities::{job::{WorkflowStepBuilder, WorkflowNameType}, job_builder::extract_container_list};

pub trait BuildJob {
    fn build_job(self) -> anyhow::Result<Job>;
}

impl BuildJob for WorkflowStepBuilder {
    fn build_job(self) -> anyhow::Result<Job> {
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
            WorkflowNameType::DefinedName(define_name) => ObjectMeta{
                name: Some(define_name.to_string()),
                namespace: Some(self.namespace.to_owned()),
                ..ObjectMeta::default()
            },
            WorkflowNameType::GenerateName(generate_name) => ObjectMeta{
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
}