use k8s_openapi::api::core::v1::{EmptyDirVolumeSource, Volume};
use crate::entities::volumes::volume_like::VolumeLike;

use super::volume::MaestroEmptydirMountVolume;

impl VolumeLike for MaestroEmptydirMountVolume {
    fn into_volume(&self) -> anyhow::Result<Volume> {
        
        let empty_dir = EmptyDirVolumeSource {
            medium: Some(self.medium.to_string()),
            size_limit: self.size.clone(),
        };

        let volume = Volume {
            name: self.volume_name.clone(),
            empty_dir: Some(empty_dir),
            ..Volume::default()
        };
        
        Ok(volume)
    }
}