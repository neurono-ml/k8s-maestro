//! Rust step examples for Kubernetes workflows using K8s Maestro.
//!
//! This example demonstrates how to configure and execute Rust-based workloads
//! in Kubernetes using the K8s Maestro library with KubeJobStepBuilder and
//! KubePodStepBuilder.

use k8s_maestro::{
    clients::MaestroK8sClient,
    entities::MaestroContainer,
    steps::{KubeJobStepBuilder, KubePodStepBuilder, KubeWorkFlowStep, ResourceLimits, RestartPolicy, WorkFlowStep},
};
use std::collections::BTreeMap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Info);

    println!("=== Rust Step Examples with K8s Maestro ===\n");

    // Create a Kubernetes client (works in dry_run mode without a cluster)
    println!("Creating Kubernetes client...");
    let k8s_client = MaestroK8sClient::new().await?;

    // Example 1: Basic Rust job with minimal configuration
    example_basic_rust_job(&k8s_client).await?;

    // Example 2: Rust job with resource limits and environment variables
    example_rust_job_with_resources(&k8s_client).await?;

    // Example 3: Rust job with parallel processing configuration
    example_parallel_rust_job(&k8s_client).await?;

    // Example 4: Rust pod with sidecar for logging
    example_rust_pod_with_sidecar(&k8s_client).await?;

    // Example 5: Data processing pipeline with multiple steps
    example_rust_pipeline(&k8s_client).await?;

    println!("\n=== All examples completed successfully ===");
    Ok(())
}

/// Example 1: Basic Rust job with minimal configuration
///
/// Demonstrates the simplest way to create a Rust-based Kubernetes job
/// using the builder pattern with dry_run enabled for safe testing.
async fn example_basic_rust_job(k8s_client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("Example 1: Basic Rust job with minimal configuration");

    let job = KubeJobStepBuilder::new()
        .with_name("rust-basic-job")
        .with_namespace("default")
        .add_container(Box::new(
            MaestroContainer::new("rust:1.75-slim", "rust-app")
                .set_arguments(&[
                    "sh".to_string(),
                    "-c".to_string(),
                    "echo 'Hello from Rust!' && rustc --version".to_string(),
                ]),
        ))
        .with_client(k8s_client.clone())
        .with_dry_run(true) // Safe for local testing without a cluster
        .build()?;

    println!("Created job: {}", job.step_id());
    println!("Namespace: {}", job.namespace());
    println!("Resource: {}", job.resource_name());
    println!("Dry run: No actual Kubernetes resources created\n");

    Ok(())
}

/// Example 2: Rust job with resource limits and environment variables
///
/// Shows how to configure CPU/memory limits and set environment variables
/// for a Rust application, which is useful for performance-critical workloads.
async fn example_rust_job_with_resources(
    k8s_client: &MaestroK8sClient,
) -> anyhow::Result<()> {
    println!("Example 2: Rust job with resource limits and environment variables");

    let mut env_vars = BTreeMap::new();
    env_vars.insert("RUST_LOG".to_string(), "info".to_string());
    env_vars.insert("RUST_BACKTRACE".to_string(), "1".to_string());
    env_vars.insert("APP_ENVIRONMENT".to_string(), "production".to_string());

    let resource_limits = ResourceLimits::new()
        .with_cpu("1000m") // 1 CPU core
        .with_memory("2Gi") // 2 GB memory
        .with_cpu_request("500m") // Request 0.5 CPU cores
        .with_memory_request("1Gi"); // Request 1 GB memory

    let job = KubeJobStepBuilder::new()
        .with_name("rust-resource-job")
        .with_namespace("default")
        .add_container(Box::new(
            MaestroContainer::new("rust:1.75-slim", "rust-processor")
                .set_arguments(&[
                    "sh".to_string(),
                    "-c".to_string(),
                    "cargo build --release && ./target/release/processor".to_string(),
                ])
                .set_environment_variables(env_vars)
                .set_resource_bounds(resource_limits),
        ))
        .with_backoff_limit(3) // Allow up to 3 retries
        .with_ttl_seconds(3600) // Clean up after 1 hour
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("Created job: {}", job.step_id());
    println!("Resource limits: CPU 1000m, Memory 2Gi");
    println!("Environment: RUST_LOG=info, APP_ENVIRONMENT=production");
    println!("Retry policy: Up to 3 retries on failure\n");

    Ok(())
}

/// Example 3: Rust job with parallel processing configuration
///
/// Demonstrates how to configure a job for parallel data processing,
/// which is common for Rust applications using Rayon or tokio.
async fn example_parallel_rust_job(k8s_client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("Example 3: Rust job with parallel processing configuration");

    let job = KubeJobStepBuilder::new()
        .with_name("rust-parallel-job")
        .with_namespace("default")
        .add_container(Box::new(
            MaestroContainer::new("rust:1.75-slim", "parallel-processor")
                .set_arguments(&[
                    "sh".to_string(),
                    "-c".to_string(),
                    "cargo run --release -- --parallel --threads 8".to_string(),
                ]),
        ))
        .with_parallelism(4) // Run 4 pods in parallel
        .with_completions(10) // Complete 10 jobs total
        .with_backoff_limit(6)
        .with_restart_policy(RestartPolicy::OnFailure)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("Created job: {}", job.step_id());
    println!("Parallel execution: 4 concurrent pods");
    println!("Total workload: 10 job completions");
    println!("Use case: Parallel CSV/Parquet processing with Rayon\n");

    Ok(())
}

/// Example 4: Rust pod with sidecar for logging
///
/// Shows how to create a pod with a main Rust container and a sidecar
/// for collecting and forwarding logs, a common pattern in microservices.
async fn example_rust_pod_with_sidecar(
    k8s_client: &MaestroK8sClient,
) -> anyhow::Result<()> {
    println!("Example 4: Rust pod with sidecar for logging");

    let job = KubePodStepBuilder::new()
        .with_name("rust-app-with-logging")
        .with_namespace("default")
        .add_container(Box::new(
            MaestroContainer::new("rust:1.75-slim", "rust-application")
                .set_arguments(&[
                    "sh".to_string(),
                    "-c".to_string(),
                    "cargo run --release > /app/logs/app.log 2>&1".to_string(),
                ]),
        ))
        .add_sidecar(Box::new(
            MaestroContainer::new("fluent/fluent-bit:2.2", "log-collector")
                .set_arguments(&[
                    "fluent-bit".to_string(),
                    "-i".to_string(),
                    "tail".to_string(),
                    "-p".to_string(),
                    "path=/app/logs/app.log".to_string(),
                    "-o".to_string(),
                    "stdout".to_string(),
                ]),
        ))
        .with_restart_policy(RestartPolicy::OnFailure)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("Created pod: {}", job.step_id());
    println!("Main container: Rust application writing logs");
    println!("Sidecar container: Fluent Bit log forwarding");
    println!("Architecture: Classic sidecar pattern for log aggregation\n");

    Ok(())
}

/// Example 5: Data processing pipeline with multiple steps
///
/// Demonstrates a realistic ETL pipeline using Rust jobs, showing how
/// to structure a complete data processing workflow.
async fn example_rust_pipeline(k8s_client: &MaestroK8sClient) -> anyhow::Result<()> {
    println!("Example 5: Data processing pipeline with multiple steps");

    // Step 1: Data ingestion
    let ingestion_job = KubeJobStepBuilder::new()
        .with_name("rust-data-ingestion")
        .with_namespace("default")
        .add_container(Box::new(
            MaestroContainer::new("rust:1.75-slim", "ingestion")
                .set_arguments(&[
                    "sh".to_string(),
                    "-c".to_string(),
                    "cargo run --bin ingest -- --source s3://data-bucket/input/".to_string(),
                ]),
        ))
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("Step 1: Data ingestion - {}", ingestion_job.step_id());

    // Step 2: Data transformation
    let transform_job = KubeJobStepBuilder::new()
        .with_name("rust-data-transform")
        .with_namespace("default")
        .add_container(Box::new(
            MaestroContainer::new("rust:1.75-slim", "transform")
                .set_arguments(&[
                    "sh".to_string(),
                    "-c".to_string(),
                    "cargo run --bin transform -- --parallel --format parquet".to_string(),
                ])
                .set_resource_bounds(
                    ResourceLimits::new()
                        .with_cpu("2000m")
                        .with_memory("4Gi")
                        .with_cpu_request("1000m")
                        .with_memory_request("2Gi"),
                ),
        ))
        .with_parallelism(8)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("Step 2: Data transformation - {}", transform_job.step_id());
    println!("Configuration: 8 parallel workers, high memory allocation");

    // Step 3: Data validation
    let validation_job = KubeJobStepBuilder::new()
        .with_name("rust-data-validation")
        .with_namespace("default")
        .add_container(Box::new(
            MaestroContainer::new("rust:1.75-slim", "validation")
                .set_arguments(&[
                    "sh".to_string(),
                    "-c".to_string(),
                    "cargo test --bin validate -- --nocapture".to_string(),
                ]),
        ))
        .with_backoff_limit(2)
        .with_client(k8s_client.clone())
        .with_dry_run(true)
        .build()?;

    println!("Step 3: Data validation - {}", validation_job.step_id());
    println!("Pipeline: Ingest -> Transform -> Validate");
    println!("All steps configured with dry_run: true\n");

    Ok(())
}
