//! WASM container configuration examples for K8s Maestro.
//!
//! This example demonstrates how to configure containers for WASM (WebAssembly)
//! workloads using the K8s Maestro library. Since Kubernetes runs containers,
//! WASM workloads are typically executed using WASM-enabled container runtimes
//! like WasmEdge, Wasmtime, or Krustlet.
//!
//! The examples show:
//! - Creating WASM-enabled containers with proper runtime configurations
//! - Using KubeJobStepBuilder and KubePodStepBuilder with dry-run mode
//! - Configuring resource limits appropriate for WASM workloads
//! - Setting environment variables for WASM runtime parameters
//! - Building multi-container WASM setups with sidecars

use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::MaestroContainer,
    steps::{KubeJobStepBuilder, KubePodStepBuilder, ResourceLimits, WorkFlowStep},
};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("=== K8s Maestro WASM Container Examples ===\n");

    // Create a Kubernetes client (will use default config or fail gracefully in dry-run)
    let client = match create_client().await {
        Ok(client) => client,
        Err(e) => {
            log::warn!("Failed to create Kubernetes client: {}", e);
            log::warn!("Running examples in demonstration mode without actual client");
            return run_demonstration_examples();
        }
    };

    // Run all examples with actual client
    example_basic_wasm_container(&client).await?;
    example_wasm_with_resource_limits(&client).await?;
    example_wasm_multi_container(&client).await?;
    example_wasm_environment_configuration(&client).await?;
    example_wasm_pod_configuration(&client).await?;

    println!("\n=== All WASM examples completed successfully ===");
    Ok(())
}

/// Creates a MaestroK8sClient for interacting with Kubernetes
async fn create_client() -> anyhow::Result<MaestroK8sClient> {
    println!("Creating Kubernetes client...");
    let client = MaestroK8sClient::new().await?;
    println!("✓ Kubernetes client created successfully\n");
    Ok(client)
}

/// Demonstrates configuration failure without a client
fn run_demonstration_examples() -> anyhow::Result<()> {
    println!("\n=== Demonstration Mode ===");
    println!("WASM container configurations require a valid Kubernetes client.");
    println!("These examples demonstrate the API patterns for WASM workloads:\n");

    println!("1. Basic WASM container:");
    println!("   - Image: wasmedge/app:latest");
    println!("   - Runtime: WasmEdge with AOT compilation");
    println!("   - Use case: Serverless functions, edge computing");

    println!("\n2. Resource-limited WASM container:");
    println!("   - CPU: 100m (minimal WASM execution overhead)");
    println!("   - Memory: 64Mi (typical WASM module footprint)");
    println!("   - Advantages: Efficient resource utilization");

    println!("\n3. Multi-container WASM setup:");
    println!("   - Main: WASM module for business logic");
    println!("   - Sidecar: HTTP proxy or monitoring");
    println!("   - Architecture: Microservices with WASM compute");

    println!("\n4. Environment configuration:");
    println!("   - WASM_MODULE_PATH: Path to compiled .wasm file");
    println!("   - WASM_RUNTIME: WasmEdge, Wasmtime, or wasmer");
    println!("   - WASM_ARGS: Runtime-specific arguments");

    println!("\nTo run these examples with a real cluster:");
    println!("  1. Ensure you have kubectl configured with cluster access");
    println!("  2. Run: cargo run --example wasm_step");
    println!("  3. The examples will use dry_run=true for safe testing");

    Ok(())
}

/// Example 1: Basic WASM container configuration
async fn example_basic_wasm_container(client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("=== Example 1: Basic WASM Container ===");

    // Create a WASM-enabled container using WasmEdge
    let wasm_container = MaestroContainer::new("wasmedge/app:latest", "wasm-runtime")
        .set_arguments(&[
            "wasmmedge".to_string(),
            "--dir".to_string(),
            "/app".to_string(),
            "/app/module.wasm".to_string(),
        ]);

    // Build a Kubernetes Job step with the WASM container
    let wasm_job = KubeJobStepBuilder::new()
        .with_name("basic-wasm-job")
        .with_namespace("default")
        .add_container(Box::new(wasm_container))
        .with_client(client.clone())
        .with_dry_run(true) // Safe for testing without cluster
        .build()?;

    println!("✓ Created WASM job: {}", wasm_job.step_id());
    println!("  - Image: wasmedge/app:latest");
    println!("  - Module: /app/module.wasm");
    println!("  - Runtime: WasmEdge with AOT compilation");
    println!("  - Use case: Serverless functions, edge computing\n");

    Ok(())
}

/// Example 2: WASM container with resource limits
async fn example_wasm_with_resource_limits(client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("=== Example 2: WASM with Resource Limits ===");

    // WASM modules typically require minimal resources
    let resource_limits = ResourceLimits::new()
        .with_cpu("100m") // Minimal CPU for WASM execution
        .with_memory("64Mi") // Typical WASM module memory footprint
        .with_cpu_request("50m")
        .with_memory_request("32Mi");

    // Create a WASM container with resource constraints
    let wasm_container = MaestroContainer::new("wasmer.io/wasmer:latest", "wasm-compute")
        .set_arguments(&[
            "wasmer".to_string(),
            "run".to_string(),
            "--dir".to_string(),
            "/data".to_string(),
            "/app/compute.wasm".to_string(),
        ])
        .set_resource_bounds(resource_limits);

    // Build job with resource-limited WASM container
    let wasm_job = KubeJobStepBuilder::new()
        .with_name("resource-limited-wasm-job")
        .with_namespace("default")
        .add_container(Box::new(wasm_container))
        .with_client(client.clone())
        .with_dry_run(true)
        .build()?;

    println!(
        "✓ Created resource-limited WASM job: {}",
        wasm_job.step_id()
    );
    println!("  - CPU limit: 100m (minimal WASM execution overhead)");
    println!("  - Memory limit: 64Mi (typical WASM module footprint)");
    println!("  - Advantages: Efficient resource utilization");
    println!("  - Use case: High-density WASM workloads\n");

    Ok(())
}

/// Example 3: Multi-container WASM setup with sidecars
async fn example_wasm_multi_container(client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("=== Example 3: Multi-Container WASM Setup ===");

    // Main WASM compute container
    let main_wasm = MaestroContainer::new(
        "ghcr.io/wasmcloud/composite-wasmcloud-http-server:latest",
        "wasm-compute",
    )
    .set_arguments(&[
        "wasmcloud".to_string(),
        "start".to_string(),
        "--actor".to_string(),
        "/app/actor.wasm".to_string(),
    ]);

    // Sidecar container for HTTP proxy/load balancing
    let sidecar_proxy = MaestroContainer::new("nginx:alpine", "wasm-proxy").set_arguments(&[
        "nginx".to_string(),
        "-g".to_string(),
        "daemon off;".to_string(),
    ]);

    // Build a job with main WASM container and sidecar
    let wasm_job = KubeJobStepBuilder::new()
        .with_name("multi-container-wasm-job")
        .with_namespace("default")
        .add_container(Box::new(main_wasm))
        .add_sidecar(Box::new(sidecar_proxy))
        .with_client(client.clone())
        .with_dry_run(true)
        .build()?;

    println!("✓ Created multi-container WASM job: {}", wasm_job.step_id());
    println!("  - Main: WASM module for business logic");
    println!("  - Sidecar: HTTP proxy for external access");
    println!("  - Architecture: Microservices with WASM compute");
    println!("  - Use case: API endpoints with WASM business logic\n");

    Ok(())
}

/// Example 4: WASM container with environment configuration
async fn example_wasm_environment_configuration(client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("=== Example 4: WASM Environment Configuration ===");

    // Configure environment variables for WASM runtime
    let mut env_vars = BTreeMap::new();
    env_vars.insert(
        "WASM_MODULE_PATH".to_string(),
        "/app/module.wasm".to_string(),
    );
    env_vars.insert("WASM_RUNTIME".to_string(), "wasmtime".to_string());
    env_vars.insert(
        "WASM_ARGS".to_string(),
        "--enable-all --wasi-modules=std".to_string(),
    );
    env_vars.insert("RUST_LOG".to_string(), "info".to_string());

    // Create WASM container with environment configuration
    let wasm_container = MaestroContainer::new("bytecodealliance/wasmtime:latest", "wasm-runtime")
        .set_arguments(&["wasmtime".to_string(), "/app/module.wasm".to_string()])
        .set_environment_variables(env_vars);

    // Build job with environment-configured WASM container
    let wasm_job = KubeJobStepBuilder::new()
        .with_name("env-configured-wasm-job")
        .with_namespace("default")
        .add_container(Box::new(wasm_container))
        .with_client(client.clone())
        .with_dry_run(true)
        .build()?;

    println!(
        "✓ Created environment-configured WASM job: {}",
        wasm_job.step_id()
    );
    println!("  - WASM_MODULE_PATH: /app/module.wasm");
    println!("  - WASM_RUNTIME: wasmtime");
    println!("  - WASM_ARGS: Runtime-specific arguments");
    println!("  - Use case: Flexible WASM deployment configurations\n");

    Ok(())
}

/// Example 5: WASM pod configuration for long-running services
async fn example_wasm_pod_configuration(client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("=== Example 5: WASM Pod for Long-Running Services ===");

    // Configure WASM for long-running service (not a one-shot job)
    let wasm_service = MaestroContainer::new("secondstate/wasmedge:latest", "wasm-service")
        .set_arguments(&[
            "wasmedge".to_string(),
            "--reactor".to_string(),
            "/app/service.wasm".to_string(),
        ]);

    // Build a Pod step (instead of Job) for long-running WASM service
    let wasm_pod = KubePodStepBuilder::new()
        .with_name("wasm-service-pod")
        .with_namespace("default")
        .add_container(Box::new(wasm_service))
        .with_client(client.clone())
        .with_dry_run(true)
        .build()?;

    println!("✓ Created WASM service pod: {}", wasm_pod.step_id());
    println!("  - Type: Pod (not Job) for long-running service");
    println!("  - Mode: Reactor mode for request/response handling");
    println!("  - Use case: WASM microservices, HTTP handlers");
    println!("  - Advantages: Fast startup, low memory footprint\n");

    Ok(())
}
