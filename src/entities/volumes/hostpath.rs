use crate::entities::volumes::traits::VolumeMountLike;
use crate::entities::volumes::types::HostPathType;
use k8s_openapi::api::core::v1::{HostPathVolumeSource, Volume, VolumeMount};

/// HostPath volume configuration.
#[derive(Debug, Clone)]
pub struct HostPathVolume {
    /// Volume name
    pub volume_name: String,
    /// Mount path in the container
    pub mount_path: String,
    /// Host path on the node
    pub host_path: String,
    /// Read only flag
    pub read_only: bool,
    /// Sub path within the volume
    pub sub_path: Option<String>,
    /// Host path type
    pub host_path_type: Option<HostPathType>,
}

/// Builder for creating HostPath volumes.
#[derive(Debug, Clone)]
pub struct HostPathVolumeBuilder {
    volume_name: String,
    mount_path: String,
    host_path: String,
    read_only: bool,
    sub_path: Option<String>,
    host_path_type: Option<HostPathType>,
}

impl HostPathVolumeBuilder {
    /// Creates a new HostPath volume builder.
    ///
    /// # Arguments
    ///
    /// * `mount_path` - Path where the volume will be mounted in the container
    /// * `host_path` - Path on the host node to mount
    /// * `volume_name` - Name to give to the volume
    pub fn new(
        mount_path: impl Into<String>,
        host_path: impl Into<String>,
        volume_name: impl Into<String>,
    ) -> Self {
        Self {
            mount_path: mount_path.into(),
            host_path: host_path.into(),
            volume_name: volume_name.into(),
            read_only: false,
            sub_path: None,
            host_path_type: None,
        }
    }

    /// Sets the host path type for validation.
    pub fn with_type(mut self, host_path_type: HostPathType) -> Self {
        self.host_path_type = Some(host_path_type);
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

    /// Builds the HostPath volume.
    pub fn build(self) -> HostPathVolume {
        HostPathVolume {
            volume_name: self.volume_name,
            mount_path: self.mount_path,
            host_path: self.host_path,
            read_only: self.read_only,
            sub_path: self.sub_path,
            host_path_type: self.host_path_type,
        }
    }
}

impl VolumeMountLike for HostPathVolume {
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
        let mut host_path_source = HostPathVolumeSource {
            path: self.host_path.clone(),
            type_: None,
        };

        if let Some(host_path_type) = self.host_path_type {
            let type_str = match host_path_type {
                HostPathType::Default => "".to_string(),
                HostPathType::Directory => "Directory".to_string(),
                HostPathType::File => "File".to_string(),
                HostPathType::Socket => "Socket".to_string(),
                HostPathType::BlockDevice => "BlockDevice".to_string(),
                HostPathType::CharDevice => "CharDevice".to_string(),
                HostPathType::DirectoryOrCreate => "DirectoryOrCreate".to_string(),
                HostPathType::FileOrCreate => "FileOrCreate".to_string(),
            };
            host_path_source.type_ = Some(type_str);
        }

        Volume {
            name: self.volume_name.clone(),
            host_path: Some(host_path_source),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostpath_builder_basic() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol").build();

        assert_eq!(volume.volume_name, "host-vol");
        assert_eq!(volume.mount_path, "/host-data");
        assert_eq!(volume.host_path, "/var/data");
        assert!(!volume.read_only);
    }

    #[test]
    fn test_hostpath_builder_with_type_directory() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
            .with_type(HostPathType::Directory)
            .build();

        assert_eq!(volume.host_path_type, Some(HostPathType::Directory));
    }

    #[test]
    fn test_hostpath_builder_with_type_file() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
            .with_type(HostPathType::File)
            .build();

        assert_eq!(volume.host_path_type, Some(HostPathType::File));
    }

    #[test]
    fn test_hostpath_builder_with_read_only() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
            .with_read_only(true)
            .build();

        assert!(volume.read_only);
    }

    #[test]
    fn test_hostpath_builder_with_sub_path() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
            .with_sub_path("subdir")
            .build();

        assert_eq!(volume.sub_path, Some("subdir".to_string()));
    }

    #[test]
    fn test_hostpath_volume_mount_like() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
            .with_read_only(true)
            .build();

        assert_eq!(volume.volume_name(), "host-vol");
        assert_eq!(volume.mount_path(), "/host-data");
        assert!(volume.read_only());
    }

    #[test]
    fn test_hostpath_as_volume_mount() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
            .with_read_only(true)
            .build();

        let vm = volume.as_volume_mount();
        assert_eq!(vm.name, "host-vol");
        assert_eq!(vm.mount_path, "/host-data");
        assert_eq!(vm.read_only, Some(true));
    }

    #[test]
    fn test_hostpath_as_volume() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol").build();

        let vol = volume.as_volume();
        assert!(vol.host_path.is_some());
        assert_eq!(vol.host_path.unwrap().path, "/var/data");
    }

    #[test]
    fn test_hostpath_as_volume_with_type() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
            .with_type(HostPathType::Directory)
            .build();

        let vol = volume.as_volume();
        let hp_source = vol.host_path.unwrap();
        assert_eq!(hp_source.type_, Some("Directory".to_string()));
    }

    #[test]
    fn test_hostpath_as_volume_default_type() {
        let volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
            .with_type(HostPathType::Default)
            .build();

        let vol = volume.as_volume();
        let hp_source = vol.host_path.unwrap();
        assert_eq!(hp_source.type_, Some("".to_string()));
    }
}
