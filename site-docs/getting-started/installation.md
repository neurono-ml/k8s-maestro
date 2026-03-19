# Installation

This guide will help you install and configure k8s-maestro.

## Prerequisites

Before installing k8s-maestro, ensure you have:

- **Rust** 1.70 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **Cargo** (comes with Rust)
- **Kubernetes** cluster (local or remote)
- **kubectl** configured to access your cluster

## Installing k8s-maestro

### Using Cargo

Add k8s-maestro to your `Cargo.toml`:

```toml
[dependencies]
k8s-maestro = "0.3"
```

Run `cargo build` to download and compile the crate:

```bash
cargo build --release
```

### Enabling Kubernetes Support

k8s-maestro supports multiple Kubernetes versions. Enable the appropriate feature flag:

```toml
k8s-maestro = { version = "0.3", features = ["k8s_v1_28"] }
```

Available features:
- `k8s_v1_28` - Kubernetes 1.28 (default)
- `k8s_v1_29` - Kubernetes 1.29
- `k8s_v1_30` - Kubernetes 1.30
- `k8s_v1_31` - Kubernetes 1.31
- `k8s_v1_32` - Kubernetes 1.32

### Enabling Additional Features

k8s-maestro provides optional features for extended functionality:

```toml
# Enable exec steps (for running scripts directly)
k8s-maestro = { version = "0.3", features = ["exec-steps"] }

# Combine features
k8s-maestro = { version = "0.3", features = ["k8s_v1_30", "exec-steps"] }
```

## Kubernetes Cluster Setup

### Using Kind (Local Development)

For local development and testing, use [Kind](https://kind.sigs.k8s.io/) to create a local Kubernetes cluster:

```bash
# Install Kind (if not already installed)
go install sigs.k8s.io/kind@v0.20.0

# Create a Kind cluster
kind create cluster --name maestro-test

# Verify cluster is running
kubectl cluster-info
```

### Using Minikube

Alternatively, use [Minikube](https://minikube.sigs.k8s.io/):

```bash
# Install Minikube
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube

# Start Minikube
minikube start

# Verify cluster is running
kubectl cluster-info
```

### Using a Remote Cluster

Configure `kubectl` to access your remote cluster:

```bash
# Update kubeconfig
kubectl config use-context your-remote-cluster

# Verify connection
kubectl cluster-info
```

## Verifying Installation

Create a test file `test_installation.rs`:

```rust
use k8s_maestro::MaestroClientBuilder;

fn main() -> anyhow::Result<()> {
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    println!("k8s-maestro installed successfully!");
    println!("Namespace: {}", client.namespace());

    Ok(())
}
```

Run the test:

```bash
cargo run --bin test_installation
```

## Troubleshooting

### Connection Issues

If you encounter connection errors to the Kubernetes cluster:

1. Verify `kubectl` is configured correctly:
   ```bash
   kubectl cluster-info
   ```

2. Check your kubeconfig file:
   ```bash
   kubectl config view
   ```

3. Ensure the kubeconfig file is in the default location:
   ```bash
   echo $KUBECONFIG
   # Usually: ~/.kube/config
   ```

### Feature Conflicts

When using multiple features, ensure compatibility:

```toml
# Correct - compatible features
k8s-maestro = { version = "0.3", features = ["k8s_v1_30", "exec-steps"] }

# Incorrect - only one k8s version feature should be enabled
k8s-maestro = { version = "0.3", features = ["k8s_v1_28", "k8s_v1_30"] }
```

### Build Errors

If you encounter build errors, try:

```bash
# Clean and rebuild
cargo clean
cargo build --release

# Update dependencies
cargo update

# Check Rust version
rustc --version  # Should be 1.70 or later
```

## Next Steps

After installation, continue to [Quick Start](quick-start.md) to run your first workflow.
