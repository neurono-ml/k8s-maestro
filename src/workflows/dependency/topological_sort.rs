use std::collections::{HashMap, HashSet, VecDeque};

pub fn topological_sort(graph: &super::dag::DependencyGraph) -> anyhow::Result<Vec<Vec<String>>> {
    detect_cycles(graph)?;

    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

    for node in &graph.nodes {
        in_degree.insert(node.clone(), 0);
        adjacency.insert(node.clone(), Vec::new());
    }

    for (from, to) in &graph.edges {
        if let Some(adj_list) = adjacency.get_mut(from) {
            adj_list.push(to.clone());
        }
        if let Some(degree) = in_degree.get_mut(to) {
            *degree += 1;
        }
    }

    let mut queue: VecDeque<String> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(node, _)| node.clone())
        .collect();

    let mut levels = Vec::new();

    while !queue.is_empty() {
        let current_level = std::mem::take(&mut queue);
        levels.push(current_level.clone());

        for node in current_level {
            if let Some(neighbors) = adjacency.get(&node) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }
    }

    if levels.iter().flatten().count() != graph.nodes.len() {
        return Err(anyhow::anyhow!("Graph has cycles, cannot perform topological sort"));
    }

    Ok(levels)
}

pub fn detect_cycles(graph: &super::dag::DependencyGraph) -> anyhow::Result<()> {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    for node in &graph.nodes {
        if !visited.contains(node) {
            if dfs_cycle_detect(graph, node, &mut visited, &mut rec_stack)? {
                return Err(anyhow::anyhow!("Graph contains a cycle involving node: {}", node));
            }
        }
    }

    Ok(())
}

fn dfs_cycle_detect(
    graph: &super::dag::DependencyGraph,
    node: &str,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
) -> anyhow::Result<bool> {
    visited.insert(node.to_string());
    rec_stack.insert(node.to_string());

    let neighbors: Vec<&String> = graph
        .edges
        .iter()
        .filter(|(from, _)| *from == node)
        .map(|(_, to)| to)
        .collect();

    for neighbor in neighbors {
        if !visited.contains(neighbor) {
            if dfs_cycle_detect(graph, neighbor, visited, rec_stack)? {
                return Ok(true);
            }
        } else if rec_stack.contains(neighbor) {
            return Ok(true);
        }
    }

    rec_stack.remove(node);
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_graph(nodes: Vec<&str>, edges: Vec<(&str, &str)>) -> super::super::dag::DependencyGraph {
        super::super::dag::DependencyGraph {
            nodes: nodes.into_iter().map(String::from).collect(),
            edges: edges
                .into_iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect(),
            conditions: std::collections::BTreeMap::new(),
            depends_on_any: std::collections::BTreeMap::new(),
        }
    }

    #[test]
    fn test_topological_sort_simple_chain() {
        let graph = create_graph(vec!["A", "B", "C"], vec![("A", "B"), ("B", "C")]);
        let levels = topological_sort(&graph).unwrap();
        assert_eq!(levels, vec![vec!["A"], vec!["B"], vec!["C"]]);
    }

    #[test]
    fn test_topological_sort_parallel() {
        let graph = create_graph(
            vec!["A", "B", "C", "D"],
            vec![("A", "D"), ("B", "D"), ("C", "D")],
        );
        let levels = topological_sort(&graph).unwrap();
        assert_eq!(levels.len(), 2);
    }

    #[test]
    fn test_cycle_detection_simple_cycle() {
        let graph = create_graph(vec!["A", "B", "C"], vec![("A", "B"), ("B", "C"), ("C", "A")]);
        assert!(detect_cycles(&graph).is_err());
    }

    #[test]
    fn test_cycle_detection_no_cycle() {
        let graph = create_graph(vec!["A", "B", "C"], vec![("A", "B"), ("B", "C")]);
        assert!(detect_cycles(&graph).is_ok());
    }
}
