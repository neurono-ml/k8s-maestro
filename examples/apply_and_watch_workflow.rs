//! This example demonstrates applying and watching workflow execution in real-time.
//!
//! The example shows:
//! - Creating multiple workflow steps using KubeJobStepBuilder
//! - Building workflows with different configurations (success and failure cases)
//! - Applying workflows to Kubernetes in parallel
//! - Watching workflow execution and completion status
//! - Handling successful and failed workflows
//! - Demonstrating dry-run mode for testing

use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::MaestroContainer,
    steps::{KubeJobStepBuilder, RestartPolicy},
    steps::traits::DeletableWorkFlowStep,
};
use kube::Api;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Error);

    let succeed_name = "succeed-job";
    let failing_name = "failing-job";
    let namespace = "staging";
    let dry_run = false; // Set to true for testing without actual resource creation

    println!("=== Apply and Watch Workflow Example ===\n");

    println!("Creating Maestro Kubernetes client...");
    let maestro_client = MaestroK8sClient::new().await?;

    println!("Building workflow steps:");
    println!("  - '{}' (will succeed)", succeed_name);
    println!("  - '{}' (will fail with exit 137)\n", failing_name);

    // Create the success job step
    let success_container = Box::new(MaestroContainer::new("docker.io/bash:5.2", "main")
        .set_arguments(&[
            "bash".to_owned(),
            "-c".to_owned(),
            "echo 'Testing success pod'; sleep 3; echo 'Job completed successfully!'".to_owned(),
        ]));

    let success_job_step = KubeJobStepBuilder::new()
        .with_name(succeed_name)
        .with_namespace(namespace)
        .add_container(success_container)
        .with_backoff_limit(2)
        .with_restart_policy(RestartPolicy::OnFailure)
        .with_client(maestro_client.clone())
        .with_dry_run(dry_run)
        .build()?;

    // Create the failing job step
    let failing_container = Box::new(MaestroContainer::new("docker.io/bash:5.2", "main")
        .set_arguments(&[
            "bash".to_owned(),
            "-c".to_owned(),
            "echo 'Testing failing pod'; sleep 3; exit 137".to_owned(),
        ]));

    let failing_job_step = KubeJobStepBuilder::new()
        .with_name(failing_name)
        .with_namespace(namespace)
        .add_container(failing_container)
        .with_backoff_limit(2)
        .with_restart_policy(RestartPolicy::OnFailure)
        .with_client(maestro_client.clone())
        .with_dry_run(dry_run)
        .build()?;

    if !dry_run {
        println!("Applying workflows to Kubernetes cluster...");

        // Build the Kubernetes Job specifications
        let success_job = build_sleep_job(succeed_name, namespace)?;
        let failing_job = build_failing_job(failing_name, namespace);

        // Create Kubernetes API client
        let jobs_api = Api::namespaced(maestro_client.inner().clone(), namespace);

        // Apply both jobs
        let success_created = jobs_api
            .create(&Default::default(), &success_job)
            .await?;
        println!("Success job '{}' created", success_created.metadata.name.as_ref().unwrap());

        let failing_created = jobs_api
            .create(&Default::default(), &failing_job)
            .await?;
        println!("Failing job '{}' created", failing_created.metadata.name.as_ref().unwrap());

        println!("\n=== Watching workflows execute in parallel ===\n");

        // Wait for both jobs to complete
        println!("Waiting for jobs to complete...");
        tokio::time::sleep(Duration::from_secs(8)).await;

        // Check the status of both jobs
        println!("\n=== Checking job statuses ===\n");

        let success_job_status = jobs_api.get(succeed_name).await?;
        if let Some(status) = success_job_status.status {
            let succeeded = status.succeeded.unwrap_or(0);
            let failed = status.failed.unwrap_or(0);
            println!("Success job status - Succeeded: {}, Failed: {}", succeeded, failed);
        }

        let failing_job_status = jobs_api.get(failing_name).await?;
        if let Some(status) = failing_job_status.status {
            let succeeded = status.succeeded.unwrap_or(0);
            let failed = status.failed.unwrap_or(0);
            println!("Failing job status - Succeeded: {}, Failed: {}", succeeded, failed);
        }

        // Demonstrate cleanup
        println!("\n=== Cleaning up workflow resources ===\n");

        println!("Cleaning up success job...");
        success_job_step.delete_associated_pods(dry_run).await?;
        success_job_step.delete_workflow(dry_run).await?;
        println!("Success job cleaned up");

        println!("Cleaning up failing job...");
        failing_job_step.delete_associated_pods(dry_run).await?;
        failing_job_step.delete_workflow(dry_run).await?;
        println!("Failing job cleaned up");

    } else {
        println!("DRY RUN: Would create job '{}'", succeed_name);
        println!("DRY RUN: Would create job '{}'", failing_name);
        println!("DRY RUN: Would wait for both jobs to complete");
        println!("DRY RUN: Would check job statuses");
        println!("DRY RUN: Would clean up both jobs");
    }

    println!("\n=== Workflow execution complete! ===");
    Ok(())
}

/// Builds a Kubernetes Job that sleeps and then exits successfully.
///
/// This function demonstrates building a basic Kubernetes Job with:
/// - A single container running bash
/// - A sleep command to simulate work
/// - OnFailure restart policy for retry on failure
fn build_sleep_job(name: &str, namespace: &str) -> anyhow::Result<k8s_openapi::api::batch::v1::Job> {
    use k8s_openapi::{
        api::{
            batch::v1::{Job, JobSpec},
            core::v1::{Container, PodSpec, PodTemplateSpec},
        },
        apimachinery::pkg::apis::meta::v1::ObjectMeta,
    };

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
        backoff_limit: Some(2),
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

/// Builds a Kubernetes Job that sleeps and then exits with error code 137.
///
/// This function demonstrates building a failing Kubernetes Job:
/// - A single container running bash
/// - A sleep command followed by an error exit
/// - OnFailure restart policy for retry on failure
/// - Used to test error handling and retry behavior
fn build_failing_job(name: &str, namespace: &str) -> k8s_openapi::api::batch::v1::Job {
    use k8s_openapi::{
        api::{
            batch::v1::{Job, JobSpec},
            core::v1::{Container, PodSpec, PodTemplateSpec},
        },
        apimachinery::pkg::apis::meta::v1::ObjectMeta,
    };

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
        backoff_limit: Some(2),
        ..Default::default()
    };

    Job {
        metadata: ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            ..Default::default()
        },
        spec: Some(job_spec),
        ..Default::default()
    }
}
