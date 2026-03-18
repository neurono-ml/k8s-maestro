//! # k8s-maestro
//!
//! A Kubernetes job orchestrator library for managing workflows and steps in Kubernetes clusters.
//!
//! This crate provides a high-level API for creating, managing, and orchestrating
//! Kubernetes jobs with simplified builders and type-safe interfaces for workflow management.

pub mod client;
pub mod clients;
pub mod entities;
pub mod images;
pub mod migration;
pub mod networking;
pub mod security;
pub mod steps;
pub mod workflows;

pub use client::{MaestroClient, MaestroClientBuilder};
pub use clients::MaestroK8sClient;
pub use entities::ComputeResource;
pub use networking::{
    headless_service_dns_pattern, pod_dns_name, service_dns_name, IngressBuilder, IngressPath,
    PathType, PluginInfo, PluginRegistry, ServiceBuilder, ServicePort, ServiceType, SidecarPlugin,
    TLSConfig,
};
pub use steps::{StepResult, StepStatus};
pub use workflows::{
    ExecutionMode, LegacyCheckpointConfig, Workflow, WorkflowBuilder, WorkflowMetadata,
};
