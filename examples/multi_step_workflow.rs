//! This example demonstrates multi-step workflows with dependency chains.
//!
//! The example shows:
//! - Creating workflows with multiple steps
//! - Setting up step dependencies (A -> B -> C)
//! - Parallel execution of independent steps
//! - Conditional execution based on step results
//! - Complex DAG (Directed Acyclic Graph) structures

use k8s_maestro::{
    client::MaestroClientBuilder,
    steps::kubernetes::JobStep,
    workflows::{ConditionBuilder, DependencyChain, ExecutionMode, Workflow, WorkflowBuilder},
};

fn main() -> anyhow::Result<()> {
    println!("=== Multi-Step Workflow Examples ===\n");

    println!("Example 1: Sequential workflow (A -> B -> C)");
    example_sequential_workflow()?;

    println!("\nExample 2: Parallel workflow (A, B, C run independently)");
    example_parallel_workflow()?;

    println!("\nExample 3: Fan-out/Fan-in (A -> B, C, D -> E)");
    example_fanout_fanin_workflow()?;

    println!("\nExample 4: Conditional execution (B only runs if A succeeds)");
    example_conditional_workflow()?;

    println!("\nExample 5: Complex DAG with multiple dependencies");
    example_complex_dag()?;

    println!("\nExample 6: Partial parallel execution");
    example_partial_parallel()?;

    println!("\n=== All examples completed successfully ===");
    Ok(())
}

fn example_sequential_workflow() -> anyhow::Result<()> {
    println!("Creating a sequential ETL pipeline: Extract -> Transform -> Load");

    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    let workflow = WorkflowBuilder::new()
        .with_name("etl-pipeline")
        .with_execution_mode(ExecutionMode::Sequential)
        .add_step(JobStep::new("extract", "python:3.11-slim"))
        .add_step(JobStep::new("transform", "python:3.11-slim"))
        .add_step(JobStep::new("load", "postgres:16"))
        .build()?;

    println!("Sequential workflow created: {:?}", workflow);
    println!("Execution order: Extract -> Transform -> Load");
    println!("Each step waits for the previous step to complete");

    let created = client.create_workflow(workflow)?;
    println!("Workflow created with ID: {}", created.id());

    Ok(())
}

fn example_parallel_workflow() -> anyhow::Result<()> {
    println!("Creating a workflow with parallel data processing steps");

    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    let workflow = WorkflowBuilder::new()
        .with_name("parallel-processing")
        .with_parallelism(3)
        .add_step(JobStep::new("process-dataset-a", "python:3.11-slim"))
        .add_step(JobStep::new("process-dataset-b", "python:3.11-slim"))
        .add_step(JobStep::new("process-dataset-c", "python:3.11-slim"))
        .build()?;

    println!("Parallel workflow created: {:?}", workflow);
    println!("Execution: All three steps run in parallel");
    println!("Parallelism: 3 steps can run simultaneously");

    let created = client.create_workflow(workflow)?;
    println!("Workflow created with ID: {}", created.id());

    Ok(())
}

fn example_fanout_fanin_workflow() -> anyhow::Result<()> {
    println!("Creating a fan-out/fan-in workflow");

    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()?;

    let workflow = WorkflowBuilder::new()
        .with_name("fanout-fanin")
        .with_execution_mode(ExecutionMode::Sequential)
        .add_step(JobStep::new("fetch-data", "python:3.11-slim"))
        .add_step(JobStep::new("process-a", "python:3.11-slim"))
        .add_step(JobStep::new("process-b", "python:3.11-slim"))
        .add_step(JobStep::new("process-c", "python:3.11-slim"))
        .add_step(JobStep::new("aggregate-results", "python:3.11-slim"))
        .build()?;

    println!("Fan-out/fan-in workflow created: {:?}", workflow);
    println!("Flow: Fetch -> [Process A, Process B, Process C] -> Aggregate");
    println!("The three process steps would be configured as parallel");

    let created = client.create_workflow(workflow)?;
    println!("Workflow created with ID: {}", created.id());

    Ok(())
}

fn example_conditional_workflow() -> anyhow::Result<()> {
    println!("Creating a workflow with conditional execution");

    let mut chain = DependencyChain::new();
    chain.add_step("validate-data");
    chain
        .add_step("process-data")
        .with_conditional_dependency("validate-data", ConditionBuilder::all_success());

    println!("Conditional workflow:");
    println!("Step 1: validate-data");
    println!("Step 2: process-data (only if validate-data succeeds)");
    println!("If validation fails, processing is skipped");

    let graph = chain.build_dag();
    let levels = graph.topological_sort()?;
    println!("Execution levels: {:?}", levels);

    Ok(())
}

fn example_complex_dag() -> anyhow::Result<()> {
    println!("Creating a complex DAG with multiple dependencies");

    let mut chain = DependencyChain::new();

    println!("Building complex DAG structure:");
    println!("         fetch");
    println!("        /      \\");
    println!("  validate   clean");
    println!("      |         |");
    println!("  transform   |");
    println!("      \\       /");
    println!("       aggregate");
    println!("            |");
    println!("         report");

    chain.add_step("fetch-data");
    chain
        .add_step("validate-data")
        .with_dependency("fetch-data");
    chain.add_step("clean-data").with_dependency("fetch-data");
    chain
        .add_step("transform-data")
        .with_dependency("validate-data");
    chain
        .add_step("aggregate-results")
        .with_dependencies(vec!["transform-data", "clean-data"]);
    chain
        .add_step("generate-report")
        .with_dependency("aggregate-results");

    let graph = chain.build_dag();
    let levels = graph.topological_sort()?;

    println!("\nExecution levels (parallelization opportunities):");
    for (level, steps) in levels.iter().enumerate() {
        println!("  Level {}: {:?}", level, steps);
    }

    let workflow = WorkflowBuilder::new()
        .with_name("complex-dag-workflow")
        .add_step(JobStep::new("fetch-data", "python:3.11-slim"))
        .add_step(JobStep::new("validate-data", "python:3.11-slim"))
        .add_step(JobStep::new("clean-data", "python:3.11-slim"))
        .add_step(JobStep::new("transform-data", "python:3.11-slim"))
        .add_step(JobStep::new("aggregate-results", "python:3.11-slim"))
        .add_step(JobStep::new("generate-report", "python:3.11-slim"))
        .build()?;

    println!("\nComplex DAG workflow created: {:?}", workflow);

    Ok(())
}

fn example_partial_parallel() -> anyhow::Result<()> {
    println!("Creating a workflow with partial parallel execution");

    let mut chain = DependencyChain::new();

    println!("Building workflow with mixed execution:");
    println!("Stage 1 (parallel): fetch-a, fetch-b, fetch-c");
    println!("Stage 2 (parallel): process-a, process-b, process-c");
    println!("Stage 3 (sequential): merge -> validate -> store");

    chain.add_step("fetch-a");
    chain.add_step("fetch-b");
    chain.add_step("fetch-c");
    chain.add_step("process-a").with_dependency("fetch-a");
    chain.add_step("process-b").with_dependency("fetch-b");
    chain.add_step("process-c").with_dependency("fetch-c");
    chain
        .add_step("merge")
        .with_dependencies(vec!["process-a", "process-b", "process-c"]);
    chain.add_step("validate").with_dependency("merge");
    chain.add_step("store").with_dependency("validate");

    let graph = chain.build_dag();
    let levels = graph.topological_sort()?;

    println!("\nExecution plan:");
    for (level, steps) in levels.iter().enumerate() {
        println!("  Stage {} (parallel): {:?}", level + 1, steps);
    }

    let workflow = WorkflowBuilder::new()
        .with_name("partial-parallel-workflow")
        .with_parallelism(3)
        .add_step(JobStep::new("fetch-a", "python:3.11-slim"))
        .add_step(JobStep::new("fetch-b", "python:3.11-slim"))
        .add_step(JobStep::new("fetch-c", "python:3.11-slim"))
        .add_step(JobStep::new("process-a", "python:3.11-slim"))
        .add_step(JobStep::new("process-b", "python:3.11-slim"))
        .add_step(JobStep::new("process-c", "python:3.11-slim"))
        .add_step(JobStep::new("merge", "python:3.11-slim"))
        .add_step(JobStep::new("validate", "python:3.11-slim"))
        .add_step(JobStep::new("store", "python:3.11-slim"))
        .build()?;

    println!("\nPartial parallel workflow created with parallelism=3");

    Ok(())
}
