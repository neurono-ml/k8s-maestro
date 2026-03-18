//! Ingress builder and types for creating Kubernetes Ingress resources.
//!
//! This module provides a fluent builder API for creating Kubernetes Ingress resources
//! with support for host/path routing, TLS configuration, and annotations.

use anyhow::Result;
use k8s_openapi::api::networking::v1::{
    HTTPIngressPath, HTTPIngressRuleValue, Ingress, IngressBackend, IngressRule, IngressSpec,
    IngressTLS, ServiceBackendPort,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

/// Ingress path type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathType {
    Exact,
    Prefix,
    ImplementationSpecific,
}

impl PathType {
    fn as_str(&self) -> &str {
        match self {
            PathType::Exact => "Exact",
            PathType::Prefix => "Prefix",
            PathType::ImplementationSpecific => "ImplementationSpecific",
        }
    }
}

/// Ingress path configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngressPath {
    pub path: String,
    pub path_type: PathType,
    pub service_name: String,
    pub service_port: i32,
}

impl Default for IngressPath {
    fn default() -> Self {
        Self {
            path: "/".to_string(),
            path_type: PathType::Prefix,
            service_name: String::new(),
            service_port: 80,
        }
    }
}

/// TLS configuration for Ingress.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TLSConfig {
    pub hosts: Vec<String>,
    pub secret_name: String,
}

/// Builder for creating Kubernetes Ingress resources.
pub struct IngressBuilder {
    name: Option<String>,
    namespace: Option<String>,
    host: Option<String>,
    paths: Vec<IngressPath>,
    tls: Option<TLSConfig>,
    annotations: Option<BTreeMap<String, String>>,
    ingress_class: Option<String>,
}

impl IngressBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            namespace: None,
            host: None,
            paths: Vec::new(),
            tls: None,
            annotations: None,
            ingress_class: None,
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

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn with_path(
        mut self,
        path: impl Into<String>,
        service_name: impl Into<String>,
        service_port: i32,
    ) -> Self {
        self.paths.push(IngressPath {
            path: path.into(),
            path_type: PathType::Prefix,
            service_name: service_name.into(),
            service_port,
        });
        self
    }

    pub fn with_paths(mut self, paths: Vec<IngressPath>) -> Self {
        self.paths = paths;
        self
    }

    pub fn with_tls_secret(mut self, secret_name: impl Into<String>) -> Self {
        self.tls = Some(TLSConfig {
            hosts: Vec::new(),
            secret_name: secret_name.into(),
        });
        self
    }

    pub fn with_tls_config(mut self, tls: TLSConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    pub fn with_annotations(mut self, annotations: BTreeMap<String, String>) -> Self {
        self.annotations = Some(annotations);
        self
    }

    pub fn with_ingress_class(mut self, class_name: impl Into<String>) -> Self {
        self.ingress_class = Some(class_name.into());
        self
    }

    pub fn build(self) -> Result<Ingress> {
        let name = self
            .name
            .ok_or_else(|| anyhow::anyhow!("Ingress name is required"))?;
        let namespace = self
            .namespace
            .ok_or_else(|| anyhow::anyhow!("Ingress namespace is required"))?;

        if self.paths.is_empty() {
            return Err(anyhow::anyhow!("At least one path is required"));
        }

        let host = self
            .host
            .ok_or_else(|| anyhow::anyhow!("Host is required"))?;

        let http_paths: Vec<HTTPIngressPath> = self
            .paths
            .into_iter()
            .map(|p| HTTPIngressPath {
                path: Some(p.path),
                path_type: p.path_type.as_str().to_string(),
                backend: IngressBackend {
                    service: Some(k8s_openapi::api::networking::v1::IngressServiceBackend {
                        name: p.service_name,
                        port: Some(ServiceBackendPort {
                            number: Some(p.service_port),
                            ..Default::default()
                        }),
                    }),
                    resource: None,
                },
            })
            .collect();

        let rules = vec![IngressRule {
            host: Some(host),
            http: Some(HTTPIngressRuleValue { paths: http_paths }),
        }];

        let tls_config = self.tls.map(|tls| {
            vec![IngressTLS {
                hosts: if tls.hosts.is_empty() {
                    None
                } else {
                    Some(tls.hosts)
                },
                secret_name: Some(tls.secret_name),
            }]
        });

        let mut metadata = ObjectMeta {
            name: Some(name),
            namespace: Some(namespace),
            ..Default::default()
        };

        if let Some(annotations) = self.annotations {
            metadata.annotations = Some(annotations);
        }

        Ok(Ingress {
            metadata,
            spec: Some(IngressSpec {
                ingress_class_name: self.ingress_class,
                rules: Some(rules),
                tls: tls_config,
                ..Default::default()
            }),
            status: None,
        })
    }
}

impl Default for IngressBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    include!("ingress_test.rs");
}
