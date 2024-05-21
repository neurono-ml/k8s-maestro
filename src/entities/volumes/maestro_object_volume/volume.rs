use super::k8s_volume_object_volume_source::K8sObjectVolumeSource;

#[derive(Debug, Clone)]
pub struct MaestroObjectMountVolume {
    pub(super) mount_path: String,
    pub(super) volume_name: String,
    pub(super) k8s_object: K8sObjectVolumeSource,
    pub(super) sub_path_expression: Option<String>
}

#[derive(Debug, Default, Clone)]
pub struct MaestroObjectMountVolumeBuilder {
    mount_path: String,
    volume_name: String,
    k8s_object: Option<K8sObjectVolumeSource>,
    sub_path_expression: Option<String>
}


impl MaestroObjectMountVolumeBuilder {
    pub fn from_configmap(mount_path: &str, volume_name: &str, config_map_name: &str) -> Self {
        let k8s_object = Some(K8sObjectVolumeSource::ConfigMap(config_map_name.to_owned()));

        Self {
            mount_path: mount_path.to_owned(),
            volume_name: volume_name.to_owned(),
            k8s_object,
            ..Self::default()
        }
    }

    pub fn from_secret(mount_path: &str, volume_name: &str, secret_name: &str) -> Self {
        let k8s_object = Some(K8sObjectVolumeSource::Secret(secret_name.to_owned()));

        Self {
            mount_path: mount_path.to_owned(),
            volume_name: volume_name.to_owned(),
            k8s_object,
            ..Self::default()
        }
    }

    pub fn set_sub_path_expression(mut self, sub_path_expression: &str) -> Self {
        self.sub_path_expression = Some(sub_path_expression.to_owned());
        self
    }

    pub fn set_k8s_object(mut self, k8s_object: &K8sObjectVolumeSource) -> Self {
        self.k8s_object = Some(k8s_object.clone());
        self
    }

    pub fn set_secret_name(mut self, secret_name: &str) -> Self {
        self.k8s_object = Some(K8sObjectVolumeSource::Secret(secret_name.to_owned()));
        self
    }

    pub fn set_config_map_name(mut self, config_map_name: &str) -> Self {
        self.k8s_object = Some(K8sObjectVolumeSource::ConfigMap(config_map_name.to_owned()));
        self
    }

    pub fn set_volume_name(mut self, volume_name: &str) -> Self {
        self.volume_name = volume_name.to_owned();
        self
    }
    
    pub fn set_mount_path(mut self, mount_path: &str) -> Self {
        self.mount_path = mount_path.to_owned();
        self
    }

    pub fn build(self) -> MaestroObjectMountVolume {
        MaestroObjectMountVolume {
            mount_path: self.mount_path,
            volume_name: self.volume_name,
            k8s_object: self.k8s_object.unwrap(),
            sub_path_expression: self.sub_path_expression
        }
    }
}