use k8s_openapi::api::core::v1::{ConfigMapVolumeSource, SecretVolumeSource, Volume};
use crate::entities::volumes::volume_like::VolumeLike;

use super::{k8s_volume_object_volume_source::K8sObjectVolumeSource, volume::MaestroObjectMountVolume};

impl VolumeLike for MaestroObjectMountVolume {
    fn into_volume(&self) -> anyhow::Result<Volume> {
        
        let volume =
            match self.k8s_object.clone() {
                K8sObjectVolumeSource::ConfigMap(config_map_name) => {
                    let configmap_volume_source = ConfigMapVolumeSource{
                        optional: Some(true),
                        name: Some(config_map_name),
                        ..Default::default()
                    };

                    Volume {
                        name: self.volume_name.clone(),
                        config_map: Some(configmap_volume_source),
                        ..Volume::default()
                    }
                },
                K8sObjectVolumeSource::Secret(secret_name) => {
                    let secret_volume_source = SecretVolumeSource{
                        optional: Some(true),
                        secret_name: Some(secret_name),
                        ..Default::default()
                    };

                    Volume {
                        name: self.volume_name.clone(),
                        secret: Some(secret_volume_source),
                        ..Volume::default()
                    }
                },
            };
        
        Ok(volume)
    }
}