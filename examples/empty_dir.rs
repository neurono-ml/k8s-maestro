use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::batch::v1::Job;
use k8s_maestro::{clients::MaestroK8sClient, entities::{builders::BuildJob,
    container::{ContainerLike, MaestroContainer}, job::{WorkflowStepBuilder, WorkflowNameType, RestartPolicy},
    volumes::{EmptyDirMedium, MaestroEmptydirMountVolumeBuilder}}};


#[tokio::main(flavor="current_thread")]
pub async fn main() -> anyhow::Result<()>{
    log::set_max_level(log::LevelFilter::Error);
    
    let job_name = "maestro";
    let namespace = "default";
    let image = "docker.io/bash:5.2";
    
    let maestro_client = MaestroK8sClient::new().await?;
    
    let test_job_input = build_job(&image, &job_name, &namespace)?;
    println!("{}", serde_yml::to_string(&test_job_input)?);

    let succeed_job = maestro_client.create_job(&test_job_input).await?;
    let log_stream = succeed_job.stream_logs(None);
    pin_mut!(log_stream);

    let _ = log_stream.await.map_ok(|log_line|{
        let message = log_line.rich_message();
        println!("{}", message);
    });

    
    succeed_job.wait().await?;
    
    Ok(())
}

fn build_job(image: &str, name: &str, namespace: &str) -> anyhow::Result<Job> {
    let job_name = WorkflowNameType::DefinedName(name.to_owned());

    let empty_dir_volume = MaestroEmptydirMountVolumeBuilder::new("/test", "empty")
        .set_medium(EmptyDirMedium::Memory)
        .set_size("1Mi")
        .build();
    
    let main_container =
        MaestroContainer::new(image, "main")
            .set_arguments(&vec![
                "bash".to_owned(),
                "-c".to_owned(),
                "ls /test/".to_owned()
            ])
            .add_volume_mount_like(Box::new(empty_dir_volume.clone()))?;

    let init_container =
        MaestroContainer::new(image, "init")
            .set_arguments(&vec![
                "bash".to_owned(),
                "-c".to_owned(),
                "touch /test/test-file".to_owned()
            ])
            .add_volume_mount_like(Box::new(empty_dir_volume))?;

    let job = 
        WorkflowStepBuilder::new(&job_name, namespace)
            .set_backoff_limit(0)
            .set_restart_policy(&RestartPolicy::OnFailure)
            .add_container(Box::new(main_container))?
            .add_init_container(Box::new(init_container))?
            .build_job()?;

    Ok(job)
}