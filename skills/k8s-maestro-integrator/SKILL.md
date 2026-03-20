---
name: k8s-maestro-integrator
description: Comprehensive integration assistant for k8s-maestro - Kubernetes workflow orchestrator. Use this skill whenever the user wants to integrate k8s-maestro into their Rust project, create Kubernetes workflows, set up Jobs/Pods/Services, handle dependencies, implement testing with Kind, deploy to production or test clusters, work with storage and networking resources, or needs guidance on resource types (Job vs Pod), integration patterns (API vs channels), crate features activation, environment variable handling, security configurations, or any Kubernetes workflow orchestration task.
---

# k8s-maestro Integration Guide

This skill helps you integrate k8s-maestro into your Rust projects and build production-ready Kubernetes workflows.

## Core Concepts

### When to Use Different Resource Types

**Use Kubernetes Jobs when:**
- One-time or batch processing tasks
- ETL pipelines that run to completion
- Data processing jobs with defined start and end
- Scheduled tasks (cron jobs)
- Parallel processing of independent tasks

**Use Kubernetes Pods when:**
- Long-running services (though consider Deployment/StatefulSet for production)
- Debugging or ad-hoc tasks
- Multi-container workloads that need to run indefinitely
- When you need finer control over pod lifecycle

**Use Workflows when:**
- Complex multi-step pipelines with dependencies
- Conditional execution based on step results
- Need for orchestration, scheduling, and monitoring
- Pipeline with parallel execution requirements
- Workflow state management and checkpointing

### Integration Patterns: API vs Channels

**Use API Integration when:**
- Consuming workflow results after completion
- Need to query job status post-execution
- Integrating with external monitoring systems
- Batch processing where results are retrieved later
- Need programmatic access to workflow metadata

**Use Channel Integration when:**
- Real-time event streaming from workflows
- Live monitoring of workflow progress
- Immediate response to workflow state changes
- WebSocket-based monitoring dashboards
- Event-driven architectures needing instant notifications

## Project Setup

### Cargo.toml Configuration

When setting up k8s-maestro in a new or existing project:

1. **Determine the appropriate Kubernetes version feature:**
   - Check your cluster version with `kubectl version`
   - Select the matching feature: `k8s_v1_28`, `k8s_v1_29`, `k8s_v1_30`, `k8s_v1_31`, or `k8s_v1_32`
   - Use the latest supported version that matches your cluster

2. **Add dependency with appropriate features:**

```toml
[dependencies]
k8s-maestro = { version = "1.0", features = ["k8s_v1_31"] }
# For exec steps (Python/Rust scripts)
k8s-maestro = { version = "1.0", features = ["k8s_v1_31", "exec-steps"] }
```

**Feature Decision Guide:**
- Always include a `k8s_v1_*` feature matching your cluster version
- Add `exec-steps` if you need to run Python/Rust scripts as workflow steps
- Default is `k8s_v1_28` with `exec-steps` enabled

3. **Ensure tokio runtime is configured:**

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Your code here
    Ok(())
}
```

## Building Workflows

### Basic Workflow Pattern

Always follow the builder pattern for workflows:

```rust
use k8s_maestro::{WorkflowBuilder, MaestroClientBuilder};
use k8s_maestro::steps::KubeJobStep;
use k8s_maestro::clients::MaestroK8sClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // 1. Create Kubernetes client
    let k8s_client = MaestroK8sClient::new().await?;

    // 2. Build Maestro client
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .with_client(k8s_client)
        .build()?;

    // 3. Build workflow with fluent API
    let workflow = WorkflowBuilder::new()
        .with_name("my-workflow")
        .add_step(KubeJobStep::new("data-fetch", "python:3.11", client.get_client()?))
        .build()?;

    // 4. Execute workflow
    let execution = client.execute_workflow(&workflow).await?;

    Ok(())
}
```

### Container Configuration

Use MaestroContainer builder for all container configurations:

```rust
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use std::collections::BTreeMap;

// Basic container
let container = MaestroContainer::new("nginx:latest", "web-server");

// With environment variables
let mut env_vars = BTreeMap::new();
env_vars.insert("DATABASE_URL".to_string(), "postgres://localhost:5432/db".to_string());
env_vars.insert("LOG_LEVEL".to_string(), "info".to_string());

let container = MaestroContainer::new("app:latest", "main")
    .set_environment_variables(env_vars);

// With arguments
let container = MaestroContainer::new("python:3.11", "processor")
    .set_arguments(&[
        "python".to_string(),
        "-m".to_string(),
        "pipeline.process".to_string(),
        "--input".to_string(),
        "/data/input.csv".to_string(),
    ]);

// With resource limits
let limits = ResourceLimits::new()
    .with_cpu("500m")
    .with_memory("512Mi");

let container = MaestroContainer::new("app:latest", "main")
    .set_resource_limits(limits);
```

### Parameter Passing Strategies

Choose the right parameter passing method based on use case:

**Environment Variables (from literals):**
```rust
let mut env = BTreeMap::new();
env.insert("MAX_RETRIES".to_string(), "3".to_string());
env.insert("TIMEOUT".to_string(), "30".to_string());
let container = MaestroContainer::new("app:latest", "main")
    .set_environment_variables(env);
```

**Environment Variables (from Secrets):**
```rust
use k8s_maestro::entities::SecretBuilder;

// Create secret first
let secret = SecretBuilder::new()
    .with_name("db-credentials")
    .with_namespace("default")
    .add_data("username", base64::encode("admin"))
    .add_data("password", base64::encode("secret"))
    .build()?;

// Reference in container (use secretRef in env)
// Note: This requires creating the secret in K8s first
```

**Command Line Arguments:**
```rust
let container = MaestroContainer::new("python:3.11", "script")
    .set_arguments(&[
        "python".to_string(),
        "script.py".to_string(),
        "--config".to_string(),
        "/etc/config/app.yaml".to_string(),
        "--verbose".to_string(),
    ]);
```

**ConfigMap Mounted as File:**
```rust
use k8s_maestro::entities::{ConfigMapVolume, ConfigMapVolumeBuilder};

let config_volume = ConfigMapVolumeBuilder::new()
    .with_name("app-config")
    .with_configmap_name("app-config")
    .build()?;

// Mount volume to container
let mut container = MaestroContainer::new("app:latest", "main");
container.volume_mounts.push(VolumeMount {
    name: "app-config".to_string(),
    mount_path: "/etc/config".to_string(),
    ..Default::default()
});
```

### Dependency Chains

Use DependencyChain for complex workflow orchestration:

```rust
use k8s_maestro::workflows::DependencyChain;

let mut chain = DependencyChain::new();

// Add steps in order
chain.add_step("extract");
chain.add_step("transform").with_dependency("extract");
chain.add_step("load").with_dependency("transform");

// For parallel execution
chain.add_step("validate-a");
chain.add_step("validate-b");
chain.add_step("merge").with_dependency_any(vec!["validate-a", "validate-b"]);

// Conditional execution
chain.add_step("process-data")
    .with_conditional_dependency("validate", |deps| {
        deps.iter().all(|r| r.is_success())
    });

// Build DAG
let graph = chain.build_dag()?;
let execution_levels = graph.topological_sort()?;
```

### Resource Limits

Apply resource limits at container or workflow level:

```rust
use k8s_maestro::steps::traits::ResourceLimits;

// Container-level limits
let limits = ResourceLimits::new()
    .with_cpu("500m")
    .with_memory("512Mi")
    .with_storage("1Gi");

let container = MaestroContainer::new("app:latest", "main")
    .set_resource_limits(limits);

// Workflow-level limits (applies to all steps)
let workflow_limits = ResourceLimits::new()
    .with_cpu("2000m")
    .with_memory("2Gi");

let workflow = WorkflowBuilder::new()
    .with_name("my-workflow")
    .with_resource_limits(workflow_limits)
    .add_step(step)
    .build()?;
```

## Storage and Volumes

### Persistent Volume Claims

```rust
use k8s_maestro::entities::{PVCVolume, MaestroPVCMountVolumeBuilder};

let pvc = MaestroPVCMountVolumeBuilder::new()
    .with_name("data-pvc")
    .with_access_mode(AccessMode::ReadWriteOnce)
    .with_storage("10Gi")
    .build()?;

// Mount to container
let container = MaestroContainer::new("app:latest", "main")
    .with_volume_mount("data-pvc", "/data");
```

### ConfigMap and Secret Volumes

```rust
use k8s_maestro::entities::{ConfigMapVolume, SecretVolume};

// ConfigMap volume
let config_volume = ConfigMapVolumeBuilder::new()
    .with_name("config-volume")
    .with_configmap_name("app-config")
    .build()?;

// Secret volume
let secret_volume = SecretVolumeBuilder::new()
    .with_name("secret-volume")
    .with_secret_name("api-keys")
    .build()?;
```

## Networking

### Services

```rust
use k8s_maestro::{ServiceBuilder, ServiceType};
use std::collections::BTreeMap;

let mut selector = BTreeMap::new();
selector.insert("app".to_string(), "my-app".to_string());

let service = ServiceBuilder::new()
    .with_name("my-service")
    .with_namespace("default")
    .with_port(80, 8080, "TCP")
    .with_selector(selector)
    .with_type(ServiceType::ClusterIP)
    .build()?;
```

### Ingress

```rust
use k8s_maestro::IngressBuilder;

let ingress = IngressBuilder::new()
    .with_name("my-ingress")
    .with_namespace("default")
    .with_host("example.com")
    .with_path("/", "my-service", 80)
    .with_tls_secret("tls-cert")
    .build()?;
```

### DNS Names

```rust
use k8s_maestro::{service_dns_name, pod_dns_name};

let service_dns = service_dns_name("my-service", "default");
// Returns: "my-service.default.svc.cluster.local"

let pod_dns = pod_dns_name("my-pod-abc123", "default");
// Returns: "my-pod-abc123.default.pod.cluster.local"
```

## Sidecars

Add sidecars for logging, monitoring, or proxies:

```rust
use k8s_maestro::entities::SidecarContainer;

// Logging sidecar
let log_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
    .set_environment_variables(env_vars);

// Monitoring sidecar
let metrics_sidecar = SidecarContainer::new("prom/prometheus-node-exporter:latest", "metrics");

// Add to step
let step = KubeJobStepBuilder::new()
    .with_name("my-job")
    .with_namespace("default")
    .add_container(Box::new(main_container))
    .add_sidecar(Box::new(log_sidecar))
    .build()?;
```

## Security

### RBAC

```rust
use k8s_maestro::security::{
    RoleBuilder, RoleBindingBuilder, ServiceAccountBuilder, PolicyRule
};

let service_account = ServiceAccountBuilder::new()
    .with_name("workflow-sa")
    .with_namespace("default")
    .build()?;

let role = RoleBuilder::new()
    .with_name("workflow-role")
    .with_namespace("default")
    .add_rule(PolicyRule::new()
        .with_api_groups(vec!["batch".to_string(), "".to_string()])
        .with_resources(vec!["jobs".to_string(), "pods".to_string()])
        .with_verbs(vec!["*".to_string()]))
    .build()?;

let role_binding = RoleBindingBuilder::new()
    .with_name("workflow-binding")
    .with_namespace("default")
    .with_role("workflow-role")
    .with_service_account("workflow-sa")
    .build()?;
```

### Network Policies

```rust
use k8s_maestro::security::{NetworkPolicyBuilder, NetworkPolicyRule, PolicyType};

let policy = NetworkPolicyBuilder::new()
    .with_name("allow-ingress")
    .with_namespace("default")
    .with_policy_types(vec![PolicyType::Ingress])
    .add_selector("app", "my-app")
    .add_rule(NetworkPolicyRule::new()
        .with_port(8080, "TCP")
        .with_from_pod_selector("app", "frontend"))
    .build()?;
```

## Testing

### Unit Tests with Mocking

```rust
use k8s_maestro::tests::common::mocking::{MockK8sClient, mock_error};

#[test]
fn test_create_configmap_success() {
    let mut mock = MockK8sClient::new()
        .add_create_response(Ok(serde_json::json!({
            "kind": "ConfigMap",
            "metadata": {"name": "test-cm"}
        })));

    let response = mock.next_create_response();
    assert!(response.is_ok());
}
```

### Integration Tests with Kind

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;
use k8s_maestro::tests::common::utilities::{
    create_namespace, apply_resource, verify_resource_exists
};

#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_workflow_execution() {
    let cluster = KindCluster::new().await.expect("Failed to create cluster");
    let client = create_client_from_cluster(&cluster);

    // Create namespace
    create_namespace(&client, "test-ns").await?;

    // Create and apply workflow
    let workflow = build_test_workflow()?;
    apply_resource(&client, &workflow, "test-ns").await?;

    // Verify execution
    verify_resource_exists::<Job>(&client, "my-job", "test-ns").await?;

    // Cleanup
    cluster.cleanup().await?;
}
```

### E2E Tests

```rust
#[tokio::test]
#[ignore = "Requires Docker"]
async fn e2e_complete_workflow() {
    let (client, cluster) = setup_e2e_test().await?;

    // Create resources
    let configmap = ConfigMapBuilder::new()
        .with_name("test-config")
        .build()?;

    apply_resource(&client, &configmap, "default").await?;

    // Execute workflow
    let workflow = WorkflowBuilder::new()
        .with_name("e2e-workflow")
        .add_step(KubeJobStep::new("test-job", "nginx:latest", client))
        .build()?;

    let execution = execute_workflow(&client, &workflow).await?;

    // Verify results
    assert!(execution.is_success());

    cluster.cleanup().await?;
}
```

## Deployment

### Dry Run Mode

Always test with dry run first:

```rust
let client = MaestroClientBuilder::new()
    .with_dry_run(true)
    .build()?;

let created = client.create_workflow(workflow)?;
assert!(created.is_dry_run());
```

### Production Deployment

```rust
let client = MaestroClientBuilder::new()
    .with_namespace("production")
    .with_dry_run(false)
    .build()?;

let execution = client.execute_workflow(&workflow).await?;

// Monitor execution
while !execution.is_complete() {
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    let status = execution.get_status().await?;
    println!("Status: {:?}", status);
}

// Clean up resources
execution.cleanup().await?;
```

## Monitoring and Debugging

### Streaming Logs

```rust
use k8s_maestro::steps::traits::LoggableWorkFlowStep;

let mut log_stream = step.stream_logs(LogStreamOptions::default()).await?;

while let Some(log_line) = log_stream.next().await {
    println!("{}", log_line?);
}
```

### Waiting for Completion

```rust
use k8s_maestro::steps::traits::WaitableWorkFlowStep;

let result = step.wait().await?;

match result.status() {
    StepStatus::Success => println!("Step completed successfully"),
    StepStatus::Failure => println!("Step failed: {:?}", result),
    _ => println!("Step status: {:?}", result),
}
```

## Code Style Guidelines

Follow these conventions from AGENTS.md:

### Imports
```rust
// Group 1: std imports
use std::collections::BTreeMap;

// Group 2: External crates
use k8s_openapi::{api::batch::v1::Job, apimachinery::pkg::api::resource::Quantity};
use anyhow::Result;

// Group 3: Local crate imports
use k8s_maestro::{clients::MaestroK8sClient, entities::container::MaestroContainer};
```

### Formatting
- 4 spaces indentation
- 100 chars max line length
- No trailing whitespace

### Naming
- Types: CamelCase
- Functions/variables: snake_case
- Constants: SCREAMING_SNAKE_CASE

### Error Handling
```rust
// Use anyhow::Result for application-level errors
pub async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    Ok(())
}

// Use thiserror for library errors
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),
}
```

## Common Patterns

### ETL Pipeline

```rust
let workflow = WorkflowBuilder::new()
    .with_name("etl-pipeline")
    .add_step(KubeJobStep::new("extract", "python:3.11", client))
    .add_step(KubeJobStep::new("transform", "python:3.11", client))
    .add_step(KubeJobStep::new("load", "postgres:16", client))
    .with_parallelism(2)
    .build()?;

let mut chain = DependencyChain::new();
chain.add_step("extract");
chain.add_step("transform").with_dependency("extract");
chain.add_step("load").with_dependency("transform");
```

### Web Application with Monitoring

```rust
let step = KubeJobStepBuilder::new()
    .with_name("web-app")
    .with_namespace("production")
    .add_container(Box::new(MaestroContainer::new("app:latest", "main")))
    .add_sidecar(Box::new(SidecarContainer::new("fluent/fluent-bit:2.2", "logs")))
    .add_sidecar(Box::new(SidecarContainer::new("prometheus-node-exporter", "metrics")))
    .build()?;
```

### Batch Processing with Checkpointing

```rust
let checkpoint_config = CheckpointConfig::new()
    .enabled(true)
    .with_interval_secs(60)
    .with_storage_path("/checkpoint");

let workflow = WorkflowBuilder::new()
    .with_name("batch-processor")
    .with_checkpointing(checkpoint_config)
    .add_step(step)
    .build()?;
```

## Checklists

### Before Writing Code
- [ ] Determine Kubernetes version (check with `kubectl version`)
- [ ] Decide on resource type: Job, Pod, or Workflow
- [ ] Choose integration method: API or channels
- [ ] Identify required crate features (k8s_v1_*, exec-steps)
- [ ] Determine storage needs (PVC, ConfigMap, Secret)
- [ ] Plan networking (Service, Ingress)
- [ ] Set resource limits appropriately

### After Writing Code
- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy`
- [ ] Run `cargo test --lib` (unit tests)
- [ ] Run integration tests with Kind: `cargo test --test '*' -- --ignored`
- [ ] Test with dry run mode first
- [ ] Verify error handling
- [ ] Check resource cleanup

### Before Deployment
- [ ] Test in Kind cluster
- [ ] Verify secrets are properly configured
- [ ] Check namespace isolation
- [ ] Validate RBAC permissions
- [ ] Confirm resource limits match cluster capacity
- [ ] Set up monitoring and logging
- [ ] Prepare rollback plan

## Getting Help

When encountering issues:

1. Check cluster connectivity: `kubectl cluster-info`
2. Verify kubeconfig: `kubectl config view`
3. Check resource status: `kubectl get jobs,pods,services`
4. View logs: `kubectl logs <pod-name>`
5. Describe resources: `kubectl describe <resource-type> <name>`

## References

See additional documentation in:
- **[README.md](README.md)** - Installation instructions and quick start
- **[usage_guide.md](usage_guide.md)** - Comprehensive usage guide with real-world examples and tutorials
- `references/builder_patterns.md` - Detailed builder pattern guide
- `references/integration_patterns.md` - Integration strategies
- `references/testing_patterns.md` - Testing best practices
- `references/deployment_patterns.md` - Deployment strategies

Run bundled scripts for automation:
- `scripts/analyze_cluster.sh` - Cluster analysis and recommendations
- `scripts/detect_crate_features.py` - Detect required crate features
- `scripts/generate_workflow_code.py` - Generate workflow boilerplate
- `scripts/generate_test_code.py` - Generate test templates

## Quick Links

- **Start here**: [usage_guide.md](usage_guide.md) - Comprehensive examples and tutorials
- **Installation**: [README.md](README.md) - How to install and get started
- **API Reference**: See references/ directory for detailed builder patterns
- **Scripts**: See scripts/ directory for automation tools
