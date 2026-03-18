use crate::entities::volumes::traits::VolumeMountLike;
use crate::entities::volumes::types::AccessMode;
use k8s_openapi::api::core::v1::{PersistentVolumeClaimVolumeSource, Volume, VolumeMount};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

/// Persistent Volume Claim volume configuration.
#[derive(Debug, Clone)]
pub struct PVCVolume {
    /// Volume name
    pub volume_name: String,
    /// Mount path in the container
    pub mount_path: String,
    /// PVC name
    pub pvc_name: String,
    /// Read only flag
    pub read_only: bool,
    /// Sub path within the volume
    pub sub_path: Option<String>,
    /// Storage class name
    pub storage_class: Option<String>,
    /// Access modes
    pub access_modes: Vec<AccessMode>,
    /// Storage size
    pub storage_size: Option<Quantity>,
}

/// Builder for creating Persistent Volume Claim volumes.
#[derive(Debug, Clone)]
pub struct MaestroPVCMountVolumeBuilder {
    volume_name: String,
    mount_path: String,
    pvc_name: String,
    read_only: bool,
    sub_path: Option<String>,
    storage_class: Option<String>,
    access_modes: Vec<AccessMode>,
    storage_size: Option<Quantity>,
}

impl MaestroPVCMountVolumeBuilder {
    /// Creates a new PVC volume builder.
    ///
    /// # Arguments
    ///
    /// * `mount_path` - Path where the volume will be mounted in the container
    /// * `pvc_name` - Name of the Persistent Volume Claim to reference
    /// * `volume_name` - Name to give to the volume
    pub fn new(
        mount_path: impl Into<String>,
        pvc_name: impl Into<String>,
        volume_name: impl Into<String>,
    ) -> Self {
        Self {
            mount_path: mount_path.into(),
            pvc_name: pvc_name.into(),
            volume_name: volume_name.into(),
            read_only: false,
            sub_path: None,
            storage_class: None,
            access_modes: Vec::new(),
            storage_size: None,
        }
    }

    /// Sets the storage class for the PVC.
    pub fn with_storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.storage_class = Some(storage_class.into());
        self
    }

    /// Sets the access modes for the PVC.
    pub fn with_access_modes(mut self, access_modes: Vec<AccessMode>) -> Self {
        self.access_modes = access_modes;
        self
    }

    /// Sets the storage size for the PVC.
    pub fn with_storage_size(mut self, size: impl Into<String>) -> Self {
        self.storage_size = Some(Quantity(size.into()));
        self
    }

    /// Sets the read-only flag for the volume mount.
    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Sets the sub path within the volume.
    pub fn with_sub_path(mut self, sub_path: impl Into<String>) -> Self {
        self.sub_path = Some(sub_path.into());
        self
    }

    /// Builds the PVC volume.
    pub fn build(self) -> PVCVolume {
        PVCVolume {
            volume_name: self.volume_name,
            mount_path: self.mount_path,
            pvc_name: self.pvc_name,
            read_only: self.read_only,
            sub_path: self.sub_path,
            storage_class: self.storage_class,
            access_modes: self.access_modes,
            storage_size: self.storage_size,
        }
    }
}

impl VolumeMountLike for PVCVolume {
    fn volume_name(&self) -> &str {
        &self.volume_name
    }

    fn mount_path(&self) -> &str {
        &self.mount_path
    }

    fn read_only(&self) -> bool {
        self.read_only
    }

    fn sub_path(&self) -> Option<&str> {
        self.sub_path.as_deref()
    }

    fn as_volume_mount(&self) -> VolumeMount {
        VolumeMount {
            name: self.volume_name.clone(),
            mount_path: self.mount_path.clone(),
            read_only: Some(self.read_only),
            sub_path: self.sub_path.clone(),
            mount_propagation: None,
            sub_path_expr: None,
        }
    }

    fn as_volume(&self) -> Volume {
        let pvc_source = PersistentVolumeClaimVolumeSource {
            claim_name: self.pvc_name.clone(),
            read_only: Some(self.read_only),
        };

        Volume {
            name: self.volume_name.clone(),
            persistent_volume_claim: Some(pvc_source),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pvc_builder_basic() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume").build();

        assert_eq!(volume.volume_name, "data-volume");
        assert_eq!(volume.mount_path, "/data");
        assert_eq!(volume.pvc_name, "my-pvc");
        assert_eq!(volume.read_only, false);
    }

    #[test]
    fn test_pvc_builder_with_storage_class() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")
            .with_storage_class("fast-ssd")
            .build();

        assert_eq!(volume.storage_class, Some("fast-ssd".to_string()));
    }

    #[test]
    fn test_pvc_builder_with_access_modes() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")
            .with_access_modes(vec![AccessMode::ReadWriteOnce])
            .build();

        assert_eq!(volume.access_modes.len(), 1);
        assert_eq!(volume.access_modes[0], AccessMode::ReadWriteOnce);
    }

    #[test]
    fn test_pvc_builder_with_read_only() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")
            .with_read_only(true)
            .build();

        assert_eq!(volume.read_only, true);
    }

    #[test]
    fn test_pvc_builder_with_storage_size() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")
            .with_storage_size("10Gi")
            .build();

        assert_eq!(volume.storage_size, Some(Quantity("10Gi".to_string())));
    }

    #[test]
    fn test_pvc_builder_with_sub_path() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")
            .with_sub_path("subdir")
            .build();

        assert_eq!(volume.sub_path, Some("subdir".to_string()));
    }

    #[test]
    fn test_pvc_volume_mount_like() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")
            .with_read_only(true)
            .build();

        assert_eq!(volume.volume_name(), "data-volume");
        assert_eq!(volume.mount_path(), "/data");
        assert!(volume.read_only());
    }

    #[test]
    fn test_pvc_as_volume_mount() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")
            .with_read_only(true)
            .build();

        let vm = volume.as_volume_mount();
        assert_eq!(vm.name, "data-volume");
        assert_eq!(vm.mount_path, "/data");
        assert_eq!(vm.read_only, Some(true));
    }

    #[test]
    fn test_pvc_as_volume() {
        let volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume").build();

        let vol = volume.as_volume();
        assert!(vol.persistent_volume_claim.is_some());
        assert_eq!(vol.persistent_volume_claim.unwrap().claim_name, "my-pvc");
    }
}
