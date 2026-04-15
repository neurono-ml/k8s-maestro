# Volumes Guide

This guide covers the volume types available in k8s-maestro and how to use them in your Kubernetes workflows.

## Overview

Volumes in Kubernetes provide persistent or temporary storage for containers. k8s-maestro provides a builder-based API for creating different volume types:

- **EmptyDir**: Temporary storage that lives as long as the pod
- **PVC**: Persistent storage that survives pod restarts
- **HostPath**: Direct access to host filesystem paths
- **Secret**: Mount sensitive data like passwords and API keys
- **ConfigMap**: Mount configuration files and settings

## Volume Types Quick Comparison

| Volume Type | Use Case | Persistence | Storage |
|-------------|----------|--------------|---------|
| EmptyDir | Temporary data, caches, scratch space | Pod lifetime | Node disk or memory |
| PVC | Database storage, file storage | Beyond pod lifecycle | External storage system |
| HostPath | Node-specific data, system access | Pod lifetime | Host filesystem |
| Secret | Passwords, tokens, certificates | Cluster-managed | etcd |
| ConfigMap | Application configuration | Cluster-managed | etcd |

## EmptyDirVolume

The `EmptyDirVolume` creates temporary storage that exists as long as the pod. It's useful for caching, scratch space, or sharing data between containers in a pod.

### Builder API

```rust
use k8s_maestro::entities::volumes::{EmptyDirVolumeBuilder, Medium};

let volume = EmptyDirVolumeBuilder::new(mount_path, volume_name)
    .with_medium(Medium::Memory)          // Optional: use tmpfs (memory-backed)
    .with_size_limit("1Gi")               // Optional: limit size
    .with_read_only(true)                 // Optional: make read-only
    .with_sub_path("subdir")              // Optional: mount sub-path
    .build();
```

### Methods

| Method | Description |
|--------|-------------|
| `new(mount_path, volume_name)` | Create a new builder |
| `with_medium(Medium)` | Set storage medium (Default or Memory) |
| `with_size_limit(size)` | Set maximum size (e.g., "1Gi") |
| `with_read_only(bool)` | Set read-only mount |
| `with_sub_path(path)` | Mount a specific sub-path |
| `build()` | Create the volume |

### Examples

#### Basic EmptyDir Volume

```rust
use k8s_maestro::entities::volumes::EmptyDirVolumeBuilder;

let temp_volume = EmptyDirVolumeBuilder::new("/tmp", "temp-storage")
    .build();
```

#### Memory-backed EmptyDir (tmpfs)

```rust
use k8s_maestro::entities::volumes::{EmptyDirVolumeBuilder, Medium};

let memory_volume = EmptyDirVolumeBuilder::new("/dev/shm", "shared-memory")
    .with_medium(Medium::Memory)
    .with_size_limit("512Mi")
    .build();
```

#### Read-only EmptyDir

```rust
use k8s_maestro::entities::volumes::EmptyDirVolumeBuilder;

let read_only_volume = EmptyDirVolumeBuilder::new("/data/readonly", "static-data")
    .with_read_only(true)
    .with_sub_path("assets")
    .build();
```

## PVCVolume

The `PVCVolume` provides persistent storage using Kubernetes PersistentVolumeClaims. Use this for data that needs to persist beyond the pod lifecycle.

### Builder API

```rust
use k8s_maestro::entities::volumes::{MaestroPVCMountVolumeBuilder, AccessMode};

let volume = MaestroPVCMountVolumeBuilder::new(mount_path, pvc_name, volume_name)
    .with_storage_class("fast-ssd")        // Optional: specify storage class
    .with_access_modes(vec![AccessMode::ReadWriteOnce])  // Optional: access modes
    .with_storage_size("10Gi")            // Optional: request storage size
    .with_read_only(true)                 // Optional: make read-only
    .with_sub_path("data")                // Optional: mount sub-path
    .build();
```

### Methods

| Method | Description |
|--------|-------------|
| `new(mount_path, pvc_name, volume_name)` | Create a new builder |
| `with_storage_class(class)` | Specify storage class |
| `with_access_modes(vec![AccessMode])` | Set access modes |
| `with_storage_size(size)` | Request storage size |
| `with_read_only(bool)` | Set read-only mount |
| `with_sub_path(path)` | Mount a specific sub-path |
| `build()` | Create the volume |

### Access Modes

- `AccessMode::ReadWriteOnce` - Single node read-write
- `AccessMode::ReadOnlyMany` - Multiple nodes read-only
- `AccessMode::ReadWriteMany` - Multiple nodes read-write
- `AccessMode::ReadWriteOncePod` - Single pod read-write (Kubernetes 1.22+)

### Examples

#### Basic PVC Volume

```rust
use k8s_maestro::entities::volumes::MaestroPVCMountVolumeBuilder;

let pvc_volume = MaestroPVCMountVolumeBuilder::new("/data", "my-pvc", "data-volume")
    .build();
```

#### PVC with Storage Class

```rust
use k8s_maestro::entities::volumes::{MaestroPVCMountVolumeBuilder, AccessMode};

let fast_storage = MaestroPVCMountVolumeBuilder::new("/data", "fast-pvc", "ssd-volume")
    .with_storage_class("fast-ssd")
    .with_access_modes(vec![AccessMode::ReadWriteOnce])
    .with_storage_size("50Gi")
    .build();
```

#### Read-only PVC

```rust
use k8s_maestro::entities::volumes::MaestroPVCMountVolumeBuilder;

let readonly_data = MaestroPVCMountVolumeBuilder::new("/input", "readonly-pvc", "input-volume")
    .with_read_only(true)
    .with_sub_path("datasets")
    .build();
```

## HostPathVolume

The `HostPathVolume` mounts a file or directory from the host node's filesystem. Use with caution in production as it couples your pod to specific node configurations.

### Builder API

```rust
use k8s_maestro::entities::volumes::{HostPathVolumeBuilder, HostPathType};

let volume = HostPathVolumeBuilder::new(mount_path, host_path, volume_name)
    .with_type(HostPathType::Directory)   // Optional: validate path type
    .with_read_only(true)                 // Optional: make read-only
    .with_sub_path("subdir")              // Optional: mount sub-path
    .build();
```

### Methods

| Method | Description |
|--------|-------------|
| `new(mount_path, host_path, volume_name)` | Create a new builder |
| `with_type(HostPathType)` | Validate path type |
| `with_read_only(bool)` | Set read-only mount |
| `with_sub_path(path)` | Mount a specific sub-path |
| `build()` | Create the volume |

### Host Path Types

- `HostPathType::Default` - No validation
- `HostPathType::Directory` - Must be existing directory
- `HostPathType::File` - Must be existing file
- `HostPathType::Socket` - Must be existing socket
- `HostPathType::BlockDevice` - Must be block device
- `HostPathType::CharDevice` - Must be character device
- `HostPathType::DirectoryOrCreate` - Create if doesn't exist
- `HostPathType::FileOrCreate` - Create file if doesn't exist

### Examples

#### Basic HostPath Volume

```rust
use k8s_maestro::entities::volumes::HostPathVolumeBuilder;

let host_volume = HostPathVolumeBuilder::new("/host-data", "/var/data", "host-vol")
    .build();
```

#### HostPath with Type Validation

```rust
use k8s_maestro::entities::volumes::{HostPathVolumeBuilder, HostPathType};

let validated_volume = HostPathVolumeBuilder::new("/logs", "/var/log/myapp", "log-volume")
    .with_type(HostPathType::DirectoryOrCreate)
    .build();
```

#### Read-only HostPath

```rust
use k8s_maestro::entities::volumes::HostPathVolumeBuilder;

let readonly_host = HostPathVolumeBuilder::new("/config/host", "/etc/config", "config-volume")
    .with_read_only(true)
    .with_type(HostPathType::Directory)
    .build();
```

## SecretVolume

The `SecretVolume` mounts Kubernetes Secrets as files. Use this for sensitive data like passwords, API keys, and certificates.

### Builder API

```rust
use k8s_maestro::entities::volumes::{SecretVolumeBuilder, VolumeItem};

let volume = SecretVolumeBuilder::new(mount_path, secret_name, volume_name)
    .with_items(vec![VolumeItem::new("key", "filename")])  // Optional: specific keys
    .with_default_mode(0o400)           // Optional: default file permissions
    .with_optional(false)                // Optional: make optional
    .with_read_only(true)                // Optional: make read-only
    .with_sub_path("subdir")             // Optional: mount sub-path
    .build();
```

### Methods

| Method | Description |
|--------|-------------|
| `new(mount_path, secret_name, volume_name)` | Create a new builder |
| `with_items(vec![VolumeItem])` | Mount specific secret keys |
| `with_default_mode(mode)` | Set default file permissions (octal) |
| `with_optional(bool)` | Make volume optional |
| `with_read_only(bool)` | Set read-only mount |
| `with_sub_path(path)` | Mount a specific sub-path |
| `build()` | Create the volume |

### VolumeItem

```rust
use k8s_maestro::entities::volumes::VolumeItem;

VolumeItem::new("password", "password.txt")  // key, path
    .with_mode(0o600)                         // optional file mode
```

### Examples

#### Basic Secret Volume

```rust
use k8s_maestro::entities::volumes::SecretVolumeBuilder;

let secret_vol = SecretVolumeBuilder::new("/secrets", "db-credentials", "secret-vol")
    .build();
```

#### Secret with Specific Keys

```rust
use k8s_maestro::entities::volumes::{SecretVolumeBuilder, VolumeItem};

let specific_keys = SecretVolumeBuilder::new("/secrets", "api-keys", "api-secret")
    .with_items(vec![
        VolumeItem::new("api-key", "api-key.txt"),
        VolumeItem::new("api-secret", "api-secret.txt").with_mode(0o600),
    ])
    .build();
```

#### Secret with Custom Permissions

```rust
use k8s_maestro::entities::volumes::SecretVolumeBuilder;

let restricted = SecretVolumeBuilder::new("/secrets", "tls-certs", "cert-volume")
    .with_default_mode(0o400)
    .with_read_only(true)
    .build();
```

## ConfigMapVolume

The `ConfigMapVolume` mounts Kubernetes ConfigMaps as files. Use this for configuration files, environment variables, or command-line arguments.

### Builder API

```rust
use k8s_maestro::entities::volumes::{ConfigMapVolumeBuilder, VolumeItem};

let volume = ConfigMapVolumeBuilder::new(mount_path, configmap_name, volume_name)
    .with_items(vec![VolumeItem::new("key", "filename")])  // Optional: specific keys
    .with_default_mode(0o644)           // Optional: default file permissions
    .with_optional(false)                // Optional: make optional
    .with_read_only(true)               // Optional: make read-only
    .with_sub_path("subdir")             // Optional: mount sub-path
    .build();
```

### Methods

| Method | Description |
|--------|-------------|
| `new(mount_path, configmap_name, volume_name)` | Create a new builder |
| `with_items(vec![VolumeItem])` | Mount specific configmap keys |
| `with_default_mode(mode)` | Set default file permissions (octal) |
| `with_optional(bool)` | Make volume optional |
| `with_read_only(bool)` | Set read-only mount |
| `with_sub_path(path)` | Mount a specific sub-path |
| `build()` | Create the volume |

### Examples

#### Basic ConfigMap Volume

```rust
use k8s_maestro::entities::volumes::ConfigMapVolumeBuilder;

let config_vol = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
    .build();
```

#### ConfigMap with Specific Files

```rust
use k8s_maestro::entities::volumes::{ConfigMapVolumeBuilder, VolumeItem};

let config_files = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
    .with_items(vec![
        VolumeItem::new("config.yaml", "config.yaml"),
        VolumeItem::new("settings.json", "settings.json"),
    ])
    .build();
```

#### ConfigMap with Custom Permissions

```rust
use k8s_maestro::entities::volumes::ConfigMapVolumeBuilder;

let readonly_config = ConfigMapVolumeBuilder::new("/config/static", "static-config", "static-vol")
    .with_default_mode(0o444)
    .with_read_only(true)
    .build();
```

## VolumeMountLike Trait

All volume types implement the `VolumeMountLike` trait, which provides a uniform interface for mounting volumes to containers:

```rust
use k8s_maestro::entities::volumes::VolumeMountLike;

// Get volume metadata
let name = volume.volume_name();
let path = volume.mount_path();
let readonly = volume.read_only();
let subpath = volume.sub_path();

// Convert to Kubernetes types
let volume_mount: VolumeMount = volume.as_volume_mount();
let k8s_volume: Volume = volume.as_volume();
```

### Trait Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `volume_name(&self)` | `&str` | The volume name |
| `mount_path(&self)` | `&str` | Container mount path |
| `read_only(&self)` | `bool` | Whether mount is read-only |
| `sub_path(&self)` | `Option<&str>` | Optional sub-path |
| `as_volume_mount(&self)` | `VolumeMount` | K8s VolumeMount |
| `as_volume(&self)` | `Volume` | K8s Volume |

## Usage Examples

### Example 1: EmptyDir for Temporary Storage

EmptyDir is perfect for temporary data, caches, or intermediate processing:

```rust
use k8s_maestro::entities::volumes::EmptyDirVolumeBuilder;
use k8s_maestro::entities::MaestroContainer;

// Create a container with temporary storage
let container = MaestroContainer::new("alpine:latest", "processor")
    .set_arguments(&vec!["sh".to_string(), "-c".to_string(), 
        "echo 'Processing...' && sleep 5 && echo 'Done'".to_string()])
    .with_volume(Box::new(
        EmptyDirVolumeBuilder::new("/tmp/work", "temp-storage")
            .with_size_limit("1Gi")
            .build()
    ));

// Use in a KubeJobStep
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

let job = KubeJobStepBuilder::new()
    .with_name("temp-storage-job")
    .with_namespace("default")
    .with_client(client)
    .add_container(Box::new(container))
    .build()?;
```

### Example 2: PVC for Persistent Storage

PVC volumes persist data beyond pod lifecycle:

```rust
use k8s_maestro::entities::volumes::{MaestroPVCMountVolumeBuilder, AccessMode};
use k8s_maestro::entities::MaestroContainer;

// Create a container that reads/writes persistent data
let container = MaestroContainer::new("postgres:15", "database")
    .set_arguments(&vec!["postgres".to_string()])
    .set_environment_variables(&[
        ("POSTGRES_DB".to_string(), "mydb".to_string()),
    ])
    .with_volume(Box::new(
        MaestroPVCMountVolumeBuilder::new("/var/lib/postgresql/data", "postgres-pvc", "data-vol")
            .with_storage_class("fast-ssd")
            .with_access_modes(vec![AccessMode::ReadWriteOnce])
            .with_storage_size("10Gi")
            .build()
    ));
```

### Example 3: Secret for Sensitive Data

Mount secrets for passwords, tokens, or certificates:

```rust
use k8s_maestro::entities::volumes::{SecretVolumeBuilder, VolumeItem};
use k8s_maestro::entities::MaestroContainer;

// First, create the secret in Kubernetes:
// kubectl create secret generic db-credentials \
//   --from-literal=username=admin \
//   --from-literal=password=secretpassword

let container = MaestroContainer::new("postgres:15", "app")
    .set_arguments(&vec!["sh".to_string(), "-c".to_string(), 
        "cat /secrets/db-credentials/password".to_string()])
    .with_volume(Box::new(
        SecretVolumeBuilder::new("/secrets/db-credentials", "db-credentials", "secret-vol")
            .with_items(vec![
                VolumeItem::new("username", "username"),
                VolumeItem::new("password", "password"),
            ])
            .with_default_mode(0o400)
            .build()
    ));
```

### Example 4: ConfigMap for Configuration

Mount configuration files:

```rust
use k8s_maestro::entities::volumes::{ConfigMapVolumeBuilder, VolumeItem};
use k8s_maestro::entities::MaestroContainer;

// First, create the ConfigMap in Kubernetes:
// kubectl create configmap app-config \
//   --from-file=config.yaml=/path/to/config.yaml

let container = MaestroContainer::new("myapp:latest", "app")
    .set_arguments(&vec!["/config/app".to_string()])
    .with_volume(Box::new(
        ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
            .with_items(vec![
                VolumeItem::new("config.yaml", "app.yaml"),
                VolumeItem::new("settings.properties", "settings.properties"),
            ])
            .build()
    ));
```

## Combined Examples

### Example 5: EmptyDir + Sidecar (Shared Data)

Use EmptyDir to share data between main container and sidecar:

```rust
use k8s_maestro::entities::volumes::EmptyDirVolumeBuilder;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

// Shared volume between main container and sidecar
let shared_volume = EmptyDirVolumeBuilder::new("/shared-data", "shared-storage")
    .build();

// Main container that produces data
let producer = MaestroContainer::new("alpine:latest", "producer")
    .set_arguments(&vec!["sh".to_string(), "-c".to_string(), 
        "echo 'data' > /shared-data/output.txt && sleep 10".to_string()])
    .with_volume(Box::new(shared_volume.clone()));

// Sidecar that consumes data
let consumer = MaestroContainer::new("alpine:latest", "consumer")
    .set_arguments(&vec!["sh".to_string(), "-c".to_string(), 
        "cat /shared-data/output.txt".to_string()])
    .with_volume(Box::new(shared_volume));

let job = KubeJobStepBuilder::new()
    .with_name("shared-data-job")
    .with_namespace("default")
    .with_client(client)
    .add_container(Box::new(producer))
    .add_sidecar(Box::new(consumer))
    .build()?;
```

### Example 6: PVC + Job with Checkpoint

Use PVC for checkpoint storage to enable job resumption:

```rust
use k8s_maestro::entities::volumes::{MaestroPVCMountVolumeBuilder, AccessMode};
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

// PVC for checkpoint data
let checkpoint_volume = MaestroPVCMountVolumeBuilder::new("/checkpoints", "checkpoint-pvc", "checkpoint-vol")
    .with_storage_class("standard")
    .with_access_modes(vec![AccessMode::ReadWriteOnce])
    .with_storage_size("5Gi")
    .build();

// Container that saves/loads checkpoints
let container = MaestroContainer::new("myapp:latest", "processor")
    .set_arguments(&vec!["sh".to_string(), "-c".to_string(), 
        r#"
        # Load checkpoint if exists
        if [ -f /checkpoints/state.json ]; then
            echo "Loading checkpoint..."
        fi
        
        # Process data and save checkpoint periodically
        echo '{"step": 1}' > /checkpoints/state.json
        echo "Processing step 1..."
        echo '{"step": 2}' > /checkpoints/state.json
        echo "Processing step 2..."
        "#.to_string()])
    .with_volume(Box::new(checkpoint_volume));

let job = KubeJobStepBuilder::new()
    .with_name("checkpoint-job")
    .with_namespace("default")
    .with_client(client)
    .with_ttl_seconds(3600)  // Clean up after 1 hour
    .add_container(Box::new(container))
    .build()?;
```

### Example 7: Multiple Volumes Combined

Combine different volume types in a single job:

```rust
use k8s_maestro::entities::volumes::{
    EmptyDirVolumeBuilder, 
    ConfigMapVolumeBuilder, 
    SecretVolumeBuilder,
    VolumeItem
};
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

// ConfigMap for application config
let config_vol = ConfigMapVolumeBuilder::new("/config", "app-config", "config-vol")
    .with_read_only(true)
    .build();

// Secret for credentials
let secret_vol = SecretVolumeBuilder::new("/secrets", "app-secrets", "secret-vol")
    .with_items(vec![VolumeItem::new("api-key", "api-key")])
    .with_default_mode(0o400)
    .build();

// EmptyDir for temporary processing
let temp_vol = EmptyDirVolumeBuilder::new("/tmp", "temp-storage")
    .with_size_limit("500Mi")
    .build();

// Container with all three volumes
let container = MaestroContainer::new("myapp:latest", "app")
    .set_arguments(&vec!["/bin/sh".to_string(), "-c".to_string(), 
        "echo 'Starting app with config from /config, secrets from /secrets' && sleep 60".to_string()])
    .with_volume(Box::new(config_vol))
    .with_volume(Box::new(secret_vol))
    .with_volume(Box::new(temp_vol));

let job = KubeJobStepBuilder::new()
    .with_name("multi-volume-job")
    .with_namespace("default")
    .with_client(client)
    .add_container(Box::new(container))
    .build()?;
```

### Example 8: Volume with Resource Limits

Combine volumes with container resource limits:

```rust
use k8s_maestro::entities::volumes::{EmptyDirVolumeBuilder, Medium};
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use k8s_maestro::steps::ResourceLimits;

// Memory-backed volume for fast temporary storage
let memory_volume = EmptyDirVolumeBuilder::new("/dev/shm", "memory-cache")
    .with_medium(Medium::Memory)
    .with_size_limit("2Gi")
    .build();

// Container with resource limits and memory volume
let container = MaestroContainer::new("redis:7", "cache")
    .set_arguments(&vec!["redis-server".to_string()])
    .with_volume(Box::new(memory_volume))
    .set_resource_bounds(ResourceLimits::builder()
        .with_memory_limit("4Gi")
        .with_cpu_limit("2000m")
        .with_ephemeral_storage_limit("10Gi")
        .build());

let job = KubeJobStepBuilder::new()
    .with_name("redis-cache-job")
    .with_namespace("default")
    .with_client(client)
    .with_resource_limits(ResourceLimits::builder()
        .with_memory_limit("4Gi")
        .with_cpu_limit("2000m")
        .build())
    .add_container(Box::new(container))
    .build()?;
```