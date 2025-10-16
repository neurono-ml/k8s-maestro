mod maestro_empty_dir_volume;
mod maestro_object_volume;
mod maestro_pvc_mount_volume;
mod volume_like;
mod volume_mount_like;

pub use volume_like::VolumeLike;
pub use volume_mount_like::{PVCVolumeMountLike, VolumeMountLike};

pub use maestro_pvc_mount_volume::{MaestroPVCMountVolume, MaestroPVCMountVolumeBuilder};
pub use maestro_object_volume::{MaestroObjectMountVolume, MaestroObjectMountVolumeBuilder, K8sObjectVolumeSource};

pub use maestro_empty_dir_volume::{EmptyDirMedium, MaestroEmptydirMountVolume, MaestroEmptydirMountVolumeBuilder};