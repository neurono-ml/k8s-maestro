use std::sync::Arc;

use super::condition::ConditionFn;
use super::dag::{DependencyGraph, DependencyInfo};

#[derive(Clone, Default)]
pub struct DependencyChain {
    steps: Vec<DependencyInfo>,
}

impl DependencyChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_step(&mut self, step_id: impl Into<String>) -> &mut Self {
        self.steps.push(DependencyInfo {
            step_id: step_id.into(),
            depends_on: Vec::new(),
            depends_on_any: false,
            condition: None,
        });
        self
    }

    pub fn with_dependency(&mut self, step_depends_on: impl Into<String>) -> &mut Self {
        if let Some(last_step) = self.steps.last_mut() {
            last_step.depends_on.push(step_depends_on.into());
        }
        self
    }

    pub fn with_conditional_dependency<F>(
        &mut self,
        step_depends_on: impl Into<String>,
        condition_fn: F,
    ) -> &mut Self
    where
        F: Fn(&Vec<crate::steps::result::StepResult>) -> bool + Send + Sync + 'static,
    {
        if let Some(last_step) = self.steps.last_mut() {
            last_step.depends_on.push(step_depends_on.into());
            last_step.condition = Some(Arc::new(condition_fn));
        }
        self
    }

    pub fn with_dependency_any(
        &mut self,
        step_depends_on_multiple: Vec<impl Into<String>>,
    ) -> &mut Self {
        if let Some(last_step) = self.steps.last_mut() {
            last_step.depends_on = step_depends_on_multiple
                .into_iter()
                .map(|s| s.into())
                .collect();
            last_step.depends_on_any = true;
        }
        self
    }

    pub fn with_conditional_dependency_any<F>(
        &mut self,
        step_depends_on_multiple: Vec<impl Into<String>>,
        condition_fn: F,
    ) -> &mut Self
    where
        F: Fn(&Vec<crate::steps::result::StepResult>) -> bool + Send + Sync + 'static,
    {
        if let Some(last_step) = self.steps.last_mut() {
            last_step.depends_on = step_depends_on_multiple
                .into_iter()
                .map(|s| s.into())
                .collect();
            last_step.depends_on_any = true;
            last_step.condition = Some(Arc::new(condition_fn));
        }
        self
    }

    pub fn with_condition<F>(&mut self, condition_fn: F) -> &mut Self
    where
        F: Fn(&Vec<crate::steps::result::StepResult>) -> bool + Send + Sync + 'static,
    {
        if let Some(last_step) = self.steps.last_mut() {
            last_step.condition = Some(Arc::new(condition_fn));
        }
        self
    }

    pub fn with_prebuilt_condition(&mut self, condition: ConditionFn) -> &mut Self {
        if let Some(last_step) = self.steps.last_mut() {
            last_step.condition = Some(condition);
        }
        self
    }

    pub fn build_dag(self) -> anyhow::Result<DependencyGraph> {
        let mut graph = DependencyGraph::new();

        for step_info in &self.steps {
            graph.add_node(step_info.step_id.clone());

            for dep in &step_info.depends_on {
                graph.add_edge(dep.clone(), step_info.step_id.clone());
            }

            if let Some(condition) = step_info.condition.clone() {
                graph.set_condition(step_info.step_id.clone(), condition);
            }

            if step_info.depends_on_any {
                graph.set_depends_on_any(step_info.step_id.clone(), true);
            }
        }

        graph.detect_cycles()?;

        Ok(graph)
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn get_step(&self, index: usize) -> Option<&DependencyInfo> {
        self.steps.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_chain() {
        let chain = DependencyChain::new();
        assert!(chain.is_empty());
        assert_eq!(chain.len(), 0);
    }

    #[test]
    fn test_add_step() {
        let mut chain = DependencyChain::new();
        chain.add_step("A");
        chain.add_step("B");
        assert_eq!(chain.len(), 2);
        assert!(!chain.is_empty());
    }

    #[test]
    fn test_with_dependency_simple() {
        let mut chain = DependencyChain::new();
        chain.add_step("A");
        chain.add_step("B").with_dependency("A");

        let step = chain.get_step(1).unwrap();
        assert_eq!(step.step_id, "B");
        assert_eq!(step.depends_on, vec!["A"]);
        assert!(!step.depends_on_any);
    }

    #[test]
    fn test_with_conditional_dependency() {
        let mut chain = DependencyChain::new();
        chain.add_step("A");
        chain
            .add_step("B")
            .with_conditional_dependency("A", |deps| deps.iter().all(|r| r.is_success()));

        let step = chain.get_step(1).unwrap();
        assert_eq!(step.step_id, "B");
        assert_eq!(step.depends_on, vec!["A"]);
        assert!(step.condition.is_some());
        assert!(!step.depends_on_any);
    }

    #[test]
    fn test_with_dependency_any() {
        let mut chain = DependencyChain::new();
        chain.add_step("A");
        chain.add_step("B");
        chain.add_step("C").with_dependency_any(vec!["A", "B"]);

        let step = chain.get_step(2).unwrap();
        assert_eq!(step.step_id, "C");
        assert_eq!(step.depends_on.len(), 2);
        assert!(step.depends_on.contains(&"A".to_string()));
        assert!(step.depends_on.contains(&"B".to_string()));
        assert!(step.depends_on_any);
    }

    #[test]
    fn test_build_dag_simple_chain() {
        let mut chain = DependencyChain::new();
        chain.add_step("A");
        chain.add_step("B").with_dependency("A");
        chain.add_step("C").with_dependency("B");

        let graph = chain.clone().build_dag().unwrap();
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn test_build_dag_topological_sort() {
        let mut chain = DependencyChain::new();
        chain.add_step("A");
        chain.add_step("B").with_dependency("A");
        chain.add_step("C").with_dependency("B");

        let graph = chain.clone().build_dag().unwrap();
        let levels = graph.topological_sort().unwrap();
        assert_eq!(levels, vec![vec!["A"], vec!["B"], vec!["C"]]);
    }

    #[test]
    fn test_chain_single_step() {
        let mut chain = DependencyChain::new();
        chain.add_step("A");
        let graph = chain.clone().build_dag().unwrap();
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.edges.len(), 0);
    }
}
