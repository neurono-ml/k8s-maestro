# k8s-maestro Integrator Usage Guide

This guide provides comprehensive examples and tutorials for using the k8s-maestro-integrator skill to assist development with k8s-maestro.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Project Setup](#project-setup)
3. [Creating Workflows](#creating-workflows)
4. [Storage and Volumes](#storage-and-volumes)
5. [Networking](#networking)
6. [Security](#security)
7. [Testing](#testing)
8. [Deployment](#deployment)
9. [Advanced Patterns](#advanced-patterns)
10. [Troubleshooting](#troubleshooting)

---

## Getting Started

### First Steps with the Skill

The k8s-maestro-integrator skill activates automatically when you mention topics related to Kubernetes workflow orchestration with k8s-maestro. Here's how to start:

**Example 1: Setting up a new project**
```
You: I'm starting a new Rust project and want to use k8s-maestro to run data
   processing jobs on Kubernetes. How do I get started?

Claude (with skill): The skill will:
   1. Ask about your Kubernetes cluster version
   2. Recommend the correct crate features (k8s_v1_28, k8s_v1_29, etc.)
   3. Provide Cargo.toml configuration
   4. Generate a complete "Hello World" workflow example
   5. Explain the build, test, and deployment process
```

**Example 2: Migrating existing code**
```
You: I have an existing Kubernetes Job configuration in YAML. Can you help me
   convert it to k8s-maestro Rust code?

Claude (with skill): The skill will:
   1. Ask you to share your YAML
   2. Parse the YAML structure
   3. Generate equivalent Rust code using builders
   4. Explain the mapping between YAML and builder methods
   5. Provide testing and deployment guidance
```

---

## Project Setup

### Scenario 1: New Project with k8s-maestro

**Your request:**
```
I want to create a new Rust project that uses k8s-maestro. My Kubernetes cluster
is running version 1.31. I need to run Python scripts as workflow steps.
```

**What the skill provides:**

1. **Cargo.toml setup:**
```toml
[package]
name = "my-k8s-workflow"
version = "0.1.0"
edition = "2021"

[dependencies]
k8s-maestro = { version = "1.0", features = ["k8s_v1_31", "exec-steps"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"

[dev-dependencies]
testcontainers = "0.27"
```

2. **Basic project structure:**
```
my-k8s-workflow/
├── Cargo.toml
├── src/
│   └── main.rs
└── tests/
    └── integration/
        └── workflow_tests.rs
```

3. **Initial main.rs:**
```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::clients::MaestroK8sClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Create Kubernetes client
    let k8s_client = MaestroK8sClient::new().await?;

    // Build Maestro client
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .with_client(k8s_client.clone())
        .build()?;

    // Build workflow
    let workflow = WorkflowBuilder::new()
        .with_name("my-first-workflow")
        .add_step(KubeJobStep::new("hello-world", "python:3.11", k8s_client))
        .build()?;

    // Execute workflow
    let execution = client.execute_workflow(&workflow).await?;
    println!("Workflow executed: {}", execution.id());

    Ok(())
}
```

4. **Build and test instructions:**
```bash
# Build the project
cargo build --release

# Run the project
cargo run

# Run tests
cargo test

# Run integration tests (requires Docker)
cargo test --test '*' -- --ignored
```

---

## Creating Workflows

### Scenario 2: ETL Pipeline

**Your request:**
```
I need to create an ETL pipeline that:
1. Extracts data from an API using Python
2. Transforms the data using pandas
3. Loads it into PostgreSQL
4. Has retry logic and resource limits
5. Runs in production namespace
```

**What the skill provides:**

**Complete implementation:**
```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::{KubeJobStep, KubeJobStepBuilder};
use k8s_maestro::workflows::DependencyChain;
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::steps::traits::ResourceLimits;
use k8s_maestro::entities::MaestroContainer;
use std::collections::BTreeMap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    // Step 1: Extract data
    let extract_env = BTreeMap::new();
    extract_env.insert("API_URL".to_string(), "https://api.example.com/data".to_string());
    extract_env.insert("BATCH_SIZE".to_string(), "1000".to_string());

    let extract_container = MaestroContainer::new("python:3.11", "extract")
        .set_arguments(&[
            "python".to_string(),
            "-m".to_string(),
            "extract_data".to_string(),
        ])
        .set_environment_variables(extract_env)
        .set_resource_limits(ResourceLimits::new()
            .with_cpu("500m")
            .with_memory("512Mi"));

    let extract_step = KubeJobStepBuilder::new()
        .with_name("extract-data")
        .with_namespace("production")
        .add_container(Box::new(extract_container))
        .with_backoff_limit(3)
        .with_restart_policy(RestartPolicy::OnFailure)
        .build()?;

    // Step 2: Transform data
    let transform_env = BTreeMap::new();
    transform_env.insert("INPUT_PATH".to_string(), "/data/extracted".to_string());
    transform_env.insert("OUTPUT_PATH".to_string(), "/data/transformed".to_string());

    let transform_container = MaestroContainer::new("python:3.11", "transform")
        .set_arguments(&[
            "python".to_string(),
            "-m".to_string(),
            "transform_data".to_string(),
        ])
        .set_environment_variables(transform_env)
        .set_resource_limits(ResourceLimits::new()
            .with_cpu("1000m")
            .with_memory("1Gi"));

    let transform_step = KubeJobStepBuilder::new()
        .with_name("transform-data")
        .with_namespace("production")
        .add_container(Box::new(transform_container))
        .with_backoff_limit(3)
        .build()?;

    // Step 3: Load data
    let load_env = BTreeMap::new();
    load_env.insert("DB_HOST".to_string(), "postgres-service".to_string());
    load_env.insert("DB_PORT".to_string(), "5432".to_string());
    load_env.insert("DB_NAME".to_string(), "analytics".to_string());
    load_env.insert("INPUT_PATH".to_string(), "/data/transformed".to_string());

    let load_container = MaestroContainer::new("postgres:16", "load")
        .set_arguments(&[
            "psql".to_string(),
            "-f".to_string(),
            "/scripts/load_data.sql".to_string(),
        ])
        .set_environment_variables(load_env)
        .set_resource_limits(ResourceLimits::new()
            .with_cpu("500m")
            .with_memory("512Mi"));

    let load_step = KubeJobStepBuilder::new()
        .with_name("load-data")
        .with_namespace("production")
        .add_container(Box::new(load_container))
        .with_backoff_limit(3)
        .build()?;

    // Build workflow
    let workflow = WorkflowBuilder::new()
        .with_name("etl-pipeline")
        .with_namespace("production")
        .add_step(extract_step)
        .add_step(transform_step)
        .add_step(load_step)
        .with_parallelism(2)
        .build()?;

    // Setup dependency chain
    let mut chain = DependencyChain::new();
    chain.add_step("extract-data");
    chain.add_step("transform-data").with_dependency("extract-data");
    chain.add_step("load-data").with_dependency("transform-data");

    let graph = chain.build_dag()?;
    println!("Execution plan: {:?}", graph.topological_sort()?);

    // Execute workflow
    let execution = client.execute_workflow(&workflow).await?;
    println!("Workflow started: {}", execution.id());

    // Wait for completion
    execution.wait_for_completion().await?;

    if execution.is_success() {
        println!("ETL pipeline completed successfully!");
    } else {
        eprintln!("ETL pipeline failed!");
    }

    Ok(())
}
```

**Explanation:**
- Each step has dedicated resource limits
- Environment variables for configuration
- Backoff limit of 3 retries per step
- Dependency chain ensures sequential execution
- Parallelism set to 2 (can run up to 2 steps in parallel when dependencies allow)

### Scenario 3: Web Application with Monitoring

**Your request:**
```
I want to deploy a web application with:
1. Nginx as the main container
2. Fluent Bit sidecar for logging
3. Prometheus exporter sidecar for metrics
4. Service for internal access
5. Ingress for external access
6. Health checks
7. Resource limits
```

**What the skill provides:**

**Complete implementation:**
```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::{KubeJobStep, KubeJobStepBuilder};
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, SidecarContainer};
use k8s_maestro::{ServiceBuilder, ServiceType, IngressBuilder};
use k8s_maestro::steps::traits::ResourceLimits;
use std::collections::BTreeMap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    // Main web container
    let main_container = MaestroContainer::new("nginx:latest", "web-server")
        .set_resource_limits(ResourceLimits::new()
            .with_cpu("200m")
            .with_memory("256Mi"));

    // Logging sidecar
    let log_env = BTreeMap::new();
    log_env.insert("FLUENT_HOST".to_string(), "logserver".to_string());
    log_env.insert("FLUENT_PORT".to_string(), "24224".to_string());

    let log_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
        .set_arguments(&["/fluent-bit/bin/fluent-bit", "-c", "/etc/fluent-bit/config.yaml"])
        .set_environment_variables(log_env)
        .set_resource_limits(ResourceLimits::new()
            .with_cpu("100m")
            .with_memory("128Mi"));

    // Metrics sidecar
    let metrics_sidecar = SidecarContainer::new("prom/prometheus-node-exporter:latest", "metrics")
        .set_resource_limits(ResourceLimits::new()
            .with_cpu("50m")
            .with_memory("64Mi"));

    // Build step with sidecars
    let step = KubeJobStepBuilder::new()
        .with_name("web-app")
        .with_namespace("production")
        .add_container(Box::new(main_container))
        .add_sidecar(Box::new(log_sidecar))
        .add_sidecar(Box::new(metrics_sidecar))
        .build()?;

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

    // Create ingress
    let ingress = IngressBuilder::new()
        .with_name("web-app-ingress")
        .with_namespace("production")
        .with_host("web-app.example.com")
        .with_path("/", "web-app-service", 80)
        .with_tls_secret("tls-cert")
        .build()?;

    client.create_ingress(&ingress).await?;

    // Execute workflow
    let workflow = WorkflowBuilder::new()
        .with_name("web-app-deployment")
        .with_namespace("production")
        .add_step(step)
        .build()?;

    let execution = client.execute_workflow(&workflow).await?;
    println!("Web application deployed: {}", execution.id());

    Ok(())
}
```

---

## Storage and Volumes

### Scenario 4: Data Processing with Persistent Storage

**Your request:**
```
I need to process large CSV files. The workflow should:
1. Download data to a PVC
2. Process it with Python
3. Save results to the same PVC
4. Use 10Gi of storage
5. Use fast SSD storage class
```

**What the skill provides:**

**Implementation:**
```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, PVCVolume, MaestroPVCMountVolumeBuilder};
use k8s_maestro::entities::volume_types::AccessMode;
use k8s_maestro::steps::traits::ResourceLimits;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    // Create PVC
    let pvc = MaestroPVCMountVolumeBuilder::new()
        .with_name("data-pvc")
        .with_access_mode(AccessMode::ReadWriteOnce)
        .with_storage("10Gi")
        .with_storage_class("fast-ssd")
        .build()?;

    client.create_pvc(&pvc).await?;

    // Container with mounted PVC
    let container = MaestroContainer::new("python:3.11", "processor")
        .with_volume_mount("data-pvc", "/data")
        .set_arguments(&[
            "python".to_string(),
            "process.py".to_string(),
            "--input".to_string(),
            "/data/input.csv".to_string(),
            "--output".to_string(),
            "/data/output.csv".to_string(),
        ])
        .set_resource_limits(ResourceLimits::new()
            .with_cpu("2000m")
            .with_memory("4Gi"));

    // Create step
    let step = KubeJobStep::new("data-processor", "python:3.11", k8s_client);

    // Build and execute workflow
    let workflow = WorkflowBuilder::new()
        .with_name("data-processing")
        .add_step(step)
        .build()?;

    let execution = client.execute_workflow(&workflow).await?;
    println!("Processing started: {}", execution.id());

    Ok(())
}
```

### Scenario 5: Using ConfigMaps and Secrets

**Your request:**
```
I have a ConfigMap with application configuration and a Secret with database credentials.
I need to:
1. Mount the ConfigMap as a file
2. Reference the Secret as environment variables
3. Use them in my application
```

**What the skill provides:**

**Implementation:**
```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, ConfigMapVolume, ConfigMapVolumeBuilder, SecretVolumeBuilder};
use k8s_maestro::steps::traits::ResourceLimits;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    // Create ConfigMap
    let configmap = ConfigMapBuilder::new()
        .with_name("app-config")
        .add_data("config.yaml", "database:\n  host: postgres.example.com\n  port: 5432")
        .build()?;

    client.create_configmap(&configmap).await?;

    // Create Secret
    let secret = SecretBuilder::new()
        .with_name("db-credentials")
        .add_data("username", base64::encode("app_user"))
        .add_data("password", base64::encode("secure_password"))
        .build()?;

    client.create_secret(&secret).await?;

    // Create volumes
    let config_volume = ConfigMapVolumeBuilder::new()
        .with_name("config-volume")
        .with_configmap_name("app-config")
        .build()?;

    let secret_volume = SecretVolumeBuilder::new()
        .with_name("secret-volume")
        .with_secret_name("db-credentials")
        .build()?;

    // Container with volumes and environment variables
    let container = MaestroContainer::new("app:latest", "main")
        .with_volume_mount("config-volume", "/etc/config")
        .with_volume_mount("data-pvc", "/data")
        .with_secret_env_var("DB_USER", "db-credentials", "username")
        .with_secret_env_var("DB_PASSWORD", "db-credentials", "password");

    // Create and execute workflow
    let step = KubeJobStep::new("app-job", "app:latest", k8s_client);
    let workflow = WorkflowBuilder::new()
        .with_name("app-with-config")
        .add_step(step)
        .build()?;

    let execution = client.execute_workflow(&workflow).await?;

    Ok(())
}
```

---

## Networking

### Scenario 6: Microservice Communication

**Your request:**
```
I have two services: frontend and backend. The frontend needs to communicate with
the backend. How do I set this up?
```

**What the skill provides:**

**Implementation:**
```rust
use k8s_maestro::{ServiceBuilder, ServiceType};
use std::collections::BTreeMap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    // Backend service
    let mut backend_selector = BTreeMap::new();
    backend_selector.insert("app".to_string(), "backend".to_string());

    let backend_service = ServiceBuilder::new()
        .with_name("backend-service")
        .with_namespace("production")
        .with_port(5000, 5000, "TCP")
        .with_selector(backend_selector)
        .with_type(ServiceType::ClusterIP)
        .build()?;

    client.create_service(&backend_service).await?;

    // Frontend service
    let mut frontend_selector = BTreeMap::new();
    frontend_selector.insert("app".to_string(), "frontend".to_string());

    let frontend_service = ServiceBuilder::new()
        .with_name("frontend-service")
        .with_namespace("production")
        .with_port(80, 3000, "TCP")
        .with_selector(frontend_selector)
        .with_type(ServiceType::ClusterIP)
        .build()?;

    client.create_service(&frontend_service).await?;

    println!("Services created!");
    println!("Frontend can access backend at: backend-service.production.svc.cluster.local:5000");

    Ok(())
}
```

---

## Security

### Scenario 7: RBAC Setup

**Your request:**
```
I need to create a service account with limited permissions for my workflow.
The workflow should only be able to create and get Jobs, and get Pods.
```

**What the skill provides:**

**Implementation:**
```rust
use k8s_maestro::security::{
    ServiceAccountBuilder,
    RoleBuilder,
    RoleBindingBuilder,
    PolicyRule
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    // Create service account
    let sa = ServiceAccountBuilder::new()
        .with_name("workflow-sa")
        .with_namespace("production")
        .build()?;

    client.create_service_account(&sa).await?;

    // Create role with limited permissions
    let role = RoleBuilder::new()
        .with_name("workflow-role")
        .with_namespace("production")
        .add_rule(PolicyRule::new()
            .with_api_groups(vec!["batch".to_string()])
            .with_resources(vec!["jobs".to_string()])
            .with_verbs(vec!["create".to_string(), "get".to_string(), "list".to_string()]))
        .add_rule(PolicyRule::new()
            .with_api_groups(vec!["".to_string()])
            .with_resources(vec!["pods".to_string()])
            .with_verbs(vec!["get".to_string(), "list".to_string(), "logs".to_string()]))
        .build()?;

    client.create_role(&role).await?;

    // Create role binding
    let binding = RoleBindingBuilder::new()
        .with_name("workflow-binding")
        .with_namespace("production")
        .with_role("workflow-role")
        .with_service_account("workflow-sa")
        .build()?;

    client.create_role_binding(&binding).await?;

    println!("RBAC configured: Service account 'workflow-sa' with limited permissions");

    Ok(())
}
```

---

## Testing

### Scenario 8: Writing Integration Tests

**Your request:**
```
I need to write integration tests for my ETL workflow. The tests should:
1. Use a Kind cluster
2. Create test data
3. Execute the workflow
4. Verify results
5. Clean up resources
```

**What the skill provides:**

**Test implementation:**
```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::tests::common::kind_cluster::KindCluster;
use k8s_maestro::tests::common::utilities::{
    create_namespace, apply_resource, verify_resource_exists, delete_resource_by_name
};
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::clients::MaestroK8sClient;
use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::core::v1::ConfigMap;
use std::collections::BTreeMap;

#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_etl_workflow() -> anyhow::Result<()> {
    // Setup Kind cluster
    let cluster = KindCluster::new().await.expect("Failed to create cluster");
    let client = create_client_from_cluster(&cluster);

    // Create test namespace
    create_namespace(&client, "test-etl").await?;

    // Create test ConfigMap
    let configmap = ConfigMapBuilder::new()
        .with_name("test-config")
        .add_data("config.yaml", "test: true")
        .build()?;

    apply_resource(&client, &configmap, "test-etl").await?;

    // Build workflow
    let workflow = WorkflowBuilder::new()
        .with_name("test-etl")
        .with_namespace("test-etl")
        .add_step(KubeJobStep::new("test-job", "python:3.11", client.clone()))
        .build()?;

    // Execute workflow
    let execution = execute_workflow(&client, &workflow).await?;

    // Verify job created
    assert!(
        verify_resource_exists::<Job>(&client, "test-job", "test-etl").await,
        "Job should exist"
    );

    // Wait for completion
    execution.wait_for_completion().await?;

    // Verify success
    assert!(execution.is_success(), "Workflow should succeed");

    // Cleanup
    delete_resource_by_name::<Job>(&client, "test-job", "test-etl").await.ok();
    delete_resource_by_name::<ConfigMap>(&client, "test-config", "test-etl").await.ok();

    // Cleanup cluster
    cluster.cleanup().await?;

    Ok(())
}
```

---

## Deployment

### Scenario 9: Production Deployment with Rollback

**Your request:**
```
I want to deploy my workflow to production with:
1. Dry run validation first
2. Real deployment
3. Monitoring
4. Automatic rollback on failure
```

**What the skill provides:**

**Implementation:**
```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::workflows::scheduler::FailureStrategy;
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;

    // Step 1: Dry run validation
    println!("Validating workflow with dry run...");
    let dry_run_client = MaestroClientBuilder::new()
        .with_namespace("production")
        .with_dry_run(true)
        .build()?;

    let workflow = build_production_workflow()?;
    let dry_run_result = dry_run_client.create_workflow(workflow.clone())?;
    assert!(dry_run_result.is_dry_run());
    println!("Dry run validation: PASSED");

    // Step 2: Deploy to production
    println!("Deploying to production...");
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .with_dry_run(false)
        .build()?;

    let execution = client.execute_workflow(&workflow).await?;
    println!("Workflow deployed: {}", execution.id());

    // Step 3: Monitor execution
    println!("Monitoring execution...");
    let mut event_stream = client.watch_workflow_events(execution.id()).await?;

    while let Some(event) = event_stream.next().await {
        match event {
            WorkflowEvent::StepStarted { step_id } => {
                println!("  Step started: {}", step_id);
            }
            WorkflowEvent::StepCompleted { step_id, result } => {
                println!("  Step completed: {} - {:?}", step_id, result.status());
            }
            WorkflowEvent::WorkflowFailed { error } => {
                eprintln!("Workflow failed: {}", error);
                // Step 4: Rollback on failure
                println!("Initiating rollback...");
                rollback_to_previous_version(&client).await?;
                return Err(error.into());
            }
            WorkflowEvent::WorkflowCompleted { .. } => {
                println!("Workflow completed successfully!");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn rollback_to_previous_version(client: &MaestroClient) -> anyhow::Result<()> {
    // Get previous version
    let previous = client.get_workflow("my-workflow-v1").await?
        .expect("Previous version not found");

    // Delete current version
    client.delete_workflow("my-workflow-v2").await?;

    // Deploy previous version
    let execution = client.execute_workflow(&previous).await?;
    println!("Rolled back to: {}", execution.id());

    Ok(())
}
```

---

## Advanced Patterns

### Scenario 10: Conditional Workflow Execution

**Your request:**
```
I need a workflow with conditional execution:
1. Validate data
2. Only if validation passes, transform data
3. Only if transformation succeeds, load to database
4. If validation fails, skip to cleanup
```

**What the skill provides:**

**Implementation:**
```rust
use k8s_maestro::workflows::DependencyChain;

fn build_conditional_workflow() -> anyhow::Result<()> {
    let mut chain = DependencyChain::new();

    // Add steps
    chain.add_step("validate-data");
    chain.add_step("transform-data");
    chain.add_step("load-data");
    chain.add_step("cleanup");

    // Add conditional dependencies
    chain.add_step("transform-data")
        .with_conditional_dependency("validate-data", |deps| {
            deps.iter().all(|r| r.is_success())
        });

    chain.add_step("load-data")
        .with_conditional_dependency("transform-data", |deps| {
            deps.iter().all(|r| r.is_success())
        });

    chain.add_step("cleanup")
        .with_conditional_dependency_any(
            vec!["validate-data", "transform-data", "load-data"],
            |deps| {
                // Cleanup runs if any step completes (success or failure)
                deps.iter().all(|r| !r.is_running())
            }
        );

    // Build DAG
    let graph = chain.build_dag()?;
    let levels = graph.topological_sort()?;

    println!("Execution plan:");
    for (i, level) in levels.iter().enumerate() {
        println!("  Level {}: {:?}", i + 1, level);
    }

    Ok(())
}
```

---

## Troubleshooting

### Common Issues and Solutions

**Issue 1: Cluster not reachable**
```
Problem: kubectl cluster-info fails
Solution:
  1. Check kubeconfig: kubectl config view
  2. Verify context: kubectl config current-context
  3. Test connectivity: kubectl get nodes
  4. Run analyze_cluster.sh script for diagnostics
```

**Issue 2: PVC stuck in Pending state**
```
Problem: PVC never binds
Solution:
  1. Check storage class: kubectl get storageclass
  2. Verify storage class exists: kubectl describe pvc <pvc-name>
  3. Check node capacity: kubectl describe nodes
  4. Ensure storage class has provisioner: kubectl describe sc <storage-class>
```

**Issue 3: Workflow fails with "ContainerCreating" timeout**
```
Problem: Pods stuck in ContainerCreating
Solution:
  1. Check pod events: kubectl describe pod <pod-name>
  2. Verify image pull secret: kubectl get secrets
  3. Check image repository access: docker pull <image-name>
  4. Verify resource limits: kubectl describe pod <pod-name>
```

**Issue 4: Integration test fails randomly**
```
Problem: Flaky integration tests
Solution:
  1. Add proper waits: wait_for_resource_ready
  2. Use unique names: format!("test-{}", uuid::Uuid::new_v4())
  3. Add retries: tokio::time::timeout(Duration::from_secs(30), operation)
  4. Verify cleanup: ensure resources deleted before next test
```

---

## Best Practices

1. **Always test locally first** - Use Kind for development
2. **Validate with dry run** - Never deploy without validation
3. **Use appropriate resource limits** - Set CPU/memory based on workload
4. **Implement proper error handling** - Use anyhow::Result throughout
5. **Add monitoring** - Use sidecars for logging and metrics
6. **Write tests** - Unit, integration, and E2E tests
7. **Document workflows** - Use labels and annotations
8. **Clean up resources** - Delete completed jobs and old resources
9. **Use secrets for sensitive data** - Never hardcode credentials
10. **Plan rollbacks** - Know how to revert changes

---

## Getting Help

When you need help:

1. **Use the skill directly**: Ask questions about k8s-maestro integration
2. **Run diagnostic scripts**: analyze_cluster.sh, detect_crate_features.py
3. **Check documentation**: See references/ directory for detailed guides
4. **Look at examples**: Check the examples/ directory in k8s-maestro
5. **Review test cases**: See evals/evals.json for test scenarios

---

## Next Steps

- Explore [Builder Patterns](references/builder_patterns.md) for detailed API reference
- Check [Integration Patterns](references/integration_patterns.md) for integration strategies
- Review [Testing Patterns](references/testing_patterns.md) for testing best practices
- Study [Deployment Patterns](references/deployment_patterns.md) for deployment guides

Happy workflow orchestration! 🚀
