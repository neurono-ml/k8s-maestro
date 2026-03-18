use crate::entities::volumes::traits::VolumeMountLike;
use crate::entities::volumes::types::Medium;
use k8s_openapi::api::core::v1::{EmptyDirVolumeSource, Volume, VolumeMount};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

/// EmptyDir volume configuration.
#[derive(Debug, Clone)]
pub struct EmptyDirVolume {
    /// Volume name
    pub volume_name: String,
    /// Mount path in the container
    pub mount_path: String,
    /// Read only flag
    pub read_only: bool,
    /// Sub path within the volume
    pub sub_path: Option<String>,
    /// Medium type
    pub medium: Option<Medium>,
    /// Size limit
    pub size_limit: Option<Quantity>,
}

/// Builder for creating EmptyDir volumes.
#[derive(Debug, Clone)]
pub struct EmptyDirVolumeBuilder {
    volume_name: String,
    mount_path: String,
    read_only: bool,
    sub_path: Option<String>,
    medium: Option<Medium>,
    size_limit: Option<Quantity>,
}

impl EmptyDirVolumeBuilder {
    /// Creates a new EmptyDir volume builder.
    ///
    /// # Arguments
    ///
    /// * `mount_path` - Path where the volume will be mounted in the container
    /// * `volume_name` - Name to give to the volume
    pub fn new(mount_path: impl Into<String>, volume_name: impl Into<String>) -> Self {
        Self {
            mount_path: mount_path.into(),
            volume_name: volume_name.into(),
            read_only: false,
            sub_path: None,
            medium: None,
            size_limit: None,
        }
    }

    /// Sets the medium type for the EmptyDir volume.
    pub fn with_medium(mut self, medium: Medium) -> Self {
        self.medium = Some(medium);
        self
    }

    /// Sets the size limit for the EmptyDir volume.
    pub fn with_size_limit(mut self, size: impl Into<String>) -> Self {
        self.size_limit = Some(Quantity(size.into()));
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

    /// Builds the EmptyDir volume.
    pub fn build(self) -> EmptyDirVolume {
        EmptyDirVolume {
            volume_name: self.volume_name,
            mount_path: self.mount_path,
            read_only: self.read_only,
            sub_path: self.sub_path,
            medium: self.medium,
            size_limit: self.size_limit,
        }
    }
}

impl VolumeMountLike for EmptyDirVolume {
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
        let mut empty_dir_source = EmptyDirVolumeSource {
            medium: None,
            size_limit: None,
        };

        if let Some(medium) = self.medium {
            let medium_str = match medium {
                Medium::Default => "".to_string(),
                Medium::Memory => "Memory".to_string(),
            };
            empty_dir_source.medium = Some(medium_str);
        }

        empty_dir_source.size_limit = self.size_limit.clone();

        Volume {
            name: self.volume_name.clone(),
            empty_dir: Some(empty_dir_source),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emptydir_builder_basic() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol").build();

        assert_eq!(volume.volume_name, "temp-vol");
        assert_eq!(volume.mount_path, "/tmp");
        assert_eq!(volume.read_only, false);
    }

    #[test]
    fn test_emptydir_builder_with_medium_memory() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_medium(Medium::Memory)
            .build();

        assert_eq!(volume.medium, Some(Medium::Memory));
    }

    #[test]
    fn test_emptydir_builder_with_medium_default() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_medium(Medium::Default)
            .build();

        assert_eq!(volume.medium, Some(Medium::Default));
    }

    #[test]
    fn test_emptydir_builder_with_size_limit() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_size_limit("1Gi")
            .build();

        assert_eq!(volume.size_limit, Some(Quantity("1Gi".to_string())));
    }

    #[test]
    fn test_emptydir_builder_with_read_only() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_read_only(true)
            .build();

        assert_eq!(volume.read_only, true);
    }

    #[test]
    fn test_emptydir_builder_with_sub_path() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_sub_path("subdir")
            .build();

        assert_eq!(volume.sub_path, Some("subdir".to_string()));
    }

    #[test]
    fn test_emptydir_volume_mount_like() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_read_only(true)
            .build();

        assert_eq!(volume.volume_name(), "temp-vol");
        assert_eq!(volume.mount_path(), "/tmp");
        assert!(volume.read_only());
    }

    #[test]
    fn test_emptydir_as_volume_mount() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_read_only(true)
            .build();

        let vm = volume.as_volume_mount();
        assert_eq!(vm.name, "temp-vol");
        assert_eq!(vm.mount_path, "/tmp");
        assert_eq!(vm.read_only, Some(true));
    }

    #[test]
    fn test_emptydir_as_volume() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol").build();

        let vol = volume.as_volume();
        assert!(vol.empty_dir.is_some());
    }

    #[test]
    fn test_emptydir_as_volume_with_medium_memory() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_medium(Medium::Memory)
            .build();

        let vol = volume.as_volume();
        let ed_source = vol.empty_dir.unwrap();
        assert_eq!(ed_source.medium, Some("Memory".to_string()));
    }

    #[test]
    fn test_emptydir_as_volume_with_size_limit() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_size_limit("1Gi")
            .build();

        let vol = volume.as_volume();
        let ed_source = vol.empty_dir.unwrap();
        assert_eq!(ed_source.size_limit, Some(Quantity("1Gi".to_string())));
    }

    #[test]
    fn test_emptydir_as_volume_default_medium() {
        let volume = EmptyDirVolumeBuilder::new("/tmp", "temp-vol")
            .with_medium(Medium::Default)
            .build();

        let vol = volume.as_volume();
        let ed_source = vol.empty_dir.unwrap();
        assert_eq!(ed_source.medium, Some("".to_string()));
    }
}
