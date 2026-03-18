# Configuration

This guide covers k8s-maestro configuration options and environment variables.

## Client Configuration

Configure the MaestroClient with various options for your use case.

### Namespace

Set the default namespace for workflow operations.

```rust
let client = MaestroClientBuilder::new()
    .with_namespace("production")
    .build()?;
```

### Dry Run

Enable dry run mode to validate workflows without execution.

```rust
let client = MaestroClientBuilder::new()
    .with_dry_run(true)
    .build()?;

// Workflow will be validated but not executed
let created = client.create_workflow(workflow)?;
assert!(created.is_dry_run());
```

### Timeout

Set a default timeout for workflow operations.

```rust
use std::time::Duration;

let client = MaestroClientBuilder::new()
    .with_default_timeout(Duration::from_secs(300))
    .build()?;
```

### Logging

Configure the log level for client operations.

```rust
let client = MaestroClientBuilder::new()
    .with_log_level("debug")
    .build()?;
```

Available log levels:
- `trace` - Most verbose
- `debug` - Detailed debugging
- `info` - Informational (default)
- `warn` - Warnings
- `error` - Errors only

### Resource Limits

Set default resource limits for all workflows.

```rust
use k8s_maestro::steps::traits::ResourceLimits;

let limits = ResourceLimits::new()
    .with_cpu("500m")
    .with_memory("512Mi")
    .with_gpu("0"); // No GPU by default

let client = MaestroClientBuilder::new()
    .with_default_resource_limits(limits)
    .build()?;
```

### Kubeconfig Path

Specify a custom kubeconfig file location.

```rust
use std::path::PathBuf;

let client = MaestroClientBuilder::new()
    .with_kube_config_path(PathBuf::from("/custom/path/to/kubeconfig"))
    .build()?;
```

## Environment Variables

k8s-maestro can be configured using environment variables.

### KUBECONFIG

Path to the kubeconfig file.

```bash
export KUBECONFIG=/path/to/kubeconfig
```

### MAESTRO_NAMESPACE

Default namespace for workflows.

```bash
export MAESTRO_NAMESPACE=production
```

### MAESTRO_LOG_LEVEL

Log level for k8s-maestro operations.

```bash
export MAESTRO_LOG_LEVEL=debug
```

### MAESTRO_DRY_RUN

Enable dry run mode (1) or disable (0).

```bash
export MAESTRO_DRY_RUN=0
```

### MAESTRO_TIMEOUT

Default timeout in seconds for operations.

```bash
export MAESTRO_TIMEOUT=300
```

### MAESTRO_DEFAULT_CPU

Default CPU limit for workflows.

```bash
export MAESTRO_DEFAULT_CPU=500m
```

### MAESTRO_DEFAULT_MEMORY

Default memory limit for workflows.

```bash
export MAESTRO_DEFAULT_MEMORY=512Mi
```

### MAESTRO_DEFAULT_GPU

Default GPU limit for workflows.

```bash
export MAESTRO_DEFAULT_GPU=0
```

## Workflow Configuration

Configure individual workflow settings.

### Checkpointing

Enable automatic checkpointing and recovery.

```rust
use k8s_maestro::workflows::LegacyCheckpointConfig;

let checkpoint_config = LegacyCheckpointConfig::new()
    .enabled(true)
    .with_interval_secs(60)
    .with_retention_count(10)
    .with_storage_path("/checkpoints");

let workflow = WorkflowBuilder::new()
    .with_name("checkpointed-workflow")
    .with_checkpointing(checkpoint_config)
    .add_step(JobStep::new("long-running", "python:3.11"))
    .build()?;
```

### Parallelism

Set the maximum number of parallel steps.

```rust
let workflow = WorkflowBuilder::new()
    .with_name("parallel-workflow")
    .with_parallelism(5)
    .add_step(JobStep::new("worker-1", "python:3.11"))
    .add_step(JobStep::new("worker-2", "python:3.11"))
    .add_step(JobStep::new("worker-3", "python:3.11"))
    .add_step(JobStep::new("worker-4", "python:3.11"))
    .add_step(JobStep::new("worker-5", "python:3.11"))
    .build()?;
```

### Execution Mode

Configure how steps execute (sequential or parallel).

```rust
use k8s_maestro::workflows::ExecutionMode;

let workflow = WorkflowBuilder::new()
    .with_name("execution-workflow")
    .with_execution_mode(ExecutionMode::Sequential)
    .add_step(JobStep::new("step-1", "python:3.11"))
    .add_step(JobStep::new("step-2", "python:3.11"))
    .build()?;
```

## Kubernetes Configuration

### Context Selection

Select a specific Kubernetes context from kubeconfig.

```bash
kubectl config use-context production-cluster
```

### Namespace Creation

Create a namespace if it doesn't exist.

```bash
kubectl create namespace production
```

### Resource Quotas

Set resource quotas to limit resource usage.

```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: maestro-quota
  namespace: production
spec:
  hard:
    requests.cpu: "4"
    requests.memory: 8Gi
    limits.cpu: "8"
    limits.memory: 16Gi
```

## Configuration File (Future)

k8s-maestro plans to support configuration files in the future:

```yaml
# maestro.yaml
client:
  namespace: production
  dry_run: false
  timeout: 300
  log_level: info

defaults:
  resources:
    cpu: 500m
    memory: 512Mi

checkpointing:
  enabled: true
  interval: 60
  retention: 10
  storage_path: /checkpoints
```

## Best Practices

1. **Use environment variables** for sensitive data (API keys, passwords)
2. **Set appropriate resource limits** to prevent resource exhaustion
3. **Enable checkpointing** for long-running workflows
4. **Use dry run mode** for testing in production environments
5. **Organize with namespaces** for different environments
6. **Monitor resource usage** with Kubernetes metrics

## Next Steps

- [Troubleshooting](troubleshooting.md) - Common configuration issues
- [API Reference](../api/) - Detailed API documentation
