# Test Fixtures

This directory contains YAML fixture files for Kubernetes resources used in tests.

## Directory Structure

```
fixtures/
├── configmaps/          # ConfigMap fixtures
│   └── test-configmap.yaml
├── secrets/             # Secret fixtures
│   └── test-secret.yaml
├── pvcs/                # PersistentVolumeClaim fixtures
│   └── test-pvc.yaml
├── workflows/           # Workflow fixtures
│   └── simple-workflow.yaml
└── failure_scenarios/   # Error/failure test fixtures
    └── failing-job.yaml
```

## Loading Fixtures

Use the fixtures module to load fixtures in tests:

```rust
use k8s_maestro::tests::common::fixtures::{
    load_configmap_fixture,
    load_secret_fixture,
    load_pvc_fixture,
    load_job_fixture,
    load_workflow_fixture,
    load_yaml_fixture,
};

// Load typed fixtures
let cm = load_configmap_fixture("test-configmap")?;
let secret = load_secret_fixture("test-secret")?;
let pvc = load_pvc_fixture("test-pvc")?;
let job = load_job_fixture("failing-job")?;

// Load generic YAML
let workflow = load_workflow_fixture("simple-workflow")?;
```

## Creating New Fixtures

### Naming Convention

- Use descriptive names: `test-<resource-type>-<scenario>.yaml`
- Example: `test-configmap-with-labels.yaml`

### Fixture Format

Follow standard Kubernetes YAML format:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: my-configmap
  namespace: default
  labels:
    app: test
data:
  key1: value1
  key2: value2
```

### Best Practices

1. **Keep fixtures minimal** - Include only necessary fields
2. **Use default namespace** - Unless testing namespace-specific behavior
3. **Add labels** - Use `app: test` for easy cleanup
4. **Document purpose** - Add a comment at the top explaining the fixture

## Example: Workflow Fixture

```yaml
# Sample workflow with two steps
apiVersion: maestro.k8s.io/v1
kind: Workflow
metadata:
  name: sample-workflow
  namespace: default
spec:
  steps:
    - name: step-one
      image: busybox:latest
      command: ["echo", "Hello from step one"]
    - name: step-two
      image: busybox:latest
      command: ["echo", "Hello from step two"]
      dependsOn:
        - step-one
```
