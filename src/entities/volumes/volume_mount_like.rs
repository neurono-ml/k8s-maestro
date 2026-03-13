use std::fmt::Debug;

use dyn_clone::DynClone;
use k8s_openapi::api::core::v1::{PersistentVolumeClaim, VolumeMount};

use super::VolumeLike;

pub trait VolumeMountLike: Debug + DynClone {
    fn into_volume_mount(&self) -> anyhow::Result<VolumeMount>;

    fn mount_path(&self) -> anyhow::Result<String>;
    fn volume_name(&self) -> anyhow::Result<String>;
    
    fn volume_like(&self) -> anyhow::Result<Box<dyn VolumeLike>>;
    fn get_pvc(&self) -> anyhow::Result<Option<PersistentVolumeClaim>> {
        Ok(None)
    }
}

dyn_clone::clone_trait_object!(VolumeMountLike);

pub trait PVCVolumeMountLike: Debug + DynClone {
    fn try_into_pvc(&self) -> anyhow::Result<PersistentVolumeClaim>;
}
