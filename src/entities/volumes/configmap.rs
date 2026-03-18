use crate::entities::volumes::traits::VolumeMountLike;
use crate::entities::volumes::types::VolumeItem;
use k8s_openapi::api::core::v1::{ConfigMapVolumeSource, Volume, VolumeMount};

/// ConfigMap volume configuration.
#[derive(Debug, Clone)]
pub struct ConfigMapVolume {
    /// Volume name
    pub volume_name: String,
    /// Mount path in the container
    pub mount_path: String,
    /// ConfigMap name
    pub config_map_name: String,
    /// Read only flag
    pub read_only: bool,
    /// Sub path within the volume
    pub sub_path: Option<String>,
    /// Specific items to mount
    pub items: Option<Vec<VolumeItem>>,
    /// Default file mode
    pub default_mode: Option<i32>,
    /// Optional volume
    pub optional: Option<bool>,
}

/// Builder for creating ConfigMap volumes.
#[derive(Debug, Clone)]
pub struct ConfigMapVolumeBuilder {
    volume_name: String,
    mount_path: String,
    config_map_name: String,
    read_only: bool,
    sub_path: Option<String>,
    items: Option<Vec<VolumeItem>>,
    default_mode: Option<i32>,
    optional: Option<bool>,
}

impl ConfigMapVolumeBuilder {
    /// Creates a new ConfigMap volume builder.
    ///
    /// # Arguments
    ///
    /// * `mount_path` - Path where the volume will be mounted in the container
    /// * `config_map_name` - Name of the ConfigMap to reference
    /// * `volume_name` - Name to give to the volume
    pub fn new(
        mount_path: impl Into<String>,
        config_map_name: impl Into<String>,
        volume_name: impl Into<String>,
    ) -> Self {
        Self {
            mount_path: mount_path.into(),
            config_map_name: config_map_name.into(),
            volume_name: volume_name.into(),
            read_only: false,
            sub_path: None,
            items: None,
            default_mode: None,
            optional: None,
        }
    }

    /// Sets specific items to mount from the ConfigMap.
    pub fn with_items(mut self, items: Vec<VolumeItem>) -> Self {
        self.items = Some(items);
        self
    }

    /// Sets the default file mode for all files in the volume.
    pub fn with_default_mode(mut self, mode: i32) -> Self {
        self.default_mode = Some(mode);
        self
    }

    /// Sets whether the volume is optional.
    pub fn with_optional(mut self, optional: bool) -> Self {
        self.optional = Some(optional);
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

    /// Builds the ConfigMap volume.
    pub fn build(self) -> ConfigMapVolume {
        ConfigMapVolume {
            volume_name: self.volume_name,
            mount_path: self.mount_path,
            config_map_name: self.config_map_name,
            read_only: self.read_only,
            sub_path: self.sub_path,
            items: self.items,
            default_mode: self.default_mode,
            optional: self.optional,
        }
    }
}

impl VolumeMountLike for ConfigMapVolume {
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
        let mut config_map_source = ConfigMapVolumeSource {
            name: self.config_map_name.clone(),
            optional: self.optional,
            default_mode: self.default_mode,
            items: None,
        };

        if let Some(items) = &self.items {
            let k8s_items: Vec<k8s_openapi::api::core::v1::KeyToPath> = items
                .iter()
                .map(|item| k8s_openapi::api::core::v1::KeyToPath {
                    key: item.key.clone(),
                    path: item.path.clone(),
                    mode: item.mode,
                })
                .collect();
            config_map_source.items = Some(k8s_items);
        }

        Volume {
            name: self.volume_name.clone(),
            config_map: Some(config_map_source),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configmap_builder_basic() {
        let volume = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol").build();

        assert_eq!(volume.volume_name, "config-vol");
        assert_eq!(volume.mount_path, "/config");
        assert_eq!(volume.config_map_name, "app-config");
        assert_eq!(volume.read_only, false);
    }

    #[test]
    fn test_configmap_builder_with_items() {
        let volume = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
            .with_items(vec![VolumeItem::new("config.yaml", "config.yaml")])
            .build();

        assert!(volume.items.is_some());
        assert_eq!(volume.items.as_ref().unwrap().len(), 1);
        assert_eq!(volume.items.as_ref().unwrap()[0].key, "config.yaml");
    }

    #[test]
    fn test_configmap_builder_with_default_mode() {
        let volume = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
            .with_default_mode(0o644)
            .build();

        assert_eq!(volume.default_mode, Some(0o644));
    }

    #[test]
    fn test_configmap_builder_with_optional() {
        let volume = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
            .with_optional(true)
            .build();

        assert_eq!(volume.optional, Some(true));
    }

    #[test]
    fn test_configmap_volume_mount_like() {
        let volume = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
            .with_read_only(true)
            .build();

        assert_eq!(volume.volume_name(), "config-vol");
        assert_eq!(volume.mount_path(), "/config");
        assert!(volume.read_only());
    }

    #[test]
    fn test_configmap_as_volume_mount() {
        let volume = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
            .with_read_only(true)
            .build();

        let vm = volume.as_volume_mount();
        assert_eq!(vm.name, "config-vol");
        assert_eq!(vm.mount_path, "/config");
        assert_eq!(vm.read_only, Some(true));
    }

    #[test]
    fn test_configmap_as_volume() {
        let volume = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol").build();

        let vol = volume.as_volume();
        assert!(vol.config_map.is_some());
        assert_eq!(vol.config_map.unwrap().name, "app-config");
    }

    #[test]
    fn test_configmap_with_items_in_volume() {
        let volume = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
            .with_items(vec![VolumeItem::new("key1", "path1").with_mode(0o644)])
            .build();

        let vol = volume.as_volume();
        let cm_source = vol.config_map.unwrap();
        assert!(cm_source.items.is_some());
        let items = cm_source.items.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].key, "key1");
        assert_eq!(items[0].path, "path1");
        assert_eq!(items[0].mode, Some(0o644));
    }
}
