//! Networking module for Kubernetes networking configurations.
//!
//! This module provides builders and utilities for creating Kubernetes networking resources
//! including Services, Ingress, and DNS utilities for service discovery.
//!
//! # Examples
//!
//! ## Creating a Service
//!
//! ```no_run
//! use k8s_maestro::{ServiceBuilder, ServiceType};
//! use std::collections::BTreeMap;
//!
//! let mut selector = BTreeMap::new();
//! selector.insert("app".to_string(), "myapp".to_string());
//!
//! let service = ServiceBuilder::new()
//!     .with_name("my-service")
//!     .with_namespace("default")
//!     .with_port(80, 8080, "TCP")
//!     .with_selector(selector)
//!     .with_type(ServiceType::ClusterIP)
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Creating an Ingress
//!
//! ```no_run
//! use k8s_maestro::IngressBuilder;
//!
//! let ingress = IngressBuilder::new()
//!     .with_name("my-ingress")
//!     .with_namespace("default")
//!     .with_host("example.com")
//!     .with_path("/", "my-service", 80)
//!     .with_tls_secret("tls-secret")
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Using DNS Utilities
//!
//! ```
//! use k8s_maestro::{service_dns_name, pod_dns_name, headless_service_dns_pattern};
//!
//! let service_dns = service_dns_name("my-service", "default");
//! assert_eq!(service_dns, "my-service.default.svc.cluster.local");
//!
//! let pod_dns = pod_dns_name("my-pod", "default");
//! assert_eq!(pod_dns, "my-pod.default.pod.cluster.local");
//!
//! let headless_dns = headless_service_dns_pattern("stateful-set", "default");
//! assert_eq!(headless_dns, "*.stateful-set.default.svc.cluster.local");
//! ```

pub mod dns;
pub mod ingress;
pub mod service;

pub use dns::{headless_service_dns_pattern, pod_dns_name, service_dns_name};
pub use ingress::{IngressBuilder, IngressPath, PathType, TLSConfig};
pub use service::{ServiceBuilder, ServicePort, ServiceType};
