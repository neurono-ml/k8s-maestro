use k8s_openapi::{api::{apps::v1::{ReplicaSet, ReplicaSetSpec}, core::v1::{LocalObjectReference, PodSpec, PodTemplateSpec}}, apimachinery::pkg::apis::meta::v1::LabelSelector};
use kube::api::ObjectMeta;

use crate::entities::{job::{JobBuilder, JobNameType}, job_builder::extract_container_list};

pub trait BuildReplicaSet {
    fn build_replicaset(self) -> anyhow::Result<ReplicaSet>;
}

impl BuildReplicaSet for JobBuilder {
    fn build_replicaset(self) -> anyhow::Result<ReplicaSet> {
    
        let replicaset_metadata = match self.name {
            JobNameType::DefinedName(defined_name) => ObjectMeta {
                name: Some(defined_name.to_string()),
                namespace: Some(self.namespace.to_string()),
                ..ObjectMeta::default()
            },
            JobNameType::GenerateName(generate_name) => ObjectMeta{
                generate_name: Some(generate_name.to_string()),
                namespace: Some(self.namespace.to_owned()),
                ..ObjectMeta::default()
            }
        };

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

        let selector = if self.match_labels.is_empty() && self.match_expression.is_empty() {
            LabelSelector {
                match_labels: None,
                ..LabelSelector::default()
            }
        } else {
            LabelSelector {
                match_expressions: Some(self.match_expression),
                match_labels: Some(self.match_labels)
            }

        };

        let replicaset_spec = ReplicaSetSpec {
            replicas: self.num_replicas,
            selector: selector,
            template: Some(pod_template_spec),
            ..ReplicaSetSpec::default()
        };

        let replica_set = ReplicaSet {
            spec: Some(replicaset_spec),
            metadata: replicaset_metadata,
            ..ReplicaSet::default()
        };

        Ok(replica_set)
    }
}