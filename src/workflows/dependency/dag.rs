use std::collections::BTreeMap;

use super::condition::ConditionFn;
use super::topological_sort;

pub type StepId = String;

#[derive(Clone)]
pub struct DependencyInfo {
    pub step_id: StepId,
    pub depends_on: Vec<StepId>,
    pub depends_on_any: bool,
    pub condition: Option<ConditionFn>,
}

#[derive(Clone, Default)]
pub struct DependencyGraph {
    pub nodes: Vec<StepId>,
    pub edges: Vec<(StepId, StepId)>,
    pub conditions: BTreeMap<StepId, ConditionFn>,
    pub depends_on_any: BTreeMap<StepId, bool>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, step_id: StepId) {
        if !self.nodes.contains(&step_id) {
            self.nodes.push(step_id);
        }
    }

    pub fn add_edge(&mut self, from: StepId, to: StepId) {
        if !self.edges.iter().any(|(f, t)| f == &from && t == &to) {
            self.edges.push((from, to));
        }
    }

    pub fn set_condition(&mut self, step_id: StepId, condition: ConditionFn) {
        self.conditions.insert(step_id, condition);
    }

    pub fn set_depends_on_any(&mut self, step_id: StepId, depends_on_any: bool) {
        self.depends_on_any.insert(step_id, depends_on_any);
    }

    pub fn topological_sort(&self) -> anyhow::Result<Vec<Vec<StepId>>> {
        topological_sort::topological_sort(self)
    }

    pub fn get_execution_levels(&self) -> anyhow::Result<Vec<Vec<StepId>>> {
        self.topological_sort()
    }

    pub fn detect_cycles(&self) -> anyhow::Result<()> {
        topological_sort::detect_cycles(self)
    }

    pub fn get_dependencies(&self, step_id: &str) -> Vec<StepId> {
        self.edges
            .iter()
            .filter(|(_, to)| *to == step_id)
            .map(|(from, _)| from.clone())
            .collect()
    }

    pub fn get_dependents(&self, step_id: &str) -> Vec<StepId> {
        self.edges
            .iter()
            .filter(|(from, _)| *from == step_id)
            .map(|(_, to)| to.clone())
            .collect()
    }

    pub fn get_condition(&self, step_id: &str) -> Option<&ConditionFn> {
        self.conditions.get(step_id)
    }

    pub fn is_depends_on_any(&self, step_id: &str) -> bool {
        self.depends_on_any.get(step_id).copied().unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_graph_from_edges(nodes: Vec<&str>, edges: Vec<(&str, &str)>) -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        for node in nodes {
            graph.add_node(node.to_string());
        }
        for (from, to) in edges {
            graph.add_edge(from.to_string(), to.to_string());
        }
        graph
    }

    #[test]
    fn test_add_node() {
        let mut graph = DependencyGraph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        assert_eq!(graph.nodes.len(), 2);
    }

    #[test]
    fn test_topological_sort_simple_chain() {
        let graph = create_graph_from_edges(vec!["A", "B", "C"], vec![("A", "B"), ("B", "C")]);
        let levels = graph.topological_sort().unwrap();
        assert_eq!(levels, vec![vec!["A"], vec!["B"], vec!["C"]]);
    }

    #[test]
    fn test_cycle_detection_no_cycle() {
        let graph = create_graph_from_edges(vec!["A", "B", "C"], vec![("A", "B"), ("B", "C")]);
        assert!(graph.detect_cycles().is_ok());
    }

    #[test]
    fn test_diamond_pattern() {
        let graph = create_graph_from_edges(
            vec!["A", "B", "C", "D"],
            vec![("A", "B"), ("A", "C"), ("B", "D"), ("C", "D")],
        );
        let levels = graph.topological_sort().unwrap();
        assert_eq!(levels.len(), 3);
        assert_eq!(levels[0].len(), 1);
        assert_eq!(levels[1].len(), 2);
        assert_eq!(levels[2].len(), 1);
    }

    #[test]
    fn test_parallel_starts() {
        let graph = create_graph_from_edges(vec!["A", "B", "C"], vec![]);
        let levels = graph.topological_sort().unwrap();
        assert_eq!(levels.len(), 1);
        assert_eq!(levels[0].len(), 3);
    }

    #[test]
    fn test_disconnected_graphs() {
        let graph = create_graph_from_edges(vec!["A", "B", "C", "D"], vec![("A", "B"), ("C", "D")]);
        let levels = graph.topological_sort().unwrap();
        assert_eq!(levels.len(), 2);
    }

    #[test]
    fn test_get_execution_levels() {
        let graph = create_graph_from_edges(vec!["A", "B", "C"], vec![("A", "B"), ("B", "C")]);
        let levels = graph.get_execution_levels().unwrap();
        assert_eq!(levels, vec![vec!["A"], vec!["B"], vec!["C"]]);
    }
}
