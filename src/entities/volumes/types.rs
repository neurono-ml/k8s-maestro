use std::fmt;

/// Volume type enumeration for identifying different volume sources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeType {
    /// Persistent Volume Claim
    PersistentVolumeClaim,
    /// ConfigMap volume
    ConfigMap,
    /// Secret volume
    Secret,
    /// Empty directory volume
    EmptyDir,
    /// Host path volume
    HostPath,
}

impl fmt::Display for VolumeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VolumeType::PersistentVolumeClaim => write!(f, "PersistentVolumeClaim"),
            VolumeType::ConfigMap => write!(f, "ConfigMap"),
            VolumeType::Secret => write!(f, "Secret"),
            VolumeType::EmptyDir => write!(f, "EmptyDir"),
            VolumeType::HostPath => write!(f, "HostPath"),
        }
    }
}

/// Volume item for ConfigMap and Secret volume projections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeItem {
    /// Key from the ConfigMap or Secret
    pub key: String,
    /// Path to mount the key
    pub path: String,
    /// Mode bits in octal notation (optional)
    pub mode: Option<i32>,
}

impl VolumeItem {
    pub fn new(key: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            path: path.into(),
            mode: None,
        }
    }

    pub fn with_mode(mut self, mode: i32) -> Self {
        self.mode = Some(mode);
        self
    }
}

/// Access mode for Persistent Volume Claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    /// Volume can be mounted as read-write by a single node
    ReadWriteOnce,
    /// Volume can be mounted as read-only by many nodes
    ReadOnlyMany,
    /// Volume can be mounted as read-write by many nodes
    ReadWriteMany,
    /// Volume can be mounted as read-write by a single pod
    ReadWriteOncePod,
}

impl fmt::Display for AccessMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessMode::ReadWriteOnce => write!(f, "ReadWriteOnce"),
            AccessMode::ReadOnlyMany => write!(f, "ReadOnlyMany"),
            AccessMode::ReadWriteMany => write!(f, "ReadWriteMany"),
            AccessMode::ReadWriteOncePod => write!(f, "ReadWriteOncePod"),
        }
    }
}

impl AsRef<str> for AccessMode {
    fn as_ref(&self) -> &str {
        match self {
            AccessMode::ReadWriteOnce => "ReadWriteOnce",
            AccessMode::ReadOnlyMany => "ReadOnlyMany",
            AccessMode::ReadWriteMany => "ReadWriteMany",
            AccessMode::ReadWriteOncePod => "ReadWriteOncePod",
        }
    }
}

/// Medium type for EmptyDir volumes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Medium {
    /// Default empty directory (backed by node's disk)
    Default,
    /// Memory-backed empty directory (tmpfs)
    Memory,
}

impl fmt::Display for Medium {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Medium::Default => write!(f, ""),
            Medium::Memory => write!(f, "Memory"),
        }
    }
}

/// Host path type for HostPath volumes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostPathType {
    /// Default behavior (no type checking)
    Default,
    /// Existing directory
    Directory,
    /// Existing file
    File,
    /// Unix socket
    Socket,
    /// Block device
    BlockDevice,
    /// Character device
    CharDevice,
    /// Directory (create if not exists)
    DirectoryOrCreate,
    /// File (create if not exists)
    FileOrCreate,
}

impl fmt::Display for HostPathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostPathType::Default => write!(f, ""),
            HostPathType::Directory => write!(f, "Directory"),
            HostPathType::File => write!(f, "File"),
            HostPathType::Socket => write!(f, "Socket"),
            HostPathType::BlockDevice => write!(f, "BlockDevice"),
            HostPathType::CharDevice => write!(f, "CharDevice"),
            HostPathType::DirectoryOrCreate => write!(f, "DirectoryOrCreate"),
            HostPathType::FileOrCreate => write!(f, "FileOrCreate"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_item() {
        let item = VolumeItem::new("config", "config.yaml").with_mode(0o644);
        assert_eq!(item.key, "config");
        assert_eq!(item.path, "config.yaml");
        assert_eq!(item.mode, Some(0o644));
    }

    #[test]
    fn test_access_mode_display() {
        assert_eq!(AccessMode::ReadWriteOnce.to_string(), "ReadWriteOnce");
        assert_eq!(AccessMode::ReadOnlyMany.to_string(), "ReadOnlyMany");
        assert_eq!(AccessMode::ReadWriteMany.to_string(), "ReadWriteMany");
        assert_eq!(AccessMode::ReadWriteOncePod.to_string(), "ReadWriteOncePod");
    }

    #[test]
    fn test_access_mode_as_ref() {
        assert_eq!(AccessMode::ReadWriteOnce.as_ref(), "ReadWriteOnce");
        assert_eq!(AccessMode::ReadOnlyMany.as_ref(), "ReadOnlyMany");
        assert_eq!(AccessMode::ReadWriteMany.as_ref(), "ReadWriteMany");
        assert_eq!(AccessMode::ReadWriteOncePod.as_ref(), "ReadWriteOncePod");
    }

    #[test]
    fn test_medium_display() {
        assert_eq!(Medium::Default.to_string(), "");
        assert_eq!(Medium::Memory.to_string(), "Memory");
    }

    #[test]
    fn test_host_path_type_display() {
        assert_eq!(HostPathType::Default.to_string(), "");
        assert_eq!(HostPathType::Directory.to_string(), "Directory");
        assert_eq!(HostPathType::File.to_string(), "File");
        assert_eq!(
            HostPathType::DirectoryOrCreate.to_string(),
            "DirectoryOrCreate"
        );
    }
}
