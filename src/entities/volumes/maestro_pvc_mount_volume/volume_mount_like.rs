use k8s_openapi::api::core::v1::{PersistentVolumeClaim, PersistentVolumeClaimSpec, ResourceRequirements, VolumeMount};
use kube::api::ObjectMeta;

use crate::entities::volumes::{volume_mount_like::VolumeMountLike, VolumeLike};

use super::volume::MaestroPVCMountVolume;

impl VolumeMountLike for MaestroPVCMountVolume {
    fn into_volume_mount(&self) -> anyhow::Result<VolumeMount> {
        let volume_mount = 
            VolumeMount {
                mount_path: self.mount_path.to_owned(),
                name: self.volume_name.clone(),
                read_only: self.read_only,
                sub_path_expr: self.sub_path_expression.to_owned(),
                ..VolumeMount::default()
            };
        
        Ok(volume_mount)
    }

    fn mount_path(&self) -> anyhow::Result<String> {
        Ok(self.mount_path.to_owned())
    }

    fn volume_name(&self) -> anyhow::Result<String> {
        Ok(self.volume_name.to_owned())
    }
    
    fn volume_like(&self) -> anyhow::Result<Box<dyn VolumeLike>> {
        Ok(Box::new(self.clone()))
    }
    
    fn into_pvc(&self) -> anyhow::Result<PersistentVolumeClaim> {
        let pvc_spec_resources = ResourceRequirements{
            limits: Some(self.pvc_resource_limits.to_owned()),
            requests: Some(self.pvc_resource_requests.to_owned()),
            ..ResourceRequirements::default()
        };

        let access_modes =
            if self.access_modes.is_empty() {
                None
            } else {
                let access_modes_as_vec =
                    self.access_modes
                        .clone().into_iter()
                        .map(|mode| mode.to_string()).collect::<Vec<_>>();
                Some(access_modes_as_vec)
            };

        let pvc_spec = PersistentVolumeClaimSpec{
            access_modes: access_modes,
            storage_class_name: self.storage_class_name.to_owned(),
            resources: Some(pvc_spec_resources),
            ..PersistentVolumeClaimSpec::default()
        };

        let pvc_template_metadata = ObjectMeta {
            name: Some(self.volume_name.clone()),
            ..ObjectMeta::default()
        };
        let pvc_template = PersistentVolumeClaim {
            metadata: pvc_template_metadata,
            spec: Some(pvc_spec),
            ..PersistentVolumeClaim::default()
        };

        Ok(pvc_template)
    }
}