use super::{Graph, GraphError, Predicate};
use roadline_util::task::Task;
use roadline_util::dependency::Dependency;

impl<'a> Graph<'a> {
    /// Adds a task to the graph with no dependencies.
    /// 
    /// This establishes the task as a node in the graph structure without
    /// any outgoing relationships. The task reference must live at least
    /// as long as the graph.
    pub fn add_task(&mut self, task: &'a Task) {
        self.facts.entry(task).or_insert_with(Vec::new);
    }

    /// Adds a dependency relationship between two tasks.
    /// 
    /// Creates a directed edge from `from_task` to `to_task` via the given
    /// `dependency`. This represents that `from_task` depends on `to_task`
    /// through the specified dependency relationship.
    /// 
    /// # Arguments
    /// * `from_task` - The task that has the dependency
    /// * `dependency` - The dependency that defines the relationship semantics
    /// * `to_task` - The task that is depended upon
    pub fn add_dependency(
        &mut self, 
        from_task: &'a Task, 
        dependency: &'a Dependency, 
        to_task: &'a Task
    ) -> Result<(), GraphError> {
        let predicate = Predicate {
            dependency,
            task: to_task,
        };
        
        self.facts.entry(from_task)
            .or_insert_with(Vec::new)
            .push(predicate);
        
        // Ensure the target task exists in the graph
        self.facts.entry(to_task).or_insert_with(Vec::new);
        
        Ok(())
    }

    /// Removes a task and all its dependencies from the graph.
    /// 
    /// This removes the task as a subject (source of dependencies) and also
    /// removes all references to this task from other tasks' predicates.
    /// The graph structure is updated to maintain consistency.
    pub fn remove_task(&mut self, task: &Task) -> Result<bool, GraphError> {
        // Remove the task itself
        let removed = self.facts.remove(task).is_some();
        
        // Remove all references to this task from other tasks' predicates
        for predicates in self.facts.values_mut() {
            predicates.retain(|predicate| predicate.task != task);
        }
        
        Ok(removed)
    }

    /// Removes a specific dependency between two tasks.
    /// 
    /// Removes only the specific dependency relationship identified by the
    /// exact dependency object and target task. Other relationships between
    /// the same tasks (via different dependencies) are preserved.
    pub fn remove_dependency(
        &mut self, 
        from_task: &Task, 
        dependency: &Dependency, 
        to_task: &Task
    ) -> Result<bool, GraphError> {
        if let Some(predicates) = self.facts.get_mut(from_task) {
            let initial_len = predicates.len();
            predicates.retain(|predicate| {
                !(predicate.dependency == dependency && predicate.task == to_task)
            });
            Ok(initial_len != predicates.len())
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task = Task::test_from_id_string("task1")?;
        
        graph.add_task(&task);
        
        assert!(graph.contains_task(&task));
        assert_eq!(graph.task_count(), 1);

        Ok(())
    }

    #[test]
    fn test_add_dependency() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let dep = Dependency::test_from_id_string("dep1")?;
        
        graph.add_dependency(&task1, &dep, &task2)?;
        
        assert!(graph.contains_task(&task1));
        assert!(graph.contains_task(&task2));
        assert!(graph.has_dependency(&task1, &task2));
        assert_eq!(graph.task_count(), 2);
        assert_eq!(graph.dependency_count(), 1);

        Ok(())
    }

    #[test]
    fn test_remove_task() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let task3 = Task::test_from_id_string("task3")?;
        let dep = Dependency::test_from_id_string("dep1")?;
        
        // Create graph: task1 -> task2 -> task3
        graph.add_dependency(&task1, &dep, &task2)?;
        graph.add_dependency(&task2, &dep, &task3)?;
        
        // Remove task2
        let removed = graph.remove_task(&task2).unwrap();
        
        assert!(removed);
        assert!(!graph.contains_task(&task2));
        assert!(graph.contains_task(&task1));
        assert!(graph.contains_task(&task3));
        assert!(!graph.has_dependency(&task1, &task2));
        assert!(!graph.has_dependency(&task2, &task3));
        assert_eq!(graph.task_count(), 2);
        assert_eq!(graph.dependency_count(), 0);

        Ok(())
    }

    #[test]
    fn test_remove_nonexistent_task() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task = Task::test_from_id_string("nonexistent")?;
        
        let removed = graph.remove_task(&task).unwrap();
        
        assert!(!removed);

        Ok(())
    }

    #[test]
    fn test_remove_dependency() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let dep = Dependency::test_from_id_string("dep1")?;
        
        graph.add_dependency(&task1, &dep, &task2)?;
        
        let removed = graph.remove_dependency(&task1, &dep, &task2)?;
        
        assert!(removed);
        assert!(!graph.has_dependency(&task1, &task2));
        assert_eq!(graph.dependency_count(), 0);

        Ok(())
    }

    #[test]
    fn test_remove_nonexistent_dependency() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let dep = Dependency::test_from_id_string("dep1")?;
        
        let removed = graph.remove_dependency(&task1, &dep, &task2)?;
        
        assert!(!removed);

        Ok(())
    }
}
