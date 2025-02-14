use std::fmt::Debug;

use dyn_clone::DynClone;
use k8s_openapi::api::core::v1::PersistentVolumeClaimSpec;

pub trait VolumeMountPvcTemplateLike: Debug + DynClone {
    fn into_pvc_spec(&self) -> anyhow::Result<PersistentVolumeClaimSpec>;
}

dyn_clone::clone_trait_object!(VolumeMountPvcTemplateLike);