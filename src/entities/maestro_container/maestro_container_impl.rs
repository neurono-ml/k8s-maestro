use std::{collections::BTreeMap, ops::Deref};

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

use crate::entities::{compute_resource::ComputeResource, container::EnvironmentVariableFromObject, environment_variable_source::EnvironmentVariableSource};

use super::{image_pull_policy::ImagePullPolicy, MaestroContainer};

impl MaestroContainer {
    pub fn new(image: &str, name: &str) -> MaestroContainer {
        MaestroContainer {
            name: name.to_owned(),
            image: image.to_owned(),
            ..MaestroContainer::default()
        }
    }

    pub fn set_image(mut self, image: &str) -> MaestroContainer {
        self.image = image.to_owned();
        self
    }

    pub fn set_image_pull_policy(mut self, pull_policy: ImagePullPolicy) -> MaestroContainer {
        self.image_pull_policy = pull_policy;
        self
    }

    pub fn set_resource_bounds(mut self, resource_bounds: BTreeMap<ComputeResource, Quantity>) -> MaestroContainer {
        self.resource_bounds = resource_bounds;
        self
    }

    pub fn set_cpu_bound(mut self, resource_bound: &str) -> MaestroContainer {
        self.resource_bounds.insert(ComputeResource::Cpu, Quantity(resource_bound.to_owned()));
        self
    }

    pub fn set_memory_bound(mut self, resource_bound: &str) -> MaestroContainer {
        self.resource_bounds.insert(ComputeResource::Memory, Quantity(resource_bound.to_owned()));
        self
    }

    pub fn set_disk_bound(mut self, resource_bound: &str) -> MaestroContainer {
        self.resource_bounds.insert(ComputeResource::Disk, Quantity(resource_bound.to_owned()));
        self
    }

    pub fn set_arguments<S>(mut self, arguments: &Vec<S>) -> MaestroContainer where S: ToString {
        self.arguments = arguments
            .iter()
            .map(|argument| argument.to_string().to_owned() )
            .collect();
        self
    }

    pub fn add_argument<S>(mut self, argument: S) -> MaestroContainer where S: Into<String> {
        self.arguments.push(argument.into());
        self
    }

    pub fn add_arguments<S>(mut self, arguments: &[S]) -> MaestroContainer where S: Into<String> + Deref + Clone {
        for argument in arguments {
            self.arguments.push((*argument).clone().into());
        }
        
        self
    }
    
    pub fn set_environment_variables(mut self, environment_variables: BTreeMap<String, EnvironmentVariableSource>) -> MaestroContainer {
        self.environment_variables = environment_variables;
        self
    }

    pub fn add_environment_variable_source<S>(mut self, name: S, source: EnvironmentVariableSource) -> MaestroContainer where S: Into<String>{
        self.environment_variables.insert(name.into(), source);
        self
    }

    pub fn set_environment_variables_from_objects(mut self, source_objects: &Vec<EnvironmentVariableFromObject>) -> MaestroContainer {
        self.environment_variables_from_objects = source_objects.to_owned();
        self
    }

    pub fn add_environment_variables_from_object(mut self, source_object: &EnvironmentVariableFromObject) -> MaestroContainer {
        self.environment_variables_from_objects.push(source_object.to_owned());
        self
    }
}