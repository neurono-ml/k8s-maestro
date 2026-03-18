//! This example demonstrates building Kubernetes Jobs with MaestroContainer.
//!
//! The example shows:
//! - Creating a Kubernetes Job with a single container
//! - Configuring container arguments and resource limits
//! - Setting job-level configurations like backoff limit
//! - Executing the job using the Kubernetes API directly

use std::collections::BTreeMap;

use futures::{pin_mut, StreamExt};
use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::{ComputeResource, ContainerLike, MaestroContainer},
    steps::ResourceLimits,
};
use k8s_openapi::{
    api::batch::v1::{Job, JobSpec},
    api::core::v1::{PodTemplateSpec, PodSpec, Container},
    apimachinery::pkg::api::resource::Quantity,
};

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Error);

    let job_generate_name = "maestro";
    let namespace = "argo";
    let image = "docker.io/bash:5.2";
    let dry_run = false;

    let maestro_client = MaestroK8sClient::new().await?;

    let test_job_input = build_job(&image, &job_generate_name)?;
    println!("{}", serde_yml::to_string(&test_job_input)?);

    // Create the job using Kubernetes API directly
    let jobs_api = kube::Api::<Job>::namespaced(maestro_client.inner().clone(), &namespace);

    if !dry_run {
        let created_job = jobs_api.create(&Default::default(), &test_job_input).await?;
        let job_name = created_job.metadata.name.as_ref().unwrap();

        println!("Job {} created, waiting for completion...", job_name);

        // Stream logs from the job
        let pods_api = kube::Api::<k8s_openapi::api::core::v1::Pod>::namespaced(
            maestro_client.inner().clone(),
            &namespace,
        );

        // Wait a bit for the pod to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let pod_list = pods_api
            .list(&Default::default())
            .await?
            .into_iter()
            .filter(|pod| {
                pod.metadata
                    .labels
                    .as_ref()
                    .and_then(|labels| labels.get("job-name"))
                    .map(|label| label.starts_with(job_name))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        if let Some(pod) = pod_list.first() {
            if let Some(pod_name) = pod.metadata.name.as_ref() {
                let logs = pods_api.logs(pod_name, &Default::default()).await?;
                println!("Job logs:\n{}", logs);
            }
        }

        // Delete the job
        jobs_api.delete(job_name, &Default::default()).await?;
        println!("Job deleted");
    } else {
        println!("DRY RUN: Job would be created");
    }

    Ok(())
}

fn build_job(image: &str, generate_name: &str) -> anyhow::Result<Job> {
    let container_name = "main";

    // Create resource limits
    let mut limits_map = BTreeMap::new();
    limits_map.insert("cpu".to_string(), Quantity("100m".to_owned()));
    limits_map.insert("memory".to_string(), Quantity("50M".to_owned()));

    // Create the MaestroContainer
    let maestro_container = MaestroContainer::new(image, container_name)
        .set_arguments(&vec![
            "bash".to_owned(),
            "-c".to_owned(),
            "ls /samba; sleep 10; exit 137".to_owned(),
        ]);

    // Convert MaestroContainer to Kubernetes Container
    let mut container = ContainerLike::as_container(&maestro_container);
    container.resources = Some(k8s_openapi::api::core::v1::ResourceRequirements {
        limits: Some(limits_map),
        ..Default::default()
    });

    // Create pod spec
    let pod_spec = PodSpec {
        containers: vec![container],
        restart_policy: Some("OnFailure".to_string()),
        ..Default::default()
    };

    // Create pod template spec
    let pod_template_spec = PodTemplateSpec {
        spec: Some(pod_spec),
        ..Default::default()
    };

    // Create job spec
    let job_spec = JobSpec {
        template: pod_template_spec,
        backoff_limit: Some(4),
        ..Default::default()
    };

    // Create job
    Ok(Job {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            generate_name: Some(generate_name.to_owned()),
            ..Default::default()
        },
        spec: Some(job_spec),
        ..Default::default()
    })
}
