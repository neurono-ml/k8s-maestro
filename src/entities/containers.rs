use crate::steps::ResourceLimits;
use k8s_openapi::api::core::v1::Container;
use std::collections::BTreeMap;

pub trait ContainerLike {
    fn as_container(&self) -> Container;
}

pub struct MaestroContainer {
    image: String,
    name: String,
    args: Option<Vec<String>>,
    env: Option<BTreeMap<String, String>>,
    resource_limits: Option<ResourceLimits>,
}

impl MaestroContainer {
    pub fn new(image: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            name: name.into(),
            args: None,
            env: None,
            resource_limits: None,
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

        Container {
            name: self.name.clone(),
            image: Some(self.image.clone()),
            args: self.args.clone(),
            env,
            resources,
            ..Default::default()
        }
    }
}

pub struct SidecarContainer {
    image: String,
    name: String,
    args: Option<Vec<String>>,
    env: Option<BTreeMap<String, String>>,
    resource_limits: Option<ResourceLimits>,
}

impl SidecarContainer {
    pub fn new(image: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            name: name.into(),
            args: None,
            env: None,
            resource_limits: None,
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

        Container {
            name: self.name.clone(),
            image: Some(self.image.clone()),
            args: self.args.clone(),
            env,
            resources,
            ..Default::default()
        }
    }
}
