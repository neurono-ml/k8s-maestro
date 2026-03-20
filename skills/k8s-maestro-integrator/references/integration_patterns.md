# Integration Patterns Reference

This reference provides detailed guidance on integrating k8s-maestro into applications, choosing the right integration method, and implementing various integration patterns.

## Integration Method Decision Guide

### API Integration Pattern

**Use when:**
- Querying workflow status after completion
- Retrieving job results post-execution
- Integrating with external monitoring systems
- Batch processing with delayed result retrieval
- Programmatic access to workflow metadata
- Building dashboards that show historical data

**Implementation:**

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
use k8s_maestro::clients::MaestroK8sClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Create client
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    // Submit workflow
    let workflow = build_workflow()?;
    let execution = client.execute_workflow(&workflow).await?;

    // Get workflow ID for later queries
    let workflow_id = execution.id();

    // Later, query workflow status
    let status = client.get_workflow_status(workflow_id).await?;
    println!("Workflow status: {:?}", status);

    // Retrieve results
    if let Some(results) = client.get_workflow_results(workflow_id).await? {
        println!("Results: {:?}", results);
    }

    Ok(())
}
```

### Channel Integration Pattern

**Use when:**
- Real-time event streaming from workflows
- Live monitoring of workflow progress
- Immediate response to workflow state changes
- WebSocket-based monitoring dashboards
- Event-driven architectures needing instant notifications
- Building reactive UIs that update in real-time

**Implementation:**

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()?;

    let workflow = build_workflow()?;
    let execution = client.execute_workflow(&workflow).await?;

    // Subscribe to real-time events
    let mut event_stream = client.watch_workflow_events(execution.id()).await?;

    while let Some(event) = event_stream.next().await {
        match event {
            WorkflowEvent::StepStarted { step_id, .. } => {
                println!("Step started: {}", step_id);
            }
            WorkflowEvent::StepCompleted { step_id, result } => {
                println!("Step completed: {} - {:?}", step_id, result);
            }
            WorkflowEvent::WorkflowCompleted { .. } => {
                println!("Workflow completed!");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
```

## Resource Type Selection Guide

### Job vs Pod vs Deployment

| Resource Type | Use Case | Lifecycle | Restart Policy | Typical Use |
|--------------|----------|------------|----------------|-------------|
| **Job** | Batch tasks, ETL, one-off processing | Runs to completion | Never/OnFailure | Data processing, backups, migrations |
| **Pod** | Ad-hoc tasks, debugging | Manual management | Always/Never | Debug pods, temporary workloads |
| **Deployment** | Long-running services | Managed by controller | Always | Web servers, APIs, microservices |

### Choosing Job When:

```rust
// Batch data processing
let job = KubeJobStep::new("batch-process", "python:3.11", client);

// ETL pipeline
let job = KubeJobStep::new("etl-job", "etl-image:latest", client);

// Scheduled task
let job = KubeJobStep::new("scheduled-backup", "backup:latest", client);

// Parallel processing
let job = KubeJobStepBuilder::new()
    .with_name("parallel-job")
    .with_parallelism(10)
    .with_completions(100)
    .build()?;
```

### Choosing Pod When:

```rust
// Debug pod
let pod = KubePodStep::new("debug-pod", "debug-tools:latest", client);

// Temporary service
let pod = KubePodStep::new("temp-service", "service:latest", client);

// Manual control over lifecycle
let pod = KubePodStepBuilder::new()
    .with_name("manual-pod")
    .with_restart_policy(RestartPolicy::Never)
    .build()?;
```

### Choosing Workflow When:

```rust
// Multi-step pipeline
let workflow = WorkflowBuilder::new()
    .add_step(extract_step)
    .add_step(transform_step)
    .add_step(load_step)
    .build()?;

// Complex dependencies
let workflow = WorkflowBuilder::new()
    .with_parallelism(3)
    .add_step(step1)
    .add_step(step2)
    .add_step(step3)
    .build()?;

// Conditional execution
let workflow = WorkflowBuilder::new()
    .add_step(validation_step)
    .add_step(processing_step)
    .build()?;
```

## Crate Features Guide

### Required Features

Always include exactly one Kubernetes version feature:

```toml
# For Kubernetes 1.28
k8s-maestro = { version = "1.0", features = ["k8s_v1_28"] }

# For Kubernetes 1.29
k8s-maestro = { version = "1.0", features = ["k8s_v1_29"] }

# For Kubernetes 1.30
k8s-maestro = { version = "1.0", features = ["k8s_v1_30"] }

# For Kubernetes 1.31
k8s-maestro = { version = "1.0", features = ["k8s_v1_31"] }

# For Kubernetes 1.32
k8s-maestro = { version = "1.0", features = ["k8s_v1_32"] }
```

### Optional Features

Add `exec-steps` when you need to run Python/Rust scripts:

```toml
k8s-maestro = { version = "1.0", features = ["k8s_v1_31", "exec-steps"] }
```

### Feature Detection

Use the bundled script to detect required features:

```bash
python3 /path/to/skill/scripts/detect_crate_features.py
```

## Environment Variable Handling Patterns

### Pattern 1: Literal Values

```rust
let mut env_vars = BTreeMap::new();
env_vars.insert("PORT".to_string(), "8080".to_string());
env_vars.insert("LOG_LEVEL".to_string(), "info".to_string());
env_vars.insert("MAX_CONNECTIONS".to_string(), "100".to_string());

let container = MaestroContainer::new("app:latest", "main")
    .set_environment_variables(env_vars);
```

**When to use:**
- Non-sensitive configuration values
- Default values that can be overridden
- Feature flags and settings
- Runtime configuration

### Pattern 2: Secret References

```rust
// Step 1: Create secret
let secret = SecretBuilder::new()
    .with_name("db-credentials")
    .add_data("DB_HOST", base64::encode("postgres.example.com"))
    .add_data("DB_USER", base64::encode("app_user"))
    .add_data("DB_PASSWORD", base64::encode("secure_password"))
    .build()?;

// Step 2: Apply secret to cluster
client.create_secret(&secret).await?;

// Step 3: Reference in container
let container = MaestroContainer::new("app:latest", "main")
    .with_secret_env_var("DB_HOST", "db-credentials", "DB_HOST")
    .with_secret_env_var("DB_USER", "db-credentials", "DB_USER")
    .with_secret_env_var("DB_PASSWORD", "db-credentials", "DB_PASSWORD");
```

**When to use:**
- Sensitive data (passwords, API keys, tokens)
- Data that should not be in code or config
- Credentials that may need rotation
- Compliance requirements

### Pattern 3: ConfigMap References

```rust
// Step 1: Create configmap
let configmap = ConfigMapBuilder::new()
    .with_name("app-config")
    .add_data("database.yaml", "host: postgres.example.com\nport: 5432")
    .add_data("cache.yaml", "host: redis.example.com\nport: 6379")
    .build()?;

// Step 2: Apply configmap
client.create_configmap(&configmap).await?;

// Step 3: Reference as file
let config_volume = ConfigMapVolumeBuilder::new()
    .with_name("config-volume")
    .with_configmap_name("app-config")
    .build()?;

let container = MaestroContainer::new("app:latest", "main")
    .with_volume_mount("config-volume", "/etc/config");
```

**When to use:**
- Configuration files (YAML, JSON, XML)
- Multiple related config values
- Config that needs to be mounted as files
- Shared configuration across containers

### Pattern 4: Field References

```rust
let container = MaestroContainer::new("app:latest", "main")
    .with_field_ref_env_var("MY_POD_IP", "status.podIP")
    .with_field_ref_env_var("MY_NODE_NAME", "spec.nodeName")
    .with_field_ref_env_var("MY_NAMESPACE", "metadata.namespace");
```

**When to use:**
- Pod metadata (name, namespace, labels)
- Pod status (IP, phase)
- Node information
- Dynamic Kubernetes values

### Pattern 5: ConfigMap as Environment Variables

```rust
let configmap = ConfigMapBuilder::new()
    .with_name("env-config")
    .add_data("API_TIMEOUT", "30")
    .add_data("MAX_RETRIES", "3")
    .build()?;

let container = MaestroContainer::new("app:latest", "main")
    .with_configmap_env_var("API_TIMEOUT", "env-config", "API_TIMEOUT")
    .with_configmap_env_var("MAX_RETRIES", "env-config", "MAX_RETRIES");
```

**When to use:**
- Config that should be environment variables
- When you need both files and env vars from same configmap
- Non-sensitive but structured configuration

## Storage Integration Patterns

### Pattern 1: Persistent Volume Claim

```rust
// Create PVC
let pvc = MaestroPVCMountVolumeBuilder::new()
    .with_name("data-pvc")
    .with_access_mode(AccessMode::ReadWriteOnce)
    .with_storage("10Gi")
    .with_storage_class("fast-ssd")
    .build()?;

// Apply to cluster
client.create_pvc(&pvc).await?;

// Mount to container
let container = MaestroContainer::new("app:latest", "main")
    .with_volume_mount("data-pvc", "/data");
```

**When to use:**
- Persistent data that survives pod restarts
- Databases, file storage, application data
- When data needs to persist beyond job completion
- Production workloads

### Pattern 2: EmptyDir for Cache

```rust
let cache_volume = EmptyDirVolumeBuilder::new()
    .with_name("cache")
    .with_medium(Medium::Memory)
    .with_size_limit("1Gi")
    .build()?;

let container = MaestroContainer::new("app:latest", "main")
    .with_volume_mount("cache", "/cache");
```

**When to use:**
- Temporary cache data
- Shared storage between containers
- Scratch space that doesn't need persistence
- Fast memory-backed storage

### Pattern 3: HostPath for Debug

```rust
let host_volume = HostPathVolumeBuilder::new()
    .with_name("host-logs")
    .with_path("/var/log")
    .with_type(HostPathType::DirectoryOrCreate)
    .build()?;

let container = MaestroContainer::new("debug:latest", "main")
    .with_volume_mount("host-logs", "/host/logs");
```

**When to use:**
- Debug pods accessing host filesystem
- Monitoring host resources
- Troubleshooting
- Not recommended for production

## Networking Integration Patterns

### Pattern 1: ClusterIP Service

```rust
let service = ServiceBuilder::new()
    .with_name("internal-service")
    .with_namespace("default")
    .with_port(80, 8080, "TCP")
    .with_selector(selector)
    .with_type(ServiceType::ClusterIP)
    .build()?;

// Access via DNS
let dns_name = service_dns_name("internal-service", "default");
// "internal-service.default.svc.cluster.local"
```

**When to use:**
- Internal cluster communication
- Microservice communication
- Default service type
- No external access needed

### Pattern 2: LoadBalancer Service

```rust
let service = ServiceBuilder::new()
    .with_name("external-service")
    .with_port(443, 8443, "TCP")
    .with_selector(selector)
    .with_type(ServiceType::LoadBalancer)
    .build()?;
```

**When to use:**
- External access via cloud load balancer
- Public-facing services
- When using managed cloud providers
- Production workloads

### Pattern 3: Ingress with TLS

```rust
let ingress = IngressBuilder::new()
    .with_name("secure-ingress")
    .with_host("secure.example.com")
    .with_path("/", "backend-service", 80)
    .with_tls_secret("tls-cert")
    .with_path_type(PathType::Prefix)
    .build()?;
```

**When to use:**
- HTTP/HTTPS routing to multiple services
- Host-based routing
- TLS termination
- Single entry point for multiple services

### Pattern 4: Headless Service for StatefulSets

```rust
let service = ServiceBuilder::new()
    .with_name("stateful-headless")
    .with_selector(selector)
    .with_type(ServiceType::ClusterIP)
    .build()?;

// Then set clusterIP to None to make it headless
```

**When to use:**
- StatefulSets with stable network identities
- Direct pod-to-pod communication
- When you need pod DNS records
- Custom service discovery

## Monitoring and Logging Integration

### Pattern 1: Sidecar Logging

```rust
let main_container = MaestroContainer::new("app:latest", "main");

let log_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
    .with_volume_mount("logs", "/var/log")
    .set_arguments(&["/fluent-bit/bin/fluent-bit", "-c", "/etc/fluent-bit/config.yaml"]);

let step = KubeJobStepBuilder::new()
    .with_name("app-with-logs")
    .add_container(Box::new(main_container))
    .add_sidecar(Box::new(log_sidecar))
    .build()?;
```

### Pattern 2: Sidecar Metrics

```rust
let metrics_sidecar = SidecarContainer::new("prom/prometheus-node-exporter:latest", "metrics")
    .with_port(9100, "metrics")
    .set_environment_variables(env_vars);

let step = KubeJobStepBuilder::new()
    .with_name("app-with-metrics")
    .add_container(Box::new(main_container))
    .add_sidecar(Box::new(metrics_sidecar))
    .build()?;
```

### Pattern 3: Readiness and Liveness Probes

```rust
let container = MaestroContainer::new("app:latest", "main")
    .with_readiness_probe(
        Probe::http_get("/health", 8080)
            .with_initial_delay_seconds(10)
            .with_period_seconds(5)
    )
    .with_liveness_probe(
        Probe::http_get("/healthz", 8080)
            .with_initial_delay_seconds(15)
            .with_period_seconds(10)
    );
```

## Error Handling Patterns

### Pattern 1: Retry with Backoff

```rust
let job = KubeJobStepBuilder::new()
    .with_name("retry-job")
    .with_backoff_limit(6)
    .with_restart_policy(RestartPolicy::OnFailure)
    .build()?;
```

### Pattern 2: Conditional Execution

```rust
let mut chain = DependencyChain::new();
chain.add_step("validate");
chain.add_step("process")
    .with_conditional_dependency("validate", |deps| {
        deps.iter().all(|r| r.is_success())
    });
```

### Pattern 3: Failure Strategy

```rust
use k8s_maestro::workflows::scheduler::FailureStrategy;

let scheduler = Scheduler::new()
    .with_failure_strategy(FailureStrategy::StopOnFailure)
    .with_retry_count(3);
```

## Best Practices

1. **Always use the builder pattern** - Type-safe and fluent API
2. **Validate configurations** - Test with dry run before applying
3. **Use secrets for sensitive data** - Never hardcode credentials
4. **Set resource limits** - Prevent resource exhaustion
5. **Implement proper error handling** - Use `anyhow::Result` throughout
6. **Add labels and annotations** - Better organization and querying
7. **Use appropriate resource types** - Job vs Pod vs Deployment
8. **Test locally with Kind** - Validate before production deployment
9. **Monitor and log** - Add sidecars for observability
10. **Clean up resources** - Remove completed jobs and old resources
