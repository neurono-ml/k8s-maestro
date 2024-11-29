use dyn_clone::DynClone;
use k8s_openapi::api::core::v1::Volume;

pub trait VolumeLike: DynClone {
    fn into_volume(&self) -> anyhow::Result<Volume>;
}

dyn_clone::clone_trait_object!(VolumeLike);