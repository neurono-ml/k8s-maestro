#!/usr/bin/env python3
"""
generate_test_code.py - Generate k8s-maestro test code from natural language

This script takes a natural language description of test requirements and generates
the corresponding Rust test code for k8s-maestro.
"""

import sys
import argparse
from typing import List, Dict, Optional


# Test patterns based on common test scenarios
TEST_PATTERNS = {
    "unit-test": {
        "description": "Unit Test (no cluster needed)",
        "requires_cluster": False,
        "example": """
#[test]
fn test_workflow_builder_basic() {
    let workflow = WorkflowBuilder::new()
        .with_name("test-workflow")
        .with_namespace("test-ns")
        .build();

    assert!(workflow.is_ok());
    let wf = workflow.unwrap();
    assert_eq!(wf.name, "test-workflow");
}
""",
    },
    "integration-test": {
        "description": "Integration Test (requires Kind cluster)",
        "requires_cluster": True,
        "example": """
#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_workflow_execution() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    let workflow = build_test_workflow()?;
    let execution = execute_workflow(&client, &workflow).await?;

    assert!(execution.is_success());

    cluster.cleanup().await?;
}
""",
    },
    "e2e-test": {
        "description": "E2E Test (requires Kind cluster, tests full workflow)",
        "requires_cluster": True,
        "example": """
#[tokio::test]
#[ignore = "Requires Docker"]
async fn e2e_complete_pipeline() {
    let (client, cluster) = setup_e2e_test().await?;

    // Create resources
    let configmap = create_configmap("test-config", "default", BTreeMap::new());
    apply_resource(&client, &configmap, "default").await?;

    // Execute workflow
    let workflow = build_production_workflow()?;
    let execution = execute_workflow(&client, &workflow).await?;

    assert!(execution.is_success());
    
    // Verify results
    let results = client.get_workflow_results(execution.id()).await?;
    assert!(!results.is_empty());

    cluster.cleanup().await?;
}
""",
    },
    "configmap-test": {
        "description": "ConfigMap Lifecycle Test",
        "requires_cluster": True,
        "example": """
#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_configmap_lifecycle() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    // Create namespace
    create_namespace(&client, "test-ns").await?;

    // Create ConfigMap
    let configmap = ConfigMapBuilder::new()
        .with_name("test-cm")
        .with_namespace("test-ns")
        .add_data("key1", "value1")
        .build()?;

    apply_resource(&client, &configmap, "test-ns").await?;

    // Verify it exists
    assert!(
        verify_resource_exists::<ConfigMap>(&client, "test-cm", "test-ns").await,
        "ConfigMap should exist"
    );

    cluster.cleanup().await?;
}
""",
    },
    "secret-test": {
        "description": "Secret Lifecycle Test",
        "requires_cluster": True,
        "example": """
#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_secret_lifecycle() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    create_namespace(&client, "test-ns").await?;

    let secret = SecretBuilder::new()
        .with_name("test-secret")
        .with_namespace("test-ns")
        .add_data("api-key", base64::encode("secret-key"))
        .build()?;

    apply_resource(&client, &secret, "test-ns").await?;

    assert!(
        verify_resource_exists::<Secret>(&client, "test-secret", "test-ns").await,
        "Secret should exist"
    );

    cluster.cleanup().await?;
}
""",
    },
    "pvc-test": {
        "description": "PVC Lifecycle Test",
        "requires_cluster": True,
        "example": """
#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_pvc_lifecycle() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    create_namespace(&client, "test-ns").await?;

    let pvc = MaestroPVCMountVolumeBuilder::new()
        .with_name("test-pvc")
        .with_access_mode(AccessMode::ReadWriteOnce)
        .with_storage("1Gi")
        .build()?;

    apply_resource(&client, &pvc, "test-ns").await?;

    // Wait for PVC to be bound
    wait_for_resource_ready::<PersistentVolumeClaim>(
        &client, "test-pvc", "test-ns",
        |pvc| pvc.status.as_ref().and_then(|s| s.phase.as_deref()) == Some("Bound")
    ).await?;

    cluster.cleanup().await?;
}
""",
    },
    "service-test": {
        "description": "Service and Ingress Test",
        "requires_cluster": True,
        "example": """
#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_service_ingress() {
    let cluster = KindCluster::new().await?;
    let client = create_client_from_cluster(&cluster);

    create_namespace(&client, "test-ns").await?;

    // Create deployment
    let deployment = create_test_deployment()?;
    apply_resource(&client, &deployment, "test-ns").await?;

    // Create service
    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "test".to_string());

    let service = ServiceBuilder::new()
        .with_name("test-service")
        .with_namespace("test-ns")
        .with_port(80, 8080, "TCP")
        .with_selector(selector)
        .build()?;

    apply_resource(&client, &service, "test-ns").await?;

    assert!(verify_resource_exists::<Service>(&client, "test-service", "test-ns").await);

    cluster.cleanup().await?;
}
""",
    },
}


def detect_test_pattern(description: str) -> Optional[str]:
    """Detect the test pattern from description."""
    description_lower = description.lower()

    keywords = {
        "unit-test": ["unit test", "builder", "logic", "no cluster", "mock"],
        "integration-test": ["integration", "workflow", "execute", "kind cluster"],
        "e2e-test": ["e2e", "end to end", "complete pipeline", "full workflow"],
        "configmap-test": ["configmap", "configuration", "config"],
        "secret-test": ["secret", "credential", "password"],
        "pvc-test": ["pvc", "persistent volume", "storage"],
        "service-test": ["service", "ingress", "networking"],
    }

    # Score each pattern
    scores = {}
    for pattern, words in keywords.items():
        score = sum(1 for word in words if word in description_lower)
        if score > 0:
            scores[pattern] = score

    # Return pattern with highest score
    if scores:
        return max(scores.keys(), key=lambda k: scores[k])

    return None


def generate_test_imports(pattern: str) -> str:
    """Generate import statements for tests."""
    imports = []

    if TEST_PATTERNS[pattern]["requires_cluster"]:
        imports.extend(
            [
                "use k8s_maestro::tests::common::kind_cluster::KindCluster;",
                "use k8s_maestro::tests::common::utilities::{",
                "    create_namespace, apply_resource, verify_resource_exists,",
                "    wait_for_resource_ready, delete_resource_by_name",
                "};",
            ]
        )

    if pattern == "unit-test":
        imports.append("use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};")

    if pattern == "configmap-test":
        imports.extend(
            [
                "use k8s_maestro::entities::ConfigMapBuilder;",
                "use k8s_openapi::api::core::v1::ConfigMap;",
            ]
        )

    if pattern == "secret-test":
        imports.extend(
            [
                "use k8s_maestro::entities::SecretBuilder;",
                "use k8s_openapi::api::core::v1::Secret;",
                "use base64;",
            ]
        )

    if pattern == "pvc-test":
        imports.extend(
            [
                "use k8s_maestro::entities::{PVCVolume, MaestroPVCMountVolumeBuilder};",
                "use k8s_maestro::entities::volume_types::AccessMode;",
                "use k8s_openapi::api::core::v1::PersistentVolumeClaim;",
            ]
        )

    if pattern == "service-test":
        imports.extend(
            [
                "use k8s_maestro::{ServiceBuilder, ServiceType};",
                "use std::collections::BTreeMap;",
                "use k8s_openapi::api::core::v1::Service;",
            ]
        )

    return "\n".join(imports)


def generate_full_test_code(pattern: str) -> str:
    """Generate complete test code."""
    pattern_data = TEST_PATTERNS[pattern]

    # Generate imports
    code = generate_test_imports(pattern) + "\n\n"

    # Add any imports from example
    if "anyhow" in pattern_data["example"]:
        code += "use anyhow::Result;\n"

    code += pattern_data["example"]

    return code


def print_usage() -> None:
    """Print usage information."""
    print("k8s-maestro Test Code Generator")
    print()
    print("Usage:")
    print("  python generate_test_code.py <description>")
    print()
    print("Examples:")
    print('  python generate_test_code.py "unit test for workflow builder"')
    print('  python generate_test_code.py "integration test with kind cluster"')
    print('  python generate_test_code.py "test configmap lifecycle"')
    print()
    print("Supported test patterns:")
    for pattern, data in TEST_PATTERNS.items():
        print(f"  - {data['description']}")


def main():
    """Main function."""
    if len(sys.argv) < 2:
        print_usage()
        sys.exit(1)

    description = " ".join(sys.argv[1:])

    print(f"Analyzing test description: {description}")
    print()

    # Detect pattern
    pattern = detect_test_pattern(description)

    if not pattern:
        print("Warning: Could not detect specific test pattern, using generic test")
        pattern = "unit-test"

    pattern_data = TEST_PATTERNS[pattern]
    print(f"Detected pattern: {pattern_data['description']}")

    if pattern_data["requires_cluster"]:
        print("Note: This test requires a Kind cluster (Docker)")
        print('Mark test with #[ignore = "Requires Docker"]')

    print()

    # Generate code
    code = generate_full_test_code(pattern)

    print("Generated test code:")
    print("=" * 70)
    print(code)
    print("=" * 70)

    # Save to file
    filename = f"{pattern}.rs"
    with open(filename, "w") as f:
        f.write(code)

    print(f"\nTest code saved to: {filename}")

    # Print running instructions
    if pattern_data["requires_cluster"]:
        print("\nTo run this test:")
        print(
            "  cargo test --test '{}' -- --ignored".format(filename.replace(".rs", ""))
        )


if __name__ == "__main__":
    main()
