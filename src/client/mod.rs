//! Client module for Kubernetes API interactions.
//!
//! This module provides the MaestroClient and MaestroClientBuilder for managing
//! Kubernetes workflows with a fluent builder pattern and centralized configuration.

mod builder;
mod maestro_client;

pub use builder::MaestroClientBuilder;
pub use maestro_client::{CreatedWorkflow, MaestroClient};
