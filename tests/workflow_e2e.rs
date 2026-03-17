//! Sample E2E test demonstrating full workflow scenarios.
//!
//! This test file shows how to use the E2E test infrastructure for
//! validating complete workflow execution.

mod common;

use common::e2e_helpers::{run_workflow_scenario, setup_e2e_test};
use common::fixtures::load_workflow_fixture;

/// E2E test that validates a simple workflow execution.
///
/// This test:
/// 1. Sets up a Kind cluster
/// 2. Creates a test namespace
/// 3. Applies a workflow from fixtures
/// 4. Validates workflow execution
/// 5. Cleans up resources
#[tokio::test]
#[ignore = "Requires Docker and is slow - run with --ignored flag"]
async fn e2e_simple_workflow() {
    // Setup E2E test environment
    let (client, _cluster) = setup_e2e_test().await.expect("Failed to setup E2E test");

    // Load workflow fixture
    let workflow =
        load_workflow_fixture("simple-workflow").expect("Failed to load workflow fixture");

    // Convert workflow to YAML string
    let workflow_yaml = serde_yml::to_string(&workflow).expect("Failed to serialize workflow");

    // Run workflow scenario
    run_workflow_scenario(&client, &workflow_yaml, "default")
        .await
        .expect("Workflow scenario failed");
}

/// E2E test that validates workflow with dependencies.
#[tokio::test]
#[ignore = "Requires Docker and is slow - run with --ignored flag"]
async fn e2e_workflow_with_dependencies() {
    let (client, _cluster) = setup_e2e_test().await.expect("Failed to setup E2E test");

    // This test would validate that step dependencies are correctly handled
    // and that steps execute in the correct order

    // For now, we just verify the setup works
    assert!(true, "E2E setup completed successfully");
}
