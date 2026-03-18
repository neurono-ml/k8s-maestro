//! Client module for Kubernetes API interactions.
//!
//! This module provides MaestroClient and MaestroClientBuilder for managing
//! Kubernetes workflows with a fluent builder pattern and centralized configuration.
//!
//! Also includes security client for managing Kubernetes security resources.
//!
//! # Example
//!
//! ```no_run
//! use k8s_maestro::{MaestroClientBuilder, MaestroClient};
//! use k8s_maestro::client::SecurityClient;
//!
//! let client = MaestroClientBuilder::new()
//!     .with_namespace("production")
//!     .build()
//!     .unwrap();
//! let security = SecurityClient { client: &client };
//! let policy = security.network_policy();
//! ```

pub mod builder;
pub mod maestro_client;
pub mod security_client;

pub use builder::MaestroClientBuilder;
pub use maestro_client::MaestroClient;
pub use security_client::SecurityClient;
