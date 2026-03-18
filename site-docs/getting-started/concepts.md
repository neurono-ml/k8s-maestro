# Concepts

This guide explains the core concepts of k8s-maestro.

## Workflows

A **Workflow** is a collection of steps that execute in a specific order or in parallel. Workflows represent a logical unit of work that can be scheduled, executed, monitored, and managed in Kubernetes.

### Workflow Properties

- **Name**: Unique identifier for the workflow
- **Namespace**: Kubernetes namespace where the workflow runs
- **Steps**: Individual tasks to execute
- **Metadata**: Labels, annotations, and custom metadata
- **Execution Mode**: Sequential or parallel execution
- **Parallelism**: Maximum number of steps running simultaneously
- **Checkpointing**: Automatic checkpoint and recovery configuration

## Steps

A **Step** is the smallest unit of work in a workflow. Each step represents a task that can be executed independently.

### Step Types

k8s-maestro supports multiple step types:

1. **JobStep**: Kubernetes Job - runs containers to completion
2. **ExecStep**: Execute commands directly (local execution)
3. **WasmStep**: WebAssembly module execution
4. **PythonStep**: Python script execution (aspirational)
5. **RustStep**: Rust code execution (aspirational)

### Step Properties

- **Step ID**: Unique identifier within the workflow
- **Container Image**: Docker image to run
- **Command/Arguments**: Command to execute in the container
- **Environment Variables**: Configuration for the step
- **Resource Limits**: CPU, memory, and GPU constraints
- **Volume Mounts**: Storage attachments
- **Dependencies**: Other steps that must complete first

## Dependencies

Dependencies define the execution order of steps. A step can depend on one or more other steps, creating a **Directed Acyclic Graph (DAG)**.

### Dependency Types

1. **Simple Dependency**: Step B runs after Step A completes
2. **Multiple Dependencies**: Step C runs after Steps A and B complete
3. **Any Dependency**: Step D runs when ANY of A, B, or C completes
4. **Conditional Dependency**: Step E runs only if a condition is met

### Dependency Conditions

- **All Success**: Run only if all dependencies succeed
- **Any Success**: Run if any dependency succeeds
- **Output Check**: Run based on dependency output values
- **Custom Condition**: User-defined condition logic

## Execution Modes

### Sequential

Steps execute one after another:

```
Step A -> Step B -> Step C -> Step D
```

### Parallel

Multiple steps execute simultaneously:

```
Step A
Step B
Step C
```

### Mixed

Combination of sequential and parallel:

```
    Step A
   /      \
Step B    Step C
   \      /
    Step D
```

## Client

The **MaestroClient** is the main interface for interacting with Kubernetes workflows.

### Client Capabilities

- Create workflows
- Retrieve workflow status
- List workflows
- Delete workflows
- Watch workflow execution
- Configure default settings (namespace, timeout, logging)

### Client Configuration

```rust
let client = MaestroClientBuilder::new()
    .with_namespace("production")
    .with_default_timeout(Duration::from_secs(300))
    .with_dry_run(false)
    .with_log_level("info")
    .build()?;
```

## Checkpointing

Checkpointing enables workflow recovery from failures. When enabled, k8s-maestro periodically saves the workflow state.

### Checkpoint Benefits

- Resume from last checkpoint after failures
- Avoid re-executing completed steps
- Debug workflow execution
- Audit workflow history

### Checkpoint Configuration

```rust
let checkpoint_config = LegacyCheckpointConfig::new()
    .enabled(true)
    .with_interval_secs(60)
    .with_retention_count(10)
    .with_storage_path("/checkpoints");

let workflow = WorkflowBuilder::new()
    .with_checkpointing(checkpoint_config)
    // ...
```

## Services and Ingress

### Services

A **Service** exposes a workflow step to network traffic within the cluster.

```rust
let service = ServiceBuilder::new()
    .with_name("my-service")
    .with_port(80, 8080, "TCP")
    .with_type(ServiceType::ClusterIP)
    .build()?;
```

### Service Types

- **ClusterIP**: Internal cluster access only
- **NodePort**: Accessible via node IPs
- **LoadBalancer**: External load balancer
- **Headless**: DNS returns pod IPs directly

### Ingress

**Ingress** provides external HTTP/HTTPS access to services.

```rust
let ingress = IngressBuilder::new()
    .with_name("my-ingress")
    .with_host("example.com")
    .with_path("/", "my-service", 80)
    .with_tls_secret("tls-secret")
    .build()?;
```

## Sidecars

A **Sidecar** is an auxiliary container that runs alongside the main container in the same pod.

### Common Sidecar Use Cases

- **Logging**: Fluent Bit, log collectors
- **Monitoring**: Prometheus exporters, statsd
- **Proxies**: Envoy, NGINX sidecars
- **Debugging**: Debug containers, network tools

```rust
let logging_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
    .with_shared_volume("/var/log", "/var/log");
```

## Next Steps

- [Basic Workflows](../guides/basic-workflow.md) - Create workflows step by step
- [Dependencies](../guides/dependencies.md) - Configure complex execution flows
- [API Reference](../api/) - Detailed API documentation
