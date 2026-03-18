# Troubleshooting

This guide helps you diagnose and fix common issues with k8s-maestro.

## Connection Issues

### "Connection refused" or "No route to host"

**Symptoms:**
- Cannot connect to Kubernetes cluster
- Timeout errors during client creation

**Solutions:**

1. Verify cluster is running:
   ```bash
   kubectl cluster-info
   ```

2. Check kubeconfig:
   ```bash
   kubectl config view
   ```

3. Test connection:
   ```bash
   kubectl get nodes
   ```

4. Verify context:
   ```bash
   kubectl config current-context
   kubectl config use-context correct-context
   ```

### "Unauthorized" or "Forbidden"

**Symptoms:**
- Authentication errors
- Permission denied errors

**Solutions:**

1. Check current user:
   ```bash
   kubectl auth whoami
   ```

2. Verify permissions:
   ```bash
   kubectl auth can-i create jobs --namespace=default
   ```

3. Update credentials:
   ```bash
   aws eks update-kubeconfig --name my-cluster  # EKS
   gcloud container clusters get-credentials my-cluster  # GKE
   az aks get-credentials --resource-group myRG --name myCluster  # AKS
   ```

## Workflow Execution Issues

### Workflow stuck in "Pending" state

**Symptoms:**
- Workflow created but not executing
- Pods remain in Pending state

**Solutions:**

1. Check pod status:
   ```bash
   kubectl describe pod <pod-name>
   ```

2. Common causes:
   - **Insufficient resources**: Add more nodes or reduce resource limits
   - **Image pull errors**: Verify image name and registry access
   - **Node affinity**: Ensure nodes match the workflow requirements
   - **Taints and tolerations**: Check if nodes have taints blocking the pod

3. Check resource usage:
   ```bash
   kubectl top nodes
   kubectl describe nodes
   ```

### Workflow failed with "CrashLoopBackOff"

**Symptoms:**
- Pod repeatedly crashes
- Container exits immediately

**Solutions:**

1. Check pod logs:
   ```bash
   kubectl logs <pod-name> --previous
   ```

2. Common causes:
   - **Application errors**: Fix the application code
   - **Missing dependencies**: Ensure all required files/packages are present
   - **Configuration errors**: Verify environment variables and arguments
   - **Health check failures**: Adjust probe configurations

3. Debug with interactive shell:
   ```bash
   kubectl exec -it <pod-name> -- /bin/bash
   ```

### Workflow execution timeout

**Symptoms:**
- Workflow takes longer than expected
- Timeout errors

**Solutions:**

1. Check pod logs for slow operations:
   ```bash
   kubectl logs -f <pod-name>
   ```

2. Common causes:
   - **Inefficient algorithms**: Optimize the workflow logic
   - **Insufficient resources**: Increase CPU/memory limits
   - **Network bottlenecks**: Check network connectivity
   - **Large data sets**: Consider data partitioning

3. Increase timeout:
   ```rust
   let client = MaestroClientBuilder::new()
       .with_default_timeout(Duration::from_secs(600))
       .build()?;
   ```

## Resource Issues

### "Insufficient cpu" or "Insufficient memory"

**Symptoms:**
- Pods not scheduled
- Resource quota exceeded errors

**Solutions:**

1. Check cluster capacity:
   ```bash
   kubectl describe nodes | grep -A 3 "Allocated resources"
   ```

2. Check resource quotas:
   ```bash
   kubectl describe resourcequota -n <namespace>
   ```

3. Solutions:
   - **Scale cluster**: Add more nodes
   - **Reduce limits**: Lower resource requirements
   - **Delete unused resources**: Free up capacity
   - **Request quota increase**: If using managed quotas

### "OOMKilled" (Out of Memory)

**Symptoms:**
- Pod killed due to memory exhaustion
- Container restarts

**Solutions:**

1. Check pod events:
   ```bash
   kubectl describe pod <pod-name>
   ```

2. Solutions:
   - **Increase memory limit**: Allocate more memory
   - **Optimize memory usage**: Reduce memory footprint
   - **Enable swap**: If supported by your runtime

3. Update workflow:
   ```rust
   let workflow = WorkflowBuilder::new()
       .with_name("memory-workflow")
       .add_step(JobStep::new("job-1", "python:3.11")
           .with_resource_limits(ResourceLimits::new()
               .with_memory("2Gi")))
       .build()?;
   ```

## Image Issues

### "ImagePullBackOff" or "ErrImagePull"

**Symptoms:**
- Cannot pull container image
- Image pull errors

**Solutions:**

1. Check image name:
   ```bash
   # Verify image exists
   docker pull <image-name>
   ```

2. Check registry access:
   ```bash
   # Test registry authentication
   docker login <registry-url>
   ```

3. Solutions:
   - **Fix image name**: Ensure correct registry and tag
   - **Configure image pull secret**: Add secrets for private registries
   - **Use public image**: If registry issues persist

4. Configure image pull secret:
   ```rust
   let workflow = WorkflowBuilder::new()
       .with_name("private-image-workflow")
       .with_image_pull_secret("my-registry-secret")
       .add_step(JobStep::new("job-1", "my-registry.com/my-image:latest"))
       .build()?;
   ```

## Dependency Issues

### "Cycle detected" in dependencies

**Symptoms:**
- Workflow creation fails
- Cycle detection error

**Solutions:**

1. Review dependency graph:
   ```rust
   let graph = chain.build_dag();
   match graph.detect_cycles() {
       Ok(()) => println!("No cycles"),
       Err(e) => println!("Cycle: {}", e),
   }
   ```

2. Common causes:
   - **Circular references**: A -> B -> C -> A
   - **Self-dependencies**: Step depends on itself

3. Fix by removing the cycle:
   ```rust
   // Incorrect (creates cycle)
   chain.add_step("A");
   chain.add_step("B").with_dependency("A");
   chain.add_step("C").with_dependency("B");
   chain.add_step("A").with_dependency("C"); // Cycle!

   // Correct
   chain.add_step("A");
   chain.add_step("B").with_dependency("A");
   chain.add_step("C").with_dependency("B");
   // Step A is the root, no self-dependency
   ```

### Step never runs (stuck waiting for dependencies)

**Symptoms:**
- Step remains in pending state
- Dependencies never complete

**Solutions:**

1. Check dependency status:
   ```bash
   kubectl get pods -l workflow=<workflow-id>
   ```

2. Common causes:
   - **Failed dependencies**: Check if dependencies succeeded
   - **Incorrect dependency names**: Verify step IDs
   - **Missing dependencies**: Ensure all dependencies exist

3. Use conditional dependencies:
   ```rust
   chain.add_step("validate");
   chain.add_step("process")
       .with_conditional_dependency("validate", ConditionBuilder::all_success());
   chain.add_step("cleanup")
       .with_conditional_dependency_any(vec!["validate", "process"], ConditionBuilder::any_failure());
   ```

## Debugging Tips

### Enable Verbose Logging

```rust
let client = MaestroClientBuilder::new()
    .with_log_level("trace")
    .build()?;
```

Or use environment variable:

```bash
export RUST_LOG=trace
export MAESTRO_LOG_LEVEL=trace
```

### Inspect Created Resources

```bash
# List all resources created by the workflow
kubectl get all -l app=<workflow-name>

# Describe a specific resource
kubectl describe pod <pod-name>

# View events
kubectl get events --sort-by='.lastTimestamp'
```

### Test with Dry Run

```rust
let client = MaestroClientBuilder::new()
    .with_dry_run(true)
    .build()?;

let created = client.create_workflow(workflow)?;
// No resources are actually created
```

### Use Kind for Local Testing

```bash
# Create a local cluster
kind create cluster --name test

# Run your workflow
cargo run

# Debug with kubectl
kubectl get pods
kubectl logs -f <pod-name>
```

## Getting Help

If you're still experiencing issues:

1. Check the [GitHub Issues](https://github.com/andreclaudino/k8s-maestro/issues)
2. Search for similar issues
3. Create a new issue with:
   - k8s-maestro version
   - Kubernetes version
   - Full error message
   - Steps to reproduce
   - Relevant logs and configuration

## Next Steps

- [Configuration](configuration.md) - Configuration options
- [API Reference](../api/) - Detailed API documentation
