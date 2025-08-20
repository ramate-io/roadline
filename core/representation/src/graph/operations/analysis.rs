use crate::graph::{Graph, GraphError};
use roadline_util::task::Id as TaskId;
use std::collections::{HashMap, VecDeque};

impl Graph {
    /// Detects if the graph contains cycles.
    pub fn has_cycles(&self) -> Result<bool, GraphError> {
        #[derive(Clone, Copy, PartialEq)]
        enum Color { White, Gray, Black }
        
        let mut colors: HashMap<TaskId, Color> = HashMap::new();
        
        // Initialize all nodes as white
        for  task_id in self.task_ids() {
            colors.insert(*task_id, Color::White);
        }
        
        fn dfs_cycle_check(
            graph: &Graph,
            task_id: TaskId,
            colors: &mut HashMap<TaskId, Color>
        ) -> Result<bool, GraphError> {
            colors.insert(task_id, Color::Gray);
            
            if let Some(predicates) = graph.facts.get(&task_id) {
                for predicate in predicates {
                    match colors.get(&predicate.task_id).copied().unwrap_or(Color::White) {
                        Color::Gray => return Ok(true), // Back edge found - cycle detected
                        Color::White => {
                            if dfs_cycle_check(graph, predicate.task_id, colors)? {
                                return Ok(true);
                            }
                        }
                        Color::Black => {} // Already processed
                    }
                }
            }
            
            colors.insert(task_id, Color::Black);
            Ok(false)
        }
        
        for  task_id in self.task_ids().copied().collect::<Vec<_>>() {
            if colors[&task_id] == Color::White {
                if dfs_cycle_check(self, task_id, &mut colors)? {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }

    /// Returns a topological ordering of tasks, or an error if cycles exist.
    pub fn topological_sort(&self) -> Result<Vec<TaskId>, GraphError> {
        if self.has_cycles()? {
            return Err(GraphError::Internal(
                "Cannot perform topological sort on graph with cycles".into()
            ));
        }
        
        // Calculate in-degrees
        let mut in_degrees: HashMap<TaskId, usize> = HashMap::new();
        for  task_id in self.task_ids() {
            in_degrees.insert(*task_id, 0);
        }
        
        for predicates in self.facts.values() {
            for predicate in predicates {
                *in_degrees.entry(predicate.task_id).or_insert(0) += 1;
            }
        }
        
        // Find all nodes with no incoming edges
        let mut queue: VecDeque<TaskId> = in_degrees
            .iter()
            .filter_map(|(task_id, &degree)| if degree == 0 { Some(*task_id) } else { None })
            .collect();
        
        let mut result = Vec::new();
        
        while let Some(task_id) = queue.pop_front() {
            result.push(task_id);
            
            // Remove this node from the graph and update in-degrees
            if let Some(predicates) = self.facts.get(&task_id) {
                for predicate in predicates {
                    let in_degree = in_degrees.get_mut(&predicate.task_id).unwrap();
                    *in_degree -= 1;
                    if *in_degree == 0 {
                        queue.push_back(predicate.task_id);
                    }
                }
            }
        }
        
        Ok(result)
    }

    /// Finds strongly connected components using Tarjan's algorithm.
    pub fn strongly_connected_components(&self) -> Result<Vec<Vec<TaskId>>, GraphError> {
        let mut index = 0;
        let mut stack = Vec::new();
        let mut indices: HashMap<TaskId, usize> = HashMap::new();
        let mut lowlinks: HashMap<TaskId, usize> = HashMap::new();
        let mut on_stack: HashMap<TaskId, bool> = HashMap::new();
        let mut components = Vec::new();

        fn strongconnect(
            graph: &Graph,
            v: TaskId,
            index: &mut usize,
            stack: &mut Vec<TaskId>,
            indices: &mut HashMap<TaskId, usize>,
            lowlinks: &mut HashMap<TaskId, usize>,
            on_stack: &mut HashMap<TaskId, bool>,
            components: &mut Vec<Vec<TaskId>>,
        ) {
            // Set the depth index for v to the smallest unused index
            indices.insert(v, *index);
            lowlinks.insert(v, *index);
            *index += 1;
            stack.push(v);
            on_stack.insert(v, true);

            // Consider successors of v
            if let Some(predicates) = graph.facts.get(&v) {
                for predicate in predicates {
                    let w = predicate.task_id;
                    if !indices.contains_key(&w) {
                        // Successor w has not yet been visited; recurse on it
                        strongconnect(graph, w, index, stack, indices, lowlinks, on_stack, components);
                        lowlinks.insert(v, lowlinks[&v].min(lowlinks[&w]));
                    } else if on_stack.get(&w).copied().unwrap_or(false) {
                        // Successor w is in stack and hence in the current SCC
                        lowlinks.insert(v, lowlinks[&v].min(indices[&w]));
                    }
                }
            }

            // If v is a root node, pop the stack and print an SCC
            if lowlinks[&v] == indices[&v] {
                let mut component = Vec::new();
                loop {
                    let w = stack.pop().unwrap();
                    on_stack.insert(w, false);
                    component.push(w);
                    if w == v {
                        break;
                    }
                }
                components.push(component);
            }
        }

        for  task_id in self.task_ids().copied().collect::<Vec<_>>() {
            if !indices.contains_key(&task_id) {
                strongconnect(
                    self,
                    task_id,
                    &mut index,
                    &mut stack,
                    &mut indices,
                    &mut lowlinks,
                    &mut on_stack,
                    &mut components,
                );
            }
        }

        Ok(components)
    }

    /// Checks if the graph is a DAG (Directed Acyclic Graph).
    pub fn is_dag(&self) -> Result<bool, GraphError> {
        Ok(!self.has_cycles()?)
    }

    /// Finds all cycles in the graph.
    pub fn find_cycles(&self) -> Result<Vec<Vec<TaskId>>, GraphError> {
        let components = self.strongly_connected_components()?;
        
        // A cycle exists if an SCC has more than one node or if a single node has a self-loop
        let mut cycles = Vec::new();
        
        for component in components {
            if component.len() > 1 {
                cycles.push(component);
            } else if component.len() == 1 {
                let task = component[0];
                // Check for self-loop
                if self.has_dependency(&task, &task) {
                    cycles.push(component);
                }
            }
        }
        
        Ok(cycles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roadline_util::dependency::id::Id as DependencyId;
    use crate::graph::operations::test_utils::*;

    #[test]
    fn test_has_cycles_acyclic() -> Result<(), anyhow::Error> {
        let graph = create_acyclic_graph()?;
        assert!(!graph.has_cycles()?);
        Ok(())
    }

    #[test]
    fn test_has_cycles_cyclic() -> Result<(), anyhow::Error> {
        let graph = create_cyclic_graph()?;
        assert!(graph.has_cycles()?);
        Ok(())
    }

    #[test]
    fn test_topological_sort_acyclic() -> Result<(), anyhow::Error> {
        let graph = create_acyclic_graph()?;
        let sorted = graph.topological_sort()?;
        
        assert_eq!(sorted.len(), 4);
        
        // task1 should come before task2, task3, and task4
        let task1 = TaskId::new(1);
        let task2 = TaskId::new(2);
        let task3 = TaskId::new(3);
        let task4 = TaskId::new(4);
        
        let task1_pos = sorted.iter().position(|&t| t == task1).ok_or(anyhow::anyhow!("task1 should be in topological sort"))?;
        let task2_pos = sorted.iter().position(|&t| t == task2).ok_or(anyhow::anyhow!("task2 should be in topological sort"))?;
        let task3_pos = sorted.iter().position(|&t| t == task3).ok_or(anyhow::anyhow!("task3 should be in topological sort"))?;
        let task4_pos = sorted.iter().position(|&t| t == task4).ok_or(anyhow::anyhow!("task4 should be in topological sort"))?;
        
        assert!(task1_pos < task2_pos);
        assert!(task1_pos < task3_pos);
        assert!(task2_pos < task4_pos);
        assert!(task3_pos < task4_pos);

        Ok(())
    }

    #[test]
    fn test_topological_sort_cyclic() -> Result<(), anyhow::Error> {
        let graph = create_cyclic_graph()?;
        let result = graph.topological_sort();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_strongly_connected_components_acyclic() -> Result<(), anyhow::Error> {
        let graph = create_acyclic_graph()?;
        let components = graph.strongly_connected_components()?;
        
        // In a DAG, each node should be its own SCC
        assert_eq!(components.len(), 4);
        for component in &components {
            assert_eq!(component.len(), 1);
        }

        Ok(())
    }

    #[test]
    fn test_strongly_connected_components_cyclic() -> Result<(), anyhow::Error> {
        let graph = create_cyclic_graph()?;
        let components = graph.strongly_connected_components()?;
        
        // Should have one component with all three tasks
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 3);

        Ok(())
    }

    #[test]
    fn test_is_dag() -> Result<(), anyhow::Error> {
        let acyclic = create_acyclic_graph()?;
        let cyclic = create_cyclic_graph()?;
        
        assert!(acyclic.is_dag().unwrap());
        assert!(!cyclic.is_dag().unwrap());

        Ok(())
    }

    #[test]
    fn test_find_cycles_acyclic() -> Result<(), anyhow::Error> {
        let graph = create_acyclic_graph()?;
        let cycles = graph.find_cycles()?;
        assert!(cycles.is_empty());

        Ok(())
    }

    #[test]
    fn test_find_cycles_cyclic() -> Result<(), anyhow::Error> {
        let graph = create_cyclic_graph()?;
        let cycles = graph.find_cycles()?;
        
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].len(), 3);

        Ok(())
    }

    #[test]
    fn test_self_loop_cycle() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task1 = TaskId::new(1);
        let dependency_id = DependencyId::new(1);
        
        graph.add_dependency(task1, dependency_id, task1)?;
        
        assert!(graph.has_cycles().unwrap());
        let cycles = graph.find_cycles().unwrap();
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].len(), 1);
        assert_eq!(cycles[0][0], task1);

        Ok(())
    }
}
