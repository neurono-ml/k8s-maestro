//! Test fixtures management module.
//!
//! This module provides utilities for loading and parsing YAML fixtures
//! for Kubernetes resources used in tests.

use std::path::{Path, PathBuf};

use k8s_openapi::api::{
    batch::v1::Job,
    core::v1::{ConfigMap, PersistentVolumeClaim, Secret},
};
use serde::de::DeserializeOwned;

/// Base directory for fixture files.
const FIXTURES_BASE_DIR: &str = "tests/common/fixtures";

/// Loads a YAML fixture file and deserializes it to the specified type.
///
/// # Arguments
///
/// * `fixture_path` - Path relative to the fixtures base directory
///
/// # Returns
///
/// The deserialized fixture data.
///
/// # Example
///
/// ```no_run
/// use k8s_maestro::tests::common::fixtures::load_yaml_fixture;
/// use k8s_openapi::api::core::v1::ConfigMap;
///
/// let configmap: ConfigMap = load_yaml_fixture("configmaps/test-configmap.yaml")
///     .expect("Failed to load fixture");
/// ```
pub fn load_yaml_fixture<T: DeserializeOwned>(
    fixture_path: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let full_path = PathBuf::from(FIXTURES_BASE_DIR).join(fixture_path);
    let content = std::fs::read_to_string(&full_path)?;
    let parsed: T = serde_yml::from_str(&content)?;
    Ok(parsed)
}

/// Loads a ConfigMap fixture from the configmaps directory.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (without extension)
pub fn load_configmap_fixture(name: &str) -> Result<ConfigMap, Box<dyn std::error::Error>> {
    let path = format!("configmaps/{}.yaml", name);
    load_yaml_fixture(&path)
}

/// Loads a Secret fixture from the secrets directory.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (without extension)
pub fn load_secret_fixture(name: &str) -> Result<Secret, Box<dyn std::error::Error>> {
    let path = format!("secrets/{}.yaml", name);
    load_yaml_fixture(&path)
}

/// Loads a PVC fixture from the pvcs directory.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (without extension)
pub fn load_pvc_fixture(name: &str) -> Result<PersistentVolumeClaim, Box<dyn std::error::Error>> {
    let path = format!("pvcs/{}.yaml", name);
    load_yaml_fixture(&path)
}

/// Loads a Job fixture from the failure_scenarios directory.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (without extension)
pub fn load_job_fixture(name: &str) -> Result<Job, Box<dyn std::error::Error>> {
    let path = format!("failure_scenarios/{}.yaml", name);
    load_yaml_fixture(&path)
}

/// Loads a workflow fixture from the workflows directory.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (without extension)
///
/// # Note
///
/// Workflows use a generic YAML value since the Workflow CRD is custom.
pub fn load_workflow_fixture(name: &str) -> Result<serde_yml::Value, Box<dyn std::error::Error>> {
    let path = format!("workflows/{}.yaml", name);
    load_yaml_fixture(&path)
}

/// Returns the path to the fixtures directory.
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(FIXTURES_BASE_DIR)
}

/// Returns the path to a specific fixture subdirectory.
///
/// # Arguments
///
/// * `subdir` - The subdirectory name (e.g., "configmaps", "secrets")
pub fn fixtures_subdir(subdir: &str) -> PathBuf {
    fixtures_dir().join(subdir)
}

/// Checks if a fixture file exists.
///
/// # Arguments
///
/// * `fixture_path` - Path relative to the fixtures base directory
pub fn fixture_exists(fixture_path: &str) -> bool {
    let full_path = PathBuf::from(FIXTURES_BASE_DIR).join(fixture_path);
    full_path.exists()
}

/// Lists all fixture files in a subdirectory.
///
/// # Arguments
///
/// * `subdir` - The subdirectory to list
pub fn list_fixtures(subdir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let dir = fixtures_subdir(subdir);
    let mut fixtures = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "yaml") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                fixtures.push(name.to_string());
            }
        }
    }

    Ok(fixtures)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixtures_dir() {
        let dir = fixtures_dir();
        assert_eq!(dir.to_str().unwrap(), FIXTURES_BASE_DIR);
    }

    #[test]
    fn test_fixtures_subdir() {
        let dir = fixtures_subdir("configmaps");
        assert!(dir.to_str().unwrap().ends_with("configmaps"));
    }

    #[test]
    fn test_fixture_exists() {
        // These tests verify the sample fixtures exist
        assert!(fixture_exists("configmaps/test-configmap.yaml"));
        assert!(fixture_exists("secrets/test-secret.yaml"));
        assert!(fixture_exists("pvcs/test-pvc.yaml"));
        assert!(fixture_exists("failure_scenarios/failing-job.yaml"));
        assert!(fixture_exists("workflows/simple-workflow.yaml"));
    }

    #[test]
    fn test_load_configmap_fixture() {
        let cm = load_configmap_fixture("test-configmap").expect("Failed to load ConfigMap");
        assert_eq!(cm.metadata.name.unwrap(), "test-configmap");
    }

    #[test]
    fn test_load_secret_fixture() {
        let secret = load_secret_fixture("test-secret").expect("Failed to load Secret");
        assert_eq!(secret.metadata.name.unwrap(), "test-secret");
    }

    #[test]
    fn test_load_pvc_fixture() {
        let pvc = load_pvc_fixture("test-pvc").expect("Failed to load PVC");
        assert_eq!(pvc.metadata.name.unwrap(), "test-pvc");
    }

    #[test]
    fn test_load_job_fixture() {
        let job = load_job_fixture("failing-job").expect("Failed to load Job");
        assert_eq!(job.metadata.name.unwrap(), "failing-job");
    }

    #[test]
    fn test_list_fixtures() {
        let fixtures = list_fixtures("configmaps").expect("Failed to list fixtures");
        assert!(fixtures.contains(&"test-configmap".to_string()));
    }
}
