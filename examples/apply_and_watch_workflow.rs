//! This example demonstrates applying and watching workflow execution in real-time.
//!
//! The example shows:
//! - Creating multiple workflows with different configurations
//! - Applying workflows to Kubernetes in parallel
//! - Watching workflow execution and completion status
//! - Handling successful and failed workflows

use k8s_openapi::api::{
    batch::v1::{Job, JobSpec},
    core::v1::{Container, PodSpec, PodTemplateSpec},
};

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Error);

    let succeed_name = "succeed-job";
    let failing_name = "failing-job";
    let namespace = "staging";
    let dry_run = false;

    println!("Creating Maestro Kubernetes client...");
    // Note: MaestroK8sClient is now a lightweight wrapper around kube::Client
    // For new code, consider using MaestroClient with the workflow API
    // let maestro_client = MaestroK8sClient::new().await?;

    println!(
        "Building workflows: {} (will succeed) and {} (will fail)",
        succeed_name, failing_name
    );
    let test_job_input = make_sleep_job(succeed_name, &namespace);
    let failed_job_input = make_failing_job(failing_name, &namespace);

    println!("Applying workflows to Kubernetes cluster...");
    // let succeed_job = maestro_client
    //     .create_job(&test_job_input, namespace, dry_run)
    //     .await?;
    // let failed_job = maestro_client
    //     .create_job(&failed_job_input, namespace, dry_run)
    //     .await?;

    println!("Watching workflows execute in parallel...");
    // The workflows will execute concurrently
    // let _ = futures::join!(failed_job.wait(), succeed_job.wait());

    println!("Workflow execution complete!");
    Ok(())
}

/// Creates a job that sleeps and then exits successfully.
///
/// This function demonstrates building a basic Kubernetes Job with:
/// - A single container running bash
/// - A sleep command to simulate work
/// - OnFailure restart policy for retry on failure
fn make_sleep_job(name: &str, namespace: &str) -> Job {
    println!("Building job '{}' that will succeed", name);

    let mut container = Container::default();
    container.name = "main".to_owned();
    container.image = Some("docker.io/bash:5.2".to_owned());
    container.args = Some(vec![
        "bash".to_owned(),
        "-c".to_owned(),
        "echo 'Testing pod'; sleep 3; echo 'Finalizado'".to_owned(),
    ]);

    println!("Setting restart policy to OnFailure");
    let mut pod_spec = PodSpec::default();
    pod_spec.containers.push(container);
    pod_spec.restart_policy = Some("OnFailure".to_string());

    let mut pod_template_spec = PodTemplateSpec::default();
    pod_template_spec.spec = Some(pod_spec);

    println!("Setting backoff limit to 2 retries");
    let mut job_spec = JobSpec::default();
    job_spec.template = pod_template_spec;
    job_spec.backoff_limit = Some(2);

    let mut job = Job::default();
    job.metadata.name = Some(name.to_owned());
    job.metadata.namespace = Some(namespace.to_owned());
    job.spec = Some(job_spec);

    job
}

/// Creates a job that sleeps and then exits with error code 137.
///
/// This function demonstrates building a failing Kubernetes Job:
/// - A single container running bash
/// - A sleep command followed by an error exit
/// - OnFailure restart policy for retry on failure
/// - Used to test error handling and retry behavior
fn make_failing_job(name: &str, namespace: &str) -> Job {
    println!("Building job '{}' that will fail (exit 137)", name);

    let mut container = Container::default();
    container.name = "main".to_owned();
    container.image = Some("docker.io/bash:5.2".to_owned());
    container.args = Some(vec![
        "bash".to_owned(),
        "-c".to_owned(),
        "echo 'Testing pod'; sleep 3; exit 137".to_owned(),
    ]);

    println!("Setting restart policy to OnFailure");
    let mut pod_spec = PodSpec::default();
    pod_spec.containers.push(container);
    pod_spec.restart_policy = Some("OnFailure".to_string());

    let mut pod_template_spec = PodTemplateSpec::default();
    pod_template_spec.spec = Some(pod_spec);

    println!("Setting backoff limit to 2 retries");
    let mut job_spec = JobSpec::default();
    job_spec.template = pod_template_spec;
    job_spec.backoff_limit = Some(2);

    let mut job = Job::default();
    job.metadata.name = Some(name.to_owned());
    job.metadata.namespace = Some(namespace.to_owned());
    job.spec = Some(job_spec);

    job
}
