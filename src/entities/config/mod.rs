//! Configuration builders for Kubernetes ConfigMaps and Secrets.
//!
//! This module provides fluent builders for creating Kubernetes configuration resources:
//! - `ConfigMapBuilder`: Build ConfigMaps for non-sensitive configuration data
//! - `SecretBuilder`: Build Secrets for sensitive data with type-safe secret types
//! - `ImagePullSecretBuilder`: Build docker-registry secrets for container image authentication

pub mod configmap;
pub mod image_pull_secret;
pub mod secret;

#[cfg(test)]
mod tests;

pub use configmap::ConfigMapBuilder;
pub use image_pull_secret::ImagePullSecretBuilder;
pub use secret::{SecretBuilder, SecretType};
