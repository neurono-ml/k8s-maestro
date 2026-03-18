# AGENTS.md

This file provides guidance for agentic coding assistants working on this codebase.

## Build, Lint, and Test Commands

```bash
# Build
cargo build --release --verbose

# Run all tests
cargo test --verbose

# Run a single test
cargo test <test_name> -- --exact

# Clippy linting
cargo clippy

# Format code
cargo fmt --check
```

## Project Structure

| Module | Description |
|--------|-------------|
| k8s-maestro | Root workspace crate - Kubernetes job orchestrator tool library |
| k8s-maestro-k8s | Kubernetes client and job orchestration implementation |

## Code Style Guidelines

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
- Use 4 spaces indentation
- Line length: 100 chars max
- No trailing whitespace

### Naming Conventions
```rust
// Types: CamelCase
struct MaestroK8sClient { }
enum JobNameType { }
trait ContainerLike { }

// Functions and variables: snake_case
fn build_job() -> Result<Job> { }
let job_name = "maestro";

// Constants: SCREAMING_SNAKE_CASE
const GHCR_IMAGE_PULL_SECRET: &str = "oci-registry";
```

### Error Handling
```rust
// Use anyhow::Result for application-level errors
pub async fn main() -> anyhow::Result<()> {
    let maestro_client = MaestroK8sClient::new().await?;
    Ok(())
}

// Use thiserror for library errors
#[derive(thiserror::Error, Debug)]
pub enum MaestroError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),
}
```

### Async Patterns
```rust
#[tokio::main(flavor="current_thread")]
pub async fn main() -> anyhow::Result<()> {
    let maestro_client = MaestroK8sClient::new().await?;
    maestro_client.create_job(&job, namespace, dry_run).await?;
    Ok(())
}
```

### Builder Patterns
```rust
// Fluent builder methods should return Self
let container = MaestroContainer::new(image, container_name)
    .set_arguments(&vec!["bash".to_owned()])
    .set_environment_variables(env_vars)
    .set_resource_bounds(resource_bounds);
```

### Traits
```rust
// Use Box<dyn Trait> for trait objects
trait ContainerLike {
    fn as_container(&self) -> Container;
}

// Use ? to propagate trait conversion
.add_container(Box::new(container))?
```

## Testing Requirements

- All code must be tested
- Use Kind for testing on Kubernetes with testcontainers
- Integration tests in `crates/k8s-maestro-k8s/src/kubernetes/tests/fixtures/`
- Example test:
```bash
cargo test integration_test_kubernetes
```

### Test Infrastructure

The project uses a three-tier test organization:

#### Test Categories

| Category | Directory | Cluster Required | Speed |
|----------|-----------|------------------|-------|
| Unit | `src/**/*_test.rs` or `tests/common/mocking/` | No | Fast (< 10s) |
| Integration | `tests/integration/` | Yes (Kind) | Medium (< 5min) |
| E2E | `tests/e2e/` | Yes (Kind) | Medium (< 5min) |

#### Running Tests

```bash
# Run unit tests only (fast, no cluster needed)
cargo test --lib

# Run integration tests (requires Docker)
cargo test --test '*' -- --ignored

# Run all tests including ignored ones
cargo test -- --include-ignored

# Run specific test file
cargo test --test kind_cluster_lifecycle
```

#### Test Utilities (tests/common/)

| Module | Purpose |
|--------|---------|
| `kind_cluster` | Kind cluster lifecycle management |
| `fixtures` | YAML fixture loading and parsing |
| `utilities` | Resource creation, cleanup, and validation helpers |
| `mocking` | Mock K8s client for unit tests |

#### Writing Integration Tests

```rust
use k8s_maestro::tests::common::kind_cluster::KindCluster;
use k8s_maestro::tests::common::utilities::{create_namespace, apply_resource};

#[tokio::test]
#[ignore = "Requires Docker"]
async fn test_with_cluster() {
    let cluster = KindCluster::new().await.expect("Failed to create cluster");
    let client = create_client_from_cluster(&cluster);
    // Test code here...
}
```

#### Writing Unit Tests with Mocking

```rust
use k8s_maestro::tests::common::mocking::{MockK8sClient, mock_error};

#[test]
fn test_with_mock() {
    let mut mock = MockK8sClient::new()
        .add_get_response(Ok(serde_json::json!({"name": "test"})));
    
    let response = mock.next_get_response();
    assert!(response.is_ok());
}
```

## Documentation Requirements

### ADRs and FDRs
- New architecture decisions → `docs/adrs/` (create new ADR)
- New features → `docs/fdrs/` (create new FDR)

### README.md Requirements
- Must describe crate features clearly
- Show crate abilities and use cases
- Include usage examples
- Add common GitHub badges

### GitHub Pages
- Documentation updates → `docs/`
- New features must update or create examples in `docs/`
- Documentation structure:
  - `docs/README.md` - Documentation landing page
  - `docs/getting-started/` - Installation, quick start, and concepts
  - `docs/guides/` - How-to guides
  - `docs/reference/` - Configuration and troubleshooting
  - `docs/migration-guide.md` - API migration guide

## Change Management

1. When adding features or changes:
   - Create/update examples
   - Add/update ADRs or FDRs if needed
   - Update CHANGELOG.md upon completion

2. Verify implementation:
   - No fake/mockup code
   - All code must be real implementations
   - Follow requirements strictly

3. Dependencies:
   - Always use most up-to-date crate versions
   - Check Cargo.toml for current versions

## Project Description

This crate is a pipeline/workflow orchestrator for Kubernetes in Rust. It provides a high-level API for creating, managing, and watching Kubernetes jobs with simplified builders and type-safe interfaces.

## Key Patterns from Examples

- Create jobs using builders: `JobBuilder::new()`
- Configure containers with fluent API: `MaestroContainer::new()`
- Use `MaestroK8sClient::new().await?` to get a client
- Set `dry_run = false` for real execution
- Use `job.wait().await?` to wait for completion
- Clean up with `job.delete_job(dry_run).await?`
