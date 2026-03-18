use k8s_openapi::api::core::v1::{Volume, VolumeMount};

/// Trait for volume mount abstraction.
///
/// This trait allows volumes to be treated polymorphically when mounting
/// to containers, enabling type-safe and ergonomic volume management.
pub trait VolumeMountLike {
    /// Returns the volume name.
    fn volume_name(&self) -> &str;

    /// Returns the mount path in the container.
    fn mount_path(&self) -> &str;

    /// Returns whether the volume is read-only.
    fn read_only(&self) -> bool;

    /// Returns the sub path within the volume (if any).
    fn sub_path(&self) -> Option<&str>;

    /// Converts this volume to a Kubernetes VolumeMount.
    fn as_volume_mount(&self) -> VolumeMount;

    /// Converts this volume to a Kubernetes Volume.
    fn as_volume(&self) -> Volume;
}

/// Trait for volume source abstraction.
///
/// This trait provides additional methods for volume source configuration.
pub trait VolumeSourceLike {
    /// Returns the volume type identifier.
    fn volume_type(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_bounds() {
        // This test ensures the traits are properly defined
        // and can be used as trait objects
        fn accepts_volume_mount(_: Box<dyn VolumeMountLike>) {}
        fn accepts_volume_source(_: Box<dyn VolumeSourceLike>) {}

        accepts_volume_mount(Box::new(TestVolume));
        accepts_volume_source(Box::new(TestVolume));
    }

    struct TestVolume;

    impl VolumeMountLike for TestVolume {
        fn volume_name(&self) -> &str {
            "test"
        }

        fn mount_path(&self) -> &str {
            "/test"
        }

        fn read_only(&self) -> bool {
            false
        }

        fn sub_path(&self) -> Option<&str> {
            None
        }

        fn as_volume_mount(&self) -> VolumeMount {
            VolumeMount {
                name: "test".to_string(),
                mount_path: "/test".to_string(),
                read_only: Some(false),
                sub_path: None,
                mount_propagation: None,
                sub_path_expr: None,
            }
        }

        fn as_volume(&self) -> Volume {
            Volume {
                name: "test".to_string(),
                empty_dir: None,
                host_path: None,
                persistent_volume_claim: None,
                config_map: None,
                secret: None,
                ..Default::default()
            }
        }
    }

    impl VolumeSourceLike for TestVolume {
        fn volume_type(&self) -> &str {
            "test"
        }
    }
}
