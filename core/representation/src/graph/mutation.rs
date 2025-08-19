use super::{Graph, GraphError, Predicate};
use roadline_util::task::Id as TaskId;
use roadline_util::dependency::id::Id as DependencyId;

impl Graph {
    /// Adds a task to the graph with no dependencies.
    pub fn add_task(&mut self, task_id: TaskId) {
        self.facts.entry(task_id).or_insert_with(Vec::new);
    }

    /// Adds a dependency relationship between two tasks.
    pub fn add_dependency(&mut self, from_task: TaskId, dependency_id: DependencyId, to_task: TaskId) -> Result<(), GraphError> {
        let predicate = Predicate {
            dependency_id,
            task_id: to_task,
        };
        
        self.facts.entry(from_task)
            .or_insert_with(Vec::new)
            .push(predicate);
        
        // Ensure the target task exists in the graph
        self.facts.entry(to_task).or_insert_with(Vec::new);
        
        Ok(())
    }

    /// Removes a task and all its dependencies from the graph.
    pub fn remove_task(&mut self, task_id: &TaskId) -> Result<bool, GraphError> {
        // Remove the task itself
        let removed = self.facts.remove(task_id).is_some();
        
        // Remove all references to this task from other tasks' predicates
        for predicates in self.facts.values_mut() {
            predicates.retain(|predicate| &predicate.task_id != task_id);
        }
        
        Ok(removed)
    }

    /// Removes a specific dependency between two tasks.
    pub fn remove_dependency(&mut self, from_task: &TaskId, dependency_id: &DependencyId, to_task: &TaskId) -> Result<bool, GraphError> {
        if let Some(predicates) = self.facts.get_mut(from_task) {
            let initial_len = predicates.len();
            predicates.retain(|predicate| {
                !(predicate.dependency_id == *dependency_id && predicate.task_id == *to_task)
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

    // Helper functions for creating test IDs
    fn task_id(s: &str) -> TaskId {
        TaskId::from_string(s).expect(&format!("Failed to create TaskId from '{}'", s))
    }

    fn dep_id(s: &str) -> DependencyId {
        DependencyId::from_string(s).expect(&format!("Failed to create DependencyId from '{}'", s))
    }

    #[test]
    fn test_add_task() {
        let mut graph = Graph::new();
        let task = task_id("task1");
        
        graph.add_task(task);
        
        assert!(graph.contains_task(&task));
        assert_eq!(graph.task_count(), 1);
    }

    #[test]
    fn test_add_dependency() {
        let mut graph = Graph::new();
        let task1 = task_id("task1");
        let task2 = task_id("task2");
        let dep = dep_id("dep1");
        
        graph.add_dependency(task1, dep, task2).unwrap();
        
        assert!(graph.contains_task(&task1));
        assert!(graph.contains_task(&task2));
        assert!(graph.has_dependency(&task1, &task2));
        assert_eq!(graph.task_count(), 2);
        assert_eq!(graph.dependency_count(), 1);
    }

    #[test]
    fn test_remove_task() {
        let mut graph = Graph::new();
        let task1 = task_id("task1");
        let task2 = task_id("task2");
        let task3 = task_id("task3");
        let dep = dep_id("dep1");
        
        // Create graph: task1 -> task2 -> task3
        graph.add_dependency(task1, dep, task2).unwrap();
        graph.add_dependency(task2, dep, task3).unwrap();
        
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
    }

    #[test]
    fn test_remove_nonexistent_task() {
        let mut graph = Graph::new();
        let task = task_id("nonexistent");
        
        let removed = graph.remove_task(&task).unwrap();
        
        assert!(!removed);
    }

    #[test]
    fn test_remove_dependency() {
        let mut graph = Graph::new();
        let task1 = task_id("task1");
        let task2 = task_id("task2");
        let dep = dep_id("dep1");
        
        graph.add_dependency(task1, dep, task2).unwrap();
        
        let removed = graph.remove_dependency(&task1, &dep, &task2).unwrap();
        
        assert!(removed);
        assert!(!graph.has_dependency(&task1, &task2));
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn test_remove_nonexistent_dependency() {
        let mut graph = Graph::new();
        let task1 = task_id("task1");
        let task2 = task_id("task2");
        let dep = dep_id("dep1");
        
        let removed = graph.remove_dependency(&task1, &dep, &task2).unwrap();
        
        assert!(!removed);
    }
}
