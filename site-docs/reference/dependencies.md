# Dependency System

The dependency system provides a fluent API for defining step dependencies in workflows, supporting complex dependency graphs with conditions, parallel execution, and cycle detection.

## Overview

The dependency system enables workflow orchestration by defining how steps depend on each other. It supports:

- **Linear dependencies** - Step B depends on Step A
- **Any dependencies** - Step C depends on any of [Step A, Step B]
- **Conditional dependencies** - Step B depends on Step A, but only executes if a condition is met
- **Cycle detection** - Automatically detects circular dependency errors

Key components:
- `DependencyChain` - Builder for creating dependencies
- `DependencyGraph` - DAG representation with operations
- `ConditionBuilder` - Pre-built condition functions

## DependencyChain

Builder for creating step dependencies with a fluent API.

```rust
use k8s_maestro::workflows::dependency::{DependencyChain, ConditionBuilder};
```

### Methods

| Method | Description |
|--------|-------------|
| `new()` | Create a new empty chain |
| `add_step(id)` | Add a step to the chain |
| `with_dependency(id)` | Make last step depend on another step |
| `with_dependency_any(vec)` | Make last step depend on any step in list |
| `with_conditional_dependency(id, condition)` | Add dependency with condition |
| `with_conditional_dependency_any(vec, condition)` | Add any-dependency with condition |
| `with_condition(condition)` | Add condition to last step |
| `with_prebuilt_condition(condition)` | Add pre-built condition to last step |
| `build_dag()` | Build the dependency graph |
| `len()` | Get number of steps |
| `get_step(index)` | Get step by index |

### Usage

```rust
let mut chain = DependencyChain::new();
chain.add_step("A");
chain.add_step("B").with_dependency("A");
chain.add_step("C").with_dependency("B");

let graph = chain.build_dag()?;
```

## DependencyGraph

DAG representation with operations for analyzing and executing dependencies.

### Methods

| Method | Description |
|--------|-------------|
| `new()` | Create a new empty graph |
| `add_node(step_id)` | Add a node to the graph |
| `add_edge(from, to)` | Add an edge (dependency) |
| `set_condition(step_id, condition)` | Set condition for a step |
| `set_depends_on_any(step_id, bool)` | Set any-dependency mode |
| `topological_sort()` | Get execution levels |
| `get_execution_levels()` | Alias for topological_sort |
| `detect_cycles()` | Detect circular dependencies |
| `get_dependencies(step_id)` | Get steps that this step depends on |
| `get_dependents(step_id)` | Get steps that depend on this step |
| `get_condition(step_id)` | Get condition for a step |
| `is_depends_on_any(step_id)` | Check if step uses any-dependency |

### Return Types

- `topological_sort()` returns `Result<Vec<Vec<StepId>>>` - execution levels where each level contains steps that can execute in parallel
- `get_dependencies()` returns `Vec<StepId>` - direct dependencies
- `get_dependents()` returns `Vec<StepId>` - direct dependents

### Example

```rust
use k8s_maestro::workflows::dependency::DependencyGraph;

let mut graph = DependencyGraph::new();
graph.add_node("A".to_string());
graph.add_node("B".to_string());
graph.add_node("C".to_string());
graph.add_edge("A".to_string(), "B".to_string());
graph.add_edge("B".to_string(), "C".to_string());

let levels = graph.topological_sort()?;
assert_eq!(levels, vec![vec!["A"], vec!["B"], vec!["C"]]);
```

## ConditionBuilder

Pre-built conditions for common dependency patterns.

```rust
use k8s_maestro::workflows::dependency::ConditionBuilder;
```

### Methods

| Method | Description |
|--------|-------------|
| `all_success()` | Execute when all dependencies succeed |
| `any_success()` | Execute when any dependency succeeds |
| `all_failure()` | Execute when all dependencies fail |
| `any_failure()` | Execute when any dependency fails |
| `output_greater_than(key, threshold)` | Execute when output value > threshold |
| `output_equals(key, value)` | Execute when output equals value |
| `exit_code_equals(code)` | Execute when exit code equals code |
| `always_execute()` | Always execute (no condition) |
| `never_execute()` | Never execute |
| `custom(fn)` | Custom condition function |
| `and(vec)` | Logical AND of conditions |
| `or(vec)` | Logical OR of conditions |
| `not(condition)` | Logical NOT of condition |

### Condition Function Type

```rust
type ConditionFn = Arc<dyn Fn(&Vec<StepResult>) -> bool + Send + Sync>;
```

The condition function receives dependency results and returns whether to execute.

## Usage Examples

### Example 1: Simple Linear Dependency (A -> B -> C)

```rust
use k8s_maestro::workflows::dependency::DependencyChain;

let mut chain = DependencyChain::new();
chain.add_step("A");
chain.add_step("B").with_dependency("A");
chain.add_step("C").with_dependency("B");

let graph = chain.build_dag()?;
let levels = graph.topological_sort()?;

assert_eq!(levels.len(), 3);
assert_eq!(levels[0], vec!["A"]);
assert_eq!(levels[1], vec!["B"]);
assert_eq!(levels[2], vec!["C"]);
```

### Example 2: Parallel Branches with Join

Diamond pattern where A branches to B and C, both joining at D:

```rust
use k8s_maestro::workflows::dependency::DependencyChain;

let mut chain = DependencyChain::new();
chain.add_step("A");
chain.add_step("B").with_dependency("A");
chain.add_step("C").with_dependency("A");
chain.add_step("D").with_dependency_any(vec!["B", "C"]);

let graph = chain.build_dag()?;
let levels = graph.topological_sort()?;

assert_eq!(levels.len(), 3);
assert_eq!(levels[0], vec!["A"]);
assert_eq!(levels[1].len(), 2); // B and C in parallel
assert_eq!(levels[2], vec!["D"]);
```

### Example 3: Conditional Dependency Based on Output

Execute step B only if step A produces output greater than 1000:

```rust
use k8s_maestro::workflows::dependency::{DependencyChain, ConditionBuilder};
use k8s_maestro::steps::result::StepResult;

let mut chain = DependencyChain::new();
chain.add_step("A");
chain.add_step("B")
    .with_conditional_dependency("A", ConditionBuilder::output_greater_than("data_size", 1000));

let graph = chain.build_dag()?;

// Check the condition
let condition = graph.get_condition("B").unwrap();
let deps = vec![StepResult::new("A")
    .with_output("data_size", serde_json::json!(1500))];
assert!(condition(&deps));
```

### Example 4: Using ConditionBuilder

Combine multiple conditions using `and()`, `or()`, and `not()`:

```rust
use k8s_maestro::workflows::dependency::{DependencyChain, ConditionBuilder};
use k8s_maestro::steps::result::{StepResult, StepStatus};

let all_success = ConditionBuilder::all_success();
let any_failure = ConditionBuilder::any_failure();

// AND: both conditions must be true
let combined = ConditionBuilder::and(vec![all_success, any_failure]);

let deps = vec![
    StepResult::new("A").with_status(StepStatus::Success),
    StepResult::new("B").with_status(StepStatus::Failure),
];

// all_success: false (B failed)
// any_failure: true (B failed)
// and([all_success, any_failure]): false
assert!(!combined(&deps));

// OR: at least one condition must be true
let or_combined = ConditionBuilder::or(vec![all_success, any_failure]);
assert!(or_combined(&deps));

// NOT: invert condition
let not_any_failure = ConditionBuilder::not(any_failure);
assert!(!not_any_failure(&deps));
```

### Example 5: Custom Condition

```rust
use k8s_maestro::workflows::dependency::{DependencyChain, ConditionBuilder};
use k8s_maestro::steps::result::StepResult;

let custom = ConditionBuilder::custom(|deps| {
    deps.iter().any(|r| r.exit_code == 0) && deps.len() >= 2
});

let deps = vec![
    StepResult::new("A").with_exit_code(0),
    StepResult::new("B").with_exit_code(0),
];

assert!(custom(&deps));
```

### Example 6: Complex Workflow with Multiple Conditions

```rust
use k8s_maestro::workflows::dependency::{DependencyChain, ConditionBuilder};

let condition = ConditionBuilder::output_greater_than("record_count", 100);

let mut chain = DependencyChain::new();
chain.add_step("initialize");
chain.add_step("process")
    .with_dependency("initialize")
    .with_conditional_dependency("validate", condition.clone());
chain.add_step("cleanup").with_dependency("process");

let graph = chain.build_dag()?;
let levels = graph.topological_sort()?;

assert_eq!(levels.len(), 3);
```

## Error Handling

### Cycle Detection

The dependency system automatically detects circular dependencies when building the DAG:

```rust
use k8s_maestro::workflows::dependency::DependencyChain;

let mut chain = DependencyChain::new();
chain.add_step("A").with_dependency("C");
chain.add_step("B").with_dependency("A");
chain.add_step("C").with_dependency("B");

let result = chain.build_dag();
assert!(result.is_err());

if let Err(e) = result {
    assert!(e.to_string().contains("cycle"));
}
```

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| "cycle detected" | Circular dependency | Check dependency chain for loops |
| "step not found" | Missing dependency | Ensure all dependended steps exist |
| "duplicate step" | Same step added twice | Use unique step IDs |

### Best Practices

1. **Always handle cycle detection errors**:

```rust
match chain.build_dag() {
    Ok(graph) => { /* use graph */ }
    Err(e) => eprintln!("Dependency error: {}", e),
}
```

2. **Validate conditions before execution**:

```rust
if let Some(condition) = graph.get_condition(step_id) {
    let should_execute = condition(&dependency_results);
    if !should_execute {
        skip_step();
    }
}
```

3. **Use meaningful step IDs**:

```rust
// Good
chain.add_step("fetch-user-data");
chain.add_step("process-user-data").with_dependency("fetch-user-data");

// Avoid
chain.add_step("a");
chain.add_step("b").with_dependency("a");
```