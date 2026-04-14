//! Python step examples for K8s Maestro workflows.
//!
//! This example demonstrates how to use Python steps in workflows
//! with real API calls and actual configuration options.

use k8s_maestro::{
    clients::MaestroK8sClient, MaestroClientBuilder, WorkflowBuilder, PythonStepBuilder, ResourceLimits,
};
use k8s_maestro::steps::{WorkFlowStep, ExecutableWorkFlowStep};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up logging to see what's happening
    log::set_max_level(log::LevelFilter::Info);
    env_logger::init();

    println!("=== K8s Maestro Python Step Examples ===\n");
    println!("Note: This example demonstrates the API usage patterns.");
    println!("In production, you would need a valid Kubernetes configuration.\n");

    // Create a Kubernetes client
    println!("Creating Kubernetes client...");
    let k8s_client = MaestroK8sClient::new().await?;
    println!("✓ Kubernetes client created\n");

    // Create a Maestro client in dry-run mode
    println!("Creating Maestro client (dry run mode)...");
    let maestro_client = MaestroClientBuilder::new()
        .with_namespace("default")
        .with_dry_run(true)  // Set to false to actually execute
        .build()?;
    println!("✓ Maestro client created in dry-run mode\n");

    // Run all examples
    example_basic_python_step(&k8s_client).await?;
    example_python_with_requirements(&k8s_client).await?;
    example_python_with_resource_limits(&k8s_client).await?;
    example_python_with_environment_variables(&k8s_client).await?;
    example_python_workflow(&k8s_client, &maestro_client).await?;

    println!("\n=== All examples completed successfully ===");
    Ok(())
}

/// Example 1: Basic Python step with inline code
async fn example_basic_python_step(k8s_client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("Example 1: Basic Python step with inline code");
    println!("─────────────────────────────────────────────");

    let python_code = r#"
import sys
print("Hello from Python step!")
print(f"Python version: {sys.version}")
print("Processing data...")

# Simple data processing
data = [1, 2, 3, 4, 5]
result = sum(data)
print(f"Sum of {data} = {result}")
"#;

    let step = PythonStepBuilder::new()
        .with_name("basic-python-step")
        .with_code(python_code)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("✓ Created basic Python step: {}", step.step_id());
    println!("  - Inline code: {} bytes", python_code.len());
    println!("  - Default image: python:3.12-slim");

    // Execute the step (will be dry run)
    let result = step.execute()?;
    println!("✓ Step execution result: {:?}", result.status);

    Ok(())
}

/// Example 2: Python step with package requirements
async fn example_python_with_requirements(k8s_client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("\nExample 2: Python step with package requirements");
    println!("─────────────────────────────────────────────────");

    let python_code = r#"
import pandas as pd
import numpy as np

# Create a sample dataset
data = {
    'name': ['Alice', 'Bob', 'Charlie'],
    'age': [25, 30, 35],
    'city': ['NYC', 'LA', 'SF']
}

df = pd.DataFrame(data)
print("Created DataFrame:")
print(df)

# Calculate statistics
print(f"\nAverage age: {df['age'].mean()}")
print(f"Age range: {df['age'].min()} - {df['age'].max()}")
"#;

    let requirements = vec![
        "pandas>=2.0.0",
        "numpy>=1.24.0",
    ];

    let step = PythonStepBuilder::new()
        .with_name("python-with-requirements")
        .with_code(python_code)
        .with_requirements(&requirements)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("✓ Created Python step with requirements: {}", step.step_id());
    println!("  - Packages: {}", requirements.join(", "));
    println!("  - Code size: {} bytes", python_code.len());

    let result = step.execute()?;
    println!("✓ Step execution result: {:?}", result.status);

    Ok(())
}

/// Example 3: Python step with resource limits
async fn example_python_with_resource_limits(k8s_client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("\nExample 3: Python step with resource limits");
    println!("───────────────────────────────────────────");

    let python_code = r#"
import time
import psutil
import os

print("Starting resource-intensive task...")
print(f"Available memory: {psutil.virtual_memory().available / (1024**3):.2f} GB")

# Simulate some work
for i in range(5):
    print(f"Processing iteration {i+1}/5")
    time.sleep(1)

print("Task completed!")
"#;

    let resource_limits = ResourceLimits::new()
        .with_cpu("500m")           // 0.5 CPU cores
        .with_cpu_request("250m")   // 0.25 CPU cores requested
        .with_memory("512Mi")       // 512 MB memory limit
        .with_memory_request("256Mi"); // 256 MB memory requested

    let step = PythonStepBuilder::new()
        .with_name("python-with-limits")
        .with_code(python_code)
        .with_requirements(&["psutil>=5.9.0"])
        .with_resource_limits(resource_limits)
        .with_timeout(Duration::from_secs(60))
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("✓ Created Python step with resource limits: {}", step.step_id());
    println!("  - CPU limit: 500m");
    println!("  - CPU request: 250m");
    println!("  - Memory limit: 512Mi");
    println!("  - Memory request: 256Mi");
    println!("  - Timeout: 60 seconds");

    let result = step.execute()?;
    println!("✓ Step execution result: {:?}", result.status);

    Ok(())
}

/// Example 4: Python step with environment variables
async fn example_python_with_environment_variables(k8s_client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("\nExample 4: Python step with environment variables");
    println!("──────────────────────────────────────────────────");

    let python_code = r#"
import os
import json

# Access environment variables
database_url = os.getenv('DATABASE_URL')
api_key = os.getenv('API_KEY')
debug_mode = os.getenv('DEBUG_MODE')
batch_size = os.getenv('BATCH_SIZE', '100')

print("Configuration loaded:")
print(f"  Database URL: {database_url}")
print(f"  API Key: {api_key[:10]}..." if api_key else "  API Key: Not set")
print(f"  Debug Mode: {debug_mode}")
print(f"  Batch Size: {batch_size}")

# Simulate processing with configuration
config = {
    'database_url': database_url,
    'debug': debug_mode == 'true',
    'batch_size': int(batch_size)
}

print(f"\nProcessing with config: {json.dumps(config, indent=2)}")
print("Processing complete!")
"#;

    let step = PythonStepBuilder::new()
        .with_name("python-with-env-vars")
        .with_code(python_code)
        .with_env("DATABASE_URL", "postgresql://localhost:5432/mydb")
        .with_env("API_KEY", "sk-proj-abc123xyz789")
        .with_env("DEBUG_MODE", "true")
        .with_env("BATCH_SIZE", "50")
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("✓ Created Python step with environment variables: {}", step.step_id());
    println!("  - DATABASE_URL: postgresql://localhost:5432/mydb");
    println!("  - API_KEY: sk-proj-*** (redacted)");
    println!("  - DEBUG_MODE: true");
    println!("  - BATCH_SIZE: 50");

    let result = step.execute()?;
    println!("✓ Step execution result: {:?}", result.status);

    Ok(())
}

/// Example 5: Complete workflow with multiple Python steps
async fn example_python_workflow(
    k8s_client: &MaestroK8sClient,
    maestro_client: &k8s_maestro::MaestroClient,
) -> anyhow::Result<()> {
    println!("\nExample 5: Complete workflow with multiple Python steps");
    println!("──────────────────────────────────────────────────────────");

    // Step 1: Data preparation
    let prepare_step = PythonStepBuilder::new()
        .with_name("prepare-data")
        .with_code(r#"
import json
data = {'records': [i for i in range(100)]}
with open('/output/data.json', 'w') as f:
    json.dump(data, f)
print(f"Prepared {len(data['records'])} records")
"#)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    // Step 2: Data processing
    let process_step = PythonStepBuilder::new()
        .with_name("process-data")
        .with_code(r#"
import json
with open('/output/data.json', 'r') as f:
    data = json.load(f)

processed = [x * 2 for x in data['records']]
print(f"Processed {len(processed)} records")
print(f"Sample results: {processed[:5]}")
"#)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    // Step 3: Data analysis
    let analyze_step = PythonStepBuilder::new()
        .with_name("analyze-data")
        .with_code(r#"
import json
with open('/output/data.json', 'r') as f:
    data = json.load(f)

records = data['records']
print(f"Analysis Results:")
print(f"  Count: {len(records)}")
print(f"  Min: {min(records)}")
print(f"  Max: {max(records)}")
print(f"  Sum: {sum(records)}")
print(f"  Average: {sum(records)/len(records):.2f}")
"#)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("✓ Created 3 Python steps:");
    println!("  1. prepare-data - Data preparation");
    println!("  2. process-data - Data processing");
    println!("  3. analyze-data - Data analysis");

    // Build workflow
    let workflow = WorkflowBuilder::new()
        .with_name("python-data-pipeline")
        .with_namespace("default")
        .add_step(prepare_step)
        .add_step(process_step)
        .add_step(analyze_step)
        .with_parallelism(1)  // Execute sequentially
        .build()?;

    println!("\n✓ Built workflow: {}", workflow.name);
    println!("  - Steps: {}", workflow.steps.len());
    println!("  - Parallelism: 1 (sequential execution)");

    // Create workflow (dry run)
    let created = maestro_client.create_workflow(workflow)?;
    println!("✓ Workflow created (dry run): {}", created.name());
    println!("  - Workflow ID: {}", created.id());
    println!("  - Namespace: {}", created.namespace());

    Ok(())
}
