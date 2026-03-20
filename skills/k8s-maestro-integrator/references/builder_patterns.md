# Builder Patterns Reference

This reference provides detailed information on all builder patterns available in k8s-maestro.

## WorkflowBuilder

The `WorkflowBuilder` is the primary builder for creating workflows.

```rust
use k8s_maestro::{WorkflowBuilder, ExecutionMode};
use k8s_maestro::steps::traits::ResourceLimits;

let workflow = WorkflowBuilder::new()
    .with_name("my-workflow")                    // Required: workflow name
    .with_namespace("production")                  // Optional: namespace (default: "default")
    .with_parallelism(4)                         // Optional: parallelism level (default: 1)
    .with_execution_mode(ExecutionMode::Parallel(2)) // Optional: execution mode
    .with_resource_limits(limits)                 // Optional: default resource limits
    .with_checkpointing(checkpoint_config)        // Optional: checkpointing config
    .with_label("env", "production")            // Optional: metadata label
    .with_annotation("owner", "devops")           // Optional: metadata annotation
    .add_step(step1)                             // Required: at least one step
    .add_step(step2)
    .build()?;                                   // Returns Result<Workflow>
```

### Execution Modes

- `ExecutionMode::Sequential` - Execute steps one at a time
- `ExecutionMode::Parallel(n)` - Execute up to n steps in parallel
- `ExecutionMode::DAG` - Execute according to dependency graph

### CheckpointConfig

```rust
use k8s_maestro::CheckpointConfig;

let config = CheckpointConfig::new()
    .enabled(true)
    .with_interval_secs(60)
    .with_storage_path("/checkpoint")
    .with_max_checkpoints(5);
```

## KubeJobStepBuilder

Builder for creating Kubernetes Job steps.

```rust
use k8s_maestro::steps::{KubeJobStep, KubeJobStepBuilder};
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, ContainerLike};

let step = KubeJobStepBuilder::new()
    .with_name("my-job")                    // Required: job name
    .with_namespace("default")              // Required: namespace
    .with_client(k8s_client)              // Required: K8s client
    .add_container(Box::new(MaestroContainer::new("nginx:latest", "main")))
    .add_sidecar(Box::new(SidecarContainer::new("fluent-bit", "logs")))
    .with_backoff_limit(4)                // Optional: retry limit (default: 6)
    .with_restart_policy(RestartPolicy::OnFailure) // Optional: policy
    .with_ttl_seconds(3600)               // Optional: cleanup TTL
    .with_completions(3)                  // Optional: desired completions
    .with_parallelism(2)                   // Optional: job parallelism
    .with_resource_limits(limits)           // Optional: resource limits
    .with_dry_run(false)                  // Optional: dry run mode
    .build()?;                             // Returns Result<KubeJobStep>
```

### RestartPolicy Options

- `RestartPolicy::Always` - Always restart on failure
- `RestartPolicy::OnFailure` - Restart only on failure
- `RestartPolicy::Never` - Never restart

### Quick Constructor

```rust
let step = KubeJobStep::new("my-job", "nginx:latest", k8s_client);
```

## KubePodStepBuilder

Builder for creating Kubernetes Pod steps.

```rust
use k8s_maestro::steps::{KubePodStep, KubePodStepBuilder};

let step = KubePodStepBuilder::new()
    .with_name("my-pod")
    .with_namespace("default")
    .add_container(Box::new(container))
    .with_restart_policy(RestartPolicy::Always)
    .build()?;
```

## MaestroContainerBuilder

Builder for container configuration.

```rust
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use std::collections::BTreeMap;

let container = MaestroContainer::new("nginx:latest", "web-server")
    .set_arguments(&["nginx".to_string(), "-g".to_string(), "daemon off;".to_string()])
    .set_environment_variables(env_vars)
    .set_resource_limits(limits)
    .with_volume_mount("data-pvc", "/data")
    .with_working_dir("/app")
    .with_command("/bin/sh");
```

### Setting Environment Variables

```rust
let mut env_vars = BTreeMap::new();
env_vars.insert("DATABASE_URL".to_string(), "postgres://localhost:5432/db".to_string());
env_vars.insert("LOG_LEVEL".to_string(), "info".to_string());
env_vars.insert("PORT".to_string(), "8080".to_string());
```

### Setting Resource Limits

```rust
use k8s_maestro::steps::traits::ResourceLimits;

let limits = ResourceLimits::new()
    .with_cpu("500m")      // CPU: 0.5 cores
    .with_memory("512Mi")   // Memory: 512 MB
    .with_storage("1Gi");   // Storage: 1 GB
```

## ServiceBuilder

Builder for creating Kubernetes Services.

```rust
use k8s_maestro::{ServiceBuilder, ServiceType, ServicePort};
use std::collections::BTreeMap;

let mut selector = BTreeMap::new();
selector.insert("app".to_string(), "my-app".to_string());

let service = ServiceBuilder::new()
    .with_name("my-service")
    .with_namespace("default")
    .add_port(ServicePort::new(80, 8080, "TCP"))
    .with_selector(selector)
    .with_type(ServiceType::ClusterIP)
    .with_label("team", "platform")
    .build()?;
```

### ServiceType Options

- `ServiceType::ClusterIP` - Internal cluster access (default)
- `ServiceType::NodePort` - External access via node ports
- `ServiceType::LoadBalancer` - Cloud load balancer
- `ServiceType::ExternalName` - DNS alias

## IngressBuilder

Builder for creating Kubernetes Ingress.

```rust
use k8s_maestro::IngressBuilder;

let ingress = IngressBuilder::new()
    .with_name("my-ingress")
    .with_namespace("default")
    .with_host("example.com")
    .with_path("/", "my-service", 80)
    .with_tls_secret("tls-cert")
    .with_path_type(PathType::Prefix)
    .build()?;
```

### PathType Options

- `PathType::Prefix` - Prefix-based path matching
- `PathType::Exact` - Exact path matching
- `PathType::ImplementationSpecific` - Implementation-specific matching

## ConfigMapBuilder

Builder for creating ConfigMaps.

```rust
use k8s_maestro::entities::ConfigMapBuilder;

let configmap = ConfigMapBuilder::new()
    .with_name("app-config")
    .with_namespace("default")
    .add_data("app.yaml", "key: value\nsetting: config")
    .add_data("config.json", "{\"setting\": \"value\"}")
    .with_label("app", "myapp")
    .with_annotation("version", "1.0")
    .build()?;
```

## SecretBuilder

Builder for creating Secrets.

```rust
use k8s_maestro::entities::{SecretBuilder, SecretType};

let secret = SecretBuilder::new()
    .with_name("api-keys")
    .with_namespace("default")
    .with_type(SecretType::Opaque)
    .add_data("api-key", base64::encode("secret-key-123"))
    .add_data("password", base64::encode("secure-password"))
    .build()?;
```

### SecretType Options

- `SecretType::Opaque` - Arbitrary key-value pairs (default)
- `SecretType::ServiceAccountToken` - Service account tokens
- `SecretType::Dockercfg` - Docker registry credentials
- `SecretType::DockerConfigJson` - Docker config.json format
- `SecretType::BasicAuth` - Basic authentication
- `SecretType::SSHAuth` - SSH credentials
- `SecretType::TLS` - TLS certificate

## PVCVolumeBuilder

Builder for creating Persistent Volume Claims.

```rust
use k8s_maestro::entities::{PVCVolume, MaestroPVCMountVolumeBuilder};
use k8s_maestro::entities::volume_types::AccessMode;

let pvc = MaestroPVCMountVolumeBuilder::new()
    .with_name("data-pvc")
    .with_access_mode(AccessMode::ReadWriteOnce)
    .with_storage("10Gi")
    .with_storage_class("fast-ssd")
    .build()?;
```

### AccessMode Options

- `AccessMode::ReadWriteOnce` - R/W by single node
- `AccessMode::ReadOnlyMany` - R/O by many nodes
- `AccessMode::ReadWriteMany` - R/W by many nodes
- `AccessMode::ReadWriteOncePod` - R/W by single pod

## ConfigMapVolumeBuilder

Builder for creating ConfigMap volumes.

```rust
use k8s_maestro::entities::{ConfigMapVolume, ConfigMapVolumeBuilder};

let config_volume = ConfigMapVolumeBuilder::new()
    .with_name("config-volume")
    .with_configmap_name("app-config")
    .with_items(vec![
        VolumeItem::new("app.yaml", "/etc/config/app.yaml"),
        VolumeItem::new("config.json", "/etc/config/config.json"),
    ])
    .with_default_mode(0o644)
    .build()?;
```

## SecretVolumeBuilder

Builder for creating Secret volumes.

```rust
use k8s_maestro::entities::{SecretVolume, SecretVolumeBuilder};

let secret_volume = SecretVolumeBuilder::new()
    .with_name("secret-volume")
    .with_secret_name("api-keys")
    .with_items(vec![
        VolumeItem::new("api-key", "/etc/secrets/api-key"),
    ])
    .with_default_mode(0o400)
    .build()?;
```

## EmptyDirVolumeBuilder

Builder for creating EmptyDir volumes.

```rust
use k8s_maestro::entities::{EmptyDirVolume, EmptyDirVolumeBuilder};
use k8s_maestro::entities::volume_types::Medium;

let empty_dir = EmptyDirVolumeBuilder::new()
    .with_name("cache")
    .with_size_limit("1Gi")
    .with_medium(Medium::Memory)
    .build()?;
```

### Medium Options

- `Medium::Default` - Default storage
- `Medium::Memory` - tmpfs (RAM-backed)

## HostPathVolumeBuilder

Builder for creating HostPath volumes.

```rust
use k8s_maestro::entities::{HostPathVolume, HostPathVolumeBuilder};
use k8s_maestro::entities::volume_types::HostPathType;

let host_path = HostPathVolumeBuilder::new()
    .with_name("host-data")
    .with_path("/var/data")
    .with_type(HostPathType::DirectoryOrCreate)
    .build()?;
```

### HostPathType Options

- `HostPathType::DirectoryOrCreate` - Create if not exists
- `HostPathType::Directory` - Must exist
- `HostPathType::FileOrCreate` - File create if not exists
- `HostPathType::File` - File must exist
- `HostPathType::Socket` - UNIX socket
- `HostPathType::CharDevice` - Character device
- `HostPathType::BlockDevice` - Block device

## SidecarContainerBuilder

Builder for creating sidecar containers.

```rust
use k8s_maestro::entities::SidecarContainer;

let sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
    .set_arguments(&["/fluent-bit/bin/fluent-bit".to_string()])
    .set_environment_variables(env_vars)
    .set_resource_limits(limits)
    .with_volume_mount("config", "/etc/fluent-bit")
    .with_working_dir("/fluent-bit");
```

## ResourceLimitsBuilder

Builder for resource limits.

```rust
use k8s_maestro::steps::traits::ResourceLimits;

let limits = ResourceLimits::new()
    .with_cpu("500m")      // CPU request/limit
    .with_cpu_request("250m") // Separate CPU request
    .with_memory("512Mi")   // Memory request/limit
    .with_memory_request("256Mi") // Separate memory request
    .with_storage("1Gi")    // Storage request/limit
    .with_storage_request("512Mi") // Separate storage request
    .with_ephemeral_storage("100Mi"); // Ephemeral storage
```

## DependencyChain

Builder for workflow dependency chains.

```rust
use k8s_maestro::workflows::DependencyChain;

let mut chain = DependencyChain::new();

// Add steps
chain.add_step("extract");
chain.add_step("transform");
chain.add_step("load");

// Add dependencies
chain.add_step("transform").with_dependency("extract");
chain.add_step("load").with_dependency("transform");

// Add ANY dependency (wait for any)
chain.add_step("validate-a");
chain.add_step("validate-b");
chain.add_step("process").with_dependency_any(vec!["validate-a", "validate-b"]);

// Add conditional dependency
chain.add_step("final-step")
    .with_conditional_dependency("process", |deps| {
        deps.iter().all(|r| r.is_success())
    });

// Build DAG
let graph = chain.build_dag()?;
let levels = graph.topological_sort()?;
```

## Security Builders

### RoleBuilder

```rust
use k8s_maestro::security::{RoleBuilder, PolicyRule};

let role = RoleBuilder::new()
    .with_name("workflow-role")
    .with_namespace("default")
    .add_rule(PolicyRule::new()
        .with_api_groups(vec!["batch".to_string()])
        .with_resources(vec!["jobs".to_string()])
        .with_verbs(vec!["get".to_string(), "list".to_string(), "create".to_string()]))
    .build()?;
```

### RoleBindingBuilder

```rust
use k8s_maestro::security::RoleBindingBuilder;

let binding = RoleBindingBuilder::new()
    .with_name("workflow-binding")
    .with_namespace("default")
    .with_role("workflow-role")
    .with_service_account("workflow-sa")
    .build()?;
```

### ServiceAccountBuilder

```rust
use k8s_maestro::security::ServiceAccountBuilder;

let sa = ServiceAccountBuilder::new()
    .with_name("workflow-sa")
    .with_namespace("default")
    .build()?;
```

### NetworkPolicyBuilder

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

## Best Practices

1. **Always validate before build**: Check required fields are set
2. **Use descriptive names**: Make resource names self-documenting
3. **Set resource limits**: Prevent runaway resource consumption
4. **Use labels and annotations**: Enable better organization and querying
5. **Test with dry run**: Validate configurations before applying
6. **Chain builders**: Use fluent API for readability
7. **Handle errors**: Use `?` operator properly with `anyhow::Result`

## Common Patterns

### Multi-container Pod with Sidecars

```rust
let step = KubeJobStepBuilder::new()
    .with_name("app")
    .with_namespace("default")
    .add_container(Box::new(MaestroContainer::new("app:latest", "main")))
    .add_sidecar(Box::new(SidecarContainer::new("fluent-bit", "logs")))
    .add_sidecar(Box::new(SidecarContainer::new("prometheus", "metrics")))
    .build()?;
```

### Service with Multiple Ports

```rust
let service = ServiceBuilder::new()
    .with_name("multi-port-service")
    .add_port(ServicePort::new(80, 8080, "HTTP"))
    .add_port(ServicePort::new(443, 8443, "HTTPS"))
    .add_port(ServicePort::new(9090, 9090, "Metrics"))
    .with_selector(selector)
    .build()?;
```

### Ingress with Multiple Hosts

```rust
let ingress = IngressBuilder::new()
    .with_name("multi-host-ingress")
    .with_host("app1.example.com")
    .with_path("/", "service1", 80)
    .with_host("app2.example.com")
    .with_path("/", "service2", 80)
    .with_tls_secret("tls-cert")
    .build()?;
```

### Workflow with Checkpointing

```rust
let workflow = WorkflowBuilder::new()
    .with_name("checkpointed-workflow")
    .with_checkpointing(CheckpointConfig::new()
        .enabled(true)
        .with_interval_secs(60)
        .with_storage_path("/checkpoint"))
    .add_step(step)
    .build()?;
```
