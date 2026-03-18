//! Service builder and types for creating Kubernetes Services.
//!
//! This module provides a fluent builder API for creating Kubernetes Service resources
//! with support for all service types (ClusterIP, Headless, NodePort, LoadBalancer).

use anyhow::Result;
use k8s_openapi::api::core::v1::{Service, ServicePort as K8sServicePort, ServiceSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

/// Kubernetes service type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    ClusterIP,
    Headless,
    NodePort,
    LoadBalancer,
}

impl ServiceType {
    fn as_str(&self) -> &str {
        match self {
            ServiceType::ClusterIP => "ClusterIP",
            ServiceType::Headless => "ClusterIP",
            ServiceType::NodePort => "NodePort",
            ServiceType::LoadBalancer => "LoadBalancer",
        }
    }
}

/// Service port configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServicePort {
    pub port: i32,
    pub target_port: i32,
    pub protocol: String,
    pub name: Option<String>,
}

impl Default for ServicePort {
    fn default() -> Self {
        Self {
            port: 80,
            target_port: 8080,
            protocol: "TCP".to_string(),
            name: None,
        }
    }
}

/// Builder for creating Kubernetes Service resources.
pub struct ServiceBuilder {
    name: Option<String>,
    namespace: Option<String>,
    service_type: ServiceType,
    cluster_ip: Option<String>,
    ports: Vec<ServicePort>,
    selector: Option<BTreeMap<String, String>>,
    session_affinity: Option<String>,
    external_traffic_policy: Option<String>,
}

impl ServiceBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            namespace: None,
            service_type: ServiceType::ClusterIP,
            cluster_ip: None,
            ports: Vec::new(),
            selector: None,
            session_affinity: None,
            external_traffic_policy: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_port(mut self, port: i32, target_port: i32, protocol: impl Into<String>) -> Self {
        self.ports.push(ServicePort {
            port,
            target_port,
            protocol: protocol.into(),
            name: None,
        });
        self
    }

    pub fn with_ports(mut self, ports: Vec<ServicePort>) -> Self {
        self.ports = ports;
        self
    }

    pub fn with_type(mut self, service_type: ServiceType) -> Self {
        self.service_type = service_type;
        self
    }

    pub fn with_selector(mut self, labels: BTreeMap<String, String>) -> Self {
        self.selector = Some(labels);
        self
    }

    pub fn with_cluster_ip(mut self, ip: impl Into<String>) -> Self {
        self.cluster_ip = Some(ip.into());
        self
    }

    pub fn with_session_affinity(mut self, affinity: impl Into<String>) -> Self {
        self.session_affinity = Some(affinity.into());
        self
    }

    pub fn with_external_traffic_policy(mut self, policy: impl Into<String>) -> Self {
        self.external_traffic_policy = Some(policy.into());
        self
    }

    pub fn build(self) -> Result<Service> {
        let name = self
            .name
            .ok_or_else(|| anyhow::anyhow!("Service name is required"))?;
        let namespace = self
            .namespace
            .ok_or_else(|| anyhow::anyhow!("Service namespace is required"))?;

        let cluster_ip = match self.service_type {
            ServiceType::Headless => Some("None".to_string()),
            _ => self.cluster_ip,
        };

        let k8s_ports: Vec<K8sServicePort> = self
            .ports
            .into_iter()
            .map(|p| K8sServicePort {
                name: p.name,
                port: p.port,
                target_port: Some(
                    k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(p.target_port),
                ),
                protocol: Some(p.protocol),
                ..Default::default()
            })
            .collect();

        Ok(Service {
            metadata: ObjectMeta {
                name: Some(name),
                namespace: Some(namespace),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                type_: Some(self.service_type.as_str().to_string()),
                cluster_ip,
                ports: Some(k8s_ports),
                selector: self.selector,
                session_affinity: self.session_affinity,
                external_traffic_policy: self.external_traffic_policy,
                ..Default::default()
            }),
            status: None,
        })
    }
}

impl Default for ServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    include!("service_test.rs");
}
