#!/usr/bin/env python3
"""
generate_workflow_code.py - Generate k8s-maestro workflow code from natural language

This script takes a natural language description of a workflow and generates
the corresponding Rust code using k8s-maestro builders.
"""

import sys
import argparse
from typing import List, Dict, Optional


# Workflow patterns based on common use cases
PATTERNS = {
    "etl": {
        "description": "ETL Pipeline (Extract, Transform, Load)",
        "steps": ["extract", "transform", "load"],
        "images": {
            "extract": "python:3.11",
            "transform": "python:3.11",
            "load": "postgres:16",
        },
        "dependencies": {"transform": ["extract"], "load": ["transform"]},
        "parallelism": 2,
    },
    "batch": {
        "description": "Batch Processing Job",
        "steps": ["batch-process"],
        "images": {"batch-process": "python:3.11"},
        "dependencies": {},
        "parallelism": 1,
    },
    "webapp": {
        "description": "Web Application with Monitoring",
        "steps": ["web-app"],
        "images": {"web-app": "nginx:latest"},
        "sidecars": [
            {"name": "logs", "image": "fluent/fluent-bit:2.2"},
            {"name": "metrics", "image": "prom/prometheus-node-exporter:latest"},
        ],
        "dependencies": {},
        "networking": {"service": True, "ingress": True},
    },
    "data-pipeline": {
        "description": "Data Processing Pipeline",
        "steps": ["ingest", "process", "validate", "output"],
        "images": {
            "ingest": "python:3.11",
            "process": "python:3.11",
            "validate": "python:3.11",
            "output": "postgres:16",
        },
        "dependencies": {
            "process": ["ingest"],
            "validate": ["process"],
            "output": ["validate"],
        },
        "parallelism": 3,
    },
    "ml-training": {
        "description": "ML Training Pipeline",
        "steps": ["prepare-data", "train-model", "evaluate-model", "deploy-model"],
        "images": {
            "prepare-data": "python:3.11",
            "train-model": "python:3.11-gpu",
            "evaluate-model": "python:3.11",
            "deploy-model": "python:3.11",
        },
        "dependencies": {
            "train-model": ["prepare-data"],
            "evaluate-model": ["train-model"],
            "deploy-model": ["evaluate-model"],
        },
        "parallelism": 1,
        "gpu": True,
    },
}


def detect_pattern(description: str) -> Optional[str]:
    """Detect the workflow pattern from description."""
    description_lower = description.lower()

    keywords = {
        "etl": ["etl", "extract", "transform", "load", "pipeline"],
        "batch": ["batch", "job", "one-time", "scheduled"],
        "webapp": ["web", "app", "service", "ingress", "http", "api"],
        "data-pipeline": ["data", "pipeline", "process", "validate", "output"],
        "ml-training": ["ml", "machine learning", "train", "model", "evaluate"],
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


def generate_imports(pattern: str) -> str:
    """Generate import statements."""
    imports = [
        "use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};",
        "use k8s_maestro::steps::{KubeJobStep, KubeJobStepBuilder};",
        "use k8s_maestro::clients::MaestroK8sClient;",
        "use std::collections::BTreeMap;",
    ]

    if pattern == "webapp":
        imports.extend(
            [
                "use k8s_maestro::{ServiceBuilder, ServiceType, IngressBuilder};",
                "use k8s_maestro::entities::{MaestroContainer, SidecarContainer};",
            ]
        )

    elif pattern in ["ml-training", "data-pipeline"]:
        imports.extend(
            [
                "use k8s_maestro::entities::{MaestroContainer, ContainerLike};",
                "use k8s_openapi::api::core::v1::{ResourceRequirements, Quantity};",
            ]
        )

    return "\n".join(imports)


def generate_container_definition(step_name: str, image: str, pattern: str) -> str:
    """Generate container definition."""
    container_def = (
        f'let {step_name}_container = MaestroContainer::new("{image}", "{step_name}")'
    )

    if pattern in ["ml-training", "data-pipeline", "etl"]:
        # Add resource limits for data-intensive workloads
        container_def += "\n    .set_resource_limits(\n"
        container_def += f"        ResourceLimits::new()\n"
        container_def += f'            .with_cpu("1000m")\n'
        container_def += f'            .with_memory("1Gi")\n'
        container_def += f"    );"

    return container_def


def generate_step_definition(step_name: str, image: str, pattern: str) -> str:
    """Generate step definition."""
    if pattern == "webapp":
        return f'let {step_name}_step = KubeJobStep::new("{step_name}", "{image}", k8s_client.clone());'
    else:
        return f'let {step_name}_step = KubeJobStep::new("{step_name}", "{image}", k8s_client.clone());'


def generate_workflow_builder(pattern: str) -> str:
    """Generate workflow builder code."""
    pattern_data = PATTERNS[pattern]

    # Start with workflow builder
    builder = f"let workflow = WorkflowBuilder::new()\n"
    builder += f'    .with_name("{pattern}-workflow")\n'
    builder += f'    .with_namespace("production")\n'

    # Add parallelism if specified
    if "parallelism" in pattern_data and pattern_data["parallelism"] > 1:
        builder += f"    .with_parallelism({pattern_data['parallelism']})\n"

    # Add steps
    for step in pattern_data["steps"]:
        builder += f"    .add_step({step}_step)\n"

    # Build workflow
    builder += "    .build()?;\n"

    return builder


def generate_dependency_chain(pattern: str) -> Optional[str]:
    """Generate dependency chain if needed."""
    pattern_data = PATTERNS[pattern]

    if not pattern_data.get("dependencies"):
        return None

    chain = "let mut chain = DependencyChain::new();\n"

    # Add steps
    for step in pattern_data["steps"]:
        chain += f'chain.add_step("{step}");\n'

    # Add dependencies
    for step, deps in pattern_data["dependencies"].items():
        for dep in deps:
            chain += f'chain.add_step("{step}").with_dependency("{dep}");\n'

    chain += "\nlet graph = chain.build_dag()?;\n"
    chain += "let execution_levels = graph.topological_sort()?;\n"

    return chain


def generate_networking(pattern: str) -> Optional[str]:
    """Generate networking configuration if needed."""
    if pattern != "webapp":
        return None

    # Generate service
    service = """
// Create service
let mut selector = BTreeMap::new();
selector.insert("app".to_string(), "web-app".to_string());

let service = ServiceBuilder::new()
    .with_name("web-app-service")
    .with_namespace("production")
    .with_port(80, 8080, "TCP")
    .with_selector(selector)
    .with_type(ServiceType::ClusterIP)
    .build()?;

client.create_service(&service).await?;
"""

    # Generate ingress
    ingress = """
// Create ingress
let ingress = IngressBuilder::new()
    .with_name("web-app-ingress")
    .with_namespace("production")
    .with_host("web-app.example.com")
    .with_path("/", "web-app-service", 80)
    .with_tls_secret("tls-cert")
    .build()?;

client.create_ingress(&ingress).await?;
"""

    return service + ingress


def generate_full_code(pattern: str) -> str:
    """Generate complete workflow code."""
    pattern_data = PATTERNS[pattern]

    # Generate imports
    code = generate_imports(pattern) + "\n\n"

    # Add dependency chain import if needed
    if pattern_data.get("dependencies"):
        code += "use k8s_maestro::workflows::DependencyChain;\n"

    # Generate main function
    code += '#[tokio::main(flavor = "current_thread")]\n'
    code += "pub async fn main() -> anyhow::Result<()> {\n"
    code += "    // Create Kubernetes client\n"
    code += "    let k8s_client = MaestroK8sClient::new().await?;\n\n"

    # Generate client builder
    code += "    let client = MaestroClientBuilder::new()\n"
    code += '        .with_namespace("production")\n'
    code += "        .build()?;\n\n"

    # Generate container definitions
    code += "    // Build containers\n"
    for step in pattern_data["steps"]:
        image = pattern_data["images"][step]
        code += generate_container_definition(step, image, pattern) + "\n\n"

    # Generate step definitions
    code += "    // Build steps\n"
    for step in pattern_data["steps"]:
        image = pattern_data["images"][step]
        code += generate_step_definition(step, image, pattern) + "\n\n"

    # Generate workflow builder
    code += "    // Build workflow\n"
    code += generate_workflow_builder(pattern) + "\n"

    # Generate dependency chain if needed
    if pattern_data.get("dependencies"):
        code += "    // Setup dependencies\n"
        dep_chain = generate_dependency_chain(pattern)
        if dep_chain:
            code += dep_chain + "\n"

    # Generate networking if needed
    if pattern == "webapp":
        code += "    // Setup networking\n"
        networking = generate_networking(pattern)
        if networking:
            code += networking + "\n"

    # Execute workflow
    code += "    // Execute workflow\n"
    code += "    let execution = client.execute_workflow(&workflow).await?;\n\n"

    code += '    println!("Workflow executed with ID: {}", execution.id());\n\n'

    # Wait for completion
    code += "    // Wait for completion\n"
    code += "    execution.wait_for_completion().await?;\n\n"

    code += "    if execution.is_success() {\n"
    code += '        println!("Workflow completed successfully!");\n'
    code += "    } else {\n"
    code += '        eprintln!("Workflow failed!");\n'
    code += "    }\n\n"

    code += "    Ok(())\n"
    code += "}\n"

    return code


def print_usage() -> None:
    """Print usage information."""
    print("k8s-maestro Workflow Code Generator")
    print()
    print("Usage:")
    print("  python generate_workflow_code.py <description>")
    print()
    print("Examples:")
    print(
        '  python generate_workflow_code.py "ETL pipeline with extract, transform, load"'
    )
    print('  python generate_workflow_code.py "Web application with monitoring"')
    print('  python generate_workflow_code.py "ML training pipeline"')
    print()
    print("Supported patterns:")
    print("  - ETL Pipeline (extract, transform, load)")
    print("  - Batch Processing Job")
    print("  - Web Application with Monitoring")
    print("  - Data Processing Pipeline")
    print("  - ML Training Pipeline")


def main():
    """Main function."""
    if len(sys.argv) < 2:
        print_usage()
        sys.exit(1)

    description = " ".join(sys.argv[1:])

    print(f"Analyzing description: {description}")
    print()

    # Detect pattern
    pattern = detect_pattern(description)

    if not pattern:
        print("Warning: Could not detect specific pattern, using generic workflow")
        pattern = "batch"

    pattern_data = PATTERNS[pattern]
    print(f"Detected pattern: {pattern_data['description']}")
    print()

    # Generate code
    code = generate_full_code(pattern)

    print("Generated code:")
    print("=" * 70)
    print(code)
    print("=" * 70)

    # Save to file
    filename = f"{pattern}_workflow.rs"
    with open(filename, "w") as f:
        f.write(code)

    print(f"\nCode saved to: {filename}")


if __name__ == "__main__":
    main()
