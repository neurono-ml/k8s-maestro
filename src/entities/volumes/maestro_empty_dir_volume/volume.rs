use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

#[derive(Debug, Default, Clone)]
pub struct MaestroEmptydirMountVolume {
    pub(super) mount_path: String,
    pub(super) volume_name: String,
    pub(super) read_only: Option<bool>,
    pub(super) medium: EmptyDirMedium,
    pub(super) size: Option<Quantity>
}

#[derive(Debug, Default, Clone)]
pub struct MaestroEmptydirMountVolumeBuilder {
    pub(super) mount_path: String,
    pub(super) volume_name: String,
    pub(super) read_only: Option<bool>,
    pub(super) medium: EmptyDirMedium,
    pub(super) size: Option<Quantity>
}

impl MaestroEmptydirMountVolumeBuilder {
    pub fn new<S1, S2>(mount_path: S1, volume_name: S2) -> MaestroEmptydirMountVolumeBuilder
    where
        S1: Into<String>,
        S2: Into<String>
    {
        MaestroEmptydirMountVolumeBuilder {
            mount_path: mount_path.into(),
            volume_name: volume_name.into(),
            read_only: None,
            medium: EmptyDirMedium::default(),
            size: None,
            
        }
    }

    pub fn set_mount_path<S>(mut self, value: S) -> Self where S: Into<String> {
        self.mount_path = value.into();
        self
    }

    pub fn set_volume_name<S>(mut self, value: S) -> Self where S: Into<String> {
        self.volume_name = value.into();
        self
    }

    pub fn set_read_only<B>(mut self, value: B) -> Self where B: Into<bool> {
        self.read_only = Some(value.into());
        self
    }

    pub fn set_medium<B>(mut self, value: B) -> Self where B: Into<EmptyDirMedium> {
        self.medium = value.into();
        self
    }

    pub fn set_size<S>(mut self, value: S) -> Self where S: Into<String> {
        self.size = Some(Quantity(value.into()));
        self
    }

    pub fn build(self) -> MaestroEmptydirMountVolume {
        MaestroEmptydirMountVolume {
            mount_path: self.mount_path,
            volume_name: self.volume_name,
            read_only: self.read_only,
            medium: self.medium,
            size: self.size
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EmptyDirMedium {
    #[default]
    Storage,
    Memory
}

impl ToString for EmptyDirMedium {
    fn to_string(&self) -> String {
        let value = 
            match self {
                EmptyDirMedium::Storage => "".to_owned(),
                EmptyDirMedium::Memory => "Memory".to_owned(),
            };
        value.to_owned()
    }
}