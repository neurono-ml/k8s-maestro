//! This example demonstrates multi-step workflows with dependency chains.
//!
//! The example shows:
//! - Creating workflows with multiple steps
//! - Setting up step dependencies (A -> B -> C)
//! - Parallel execution of independent steps
//! - Conditional execution based on step results
//! - Complex DAG (Directed Acyclic Graph) structures

use k8s_maestro::workflows::DependencyChain;

fn main() -> anyhow::Result<()> {
    println!("=== Multi-Step Workflow Examples ===\n");

    println!("Example 1: Conditional execution (B only runs if A succeeds)");
    example_conditional_workflow()?;

    println!("\n=== All examples completed successfully ===");
    Ok(())
}

fn example_conditional_workflow() -> anyhow::Result<()> {
    println!("Creating a workflow with conditional execution");

    let mut chain = DependencyChain::new();
    chain.add_step("validate-data");
    chain
        .add_step("process-data")
        .with_conditional_dependency("validate-data", |deps| deps.iter().all(|r| r.is_success()));

    println!("Conditional workflow:");
    println!("Step 1: validate-data");
    println!("Step 2: process-data (only if validate-data succeeds)");
    println!("If validation fails, processing is skipped");

    let graph = chain.build_dag()?;
    let levels = graph.topological_sort()?;
    println!("Execution levels: {:?}", levels);

    Ok(())
}
