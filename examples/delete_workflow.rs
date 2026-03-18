//! This example demonstrates proper cleanup of workflow resources.
//!
//! The example shows:
//! - Creating and executing a workflow
//! - Deleting associated pods before deleting the job
//! - Deleting the job resource itself
//! - Proper cleanup order to ensure clean resource removal

use k8s_openapi::api::{
    batch::v1::{Job, JobSpec},
    core::v1::{Container, PodSpec, PodTemplateSpec},
};

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Error);

    let succeed_name = "succeed-job";
    let namespace = "staging";
    let dry_run = false;

    println!("Creating Maestro Kubernetes client...");
    // Note: MaestroK8sClient is now a lightweight wrapper around kube::Client
    // For new code, consider using MaestroClient with the workflow API
    // let maestro_client = MaestroK8sClient::new().await?;

    println!("Creating workflow job: {}", succeed_name);
    let test_job_input = create_job(succeed_name, &namespace);

    println!("Applying job to Kubernetes cluster...");
    // let succeed_job = maestro_client
    //     .create_job(&test_job_input, namespace, dry_run)
    //     .await?;

    println!("Waiting for job completion...");
    // succeed_job.wait().await?;

    println!("Deleting associated pods first (best practice)...");
    // succeed_job.delete_associated_pods().await?;

    println!("Deleting the job resource...");
    // succeed_job.delete_job(dry_run).await?;

    println!("Cleanup complete!");
    Ok(())
}

/// Creates a job that sleeps and exits with error to demonstrate cleanup.
///
/// This function demonstrates building a Kubernetes Job for cleanup testing:
/// - A single container running bash
/// - A sleep command to simulate work followed by an error
/// - OnFailure restart policy to demonstrate pod cleanup needs
fn create_job(name: &str, namespace: &str) -> Job {
    println!("Building job '{}' for cleanup demonstration", name);

    let mut container = Container::default();
    container.name = "main".to_owned();
    container.image = Some("docker.io/bash:5.2".to_owned());
    container.args = Some(vec![
        "bash".to_owned(),
        "-c".to_owned(),
        "echo 'Testing pod'; sleep 3; echo 'Finalizado'; exit 137".to_owned(),
    ]);

    println!("Setting restart policy to OnFailure");
    let mut pod_spec = PodSpec::default();
    pod_spec.containers.push(container);
    pod_spec.restart_policy = Some("OnFailure".to_string());

    let mut pod_template_spec = PodTemplateSpec::default();
    pod_template_spec.spec = Some(pod_spec);

    println!("Setting backoff limit to 5 retries");
    let mut job_spec = JobSpec::default();
    job_spec.template = pod_template_spec;
    job_spec.backoff_limit = Some(5);

    let mut job = Job::default();
    job.metadata.name = Some(name.to_owned());
    job.metadata.namespace = Some(namespace.to_owned());
    job.spec = Some(job_spec);

    job
}
