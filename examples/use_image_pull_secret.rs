use k8s_maestro::{clients::MaestroK8sClient, entities::{container::{EnvironmentVariableFromObject, MaestroContainer}, job::{JobBuilder, JobNameType}}};
use k8s_openapi::api::batch::v1::Job;


const GHCR_IMAGE_PULL_SECRET: &str = "oci-registry";


#[tokio::main(flavor="current_thread")]
pub async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Error);
    let bucket = "mobilidade-ne";
    let prefix = "creation_date=2024-01-01";
    let glob_pattern = "creation_date=2024-01-01/*.254*.parquet";
    let output_path= "s3://mobilidade-etl-polars/argo/output/";

    let backoff_limit = 5;
    let job_generate_name = "maestro";
    let namespace = "mobilidade";
    let image = "ghcr.io/kognitalab/infra_s3_glob_list:v3.0.0";
    let dry_run = false;

    let maestro_client = MaestroK8sClient::new().await?;
    let test_job_input = build_job(
        &image, &job_generate_name, &namespace, backoff_limit,
        bucket, prefix, glob_pattern, output_path
    )?;

    let list_job = maestro_client.create_job(&test_job_input, namespace, dry_run).await?;
    list_job.wait().await?;
    list_job.delete_job(dry_run).await?;

    Ok(())
}


pub fn build_job(image: &str, generate_name: &str, namespace: &str, backoff_limit: usize,
    bucket: &str, prefix: &str, glob_pattern: &str, output_path: &str
) -> anyhow::Result<Job> {
    let job_name = JobNameType::GenerateName(generate_name.to_owned());
    let container_name = "main";

    let s3_environment_variables_secret =
        EnvironmentVariableFromObject::Secret("s3-storage-ne1".into());

    let container = MaestroContainer::new(image, &container_name)
        .add_arguments(&["--bucket", bucket])
        .add_arguments(&["--prefix", prefix])
        .add_arguments(&["--glob-pattern", glob_pattern])
        .add_arguments(&["--output-path", output_path])
        .add_environment_variables_from_object(&s3_environment_variables_secret);

    let job = JobBuilder::new(&job_name, namespace)
        .set_backoff_limit(backoff_limit)
        .set_restart_policy(&k8s_maestro::entities::job::RestartPolicy::OnFailure)
        .add_container(Box::new(container))?
        .add_image_pull_secret_name(GHCR_IMAGE_PULL_SECRET)
        .build()?;

    Ok(job)
}