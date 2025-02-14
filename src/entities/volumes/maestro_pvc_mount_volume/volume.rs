use std::collections::{BTreeMap, HashSet};

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;


#[derive(Debug, Default, Clone)]
pub struct MaestroPVCMountVolume {
    pub(super) mount_path: String,
    pub(super) volume_name: String,
    pub(super) pvc_name: String,
    pub(super) read_only: Option<bool>,
    pub(super) sub_path_expression: Option<String>,
    
    pub(super) access_modes: HashSet<AccessMode>,
    pub(super) storage_class_name: Option<String>,
    pub(super) pvc_resource_limits: BTreeMap<String, Quantity>,
    pub(super) pvc_resource_requests: BTreeMap<String, Quantity>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessMode {
    ReadWriteOnce,
    ReadOnlyMany,
    ReadWriteMany,
    ReadWriteOncePod
}


impl ToString for AccessMode {
    fn to_string(&self) -> String {
        let value = 
            match self {
                AccessMode::ReadWriteOnce => "ReadWriteOnce".to_owned(),
                AccessMode::ReadOnlyMany => "ReadOnlyMany".to_owned(),
                AccessMode::ReadWriteMany => "ReadWriteMany".to_owned(),
                AccessMode::ReadWriteOncePod => "ReadWriteOncePod".to_owned(),
            };
        value.to_owned()
    }
}

#[derive(Debug, Default, Clone)]
pub struct MaestroPVCMountVolumeBuilder {
    mount_path: String,
    volume_name: String,
    pvc_name: String,
    read_only: Option<bool>,
    sub_path_expression: Option<String>,
    access_modes: HashSet<AccessMode>,
    storage_class_name: Option<String>,
    pvc_resource_limits: BTreeMap<String, Quantity>,
    pvc_resource_requests: BTreeMap<String, Quantity>
}

impl MaestroPVCMountVolumeBuilder {
    pub fn new<S1, S2, S3>(mount_path: S1, volume_name:S2 , pvc_name: S3) -> MaestroPVCMountVolumeBuilder where S1: Into<String>, S2: Into<String>, S3: Into<String>{
        MaestroPVCMountVolumeBuilder {
            mount_path: mount_path.into(),
            volume_name: volume_name.into(),
            pvc_name: pvc_name.into(),
            ..MaestroPVCMountVolumeBuilder::default()
        }
    }

    pub fn set_read_only(mut self, read_only: bool) -> MaestroPVCMountVolumeBuilder {
        self.read_only = Some(read_only);
        self
    }

    pub fn set_sub_path_expression<S>(mut self, sub_path_expression: S) -> MaestroPVCMountVolumeBuilder where S: Into<String> {
        self.sub_path_expression = Some(sub_path_expression.into());
        self
    }

    pub fn set_pvc_name<S>(mut self, pvc_name: S) -> MaestroPVCMountVolumeBuilder where S: Into<String> {
        self.pvc_name = pvc_name.into();
        self
    }

    pub fn set_volume_name<S>(mut self, volume_name: S) -> MaestroPVCMountVolumeBuilder where S: Into<String>{
        self.volume_name = volume_name.into();
        self
    }
    
    pub fn set_mount_path<S>(mut self, mount_path: S) -> MaestroPVCMountVolumeBuilder where S: Into<String> {
        self.mount_path = mount_path.into().to_owned();
        self
    }

    pub fn set_pvc_resource_limit<S>(mut self, resource: &str, limit: S) -> MaestroPVCMountVolumeBuilder where S: Into<String> {
        self.pvc_resource_limits.insert(resource.to_owned(), Quantity(limit.into().to_owned()));
        self
    }

    pub fn set_pvc_resource_request<S>(mut self, resource: &str, request: S) -> MaestroPVCMountVolumeBuilder where S: Into<String> {
        self.pvc_resource_requests.insert(resource.to_owned(), Quantity(request.into().to_owned()));
        self
    }

    pub fn set_storage_class_name<S>(mut self, storage_class_name: S) -> MaestroPVCMountVolumeBuilder where S: Into<String> {
        self.storage_class_name = Some(storage_class_name.into());
        self
    }

    pub fn add_access_mode(mut self, access_mode: AccessMode) -> MaestroPVCMountVolumeBuilder {
        self.access_modes.insert(access_mode);
        self
    }

    pub fn set_access_modes<S>(mut self, access_modes: S) -> MaestroPVCMountVolumeBuilder where S: Into<HashSet<AccessMode>> {
        self.access_modes = access_modes.into();
        self
    }

    pub fn build(self) -> MaestroPVCMountVolume {
        MaestroPVCMountVolume {
            mount_path: self.mount_path,
            volume_name: self.volume_name,
            pvc_name: self.pvc_name,
            read_only: self.read_only,
            sub_path_expression: self.sub_path_expression,
            access_modes: self.access_modes,
            storage_class_name: self.storage_class_name,
            pvc_resource_limits: self.pvc_resource_limits,
            pvc_resource_requests: self.pvc_resource_requests
        }
    }
}