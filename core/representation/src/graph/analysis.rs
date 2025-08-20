use super::{Graph, GraphError};
use roadline_util::task::Task;
use std::collections::{HashMap, VecDeque};

impl<'a> Graph<'a> {
    /// Detects if the graph contains cycles.
    pub fn has_cycles(&self) -> Result<bool, GraphError> {
        #[derive(Clone, Copy, PartialEq)]
        enum Color { White, Gray, Black }
        
        let mut colors: HashMap<&Task, Color> = HashMap::new();
        
        // Initialize all nodes as white
        for task in self.tasks() {
            colors.insert(task, Color::White);
        }
        
        fn dfs_cycle_check<'a>(
            graph: &Graph<'a>,
            task: &'a Task,
            colors: &mut HashMap<&'a Task, Color>
        ) -> Result<bool, GraphError> {
            colors.insert(task, Color::Gray);
            
            if let Some(predicates) = graph.facts.get(task) {
                for predicate in predicates {
                    match colors.get(&predicate.task).copied().unwrap_or(Color::White) {
                        Color::Gray => return Ok(true), // Back edge found - cycle detected
                        Color::White => {
                            if dfs_cycle_check(graph, predicate.task, colors)? {
                                return Ok(true);
                            }
                        }
                        Color::Black => {} // Already processed
                    }
                }
            }
            
            colors.insert(task, Color::Black);
            Ok(false)
        }
        
        for task in self.tasks().collect::<Vec<_>>() {
            if colors[&task] == Color::White {
                if dfs_cycle_check(self, task, &mut colors)? {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }

    /// Returns a topological ordering of tasks, or an error if cycles exist.
    pub fn topological_sort(&self) -> Result<Vec<&'a Task>, GraphError> {
        if self.has_cycles()? {
            return Err(GraphError::CycleDetected);
        }
        
        // Calculate in-degrees
        let mut in_degrees: HashMap<&Task, usize> = HashMap::new();
        for task in self.tasks() {
            in_degrees.insert(task, 0);
        }
        
        for predicates in self.facts.values() {
            for predicate in predicates {
                *in_degrees.entry(predicate.task).or_insert(0) += 1;
            }
        }
        
        // Find all nodes with no incoming edges
        let mut queue: VecDeque<&Task> = in_degrees
            .iter()
            .filter_map(|(task, &degree)| if degree == 0 { Some(*task) } else { None })
            .collect();
        
        let mut result = Vec::new();
        
        while let Some(task) = queue.pop_front() {
            result.push(task);
            
            // Remove this node from the graph and update in-degrees
            if let Some(predicates) = self.facts.get(task) {
                for predicate in predicates {
                    let in_degree = in_degrees.get_mut(&predicate.task).unwrap();
                    *in_degree -= 1;
                    if *in_degree == 0 {
                        queue.push_back(predicate.task);
                    }
                }
            }
        }
        
        Ok(result)
    }

    /// Finds strongly connected components using Tarjan's algorithm.
    pub fn strongly_connected_components(&self) -> Result<Vec<Vec<&'a Task>>, GraphError> {
        let mut index = 0;
        let mut stack = Vec::new();
        let mut indices: HashMap<&Task, usize> = HashMap::new();
        let mut lowlinks: HashMap<&Task, usize> = HashMap::new();
        let mut on_stack: HashMap<&Task, bool> = HashMap::new();
        let mut components = Vec::new();

        fn strongconnect<'a>(
            graph: &Graph<'a>,
            v: &'a Task,
            index: &mut usize,
            stack: &mut Vec<&'a Task>,
            indices: &mut HashMap<&'a Task, usize>,
            lowlinks: &mut HashMap<&'a Task, usize>,
            on_stack: &mut HashMap<&'a Task, bool>,
            components: &mut Vec<Vec<&'a Task>>,
        ) {
            // Set the depth index for v to the smallest unused index
            indices.insert(v, *index);
            lowlinks.insert(v, *index);
            *index += 1;
            stack.push(v);
            on_stack.insert(v, true);

            // Consider successors of v
            if let Some(predicates) = graph.facts.get(v) {
                for predicate in predicates {
                    let w = predicate.task;
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

        for task in self.tasks().collect::<Vec<_>>() {
            if !indices.contains_key(&task) {
                strongconnect(
                    self,
                    task,
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
    pub fn find_cycles(&self) -> Result<Vec<Vec<&'a Task>>, GraphError> {
        let components = self.strongly_connected_components()?;
        
        // A cycle exists if an SCC has more than one node or if a single node has a self-loop
        let mut cycles = Vec::new();
        
        for component in components {
            if component.len() > 1 {
                cycles.push(component);
            } else if component.len() == 1 {
                let task = component[0];
                // Check for self-loop
                if self.has_dependency(task, task) {
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
    use roadline_util::dependency::Dependency;
    use crate::graph::test_utils::*;

    #[test]
    fn test_has_cycles_acyclic() -> Result<(), anyhow::Error> {
        let frame = create_acyclic_frame()?;
        let graph = create_acyclic_graph(&frame)?;
        assert!(!graph.has_cycles()?);
        Ok(())
    }

    #[test]
    fn test_has_cycles_cyclic() -> Result<(), anyhow::Error> {
        let frame = create_cyclic_frame()?;
        let graph = create_cyclic_graph(&frame)?;
        assert!(graph.has_cycles()?);
        Ok(())
    }

    #[test]
    fn test_topological_sort_acyclic() -> Result<(), anyhow::Error> {
        let frame = create_acyclic_frame()?;
        let graph = create_acyclic_graph(&frame)?;
        let sorted = graph.topological_sort()?;
        
        assert_eq!(sorted.len(), 4);
        
        // task1 should come before task2, task3, and task4
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let task3 = Task::test_from_id_string("task3")?;
        let task4 = Task::test_from_id_string("task4")?;
        
        let task1_pos = sorted.iter().position(|&t| *t == task1).ok_or(anyhow::anyhow!("task1 should be in topological sort"))?;
        let task2_pos = sorted.iter().position(|&t| *t == task2).ok_or(anyhow::anyhow!("task2 should be in topological sort"))?;
        let task3_pos = sorted.iter().position(|&t| *t == task3).ok_or(anyhow::anyhow!("task3 should be in topological sort"))?;
        let task4_pos = sorted.iter().position(|&t| *t == task4).ok_or(anyhow::anyhow!("task4 should be in topological sort"))?;
        
        assert!(task1_pos < task2_pos);
        assert!(task1_pos < task3_pos);
        assert!(task2_pos < task4_pos);
        assert!(task3_pos < task4_pos);

        Ok(())
    }

    #[test]
    fn test_topological_sort_cyclic() -> Result<(), anyhow::Error> {
        let frame = create_cyclic_frame()?;
        let graph = create_cyclic_graph(&frame)?;
        let result = graph.topological_sort();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_strongly_connected_components_acyclic() -> Result<(), anyhow::Error> {
        let frame = create_acyclic_frame()?;
        let graph = create_acyclic_graph(&frame)?;
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
        let frame = create_cyclic_frame()?;
        let graph = create_cyclic_graph(&frame)?;
        let components = graph.strongly_connected_components()?;
        
        // Should have one component with all three tasks
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 3);

        Ok(())
    }

    #[test]
    fn test_is_dag() -> Result<(), anyhow::Error> {
        let acyclic_frame = create_acyclic_frame()?;
        let acyclic = create_acyclic_graph(&acyclic_frame)?;
        let cyclic_frame = create_cyclic_frame()?;
        let cyclic = create_cyclic_graph(&cyclic_frame)?;
        
        assert!(acyclic.is_dag()?);
        assert!(!cyclic.is_dag()?);

        Ok(())
    }

    #[test]
    fn test_find_cycles_acyclic() -> Result<(), anyhow::Error> {
        let acyclic_frame = create_acyclic_frame()?;
        let graph = create_acyclic_graph(&acyclic_frame)?;
        let cycles = graph.find_cycles()?;
        assert!(cycles.is_empty());

        Ok(())
    }

    #[test]
    fn test_find_cycles_cyclic() -> Result<(), anyhow::Error> {
        let frame = create_cyclic_frame()?;
        let graph = create_cyclic_graph(&frame)?;
        let cycles = graph.find_cycles()?;
        
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].len(), 3);

        Ok(())
    }

    #[test]
    fn test_self_loop_cycle() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task1 = Task::test_from_id_string("task1")?;
        let dep_id = Dependency::test_from_id_string("dep1")?;
        
        graph.add_dependency(&task1, &dep_id, &task1)?;
        
        assert!(graph.has_cycles().unwrap());
        let cycles = graph.find_cycles().unwrap();
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].len(), 1);
        assert_eq!(cycles[0][0], &task1);

        Ok(())
    }
}
