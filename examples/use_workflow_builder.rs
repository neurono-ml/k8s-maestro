//! This example demonstrates building Kubernetes Jobs with MaestroContainer.
//!
//! The example shows:
//! - Creating a Kubernetes Job with a single container
//! - Configuring container arguments and resource limits
//! - Setting job-level configurations like backoff limit and restart policy
//! - Executing the job using the Kubernetes API directly

use std::collections::BTreeMap;

use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::{ContainerLike, MaestroContainer},
};
use k8s_openapi::{
    api::batch::v1::{Job, JobSpec},
    api::core::v1::{PodSpec, PodTemplateSpec},
    apimachinery::pkg::api::resource::Quantity,
};

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Error);

    let job_name = "maestro";
    let namespace = "staging";
    let image = "docker.io/bash:5.2";
    let dry_run = false;

    println!("Creating Maestro Kubernetes client...");
    let maestro_client = MaestroK8sClient::new().await?;

    println!("Building job workflow...");
    let test_job_input = build_job(image, job_name)?;
    println!("{}", serde_yml::to_string(&test_job_input)?);

    println!("Applying job to Kubernetes cluster...");

    // Create the job using Kubernetes API directly
    let jobs_api = kube::Api::<Job>::namespaced(maestro_client.inner().clone(), namespace);

    if !dry_run {
        let created_job = jobs_api
            .create(&Default::default(), &test_job_input)
            .await?;
        let job_name = created_job.metadata.name.as_ref().unwrap();

        println!("Job {} created, waiting for completion...", job_name);

        // Wait for job completion
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let job = jobs_api.get(job_name).await?;
        if let Some(status) = job.status {
            if status.succeeded.unwrap_or(0) > 0 {
                println!("Job completed successfully");
            } else if status.failed.unwrap_or(0) > 0 {
                println!("Job failed");
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

fn build_job(image: &str, name: &str) -> anyhow::Result<Job> {
    println!("Configuring job: {} in namespace: staging", name);

    let container_name = "main";

    println!("Configuring resource limits (CPU: 100m, Memory: 50M)...");
    let mut limits_map = BTreeMap::new();
    limits_map.insert("cpu".to_string(), Quantity("100m".to_owned()));
    limits_map.insert("memory".to_string(), Quantity("50M".to_owned()));

    println!("Building container with image: {}", image);
    let maestro_container = MaestroContainer::new(image, container_name).set_arguments(&[
        "bash".to_owned(),
        "-c".to_owned(),
        "echo 'Testing pod'; sleep 3; echo 'Finalizado'".to_owned(),
    ]);

    // Convert MaestroContainer to Kubernetes Container
    let mut container = ContainerLike::as_container(&maestro_container);
    container.resources = Some(k8s_openapi::api::core::v1::ResourceRequirements {
        limits: Some(limits_map),
        ..Default::default()
    });

    println!("Building job with backoff limit: 4, restart policy: OnFailure");
    let pod_spec = PodSpec {
        containers: vec![container],
        restart_policy: Some("OnFailure".to_string()),
        ..Default::default()
    };

    let pod_template_spec = PodTemplateSpec {
        spec: Some(pod_spec),
        ..Default::default()
    };

    let job_spec = JobSpec {
        template: pod_template_spec,
        backoff_limit: Some(4),
        ..Default::default()
    };

    Ok(Job {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(name.to_string()),
            ..Default::default()
        },
        spec: Some(job_spec),
        ..Default::default()
    })
}
