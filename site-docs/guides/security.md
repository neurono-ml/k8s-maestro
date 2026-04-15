# Security Resources Guide

This guide covers all security-related builders and configurations available in k8s-maestro for securing Kubernetes workloads.

## Overview

The `k8s_maestro::security` module provides builders for creating Kubernetes security resources:

- **SecurityContext** - Container and pod-level security settings
- **RBAC** - Role-based access control (ServiceAccount, Role, ClusterRole, bindings)
- **ResourceQuota** - Namespace resource limits
- **NetworkPolicy** - Network isolation rules
- **LimitRange** - Default container/Pod resource limits
- **ImagePullSecret** - Registry authentication secrets

## SecurityContext

The SecurityContext module provides configuration for pod and container security.

### SecurityContextConfig

A configuration preset builder with predefined security profiles:

```rust
use k8s_maestro::security::SecurityContextConfig;

// Restricted profile (most secure)
// - Run as non-root
// - No privilege escalation
// - Read-only root filesystem
// - Drop ALL capabilities
// - RuntimeDefault seccomp profile
let restricted = SecurityContextConfig::restricted();

// Baseline profile (moderate security)
// - Run as non-root
// - No privilege escalation
// - Drop NET_RAW capability
let baseline = SecurityContextConfig::baseline();

// Privileged profile (full access)
// - Privileged mode enabled
// - Allow privilege escalation
let privileged = SecurityContextConfig::privileged();

// Custom configuration
let custom = SecurityContextConfig::new()
    .with_run_as_user(1000)
    .with_run_as_group(1000)
    .with_run_as_non_root(true)
    .with_read_only_root_filesystem(true)
    .with_allow_privilege_escalation(false)
    .add_capability("NET_ADMIN")
    .drop_capability("ALL")
    .with_seccomp_profile("RuntimeDefault")
    .with_fs_group(2000)
    .with_supplemental_groups(vec![1000, 2000]);
```

#### Available Methods

| Method | Description |
|--------|-------------|
| `with_run_as_user(i64)` | Set the user ID to run the container as |
| `with_run_as_group(i64)` | Set the group ID to run the container as |
| `with_run_as_non_root(bool)` | Require container to run as non-root user |
| `with_read_only_root_filesystem(bool)` | Mount root filesystem as read-only |
| `with_allow_privilege_escalation(bool)` | Allow privilege escalation |
| `with_privileged(bool)` | Run container in privileged mode |
| `add_capability(&str)` | Add Linux capability |
| `drop_capability(&str)` | Drop Linux capability |
| `with_seccomp_profile(&str)` | Set seccomp profile type |
| `with_fs_group(i64)` | Set FSGroup for pod |
| `with_supplemental_groups(Vec<i64>)` | Set supplemental groups |
| `with_fs_group_change_policy(&str)` | Set FSGroup change policy |

### ContainerSecurityContextBuilder

Builds container-level security context:

```rust
use k8s_maestro::security::ContainerSecurityContextBuilder;

let container_security = ContainerSecurityContextBuilder::new()
    .with_run_as_user(1000)
    .with_run_as_non_root(true)
    .with_read_only_root_filesystem(true)
    .with_allow_privilege_escalation(false)
    .add_capability("NET_ADMIN")
    .drop_capability("KILL")
    .with_seccomp_profile("RuntimeDefault")
    .build();
```

#### Builder Methods

- `with_run_as_user(user: i64)` - Set container user ID
- `with_run_as_group(group: i64)` - Set container group ID
- `with_run_as_non_root(non_root: bool)` - Require non-root execution
- `with_read_only_root_filesystem(read_only: bool)` - Read-only root filesystem
- `with_allow_privilege_escalation(allow: bool)` - Control privilege escalation
- `with_privileged(privileged: bool)` - Privileged mode
- `add_capability(cap: &str)` - Add Linux capability
- `drop_capability(cap: &str)` - Drop Linux capability
- `with_seccomp_profile(profile: &str)` - Set seccomp profile
- `from_config(config: &SecurityContextConfig)` - Build from config preset

### PodSecurityContextBuilder

Builds pod-level security context:

```rust
use k8s_maestro::security::PodSecurityContextBuilder;

let pod_security = PodSecurityContextBuilder::new()
    .with_fs_group(2000)
    .with_supplemental_groups(vec![1000, 2000])
    .with_run_as_user(1000)
    .with_run_as_non_root(true)
    .with_seccomp_profile("RuntimeDefault")
    .build();
```

#### Builder Methods

- `with_fs_group(group: i64)` - Set FSGroup
- `with_fs_group_change_policy(policy: &str)` - Set FSGroup change policy
- `with_supplemental_groups(groups: Vec<i64>)` - Set supplemental groups
- `with_run_as_user(user: i64)` - Set pod user ID
- `with_run_as_group(group: i64)` - Set pod group ID
- `with_run_as_non_root(non_root: bool)` - Require non-root execution
- `with_seccomp_profile(profile: &str)` - Set seccomp profile
- `from_config(config: &SecurityContextConfig)` - Build from config preset

## RBAC

Role-based access control builders for managing permissions.

### ServiceAccountBuilder

Creates a ServiceAccount for pod identity:

```rust
use k8s_maestro::security::ServiceAccountBuilder;

let service_account = ServiceAccountBuilder::new("workflow-sa", "production")
    .with_annotation("eks.amazonaws.com/role-arn", "arn:aws:iam::123456:role/workflow-role")
    .with_label("app", "workflow")
    .build()
    .expect("Failed to build service account");
```

#### Builder Methods

- `with_annotation(key: &str, value: &str)` - Add annotation
- `with_label(key: &str, value: &str)` - Add label

### RoleBuilder

Creates a Role for namespace-scoped permissions:

```rust
use k8s_maestro::security::{PolicyRule, RoleBuilder};

// Custom role with specific permissions
let role = RoleBuilder::new("pod-reader", "production")
    .add_rule(
        PolicyRule::new()
            .with_api_groups(vec!["".to_string()])
            .with_resources(vec!["pods".to_string()])
            .with_verbs(vec!["get".to_string(), "list".to_string()])
    )
    .build()
    .expect("Failed to build role");
```

#### Preset Roles

```rust
use k8s_maestro::security::RoleBuilder;

// Workflow executor role (create, read, update, delete jobs and pods)
let executor_role = RoleBuilder::workflow_executor("workflow-executor", "production")
    .expect("Failed to create executor role")
    .build()
    .expect("Failed to build role");

// Workflow viewer role (read-only access to jobs, pods, deployments)
let viewer_role = RoleBuilder::workflow_viewer("workflow-viewer", "production")
    .expect("Failed to create viewer role")
    .build()
    .expect("Failed to build role");

// Admin role (full access to all resources)
let admin_role = RoleBuilder::admin("admin", "production")
    .expect("Failed to create admin role")
    .build()
    .expect("Failed to build role");
```

### RoleBindingBuilder

Binds a Role to subjects:

```rust
use k8s_maestro::security::RoleBindingBuilder;

let role_binding = RoleBindingBuilder::new("workflow-binding", "production")
    .with_subject_service_account("workflow-sa", "production")
    .with_role_ref_role("workflow-executor")
    .with_label("app", "workflow")
    .build()
    .expect("Failed to build role binding");
```

#### Builder Methods

- `with_subject(subject: Subject)` - Add a subject
- `with_subject_service_account(name: &str, namespace: &str)` - Add SA subject
- `with_subject_user(name: &str)` - Add user subject
- `with_role_ref(role_ref: RoleRef)` - Set role reference
- `with_role_ref_role(name: &str)` - Reference a Role
- `with_role_ref_cluster_role(name: &str)` - Reference a ClusterRole
- `with_label(key: &str, value: &str)` - Add label
- `with_annotation(key: &str, value: &str)` - Add annotation

### ClusterRoleBuilder

Creates a ClusterRole for cluster-wide permissions:

```rust
use k8s_maestro::security::{ClusterRoleBuilder, PolicyRule};

let cluster_role = ClusterRoleBuilder::new("node-reader")
    .add_rule(
        PolicyRule::new()
            .with_api_groups(vec!["".to_string()])
            .with_resources(vec!["nodes".to_string()])
            .with_verbs(vec!["get".to_string(), "list".to_string()])
    )
    .build()
    .expect("Failed to build cluster role");
```

### ClusterRoleBindingBuilder

Binds a ClusterRole to subjects:

```rust
use k8s_maestro::security::ClusterRoleBindingBuilder;

let cluster_binding = ClusterRoleBindingBuilder::new("cluster-admin-binding")
    .with_subject_service_account("workflow-sa", "production")
    .with_role_ref_cluster_role("cluster-admin")
    .build()
    .expect("Failed to build cluster role binding");
```

### PolicyRule

Defines permissions for roles:

```rust
use k8s_maestro::security::PolicyRule;

let rule = PolicyRule::new()
    .with_api_groups(vec!["batch".to_string(), "".to_string()])
    .with_resources(vec!["jobs".to_string(), "pods".to_string()])
    .with_verbs(vec!["get".to_string(), "list".to_string(), "watch".to_string()])
    .with_resource_names(vec!["specific-job".to_string()]);
```

#### Builder Methods

- `with_api_groups(groups: Vec<String>)` - Set API groups
- `with_resources(resources: Vec<String>)` - Set resources
- `with_verbs(verbs: Vec<String>)` - Set verbs (get, list, watch, create, update, patch, delete)
- `with_resource_names(names: Vec<String>)` - Set specific resource names

## ResourceQuota

Sets hard limits on resource usage in a namespace.

### ResourceQuotaBuilder

```rust
use k8s_maestro::security::ResourceQuotaBuilder;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use std::collections::BTreeMap;

// Custom quota
let mut limits = BTreeMap::new();
limits.insert("requests.cpu".to_string(), Quantity("4".to_string()));
limits.insert("limits.memory".to_string(), Quantity("16Gi".to_string()));
limits.insert("count/pods".to_string(), Quantity("10".to_string()));

let quota = ResourceQuotaBuilder::new("team-quota", "production")
    .with_hard_limits(limits)
    .with_scope(k8s_maestro::security::QuotaScope::Terminating)
    .build()
    .expect("Failed to build resource quota");
```

#### Preset Quotas

```rust
use k8s_maestro::security::ResourceQuotaBuilder;

// Small workload: 2 CPU, 4Gi memory request, 8Gi limit, 10 pods
let small_quota = ResourceQuotaBuilder::small_workload("small", "team-a")
    .expect("Failed to create small workload preset")
    .build()
    .expect("Failed to build quota");

// Medium workload: 10 CPU, 20Gi memory request, 40Gi limit, 50 pods
let medium_quota = ResourceQuotaBuilder::medium_workload("medium", "team-a")
    .expect("Failed to create medium workload preset")
    .build()
    .expect("Failed to build quota");

// Large workload: 50 CPU, 100Gi memory request, 200Gi limit, 200 pods
let large_quota = ResourceQuotaBuilder::large_workload("large", "team-a")
    .expect("Failed to create large workload preset")
    .build()
    .expect("Failed to build quota");
```

### QuotaScope

Limits quota to specific pod types:

```rust
use k8s_maestro::security::QuotaScope;

// Terminating - for pods with restart policy
QuotaScope::Terminating

// NotTerminating - for long-running pods
QuotaScope::NotTerminating

// BestEffort - for pods without resource requests
QuotaScope::BestEffort

// NotBestEffort - for pods with resource requests
QuotaScope::NotBestEffort
```

#### Builder Methods

- `with_hard_limits(limits: BTreeMap<String, Quantity>)` - Set all limits
- `with_hard_limit(key: &str, value: &str)` - Add single limit
- `with_scopes(scopes: Vec<QuotaScope>)` - Set scopes
- `with_scope(scope: QuotaScope)` - Add scope
- `with_label(key: &str, value: &str)` - Add label
- `with_annotation(key: &str, value: &str)` - Add annotation

## NetworkPolicy

Controls network traffic between pods.

### NetworkPolicyBuilder

```rust
use k8s_maestro::security::NetworkPolicyBuilder;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use k8s_openapi::api::networking::v1::NetworkPolicyPort;

// Custom policy with ingress from specific pods
let selector = LabelSelector {
    match_labels: Some([("app".to_string(), "api".to_string())].iter().cloned().collect()),
    match_expressions: None,
};

let policy = NetworkPolicyBuilder::new("api-policy", "production")
    .with_pod_selector(selector.clone())
    .with_ingress_rule(
        k8s_maestro::security::NetworkPolicyRule::new()
            .with_pod_selector(selector)
    )
    .build()
    .expect("Failed to build network policy");
```

#### Preset Policies

```rust
use k8s_maestro::security::NetworkPolicyBuilder;

// Deny all traffic (empty policy blocks all)
let deny_all = NetworkPolicyBuilder::deny_all("deny-all", "production")
    .expect("Failed to create deny-all preset")
    .build()
    .expect("Failed to build policy");

// Allow all traffic (no restrictions)
let allow_all = NetworkPolicyBuilder::allow_all("allow-all", "production")
    .expect("Failed to create allow-all preset")
    .build()
    .expect("Failed to build policy");

// Allow traffic within same namespace
let allow_ns = NetworkPolicyBuilder::allow_within_namespace("allow-ns", "production")
    .expect("Failed to create allow-within-namespace preset")
    .build()
    .expect("Failed to build policy");
```

### PolicyType

Defines network policy direction:

```rust
use k8s_maestro::security::PolicyType;

// Ingress only
PolicyType::Ingress

// Egress only
PolicyType::Egress

// Both directions
PolicyType::Both
```

#### Builder Methods

- `with_pod_selector(selector: LabelSelector)` - Set pod selector
- `with_ingress_rule(rule: NetworkPolicyRule)` - Add ingress rule
- `with_egress_rule(rule: NetworkPolicyRule)` - Add egress rule
- `with_policy_types(types: Vec<PolicyType>)` - Set policy types
- `with_label(key: &str, value: &str)` - Add label
- `with_annotation(key: &str, value: &str)` - Add annotation

## LimitRange

Sets default and maximum resource limits for containers and pods.

### LimitRangeBuilder

```rust
use k8s_maestro::security::{LimitRangeBuilder, LimitRangeItemBuilder, LimitRangeType};

let container_limit = LimitRangeItemBuilder::new(LimitRangeType::Container)
    .with_default_value("cpu", "500m")
    .with_default_value("memory", "512Mi")
    .with_max_value("cpu", "2")
    .with_max_value("memory", "4Gi")
    .with_min_value("cpu", "100m")
    .with_min_value("memory", "256Mi")
    .build();

let limit_range = LimitRangeBuilder::new("container-limits", "production")
    .with_limit(container_limit)
    .build()
    .expect("Failed to build limit range");
```

### LimitRangeItemBuilder

#### LimitRangeType

- `LimitRangeType::Container` - Container-level limits
- `LimitRangeType::Pod` - Pod-level limits
- `LimitRangeType::PersistentVolumeClaim` - PVC limits

#### Builder Methods

- `with_max(max: BTreeMap<String, Quantity>)` - Set maximum limits
- `with_max_value(key: &str, value: &str)` - Add maximum limit
- `with_min(min: BTreeMap<String, Quantity>)` - Set minimum limits
- `with_min_value(key: &str, value: &str)` - Add minimum limit
- `with_default(default: BTreeMap<String, Quantity>)` - Set default limits
- `with_default_value(key: &str, value: &str)` - Add default limit
- `with_default_request(default_request: BTreeMap<String, Quantity>)` - Set default request
- `with_default_request_value(key: &str, value: &str)` - Add default request
- `with_max_limit_request_ratio(ratio: BTreeMap<String, String>)` - Set limit/request ratio

## ImagePullSecret

Creates docker-registry secrets for pulling images from private registries.

### ImagePullSecretBuilder

```rust
use k8s_maestro::entities::config::ImagePullSecretBuilder;

let secret = ImagePullSecretBuilder::new("my-registry-secret")
    .with_registry("https://index.docker.io/v1/")
    .with_username("myuser")
    .with_password("mypassword")
    .with_email("user@example.com")
    .build()
    .expect("Failed to build image pull secret");
```

#### Builder Methods

- `with_registry(registry: impl Into<String>)` - Set registry URL
- `with_username(username: impl Into<String>)` - Set username
- `with_password(password: impl Into<String>)` - Set password
- `with_email(email: impl Into<String>)` - Set email

## Combined Examples

### Job with SecurityContext

```rust
use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::container::MaestroContainer,
    entities::config::MaestroJobConfig,
    security::{SecurityContextConfig, ContainerSecurityContextBuilder},
};
use k8s_maestro::steps::{KubeJobStepBuilder, RestartPolicy};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;

    let security_config = SecurityContextConfig::restricted();
    let container_security = ContainerSecurityContextBuilder::from_config(&security_config).build();

    let container = MaestroContainer::new("nginx:latest", "web")
        .set_image_pull_policy("IfNotPresent")
        .set_security_context(container_security);

    let job = KubeJobStepBuilder::new()
        .with_name("secure-web-job")
        .with_namespace("production")
        .with_container(Box::new(container))
        .with_restart_policy(RestartPolicy::Never)
        .build()?;

    client.create_job(&job, "production", false).await?;

    Ok(())
}
```

### Job with ResourceQuota and NetworkPolicy

```rust
use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::container::MaestroContainer,
    security::{
        ResourceQuotaBuilder, NetworkPolicyBuilder, NetworkPolicyRule,
    },
};
use k8s_maestro::steps::{KubeJobStepBuilder, RestartPolicy};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    let namespace = "production";

    let quota = ResourceQuotaBuilder::small_workload("team-quota", namespace)
        .expect("Failed to create quota preset")
        .build()
        .expect("Failed to build quota");

    let policy = NetworkPolicyBuilder::allow_within_namespace("team-network-policy", namespace)
        .expect("Failed to create network policy preset")
        .build()
        .expect("Failed to build network policy");

    client.create_resource(&quota, namespace).await?;
    client.create_resource(&policy, namespace).await?;

    let container = MaestroContainer::new("app:latest", "app")
        .set_resource_requirements(
            Some(k8s_openapi::api::core::v1::ResourceRequirements {
                requests: Some([
                    ("cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("100m".to_string())),
                    ("memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("256Mi".to_string())),
                ].into_iter().collect()),
                limits: Some([
                    ("cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("500m".to_string())),
                    ("memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("512Mi".to_string())),
                ].into_iter().collect()),
            }),
        );

    let job = KubeJobStepBuilder::new()
        .with_name("quota-constrained-job")
        .with_namespace(namespace)
        .with_container(Box::new(container))
        .with_restart_policy(RestartPolicy::Never)
        .build()?;

    client.create_job(&job, namespace, false).await?;

    Ok(())
}
```

### Complete Security Stack Example

```rust
use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::container::MaestroContainer,
    security::{
        ServiceAccountBuilder, RoleBuilder, RoleBindingBuilder,
        ResourceQuotaBuilder, NetworkPolicyBuilder,
        SecurityContextConfig, ContainerSecurityContextBuilder,
    },
    steps::{KubeJobStepBuilder, RestartPolicy},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    let namespace = "secure-workflow";

    let service_account = ServiceAccountBuilder::new("workflow-sa", namespace)
        .with_annotation("eks.amazonaws.com/role-arn", "arn:aws:iam::123456:role/workflow")
        .build()?;

    let role = RoleBuilder::workflow_executor("workflow-executor", namespace)?
        .build()?;

    let role_binding = RoleBindingBuilder::new("workflow-binding", namespace)
        .with_subject_service_account("workflow-sa", namespace)
        .with_role_ref_role("workflow-executor")
        .build()?;

    let quota = ResourceQuotaBuilder::medium_workload("workflow-quota", namespace)?
        .build()?;

    let network_policy = NetworkPolicyBuilder::deny_all("strict-network", namespace)?
        .build()?;

    client.create_resource(&service_account, namespace).await?;
    client.create_resource(&role, namespace).await?;
    client.create_resource(&role_binding, namespace).await?;
    client.create_resource(&quota, namespace).await?;
    client.create_resource(&network_policy, namespace).await?;

    let security_config = SecurityContextConfig::restricted();
    let container_security = ContainerSecurityContextBuilder::from_config(&security_config).build();

    let container = MaestroContainer::new("secure-app:latest", "app")
        .set_security_context(container_security)
        .set_image_pull_policy("Always");

    let job = KubeJobStepBuilder::new()
        .with_name("secure-workflow-job")
        .with_namespace(namespace)
        .with_container(Box::new(container))
        .with_service_account("workflow-sa")
        .with_restart_policy(RestartPolicy::Never)
        .build()?;

    client.create_job(&job, namespace, false).await?;

    Ok(())
}
```
