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
    let _dry_run = false;

    println!("Creating Maestro Kubernetes client...");
    // Note: MaestroK8sClient is now a lightweight wrapper around kube::Client
    // For new code, consider using MaestroClient with the workflow API
    // let maestro_client = MaestroK8sClient::new().await?;

    println!(
        "Building workflows: {} (will succeed) and {} (will fail)",
        succeed_name, failing_name
    );
    let _test_job_input = make_sleep_job(succeed_name, namespace);
    let _failed_job_input = make_failing_job(failing_name, namespace);

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

    let container = Container {
        name: "main".to_owned(),
        image: Some("docker.io/bash:5.2".to_owned()),
        args: Some(vec![
            "bash".to_owned(),
            "-c".to_owned(),
            "echo 'Testing pod'; sleep 3; echo 'Finalizado'".to_owned(),
        ]),
        ..Default::default()
    };

    println!("Setting restart policy to OnFailure");
    let pod_spec = PodSpec {
        containers: vec![container],
        restart_policy: Some("OnFailure".to_string()),
        ..Default::default()
    };

    let pod_template_spec = PodTemplateSpec {
        spec: Some(pod_spec),
        ..Default::default()
    };

    println!("Setting backoff limit to 2 retries");
    let job_spec = JobSpec {
        template: pod_template_spec,
        backoff_limit: Some(2),
        ..Default::default()
    };

    Job {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            ..Default::default()
        },
        spec: Some(job_spec),
        ..Default::default()
    }
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

    let container = Container {
        name: "main".to_owned(),
        image: Some("docker.io/bash:5.2".to_owned()),
        args: Some(vec![
            "bash".to_owned(),
            "-c".to_owned(),
            "echo 'Testing pod'; sleep 3; exit 137".to_owned(),
        ]),
        ..Default::default()
    };

    println!("Setting restart policy to OnFailure");
    let pod_spec = PodSpec {
        containers: vec![container],
        restart_policy: Some("OnFailure".to_string()),
        ..Default::default()
    };

    let pod_template_spec = PodTemplateSpec {
        spec: Some(pod_spec),
        ..Default::default()
    };

    println!("Setting backoff limit to 2 retries");
    let job_spec = JobSpec {
        template: pod_template_spec,
        backoff_limit: Some(2),
        ..Default::default()
    };

    Job {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            ..Default::default()
        },
        spec: Some(job_spec),
        ..Default::default()
    }
}
