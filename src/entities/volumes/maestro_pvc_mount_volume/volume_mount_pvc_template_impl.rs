use k8s_openapi::api::core::v1::{PersistentVolumeClaimSpec, ResourceRequirements};

use crate::entities::volumes::VolumeMountPvcTemplateLike;

use super::MaestroPVCMountVolume;

impl VolumeMountPvcTemplateLike for MaestroPVCMountVolume {
    fn into_pvc_spec(&self) -> anyhow::Result<PersistentVolumeClaimSpec> {
        let pvc_spec_resources = ResourceRequirements{
            limits: Some(self.pvc_resource_limits.to_owned()),
            requests: Some(self.pvc_resource_requests.to_owned()),
            ..ResourceRequirements::default()
        };

        let pvc_spec = PersistentVolumeClaimSpec{
            access_modes: self.access_mode.to_owned(),
            storage_class_name: self.storage_class_name.to_owned(),
            resources: Some(pvc_spec_resources),
            ..PersistentVolumeClaimSpec::default()
        };

        Ok(pvc_spec)
    }
}