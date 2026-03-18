//! This example demonstrates building workflows using the WorkflowBuilder API.
//!
//! The example shows:
//! - Creating a workflow with a single Kubernetes Job step
//! - Configuring container arguments, environment variables, and resource limits
//! - Setting job-level configurations like backoff limit and restart policy
//! - Executing the workflow and waiting for completion
//! - Proper cleanup of workflow resources

use std::collections::BTreeMap;

use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::{
        container::{
            ComputeResource, EnvironmentVariableFromObject, EnvironmentVariableSource,
            MaestroContainer,
        },
        job::{JobBuilder, JobNameType, RestartPolicy},
    },
};
use k8s_openapi::{api::batch::v1::Job, apimachinery::pkg::api::resource::Quantity};

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
    let test_job_input = build_job(&image, &job_name, &namespace)?;
    println!("{}", serde_yml::to_string(&test_job_input)?);

    println!("Applying job to Kubernetes cluster...");
    let succeed_job = maestro_client
        .create_job(&test_job_input, namespace, dry_run)
        .await?;

    println!("Waiting for job completion...");
    succeed_job.wait().await?;

    println!("Cleaning up job resources...");
    succeed_job.delete_job(dry_run).await?;

    Ok(())
}

fn build_job(image: &str, name: &str, namespace: &str) -> anyhow::Result<Job> {
    println!("Configuring job: {} in namespace: {}", name, namespace);

    let job_name = JobNameType::DefinedName(name.to_owned());
    let container_name = "main";

    println!("Setting up environment variables from secrets...");
    let environment_from_object = vec![EnvironmentVariableFromObject::Secret("s3-storage".into())];

    println!("Configuring resource limits (CPU: 100m, Memory: 50M)...");
    let resource_bounds: BTreeMap<ComputeResource, Quantity> = vec![
        (ComputeResource::Cpu, Quantity("100m".to_owned())),
        (ComputeResource::Memory, Quantity("50M".to_owned())),
    ]
    .into_iter()
    .collect();

    println!("Setting environment variables...");
    let environment_variables = vec![(
        "MAESTRO_TEST".to_owned(),
        EnvironmentVariableSource::Value("MAESTRO_TEST_VARIABLE".to_owned()),
    )]
    .into_iter()
    .collect();

    println!("Building container with image: {}", image);
    let container = MaestroContainer::new(image, container_name)
        .set_arguments(&vec![
            "bash".to_owned(),
            "-c".to_owned(),
            "echo 'Testing pod'; sleep 3; echo 'Finalizado'".to_owned(),
        ])
        .set_environment_variables_from_objects(&environment_from_object)
        .set_environment_variables(environment_variables)
        .set_resource_bounds(resource_bounds);

    println!("Building job with backoff limit: 4, restart policy: OnFailure");
    let job = JobBuilder::new(&job_name, namespace)
        .set_backoff_limit(4)
        .set_restart_policy(&RestartPolicy::OnFailure)
        .add_container(Box::new(container))?
        .build()?;

    Ok(job)
}
