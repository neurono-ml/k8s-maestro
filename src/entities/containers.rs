use crate::steps::ResourceLimits;
use k8s_openapi::api::core::v1::Container;
use std::collections::BTreeMap;

pub trait ContainerLike {
    fn as_container(&self) -> Container;
}

#[derive(Debug)]
pub struct MaestroContainer {
    image: String,
    name: String,
    args: Option<Vec<String>>,
    env: Option<BTreeMap<String, String>>,
    resource_limits: Option<ResourceLimits>,
    volume_mounts: Vec<k8s_openapi::api::core::v1::VolumeMount>,
    env_from: Vec<k8s_openapi::api::core::v1::EnvFromSource>,
}

impl MaestroContainer {
    pub fn new(image: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            name: name.into(),
            args: None,
            env: None,
            resource_limits: None,
            volume_mounts: Vec::new(),
            env_from: Vec::new(),
        }
    }

    pub fn set_arguments(mut self, args: &[String]) -> Self {
        self.args = Some(args.to_vec());
        self
    }

    pub fn set_environment_variables(mut self, env_vars: BTreeMap<String, String>) -> Self {
        self.env = Some(env_vars);
        self
    }

    pub fn set_resource_bounds(mut self, bounds: ResourceLimits) -> Self {
        self.resource_limits = Some(bounds);
        self
    }

    pub fn add_volume_mount(mut self, volume_name: &str, mount_path: &str, read_only: bool) -> Self {
        self.volume_mounts.push(k8s_openapi::api::core::v1::VolumeMount {
            name: volume_name.to_string(),
            mount_path: mount_path.to_string(),
            read_only: Some(read_only),
            ..Default::default()
        });
        self
    }

    pub fn add_env_from_secret(mut self, secret_name: &str) -> Self {
        self.env_from.push(k8s_openapi::api::core::v1::EnvFromSource {
            secret_ref: Some(k8s_openapi::api::core::v1::SecretEnvSource {
                name: secret_name.to_string(),
                optional: None,
            }),
            ..Default::default()
        });
        self
    }
}

impl ContainerLike for MaestroContainer {
    fn as_container(&self) -> Container {
        let env = self.env.as_ref().map(|env_vars| {
            env_vars
                .iter()
                .map(|(k, v)| k8s_openapi::api::core::v1::EnvVar {
                    name: k.clone(),
                    value: Some(v.clone()),
                    ..Default::default()
                })
                .collect()
        });

        let mut resources = None;
        if let Some(limits) = &self.resource_limits {
            let mut requests = BTreeMap::new();
            let mut limits_map = BTreeMap::new();

            if let Some(cpu) = &limits.cpu {
                limits_map.insert(
                    "cpu".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(cpu.clone()),
                );
            }
            if let Some(memory) = &limits.memory {
                limits_map.insert(
                    "memory".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(memory.clone()),
                );
            }
            if let Some(ephemeral_storage) = &limits.ephemeral_storage {
                limits_map.insert(
                    "ephemeral-storage".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(
                        ephemeral_storage.clone(),
                    ),
                );
            }

            if let Some(cpu_request) = &limits.cpu_request {
                requests.insert(
                    "cpu".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(cpu_request.clone()),
                );
            }
            if let Some(memory_request) = &limits.memory_request {
                requests.insert(
                    "memory".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(memory_request.clone()),
                );
            }
            if let Some(ephemeral_storage_request) = &limits.ephemeral_storage_request {
                requests.insert(
                    "ephemeral-storage".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(
                        ephemeral_storage_request.clone(),
                    ),
                );
            }

            resources = Some(k8s_openapi::api::core::v1::ResourceRequirements {
                limits: if limits_map.is_empty() {
                    None
                } else {
                    Some(limits_map)
                },
                requests: if requests.is_empty() {
                    None
                } else {
                    Some(requests)
                },
                ..Default::default()
            });
        }

        let volume_mounts = if self.volume_mounts.is_empty() {
            None
        } else {
            Some(self.volume_mounts.clone())
        };

        let env_from = if self.env_from.is_empty() {
            None
        } else {
            Some(self.env_from.clone())
        };

        Container {
            name: self.name.clone(),
            image: Some(self.image.clone()),
            args: self.args.clone(),
            env,
            resources,
            volume_mounts,
            env_from,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct SidecarContainer {
    image: String,
    name: String,
    args: Option<Vec<String>>,
    env: Option<BTreeMap<String, String>>,
    resource_limits: Option<ResourceLimits>,
    volume_mounts: Vec<k8s_openapi::api::core::v1::VolumeMount>,
    env_from: Vec<k8s_openapi::api::core::v1::EnvFromSource>,
}

impl SidecarContainer {
    pub fn new(image: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            name: name.into(),
            args: None,
            env: None,
            resource_limits: None,
            volume_mounts: Vec::new(),
            env_from: Vec::new(),
        }
    }

    pub fn set_arguments(mut self, args: &[String]) -> Self {
        self.args = Some(args.to_vec());
        self
    }

    pub fn set_environment_variables(mut self, env_vars: BTreeMap<String, String>) -> Self {
        self.env = Some(env_vars);
        self
    }

    pub fn set_resource_bounds(mut self, bounds: ResourceLimits) -> Self {
        self.resource_limits = Some(bounds);
        self
    }

    pub fn add_volume_mount(mut self, volume_name: &str, mount_path: &str, read_only: bool) -> Self {
        self.volume_mounts.push(k8s_openapi::api::core::v1::VolumeMount {
            name: volume_name.to_string(),
            mount_path: mount_path.to_string(),
            read_only: Some(read_only),
            ..Default::default()
        });
        self
    }

    pub fn add_env_from_secret(mut self, secret_name: &str) -> Self {
        self.env_from.push(k8s_openapi::api::core::v1::EnvFromSource {
            secret_ref: Some(k8s_openapi::api::core::v1::SecretEnvSource {
                name: secret_name.to_string(),
                optional: None,
            }),
            ..Default::default()
        });
        self
    }
}

impl ContainerLike for SidecarContainer {
    fn as_container(&self) -> Container {
        let env = self.env.as_ref().map(|env_vars| {
            env_vars
                .iter()
                .map(|(k, v)| k8s_openapi::api::core::v1::EnvVar {
                    name: k.clone(),
                    value: Some(v.clone()),
                    ..Default::default()
                })
                .collect()
        });

        let mut resources = None;
        if let Some(limits) = &self.resource_limits {
            let mut requests = BTreeMap::new();
            let mut limits_map = BTreeMap::new();

            if let Some(cpu) = &limits.cpu {
                limits_map.insert(
                    "cpu".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(cpu.clone()),
                );
            }
            if let Some(memory) = &limits.memory {
                limits_map.insert(
                    "memory".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(memory.clone()),
                );
            }
            if let Some(ephemeral_storage) = &limits.ephemeral_storage {
                limits_map.insert(
                    "ephemeral-storage".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(
                        ephemeral_storage.clone(),
                    ),
                );
            }

            if let Some(cpu_request) = &limits.cpu_request {
                requests.insert(
                    "cpu".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(cpu_request.clone()),
                );
            }
            if let Some(memory_request) = &limits.memory_request {
                requests.insert(
                    "memory".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(memory_request.clone()),
                );
            }
            if let Some(ephemeral_storage_request) = &limits.ephemeral_storage_request {
                requests.insert(
                    "ephemeral-storage".to_string(),
                    k8s_openapi::apimachinery::pkg::api::resource::Quantity(
                        ephemeral_storage_request.clone(),
                    ),
                );
            }

            resources = Some(k8s_openapi::api::core::v1::ResourceRequirements {
                limits: if limits_map.is_empty() {
                    None
                } else {
                    Some(limits_map)
                },
                requests: if requests.is_empty() {
                    None
                } else {
                    Some(requests)
                },
                ..Default::default()
            });
        }

        let volume_mounts = if self.volume_mounts.is_empty() {
            None
        } else {
            Some(self.volume_mounts.clone())
        };

        let env_from = if self.env_from.is_empty() {
            None
        } else {
            Some(self.env_from.clone())
        };

        Container {
            name: self.name.clone(),
            image: Some(self.image.clone()),
            args: self.args.clone(),
            env,
            resources,
            volume_mounts,
            env_from,
            ..Default::default()
        }
    }
}
