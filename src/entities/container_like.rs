use k8s_openapi::api::core::v1::{Container, PersistentVolumeClaim, Volume};

use super::volumes::VolumeMountLike;

pub trait ContainerLike {
    fn into_container(&self) -> anyhow::Result<Container>;

    fn get_volumes(&self) -> anyhow::Result<Vec<Volume>>;

    fn get_pvcs(&self) -> anyhow::Result<Vec<PersistentVolumeClaim>>;
    
    fn add_volume_mount_like(self, volume_mount_like: Box<dyn VolumeMountLike>) -> anyhow::Result<Self> where Self: Sized;

}