# k8s-maestro Integrator

A comprehensive integration assistant for k8s-maestro - Kubernetes workflow orchestrator for Rust.

This skill helps you integrate k8s-maestro into your Rust projects and build production-ready Kubernetes workflows.

## What This Skill Does

The k8s-maestro-integrator skill provides:

- **Smart Resource Selection**: Guidance on when to use Jobs, Pods, or Workflows
- **Integration Patterns**: Help choosing between API and channel integration
- **Crate Feature Detection**: Automatic detection of required k8s-maestro features based on cluster version
- **Parameter Handling**: Multiple strategies for environment variables, secrets, ConfigMaps, and CLI arguments
- **Storage & Networking**: PVCs, ConfigMaps, Secrets, Services, and Ingress configuration
- **Security**: RBAC, network policies, and pod security context setup
- **Testing**: Unit, integration, and E2E test patterns with Kind clusters
- **Deployment**: Development (Kind), staging, and production deployment strategies
- **Monitoring**: Sidecars for logging and metrics, health checks, and observability

## Installation

### Option 1: Using skills.sh (Recommended)

If you have the [skills.sh](https://github.com/vercel-labs/skills) utility installed:

```bash
# Install directly from this repository
skills.sh install andreclaudino/k8s-maestro .agents/skills/k8s-maestro-integrator

# Or install from a local clone
cd k8s-maestro
skills.sh install .agents/skills/k8s-maestro-integrator
```

### Option 2: Using Git Sparse Checkout

Clone only the skill folder to avoid downloading the entire repository:

```bash
# Clone with sparse checkout to .agents/skills
git clone --filter=blob:none --sparse \
  https://github.com/andreclaudino/k8s-maestro.git \
  ~/.agents/skills/k8s-maestro-integrator

cd ~/.agents/skills/k8s-maestro-integrator
git sparse-checkout init
git sparse-checkout set .agents/skills/k8s-maestro-integrator
git sparse-checkout apply
```

Or for `.claude/skills`:

```bash
# Clone with sparse checkout to .claude/skills
git clone --filter=blob:none --sparse \
  https://github.com/andreclaudino/k8s-maestro.git \
  ~/.claude/skills/k8s-maestro-integrator

cd ~/.claude/skills/k8s-maestro-integrator
git sparse-checkout init
git sparse-checkout set .agents/skills/k8s-maestro-integrator
git sparse-checkout apply
```

### Option 3: Manual Installation

Copy the skill folder manually:

```bash
# Copy to .agents/skills (for Claude Code)
cp -r k8s-maestro/.agents/skills/k8s-maestro-integrator ~/.agents/skills/

# Or copy to .claude/skills (for Claude.ai)
cp -r k8s-maestro/.agents/skills/k8s-maestro-integrator ~/.claude/skills/
```

### Option 4: Using the skill directory directly

If you're already in the k8s-maestro repository, you can reference the skill directly by its path. The skill is located at:

```
/workspaces/k8s-maestro/.agents/skills/k8s-maestro-integrator
```

## Verification

After installation, verify the skill is loaded:

```bash
# For Claude Code with skills.sh
skills.sh list

# Or check if the skill file exists
ls ~/.agents/skills/k8s-maestro-integrator/SKILL.md
# or
ls ~/.claude/skills/k8s-maestro-integrator/SKILL.md
```

## Quick Start

Once installed, the skill will automatically activate when you ask about:

- Integrating k8s-maestro into Rust projects
- Creating Kubernetes workflows, jobs, or pods
- Setting up storage (PVCs), networking (Services, Ingress), or security
- Writing tests with Kind clusters
- Deploying to production or test environments

### Example Conversations

**Creating an ETL Pipeline:**
```
You: I need to create a data processing pipeline that extracts data from an API, 
   transforms it using Python, and loads it into PostgreSQL. The pipeline should 
   run as a Kubernetes Job with resource limits and retry logic.

Claude (with skill): [Provides complete Rust implementation using WorkflowBuilder, 
   KubeJobStep, dependency chains, resource limits, and proper error handling]
```

**Setting up Monitoring:**
```
You: I want to deploy a web application with a service and ingress for external access. 
   The application should have health checks, environment variables from secrets, 
   and persistent storage for data.

Claude (with skill): [Provides code for KubeJobStep, ServiceBuilder, 
   IngressBuilder, SecretBuilder, PVCVolume, health check configuration, and namespace isolation]
```

**Writing Tests:**
```
You: Write integration tests for a workflow that processes CSV files. The tests 
   should use a Kind cluster and verify workflow executes successfully with mock data.

Claude (with skill): [Provides test code using KindCluster, integration test patterns, 
   workflow creation and execution, resource validation, and proper cleanup]
```

## Documentation

- [**Usage Guide**](usage_guide.md) - Comprehensive guide with usage examples and tutorials
- [**Builder Patterns Reference**](references/builder_patterns.md) - Detailed builder pattern documentation
- [**Integration Patterns Reference**](references/integration_patterns.md) - Integration strategies and patterns
- [**Testing Patterns Reference**](references/testing_patterns.md) - Testing best practices and examples
- [**Deployment Patterns Reference**](references/deployment_patterns.md) - Deployment strategies for all environments

## Scripts

The skill includes deterministic scripts for automation:

- **analyze_cluster.sh** - Analyzes Kubernetes cluster and provides recommendations
- **detect_crate_features.py** - Detects required crate features based on cluster version
- **generate_workflow_code.py** - Generates workflow code from natural language
- **generate_test_code.py** - Generates test code from descriptions

### Running Scripts

```bash
# Analyze your cluster
./scripts/analyze_cluster.sh

# Detect required crate features
python3 ./scripts/detect_crate_features.py

# Generate workflow code from description
python3 ./scripts/generate_workflow_code.py "ETL pipeline with extract, transform, load"

# Generate test code from description
python3 ./scripts/generate_test_code.py "integration test with kind cluster"
```

## Test Cases

The skill includes 5 comprehensive test scenarios:

1. ETL pipeline with resource limits and retry logic
2. File observer workflow with logging and monitoring sidecars
3. Web application with service, ingress, health checks, and persistent storage
4. Integration tests with Kind cluster and mock data
5. Multi-step ETL pipeline with conditional execution and checkpointing

See `evals/evals.json` for details.

## Requirements

- Rust 1.70 or later
- kubectl configured to access your Kubernetes cluster
- Docker (for Kind cluster testing)
- Python 3.7+ (for scripts)

## Contributing

This skill is part of the k8s-maestro project. Contributions are welcome!

## License

MIT License - See k8s-maestro repository for details.

## Links

- [k8s-maestro Repository](https://github.com/andreclaudino/k8s-maestro)
- [Documentation](https://andreclaudino.github.io/k8s-maestro/)
- [Usage Guide](usage_guide.md) - Start here for detailed examples and tutorials
- [Crates.io](https://crates.io/crates/k8s-maestro)
- [Issue Tracker](https://github.com/andreclaudino/k8s-maestro/issues)
