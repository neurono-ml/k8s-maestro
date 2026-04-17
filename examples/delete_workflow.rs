//! This example demonstrates proper cleanup of workflow resources.
//!
//! The example shows:
//! - Creating a workflow step using KubeJobStepBuilder
//! - Executing the workflow step and waiting for completion
//! - Deleting associated pods before deleting the job
//! - Deleting the job resource itself
//! - Proper cleanup order to ensure clean resource removal
//! - Dry run mode for testing without actual resource creation

use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::MaestroContainer,
    steps::traits::DeletableWorkFlowStep,
    steps::{KubeJobStepBuilder, RestartPolicy},
};
use kube::Api;

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Error);

    let job_name = "cleanup-test-job";
    let namespace = "staging";
    let dry_run = false; // Set to true for testing without actual resource creation

    println!("=== Workflow Cleanup Example ===\n");

    println!("Creating Maestro Kubernetes client...");
    let maestro_client = MaestroK8sClient::new().await?;

    println!("Creating workflow job step: {}", job_name);
    let container = Box::new(
        MaestroContainer::new("docker.io/bash:5.2", "main").set_arguments(&[
            "bash".to_owned(),
            "-c".to_owned(),
            "echo 'Testing pod'; sleep 3; echo 'Finalizado'; exit 137".to_owned(),
        ]),
    );

    let job_step = KubeJobStepBuilder::new()
        .with_name(job_name)
        .with_namespace(namespace)
        .add_container(container)
        .with_backoff_limit(5)
        .with_restart_policy(RestartPolicy::OnFailure)
        .with_client(maestro_client.clone())
        .with_dry_run(dry_run)
        .build()?;

    println!("Applying job to Kubernetes cluster...");
    // Create the job using Kubernetes API directly
    let jobs_api = Api::namespaced(maestro_client.inner().clone(), namespace);

    if !dry_run {
        // Build the Kubernetes Job specification
        let k8s_job = build_kubernetes_job(job_name, namespace)?;

        let created_job = jobs_api.create(&Default::default(), &k8s_job).await?;

        let created_job_name = created_job.metadata.name.as_ref().unwrap();
        println!("Job '{}' created successfully", created_job_name);

        // Wait for job execution
        println!("Waiting for job completion...");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Check job status
        let job = jobs_api.get(created_job_name).await?;
        if let Some(status) = job.status {
            let succeeded = status.succeeded.unwrap_or(0);
            let failed = status.failed.unwrap_or(0);
            println!("Job status - Succeeded: {}, Failed: {}", succeeded, failed);
        }

        // Demonstrate cleanup using the step traits
        println!("\n=== Cleanup Operations ===\n");

        println!("Deleting associated pods first (best practice)...");
        job_step.delete_associated_pods(dry_run).await?;
        println!("Associated pods deleted");

        println!("Deleting the job resource...");
        job_step.delete_workflow(dry_run).await?;
        println!("Job resource deleted");
    } else {
        println!("DRY RUN: Would create job '{}'", job_name);
        println!("DRY RUN: Would wait for completion");
        println!("DRY RUN: Would delete associated pods");
        println!("DRY RUN: Would delete job resource");
    }

    println!("\n=== Cleanup complete! ===");
    Ok(())
}

/// Builds a Kubernetes Job specification for cleanup testing.
///
/// This function demonstrates building a Kubernetes Job that:
/// - Runs a bash container with a sleep command
/// - Exits with error code 137 to demonstrate cleanup
/// - Uses OnFailure restart policy with retry attempts
fn build_kubernetes_job(
    name: &str,
    namespace: &str,
) -> anyhow::Result<k8s_openapi::api::batch::v1::Job> {
    use k8s_openapi::{
        api::{
            batch::v1::{Job, JobSpec},
            core::v1::{Container, PodSpec, PodTemplateSpec},
        },
        apimachinery::pkg::apis::meta::v1::ObjectMeta,
    };

    let container = Container {
        name: "main".to_owned(),
        image: Some("docker.io/bash:5.2".to_owned()),
        args: Some(vec![
            "bash".to_owned(),
            "-c".to_owned(),
            "echo 'Testing pod'; sleep 3; echo 'Finalizado'; exit 137".to_owned(),
        ]),
        ..Default::default()
    };

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
        backoff_limit: Some(5),
        ..Default::default()
    };

    Ok(Job {
        metadata: ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            ..Default::default()
        },
        spec: Some(job_spec),
        ..Default::default()
    })
}
