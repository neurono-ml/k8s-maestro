use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::{ContainerLike, MaestroContainer},
};
use k8s_openapi::{
    api::batch::v1::{Job, JobSpec},
    api::core::v1::{PodTemplateSpec, PodSpec, LocalObjectReference},
};

const GHCR_IMAGE_PULL_SECRET: &str = "oci-registry";

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Error);
    let bucket = "mobilidade-ne";
    let prefix = "creation_date=2024-01-01";
    let glob_pattern = "creation_date=2024-01-01/*.254*.parquet";
    let output_path = "s3://mobilidade-etl-polars/argo/output/";

    let backoff_limit = 5;
    let job_generate_name = "maestro";
    let namespace = "mobilidade";
    let image = "ghcr.io/kognitalab/infra_s3_glob_list:v3.0.0";
    let dry_run = false;

    let maestro_client = MaestroK8sClient::new().await?;
    let test_job_input = build_job(
        image,
        job_generate_name,
        namespace,
        backoff_limit,
        bucket,
        prefix,
        glob_pattern,
        output_path,
    )?;

    println!("{}", serde_yml::to_string(&test_job_input)?);

    // Create the job using Kubernetes API directly
    let jobs_api = kube::Api::<Job>::namespaced(maestro_client.inner().clone(), namespace);

    if !dry_run {
        let created_job = jobs_api.create(&Default::default(), &test_job_input).await?;
        let job_name = created_job.metadata.name.as_ref().unwrap();

        println!("Job {} created", job_name);

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

#[allow(clippy::too_many_arguments)]
pub fn build_job(
    image: &str,
    generate_name: &str,
    namespace: &str,
    backoff_limit: usize,
    bucket: &str,
    prefix: &str,
    glob_pattern: &str,
    output_path: &str,
) -> anyhow::Result<Job> {
    let container_name = "main";

    let maestro_container = MaestroContainer::new(image, container_name)
        .set_arguments(&["--bucket".to_owned(),
            bucket.to_owned(),
            "--prefix".to_owned(),
            prefix.to_owned(),
            "--glob-pattern".to_owned(),
            glob_pattern.to_owned(),
            "--output-path".to_owned(),
            output_path.to_owned()]);

    // Convert MaestroContainer to Kubernetes Container
    let container = ContainerLike::as_container(&maestro_container);

    // Create pod spec
    let mut pod_spec = PodSpec {
        containers: vec![container],
        restart_policy: Some("OnFailure".to_string()),
        ..Default::default()
    };

    // Add image pull secret
    pod_spec.image_pull_secrets = Some(vec![LocalObjectReference {
        name: GHCR_IMAGE_PULL_SECRET.to_string(),
    }]);

    // Create pod template spec
    let pod_template_spec = PodTemplateSpec {
        spec: Some(pod_spec),
        ..Default::default()
    };

    // Create job spec
    let job_spec = JobSpec {
        template: pod_template_spec,
        backoff_limit: Some(backoff_limit as i32),
        ..Default::default()
    };

    Ok(Job {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            generate_name: Some(generate_name.to_owned()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        spec: Some(job_spec),
        ..Default::default()
    })
}
