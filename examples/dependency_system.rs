// Example demonstrating the dependency chain system with conditional execution

use k8s_maestro::{
    ChainBuilder, ConditionBuilder, DependencyChain, DependencyGraph, StepResult, StepStatus,
};
use serde_json::json;

fn main() {
    println!("=== Dependency Chain System Example ===\n");

    // Example 1: Simple chain A -> B -> C
    println!("1. Simple Chain (A -> B -> C)");
    let mut chain = DependencyChain::new();
    chain.add_step("A");
    chain.add_step("B").with_dependency("A");
    chain.add_step("C").with_dependency("B");

    let graph = chain.clone().build_dag();
    let levels = graph.topological_sort().unwrap();
    println!("   Execution levels: {:?}", levels);
    println!("   Dependencies of C: {:?}", graph.get_dependencies("C"));
    println!();

    // Example 2: Parallel dependencies (A, B, C -> D)
    println!("2. Parallel Dependencies (A, B, C -> D)");
    let mut chain = DependencyChain::new();
    chain.add_step("A");
    chain.add_step("B");
    chain.add_step("C");
    chain.add_step("D").with_dependency_any(vec!["A", "B", "C"]);

    let graph = chain.clone().build_dag();
    let levels = graph.topological_sort().unwrap();
    println!("   Execution levels: {:?}", levels);
    println!("   D depends on ANY of: {:?}", graph.get_dependencies("D"));
    println!("   D is_depends_on_any: {}", graph.is_depends_on_any("D"));
    println!();

    // Example 3: Conditional dependency - execute only if all dependencies succeed
    println!("3. Conditional Dependency (all_success)");
    let mut chain = DependencyChain::new();
    chain.add_step("fetch-data");
    chain
        .add_step("process-data")
        .with_conditional_dependency("fetch-data", ConditionBuilder::all_success());

    let graph = chain.clone().build_dag();
    println!(
        "   Execution levels: {:?}",
        graph.topological_sort().unwrap()
    );
    println!(
        "   Has condition: {:?}",
        graph.get_condition("process-data").is_some()
    );
    println!();

    // Example 4: Conditional dependency with output check
    println!("4. Conditional Dependency with Output Check");
    let mut chain = DependencyChain::new();
    chain.add_step("generate-report");
    chain
        .add_step("analyze-report")
        .with_conditional_dependency(
            "generate-report",
            ConditionBuilder::output_greater_than("report_size", 1000),
        );

    let graph = chain.clone().build_dag();
    println!(
        "   Execution levels: {:?}",
        graph.topological_sort().unwrap()
    );
    println!();

    // Example 5: Conditional dependency_any - execute if ANY dependency succeeds
    println!("5. Conditional Dependency Any (any_success)");
    let mut chain = DependencyChain::new();
    chain.add_step("primary-db");
    chain.add_step("backup-db");
    chain
        .add_step("query-data")
        .with_conditional_dependency_any(
            vec!["primary-db", "backup-db"],
            ConditionBuilder::any_success(),
        );

    let graph = chain.clone().build_dag();
    println!(
        "   Execution levels: {:?}",
        graph.topological_sort().unwrap()
    );
    println!(
        "   query-data dependencies: {:?}",
        graph.get_dependencies("query-data")
    );
    println!();

    // Example 6: Complex DAG with multiple conditions
    println!("6. Complex DAG with Multiple Conditions");
    let mut chain = DependencyChain::new();
    chain.add_step("fetch-data");
    chain
        .add_step("validate-data")
        .with_dependency("fetch-data");
    chain
        .add_step("transform-data")
        .with_conditional_dependency("validate-data", ConditionBuilder::all_success());
    chain
        .add_step("backup-data")
        .with_conditional_dependency_any(
            vec!["validate-data", "transform-data"],
            ConditionBuilder::any_success(),
        );
    chain
        .add_step("archive-data")
        .with_dependency("transform-data");

    let graph = chain.clone().build_dag();
    let levels = graph.topological_sort().unwrap();
    println!("   Execution levels: {:?}", levels);
    println!();

    // Example 7: Cycle detection
    println!("7. Cycle Detection");
    let mut chain = DependencyChain::new();
    chain.add_step("A");
    chain.add_step("B").with_dependency("A");
    chain.add_step("C").with_dependency("B");
    chain.add_step("A").with_dependency("C"); // Creates a cycle!

    let graph = chain.clone().build_dag();
    match graph.detect_cycles() {
        Ok(()) => println!("   No cycles detected"),
        Err(e) => println!("   Cycle detected: {}", e),
    }
    println!();

    // Example 8: Getting ready steps based on completion
    println!("8. Getting Ready Steps");
    let mut chain = DependencyChain::new();
    chain.add_step("A");
    chain.add_step("B").with_dependency("A");
    chain.add_step("C").with_dependency("A");
    chain.add_step("D").with_dependency_any(vec!["B", "C"]);

    let graph = chain.clone().build_dag();

    let ready = graph.get_ready_steps(&vec![]);
    println!("   Ready steps (none completed): {:?}", ready);

    let ready = graph.get_ready_steps(&vec!["A".to_string()]);
    println!("   Ready steps (A completed): {:?}", ready);

    let ready = graph.get_ready_steps(&vec!["A".to_string(), "B".to_string()]);
    println!("   Ready steps (A, B completed): {:?}", ready);
    println!();

    // Example 9: Complex condition using custom closures
    println!("9. Custom Condition with Complex Logic");
    let mut chain = DependencyChain::new();
    chain.add_step("collect-metrics");
    chain
        .add_step("analyze-metrics")
        .with_conditional_dependency(
            "collect-metrics",
            ConditionBuilder::custom(|deps| {
                let total: i64 = deps
                    .iter()
                    .filter_map(|r| r.get_output_value("total_count").and_then(|v| v.as_i64()))
                    .sum();
                total > 100 && deps.iter().all(|r| r.is_success())
            }),
        );

    let graph = chain.clone().build_dag();
    println!(
        "   Execution levels: {:?}",
        graph.topological_sort().unwrap()
    );
    println!();

    // Example 10: Combined conditions (AND/OR/NOT)
    println!("10. Combined Conditions (AND/OR/NOT)");
    let mut chain = DependencyChain::new();
    chain.add_step("step1");
    chain.add_step("step2");
    chain.add_step("step3").with_conditional_dependency(
        "step1",
        ConditionBuilder::and(vec![
            ConditionBuilder::all_success(),
            ConditionBuilder::not(ConditionBuilder::any_failure()),
        ]),
    );

    let graph = chain.clone().build_dag();
    println!(
        "   Execution levels: {:?}",
        graph.topological_sort().unwrap()
    );
    println!();

    println!("=== All Examples Completed ===");
}
