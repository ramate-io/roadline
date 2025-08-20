use super::{Graph, Predicate};
use roadline_util::task::Task;
use std::collections::HashSet;

impl<'a> Graph<'a> {
    /// Gets all predicates for a given task.
    pub fn get_predicates(&self, task: &Task) -> Option<&[Predicate<'a>]> {
        self.facts.get(task).map(|v| v.as_slice())
    }

    /// Gets all tasks that depend on the given task.
    pub fn get_dependents(&self, task: &Task) -> Vec<&'a Task> {
        self.facts
            .iter()
            .filter_map(|(from_task, predicates)| {
                if predicates.iter().any(|p| p.task == task) {
                    Some(*from_task)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets all tasks that the given task depends on.
    pub fn get_dependencies(&self, task: &Task) -> Vec<&'a Task> {
        self.facts
            .get(task)
            .map(|predicates| predicates.iter().map(|p| p.task).collect())
            .unwrap_or_default()
    }

    /// Returns all tasks in the graph.
    pub fn tasks(&self) -> impl Iterator<Item = &'a Task> + '_ {
        self.facts.keys().copied()
    }

    /// Returns the number of tasks in the graph.
    pub fn task_count(&self) -> usize {
        self.facts.len()
    }

    /// Returns the total number of dependencies in the graph.
    pub fn dependency_count(&self) -> usize {
        self.facts.values().map(|v| v.len()).sum()
    }

    /// Checks if the graph contains a specific task.
    pub fn contains_task(&self, task: &Task) -> bool {
        self.facts.contains_key(task)
    }

    /// Checks if there's a direct dependency between two tasks.
    pub fn has_dependency(&self, from_task: &Task, to_task: &Task) -> bool {
        self.facts
            .get(from_task)
            .map(|predicates| predicates.iter().any(|p| p.task == to_task))
            .unwrap_or(false)
    }

    /// Finds all tasks that have no dependencies (root tasks).
    pub fn root_tasks(&self) -> Vec<&'a Task> {
        let mut has_incoming = HashSet::new();
        
        // Mark all tasks that have incoming edges
        for predicates in self.facts.values() {
            for predicate in predicates {
                has_incoming.insert(predicate.task);
            }
        }
        
        // Return tasks that have no incoming edges
        self.tasks()
            .filter(|task| !has_incoming.contains(task))
            .collect()
    }

    /// Finds all tasks that have no dependents (leaf tasks).
    pub fn leaf_tasks(&self) -> Vec<&'a Task> {
        self.tasks()
            .filter(|task| {
                self.facts.get(task)
                    .map(|predicates| predicates.is_empty())
                    .unwrap_or(true)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::test_utils::*;

    #[test]
    fn test_get_predicates() -> Result<(), anyhow::Error> {
        let frame = create_test_frame()?;
        let graph = create_test_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let task3 = Task::test_from_id_string("task3")?;
        
        let predicates1 = graph.get_predicates(&task1).ok_or(anyhow::anyhow!("task1 should have predicates"))?;
        assert_eq!(predicates1.len(), 1);
        assert_eq!(*predicates1[0].task, task2);
        
        let predicates2 = graph.get_predicates(&task2).ok_or(anyhow::anyhow!("task2 should have predicates"))?;
        assert_eq!(predicates2.len(), 1);
        assert_eq!(*predicates2[0].task, task3);
        
        let predicates3 = graph.get_predicates(&task3).ok_or(anyhow::anyhow!("task3 should have predicates"))?;
        assert_eq!(predicates3.len(), 0);

        Ok(())
    }

    #[test]
    fn test_get_dependents() -> Result<(), anyhow::Error> {
        let frame = create_test_frame()?;
        let graph = create_test_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let task3 = Task::test_from_id_string("task3")?;
        let task4 = Task::test_from_id_string("task4")?;
        
        let dependents2 = graph.get_dependents(&task2);
        assert_eq!(dependents2.len(), 1);
        assert!(dependents2.contains(&&task1));
        
        let dependents3 = graph.get_dependents(&task3);
        assert_eq!(dependents3.len(), 1);
        assert!(dependents3.contains(&&task2));
        
        let dependents1 = graph.get_dependents(&task1);
        assert_eq!(dependents1.len(), 0);
        
        let dependents4 = graph.get_dependents(&task4);
        assert_eq!(dependents4.len(), 0);

        Ok(())
    }

    #[test]
    fn test_get_dependencies() -> Result<(), anyhow::Error> {
        let frame = create_test_frame()?;
        let graph = create_test_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let task3 = Task::test_from_id_string("task3")?;
        let task4 = Task::test_from_id_string("task4")?;
        
        let deps1 = graph.get_dependencies(&task1);
        assert_eq!(deps1.len(), 1);
        assert!(deps1.contains(&&task2));
        
        let deps2 = graph.get_dependencies(&task2);
        assert_eq!(deps2.len(), 1);
        assert!(deps2.contains(&&task3));
        
        let deps3 = graph.get_dependencies(&task3);
        assert_eq!(deps3.len(), 0);
        
        let deps4 = graph.get_dependencies(&task4);
        assert_eq!(deps4.len(), 0);

        Ok(())
    }

    #[test]
    fn test_task_counts() -> Result<(), anyhow::Error> {
        let frame = create_test_frame()?;
        let graph = create_test_graph(&frame)?;
        
        assert_eq!(graph.task_count(), 4);
        assert_eq!(graph.dependency_count(), 2);

        Ok(())
    }

    #[test]
    fn test_contains_task() -> Result<(), anyhow::Error> {
        let frame = create_test_frame()?;
        let graph = create_test_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let nonexistent = Task::test_from_id_string("nonexistent")?;
        
        assert!(graph.contains_task(&task1));
        assert!(!graph.contains_task(&nonexistent));

        Ok(())
    }

    #[test]
    fn test_has_dependency() -> Result<(), anyhow::Error> {
        let frame = create_test_frame()?;
        let graph = create_test_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let task2 = Task::test_from_id_string("task2")?;
        let task3 = Task::test_from_id_string("task3")?;
        let task4 = Task::test_from_id_string("task4")?;
        
        assert!(graph.has_dependency(&task1, &task2));
        assert!(graph.has_dependency(&task2, &task3));
        assert!(!graph.has_dependency(&task1, &task3)); // Not direct
        assert!(!graph.has_dependency(&task1, &task4));

        Ok(())
    }

    #[test]
    fn test_root_tasks() -> Result<(), anyhow::Error> {
        let frame = create_test_frame()?;
        let graph = create_test_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let task4 = Task::test_from_id_string("task4")?;
        
        let roots = graph.root_tasks();
        assert_eq!(roots.len(), 2);
        assert!(roots.contains(&&task1));
        assert!(roots.contains(&&task4));

        Ok(())
    }

    #[test]
    fn test_leaf_tasks() -> Result<(), anyhow::Error> {
        let frame = create_test_frame()?;
        let graph = create_test_graph(&frame)?;
        let task3 = Task::test_from_id_string("task3")?;
        let task4 = Task::test_from_id_string("task4")?;
        
        let leaves = graph.leaf_tasks();
        assert_eq!(leaves.len(), 2);
        assert!(leaves.contains(&&task3));
        assert!(leaves.contains(&&task4));

        Ok(())
    }
}
