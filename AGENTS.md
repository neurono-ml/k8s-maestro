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
- Documentation updates → `site-docs/`
- New features must update or create examples in `site-docs/`

## Documentation Structure

The project maintains two complementary documentation systems:

### Reference Documentation (docs/)

The `docs/` directory contains reference documentation for maintainers and contributors:

- **docs/adrs/** - Architecture Decision Records
  - `template.md` - ADR template with commit tracking
  - `README.md` - Index of all ADRs
  - ADRs capture significant technical decisions with context, rationale, and consequences
  - Each ADR includes implementation tracking: commit hashes, code paths, and test locations

- **docs/fdrs/** - Feature Description Records
  - `template.md` - FDR template with phase-based implementation tracking
  - `README.md` - Index of all FDRs
  - FDRs document planned features with requirements, design, and implementation phases
  - Each phase includes: commits, code paths, tests, and status

- **docs/prd.md** - Product Description Record
  - Executive summary, vision, and product goals
  - Target audience and core features
  - Technical requirements (performance, scalability, reliability, security)
  - Quality standards and roadmap (v1.1.0, v1.2.0, v2.0.0)
  - Version history note explaining v0.4.0 was never released

**When to use reference documentation:**
- Making significant architectural decisions (create ADR)
- Planning new features or improvements (create FDR)
- Reviewing historical decisions and rationale
- Understanding product direction and requirements
- Tracking implementation progress of features

### User Documentation (site-docs/)

The `site-docs/` directory contains user-facing documentation published via mdBook to GitHub Pages:

- **site-docs/index.md** - Landing page with overview
- **site-docs/getting-started/** - Installation, quick start, and concepts
- **site-docs/guides/** - How-to guides and examples
- **site-docs/reference/** - Configuration reference and troubleshooting
- **site-docs/testing.md** - Testing guide (also referenced in AGENTS.md)

**When to use user documentation:**
- Adding or updating user guides and tutorials
- Documenting new features for users
- Updating configuration reference
- Adding troubleshooting information
- Creating usage examples

## Documentation Updates

When making changes to the codebase:

1. **For Reference Documentation:**
   - Create ADR when making architectural decisions
   - Create FDR when planning significant features
   - Update commit tracking sections with actual commit hashes
   - Link code paths and test locations after implementation
   - Update index READMEs when adding new records

2. **For User Documentation:**
   - Update site-docs/ when adding or modifying features
   - Add examples to demonstrate new capabilities
   - Update migration guides for breaking changes
   - Keep configuration reference in sync with code changes

3. **Documentation Quality:**
   - All documentation must be well-formatted markdown
   - Include code examples where appropriate
   - Keep documentation up-to-date with code changes
   - Use clear, concise language

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

## Crates.io Publishing

### Troubleshooting

If a package publish fails:

1. **Check token**:
   ```bash
   # Verify CARGO_REGISTRY_TOKEN is set in GitHub repository secrets
   # Go to: Settings > Secrets and variables > Actions
   ```

2. **Check version**:
   ```bash
   # Verify version not already published
   cargo search k8s-maestro --limit 10
   ```

3. **Test locally**:
   ```bash
   # Dry-run to test without publishing
   cargo publish --dry-run
   ```

4. **Check workflow logs**:
   - Go to: Actions > Publish to crates.io
   - Check the "Publish to crates.io" step logs

5. **Common issues**:
   - Version already published: See workflow logs, will skip if already exists
   - Token invalid: Re-generate token at https://crates.io/settings/tokens
   - Network timeout: May retry, crates.io API can be slow
   - Invalid Cargo.toml: Check metadata (description, license, repository)

6. **Verify Cargo.toml metadata**:
   - Ensure `description` field is present
   - Ensure `license` field is present
   - Ensure `repository` link is correct (optional but recommended)
