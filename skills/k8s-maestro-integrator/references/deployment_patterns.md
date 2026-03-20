# Deployment Patterns Reference

This reference provides comprehensive guidance on deploying k8s-maestro workflows to Kubernetes, including development, staging, and production environments.

## Environment Setup

### Kubernetes Cluster Detection

Before deploying, always verify your cluster configuration:

```bash
# Check cluster connectivity
kubectl cluster-info

# Check cluster version
kubectl version --short

# List nodes
kubectl get nodes

# Check available namespaces
kubectl get namespaces
```

### Detect Available Resources

Use the bundled script to analyze your cluster:

```bash
./scripts/analyze_cluster.sh
```

This script will:
- Detect cluster version (for feature selection)
- List available storage classes
- List ingress classes
- Show namespace information
- Display resource quotas
- Identify secrets and configmaps

## Deployment Workflow

### Step 1: Dry Run Validation

Always validate your configuration before deploying:

```rust
use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Create client in dry run mode
    let k8s_client = MaestroK8sClient::new().await?;
    let client = MaestroClientBuilder::new()
        .with_dry_run(true)
        .build()?;

    // Build workflow
    let workflow = WorkflowBuilder::new()
        .with_name("test-workflow")
        .with_namespace("production")
        .add_step(step)
        .build()?;

    // Validate with dry run
    let created = client.create_workflow(workflow)?;
    assert!(created.is_dry_run());

    println!("Dry run successful! Workflow is valid.");
    Ok(())
}
```

### Step 2: Namespace Preparation

Ensure the target namespace exists and is configured:

```rust
use k8s_maestro::tests::common::utilities::create_namespace;

// Create namespace
create_namespace(&client, "production").await?;

// Or use kubectl
kubectl create namespace production

// Add labels to namespace
kubectl label namespace production environment=production team=platform
```

### Step 3: Resource Prerequisites

Deploy required resources before the workflow:

```rust
// Create ConfigMaps
let configmap = ConfigMapBuilder::new()
    .with_name("app-config")
    .add_data("config.yaml", config_content)
    .build()?;
client.create_configmap(&configmap).await?;

// Create Secrets
let secret = SecretBuilder::new()
    .with_name("db-credentials")
    .add_data("password", base64::encode(password))
    .build()?;
client.create_secret(&secret).await?;

// Create PVCs
let pvc = MaestroPVCMountVolumeBuilder::new()
    .with_name("data-pvc")
    .with_storage("10Gi")
    .build()?;
client.create_pvc(&pvc).await?;
```

### Step 4: RBAC Configuration

Set up appropriate service accounts and permissions:

```rust
use k8s_maestro::security::{
    ServiceAccountBuilder,
    RoleBuilder,
    RoleBindingBuilder,
    PolicyRule
};

// Create service account
let sa = ServiceAccountBuilder::new()
    .with_name("workflow-sa")
    .with_namespace("production")
    .build()?;
client.create_service_account(&sa).await?;

// Create role with necessary permissions
let role = RoleBuilder::new()
    .with_name("workflow-role")
    .with_namespace("production")
    .add_rule(PolicyRule::new()
        .with_api_groups(vec!["batch".to_string(), "".to_string()])
        .with_resources(vec!["jobs".to_string(), "pods".to_string(), "pods/log".to_string()])
        .with_verbs(vec!["*".to_string()]))
    .build()?;
client.create_role(&role).await?;

// Bind role to service account
let binding = RoleBindingBuilder::new()
    .with_name("workflow-binding")
    .with_namespace("production")
    .with_role("workflow-role")
    .with_service_account("workflow-sa")
    .build()?;
client.create_role_binding(&binding).await?;
```

### Step 5: Deploy Workflow

Deploy the workflow to production:

```rust
let client = MaestroClientBuilder::new()
    .with_namespace("production")
    .with_dry_run(false)  // Enable real deployment
    .build()?;

let workflow = build_production_workflow()?;
let execution = client.execute_workflow(&workflow).await?;

println!("Workflow deployed with ID: {}", execution.id());
```

### Step 6: Monitor Execution

Monitor workflow execution in real-time:

```rust
use futures::StreamExt;

// Watch workflow events
let mut event_stream = client.watch_workflow_events(execution.id()).await?;

while let Some(event) = event_stream.next().await {
    match event {
        WorkflowEvent::StepStarted { step_id } => {
            println!("Step started: {}", step_id);
        }
        WorkflowEvent::StepCompleted { step_id, result } => {
            println!("Step completed: {} - {:?}", step_id, result.status());
        }
        WorkflowEvent::WorkflowFailed { error } => {
            eprintln!("Workflow failed: {}", error);
            break;
        }
        WorkflowEvent::WorkflowCompleted { .. } => {
            println!("Workflow completed successfully!");
            break;
        }
        _ => {}
    }
}
```

### Step 7: Verify and Clean Up

After completion, verify results and clean up:

```rust
// Verify success
if execution.is_success() {
    println!("Workflow completed successfully");

    // Retrieve results
    let results = client.get_workflow_results(execution.id()).await?;
    println!("Results: {:?}", results);
} else {
    eprintln!("Workflow failed!");
    // Retrieve error details
    let logs = execution.get_logs().await?;
    eprintln!("Logs: {}", logs);
}

// Clean up resources (optional, depends on your needs)
// Jobs may auto-clean based on TTL
execution.cleanup().await?;
```

## Deployment Environments

### Development (Local Kind Cluster)

For local development, use Kind:

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create Kind cluster
    let cluster = KindCluster::new().await?;

    // Get client configured for Kind
    let k8s_client = create_client_from_cluster(&cluster);

    // Deploy workflow
    let workflow = build_dev_workflow()?;
    let execution = deploy_workflow(&k8s_client, &workflow).await?;

    // Test locally
    println!("Workflow ID: {}", execution.id());

    // Cleanup when done
    cluster.cleanup().await?;
    Ok(())
}
```

### Staging (Test Cluster)

For staging, use a dedicated test cluster:

```rust
// Load staging kubeconfig
let kubeconfig_path = PathBuf::from("~/.kube/staging-config");
let config = Config::from_kubeconfig(&kubeconfig_path).await?;
let client = Client::try_from(config)?;

// Deploy with staging-specific settings
let workflow = WorkflowBuilder::new()
    .with_name("staging-workflow")
    .with_namespace("staging")
    .with_label("environment", "staging")
    .with_resource_limits(ResourceLimits::new()
        .with_cpu("500m")  // Lower limits for staging
        .with_memory("512Mi"))
    .add_step(step)
    .build()?;
```

### Production (Production Cluster)

For production, use hardened configuration:

```rust
let workflow = WorkflowBuilder::new()
    .with_name("production-workflow")
    .with_namespace("production")
    .with_label("environment", "production")
    .with_annotation("team", "platform")
    .with_resource_limits(ResourceLimits::new()
        .with_cpu("2000m")  // Higher limits for production
        .with_memory("4Gi"))
    .with_checkpointing(CheckpointConfig::new()
        .enabled(true)
        .with_interval_secs(60))
    .add_step(step)
    .build()?;
```

## Resource Management

### Storage Classes

Detect and use appropriate storage classes:

```bash
# List available storage classes
kubectl get storageclasses

# Describe specific storage class
kubectl describe storageclass fast-ssd
```

```rust
// Use specific storage class
let pvc = MaestroPVCMountVolumeBuilder::new()
    .with_name("production-pvc")
    .with_storage("100Gi")
    .with_storage_class("fast-ssd")  // Use fast storage for production
    .with_access_mode(AccessMode::ReadWriteOnce)
    .build()?;
```

### Ingress Classes

Configure ingress with appropriate class:

```bash
# List ingress classes
kubectl get ingressclass

# Describe ingress class
kubectl describe ingressclass nginx
```

```rust
let ingress = IngressBuilder::new()
    .with_name("production-ingress")
    .with_ingress_class("nginx")  // Use nginx ingress
    .with_host("app.example.com")
    .with_path("/", "backend-service", 80)
    .with_tls_secret("tls-cert")
    .build()?;
```

### Resource Quotas

Set resource quotas to prevent overconsumption:

```rust
use k8s_maestro::security::ResourceQuotaBuilder;

let quota = ResourceQuotaBuilder::new()
    .with_name("production-quota")
    .with_namespace("production")
    .with_hard_cpu("10")
    .with_hard_memory("20Gi")
    .with_hard_pods("50")
    .build()?;

client.create_resource_quota(&quota).await?;
```

### Limit Ranges

Set default resource limits:

```rust
use k8s_maestro::security::LimitRangeBuilder;

let limit_range = LimitRangeBuilder::new()
    .with_name("default-limits")
    .with_namespace("production")
    .add_item(LimitRangeItemBuilder::new()
        .with_type(LimitRangeType::Container)
        .with_default_cpu("500m")
        .with_default_memory("512Mi")
        .with_max_cpu("2000m")
        .with_max_memory("4Gi")
        .build()?)
    .build()?;

client.create_limit_range(&limit_range).await?;
```

## Network Policies

### Isolate Workloads

```rust
use k8s_maestro::security::{NetworkPolicyBuilder, NetworkPolicyRule, PolicyType};

// Policy: Only allow traffic from ingress to frontend
let frontend_policy = NetworkPolicyBuilder::new()
    .with_name("frontend-allow-ingress")
    .with_namespace("production")
    .with_policy_types(vec![PolicyType::Ingress])
    .add_selector("app", "frontend")
    .add_rule(NetworkPolicyRule::new()
        .with_port(8080, "TCP")
        .with_from_namespace_selector("kubernetes.io/metadata.name", "ingress-nginx"))
    .build()?;

client.create_network_policy(&frontend_policy).await?;

// Policy: Frontend can talk to backend
let backend_policy = NetworkPolicyBuilder::new()
    .with_name("backend-allow-frontend")
    .with_namespace("production")
    .with_policy_types(vec![PolicyType::Ingress])
    .add_selector("app", "backend")
    .add_rule(NetworkPolicyRule::new()
        .with_port(5432, "TCP")
        .with_from_pod_selector("app", "frontend"))
    .build()?;

client.create_network_policy(&backend_policy).await?;
```

## Security Hardening

### Pod Security Context

```rust
use k8s_maestro::security::{PodSecurityContextBuilder, ContainerSecurityContextBuilder};

let container = MaestroContainer::new("app:latest", "main")
    .with_security_context(ContainerSecurityContextBuilder::new()
        .with_run_as_non_root(true)
        .with_run_as_user(1000)
        .with_read_only_root_filesystem(true)
        .with_allow_privilege_escalation(false)
        .with_drop_capabilities(vec!["ALL"])
        .build()?);

let pod_security = PodSecurityContextBuilder::new()
    .with_fs_group(1000)
    .with_seccomp_profile("RuntimeDefault")
    .build()?;
```

### Secrets Management

**Best Practices:**

1. **Never hardcode secrets**:
   ```rust
   // BAD
   let password = "my-password";
   ```

   ```rust
   // GOOD
   let secret = client.get_secret("db-credentials").await?;
   let password = decode_secret(&secret.data["password"]);
   ```

2. **Use external secret management**:
   - HashiCorp Vault
   - AWS Secrets Manager
   - Azure Key Vault
   - GCP Secret Manager

3. **Rotate secrets regularly**:
   ```rust
   // Schedule secret rotation
   let rotation_schedule = "0 0 * * 0";  // Every Sunday
   create_secret_rotation_job("db-credentials", rotation_schedule).await?;
   ```

4. **Use service account tokens**:
   ```rust
   let sa = ServiceAccountBuilder::new()
       .with_name("workflow-sa")
       .with_auto_mount_token(true)
       .build()?;
   ```

## Monitoring and Observability

### Prometheus Metrics

```rust
let metrics_sidecar = SidecarContainer::new("prom/prometheus-node-exporter:latest", "metrics")
    .with_port(9100, "metrics")
    .set_arguments(&["/bin/node_exporter"])
    .set_environment_variables(env_vars);

let step = KubeJobStepBuilder::new()
    .add_container(Box::new(main_container))
    .add_sidecar(Box::new(metrics_sidecar))
    .build()?;
```

### Distributed Tracing

```rust
let tracing_sidecar = SidecarContainer::new("jaegertracing/all-in-one:latest", "jaeger")
    .with_port(5775, "zipkin")
    .with_port(16686, "query")
    .with_port(14268, "thrift")
    .set_environment_variables(vec![
        ("COLLECTOR_ZIPKIN_HOST_PORT", ":9411"),
        ("COLLECTOR_ZIPKIN_HTTP_PORT", "9411"),
    ]);
```

### Logging

```rust
let log_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
    .with_volume_mount("logs", "/var/log")
    .set_arguments(&["/fluent-bit/bin/fluent-bit", "-c", "/etc/fluent-bit/config.yaml"])
    .set_environment_variables(vec![
        ("FLUENT_HOST", "logserver"),
        ("FLUENT_PORT", "24224"),
    ]);
```

## Rollback and Recovery

### Manual Rollback

```rust
// Get previous successful version
let previous_version = client.get_workflow("my-workflow-v1").await?;

// Delete current version
client.delete_workflow("my-workflow-v2").await?;

// Deploy previous version
let execution = client.execute_workflow(&previous_version).await?;
```

### Automatic Rollback on Failure

```rust
use k8s_maestro::workflows::scheduler::FailureStrategy;

let scheduler = Scheduler::new()
    .with_failure_strategy(FailureStrategy::RollbackOnFailure)
    .with_retry_count(3)
    .with_rollback_version("v1.0.0");
```

### Checkpoint Recovery

```rust
// Enable checkpointing
let workflow = WorkflowBuilder::new()
    .with_checkpointing(CheckpointConfig::new()
        .enabled(true)
        .with_storage_path("/checkpoint")
        .with_interval_secs(60))
    .add_step(step)
    .build()?;

// After failure, recover from checkpoint
let recovered_workflow = client.recover_from_checkpoint("my-workflow").await?;
let execution = client.execute_workflow(&recovered_workflow).await?;
```

## Health Checks

### Readiness Probes

```rust
let container = MaestroContainer::new("app:latest", "main")
    .with_readiness_probe(
        Probe::http_get("/health", 8080)
            .with_initial_delay_seconds(10)
            .with_period_seconds(5)
            .with_timeout_seconds(1)
            .with_success_threshold(1)
            .with_failure_threshold(3)
    );
```

### Liveness Probes

```rust
let container = MaestroContainer::new("app:latest", "main")
    .with_liveness_probe(
        Probe::http_get("/healthz", 8080)
            .with_initial_delay_seconds(30)
            .with_period_seconds(10)
            .with_timeout_seconds(5)
    );
```

### Startup Probes

```rust
let container = MaestroContainer::new("app:latest", "main")
    .with_startup_probe(
        Probe::http_get("/startup", 8080)
            .with_initial_delay_seconds(0)
            .with_period_seconds(5)
            .with_failure_threshold(30)
    );
```

## Scaling Strategies

### Horizontal Scaling

```rust
let workflow = WorkflowBuilder::new()
    .with_parallelism(10)  // Run up to 10 steps in parallel
    .add_step(step)
    .build()?;
```

### Vertical Scaling

```rust
let workflow = WorkflowBuilder::new()
    .with_resource_limits(ResourceLimits::new()
        .with_cpu("4000m")
        .with_memory("8Gi"))
    .add_step(step)
    .build()?;
```

### Auto-scaling

```rust
// Use Kubernetes Horizontal Pod Autoscaler
let hpa = HorizontalPodAutoscalerBuilder::new()
    .with_name("workflow-hpa")
    .with_scale_target_ref("Workflow", "my-workflow")
    .with_min_replicas(1)
    .with_max_replicas(10)
    .with_target_cpu_utilization(80)
    .build()?;

client.create_hpa(&hpa).await?;
```

## Deployment Checklists

### Pre-Deployment Checklist

- [ ] Cluster connectivity verified
- [ ] Kubernetes version matches feature flags
- [ ] Namespace exists and is configured
- [ ] RBAC permissions are set up
- [ ] Secrets are properly created
- [ ] ConfigMaps are applied
- [ ] PVCs are ready
- [ ] Resource limits are appropriate
- [ ] Network policies are configured
- [ ] Monitoring and logging are set up
- [ ] Rollback plan is prepared
- [ ] Backup plan is in place

### Post-Deployment Checklist

- [ ] Workflow deployed successfully
- [ ] All steps completed successfully
- [ ] Logs show no errors
- [ ] Metrics are being collected
- [ ] Health checks are passing
- [ ] Resources are within limits
- [ ] Network policies are working
- [ ] Security context is applied
- [ ] Backup of deployment is saved

## Troubleshooting

### Common Issues

**Workflow not starting:**
```bash
# Check pod status
kubectl get pods -n production

# Describe pod
kubectl describe pod <pod-name> -n production

# Check events
kubectl get events -n production --sort-by='.lastTimestamp'
```

**Secret not found:**
```bash
# List secrets
kubectl get secrets -n production

# Describe secret
kubectl describe secret <secret-name> -n production
```

**PVC pending:**
```bash
# Check PVC status
kubectl get pvc -n production

# Describe PVC
kubectl describe pvc <pvc-name> -n production

# Check storage class
kubectl get storageclass
```

**Network policy blocking traffic:**
```bash
# List network policies
kubectl get networkpolicies -n production

# Describe policy
kubectl describe networkpolicy <policy-name> -n production

# Test connectivity
kubectl run -it --rm debug --image=nicolaka/netshoot --restart=Never -- curl http://service-name
```

### Debug Mode

```rust
// Enable debug logging
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
    .init();

// Run workflow with debug output
let execution = client.execute_workflow(&workflow).await?;

// Get detailed logs
let logs = execution.get_logs(LogStreamOptions {
    follow: true,
    tail_lines: 1000,
    ..Default::default()
}).await?;
```

## Best Practices

1. **Always test locally first** - Use Kind for development
2. **Use dry run** - Validate before deploying
3. **Enable checkpointing** - For long-running workflows
4. **Set appropriate limits** - Prevent resource exhaustion
5. **Monitor actively** - Use sidecars for observability
6. **Plan rollbacks** - Know how to revert changes
7. **Document deployments** - Keep track of what was deployed
8. **Automate where possible** - Use CI/CD for deployments
9. **Review security** - Regularly audit permissions and secrets
10. **Clean up resources** - Remove unused resources regularly
