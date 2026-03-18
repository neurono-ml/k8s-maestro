//! Common test infrastructure for k8s-maestro integration and E2E tests.
//!
//! This module provides shared test utilities including:
//! - Kind cluster lifecycle management
//! - Test fixtures for Kubernetes resources
//! - Helper utilities for resource creation, cleanup, and validation
//! - Mocking utilities for unit tests

pub mod e2e_helpers;
pub mod fixtures;
pub mod kind_cluster;
pub mod mocking;
pub mod utilities;

// Re-export commonly used types for convenience
// TODO: These functions are defined in utilities but are not yet re-exported here
// Uncomment when needed: pub use utilities::{create_configmap, create_namespace, create_pvc, create_secret};
