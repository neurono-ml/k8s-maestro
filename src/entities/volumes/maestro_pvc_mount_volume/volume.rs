use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

#[derive(Debug, Default, Clone)]
pub struct MaestroPVCMountVolume {
    pub(super) mount_path: String,
    pub(super) volume_name: String,
    pub(super) pvc_name: String,
    pub(super) read_only: Option<bool>,
    pub(super) sub_path_expression: Option<String>,
    pub(super) access_mode: Option<Vec<String>>,
    pub(super) storage_class_name: Option<String>,
    pub(super) pvc_resource_limits: BTreeMap<String, Quantity>,
    pub(super) pvc_resource_requests: BTreeMap<String, Quantity>
}

#[derive(Debug, Default, Clone)]
pub struct MaestroPVCMountVolumeBuilder {
    mount_path: String,
    volume_name: String,
    pvc_name: String,
    read_only: Option<bool>,
    sub_path_expression: Option<String>,
    access_mode: Option<Vec<String>>,
    storage_class_name: Option<String>,
    pvc_resource_limits: BTreeMap<String, Quantity>,
    pvc_resource_requests: BTreeMap<String, Quantity>
}

impl MaestroPVCMountVolumeBuilder {
    pub fn new(mount_path: &str, volume_name: &str, pvc_name: &str) -> MaestroPVCMountVolumeBuilder {
        MaestroPVCMountVolumeBuilder {
            mount_path: mount_path.to_owned(),
            volume_name: volume_name.to_owned(),
            pvc_name: pvc_name.to_owned(),
            ..MaestroPVCMountVolumeBuilder::default()
        }
    }

    pub fn set_read_only(mut self, read_only: bool) -> MaestroPVCMountVolumeBuilder {
        self.read_only = Some(read_only);
        self
    }

    pub fn set_sub_path_expression(mut self, sub_path_expression: &str) -> MaestroPVCMountVolumeBuilder {
        self.sub_path_expression = Some(sub_path_expression.to_owned());
        self
    }

    pub fn set_pvc_name(mut self, pvc_name: &str) -> MaestroPVCMountVolumeBuilder {
        self.pvc_name = pvc_name.to_owned();
        self
    }

    pub fn set_volume_name(mut self, volume_name: &str) -> MaestroPVCMountVolumeBuilder {
        self.volume_name = volume_name.to_owned();
        self
    }
    
    pub fn set_mount_path(mut self, mount_path: &str) -> MaestroPVCMountVolumeBuilder {
        self.mount_path = mount_path.to_owned();
        self
    }

    pub fn set_pvc_resource_limit(mut self, resource: &str, limit: &str) ->MaestroPVCMountVolumeBuilder {
        self.pvc_resource_limits.insert(resource.to_owned(), Quantity(limit.to_owned()));
        self
    }

    pub fn set_pvc_resource_request(mut self, resource: &str, request: &str) ->MaestroPVCMountVolumeBuilder {
        self.pvc_resource_requests.insert(resource.to_owned(), Quantity(request.to_owned()));
        self
    }

    pub fn build(self) -> MaestroPVCMountVolume {
        MaestroPVCMountVolume {
            mount_path: self.mount_path,
            volume_name: self.volume_name,
            pvc_name: self.pvc_name,
            read_only: self.read_only,
            sub_path_expression: self.sub_path_expression,
            access_mode: self.access_mode,
            storage_class_name: self.storage_class_name,
            pvc_resource_limits: self.pvc_resource_limits,
            pvc_resource_requests: self.pvc_resource_requests
        }
    }
}