use super::{Graph, GraphError};
use std::collections::{HashSet, VecDeque};
use roadline_util::task::Task;

impl<'a> Graph<'a> {
    /// Performs a depth-first search starting from the given task.
    /// The closure is called for each visited task with the task ID and current depth.
    pub fn dfs<F>(&self, start_task: &'a Task, mut visit: F) -> Result<(), GraphError>
    where
        F: FnMut(&'a Task, usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        if !self.contains_task(start_task) {
            return Err(GraphError::Internal(
                format!("Task {:?} not found in graph", start_task).into()
            ));
        }

        let mut visited = HashSet::new();
        let mut stack = vec![(start_task, 0)];

        while let Some((task_id, depth)) = stack.pop() {
            if visited.insert(task_id) {
                visit(&task_id, depth)
                    .map_err(|e| GraphError::Internal(e))?;

                // Add dependencies to stack in reverse order to maintain left-to-right traversal
                if let Some(predicates) = self.facts.get(&task_id) {
                    for predicate in predicates.iter().rev() {
                        if !visited.contains(&predicate.task) {
                            stack.push((predicate.task, depth + 1));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Performs a breadth-first search starting from the given task.
    /// The closure is called for each visited task with the task ID and current depth.
    pub fn bfs<F>(&self, start_task: &'a Task, mut visit: F) -> Result<(), GraphError>
    where
        F: FnMut(&'a Task, usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        if !self.contains_task(start_task) {
            return Err(GraphError::Internal(
                format!("Task {:?} not found in graph", start_task).into()
            ));
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_task, 0));

        while let Some((task_id, depth)) = queue.pop_front() {
            if visited.insert(task_id) {
                visit(&task_id, depth)
                    .map_err(|e| GraphError::Internal(e))?;

                // Add dependencies to queue
                if let Some(predicates) = self.facts.get(&task_id) {
                    for predicate in predicates {
                        if !visited.contains(&predicate.task) {
                            queue.push_back((predicate.task, depth + 1));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Performs DFS and collects all reachable tasks from the start task.
    pub fn reachable_tasks(&self, start_task: &'a Task) -> Result<Vec<&'a Task>, GraphError> {
        let mut reachable = Vec::new();
        self.dfs(start_task, |task, _depth| {
            reachable.push(task);
            Ok(())
        })?;
        Ok(reachable)
    }

    /// Finds the shortest path between two tasks using BFS.
    /// Returns None if no path exists.
    pub fn shortest_path(&self, from: &'a Task, to: &'a Task) -> Result<Option<Vec<&'a Task>>, GraphError> {
        if !self.contains_task(from) || !self.contains_task(to) {
            return Err(GraphError::Internal(
                "One or both tasks not found in graph".into()
            ));
        }

        if from == to {
            return Ok(Some(vec![from]));
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent: std::collections::HashMap<&'a Task, &'a Task> = std::collections::HashMap::new();
        
        queue.push_back(from);
        visited.insert(from);

        while let Some(current) = queue.pop_front() {
            if let Some(predicates) = self.facts.get(&current) {
                for predicate in predicates {
                    let next_task = predicate.task;
                    
                    if next_task == to {
                        // Found the target, reconstruct path
                        let mut path = vec![to];
                        let mut current = current;
                        
                        while current != from {
                            path.push(current);
                            current = parent[&current];
                        }
                        path.push(from);
                        path.reverse();
                        
                        return Ok(Some(path));
                    }
                    
                    if visited.insert(next_task) {
                        parent.insert(next_task, current);
                        queue.push_back(next_task);
                    }
                }
            }
        }

        Ok(None) // No path found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::test_utils::*;

    #[test]
    fn test_dfs_linear_graph() -> Result<(), anyhow::Error> {
        let frame = create_linear_frame()?;
        let graph = create_linear_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        
        let mut visited = Vec::new();
        let mut depths = Vec::new();
        
        graph.dfs(&task1, |task_id, depth| {
            visited.push(task_id);
            depths.push(depth);
            Ok(())
        })?;
        
        assert_eq!(visited.len(), 4);
        assert_eq!(visited[0], &task1); // Should start with task1
        assert_eq!(depths[0], 0);
        assert_eq!(depths[3], 3); // Last task should be at depth 3

        Ok(())
    }

    #[test]
    fn test_bfs_linear_graph() -> Result<(), anyhow::Error> {
        let frame = create_linear_frame()?;
        let graph = create_linear_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        
        let mut visited = Vec::new();
        let mut depths = Vec::new();
        
        graph.bfs(&task1, |task_id, depth| {
            visited.push(task_id);
            depths.push(depth);
            Ok(())
        })?;
        
        assert_eq!(visited.len(), 4);
        assert_eq!(visited[0], &task1); // Should start with task1
        assert_eq!(depths, vec![0, 1, 2, 3]); // BFS should visit in order of depth

        Ok(())
    }

    #[test]
    fn test_dfs_branched_graph() -> Result<(), anyhow::Error> {
        let frame = create_branched_frame()?;
        let graph = create_branched_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        
        let mut visited = Vec::new();
        
        graph.dfs(&task1, |task_id, _depth| {
            visited.push(task_id);
            Ok(())
        })?;
        
        assert_eq!(visited.len(), 5);
        assert_eq!(visited[0], &task1); // Should start with task1

        Ok(())
    }

    #[test]
    fn test_bfs_branched_graph() -> Result<(), anyhow::Error> {
        let frame = create_branched_frame()?;
        let graph = create_branched_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        
        let mut visited = Vec::new();
        let mut depths = Vec::new();
        
        graph.bfs(&task1, |task_id, depth| {
            visited.push(task_id);
            depths.push(depth);
            Ok(())
        })?;
        
        assert_eq!(visited.len(), 5);
        assert_eq!(visited[0], &task1); // Should start with task1
        
        // task2, task3, task5 should all be at depth 1
        let depth_1_count = depths.iter().filter(|&&d| d == 1).count();
        assert_eq!(depth_1_count, 3);

        Ok(())
    }

    #[test]
    fn test_dfs_nonexistent_task() -> Result<(), anyhow::Error> {
        let frame = create_linear_frame()?;
        let graph = create_linear_graph(&frame)?;
        let nonexistent = Task::test_from_id_string("nonexistent")?;
        
        let result = graph.dfs(&nonexistent, |_task_id, _depth| Ok(()));
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_reachable_tasks() -> Result<(), anyhow::Error> {
        let frame = create_linear_frame()?;
        let graph = create_linear_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        
        let reachable = graph.reachable_tasks(&task1)?;
        assert_eq!(reachable.len(), 4);

        Ok(())
    }

    #[test]
    fn test_shortest_path_linear() -> Result<(), anyhow::Error> {
        let frame = create_linear_frame()?;
        let graph = create_linear_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let task4 = Task::test_from_id_string("task4")?;
        
        let path = graph.shortest_path(&task1, &task4).unwrap().unwrap();
        assert_eq!(path.len(), 4);
        assert_eq!(path[0], &task1);
        assert_eq!(path[3], &task4);

        Ok(())
    }

    #[test]
    fn test_shortest_path_same_task() -> Result<(), anyhow::Error> {
        let frame = create_linear_frame()?;
        let graph = create_linear_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        
        let path = graph.shortest_path(&task1, &task1).unwrap().unwrap();
        assert_eq!(path, vec![&task1]);

        Ok(())
    }

    #[test]
    fn test_shortest_path_no_path() -> Result<(), anyhow::Error> {
        let frame = create_linear_frame()?;
        let graph = create_linear_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let task4 = Task::test_from_id_string("task4")?;
        
        // Try to find path in reverse direction (should fail)
        let path = graph.shortest_path(&task4, &task1).unwrap();
        assert!(path.is_none());

        Ok(())
    }

    #[test]
    fn test_shortest_path_nonexistent_task() -> Result<(), anyhow::Error> {
        let frame = create_linear_frame()?;
        let graph = create_linear_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let nonexistent = Task::test_from_id_string("nonexistent")?;
        
        let result = graph.shortest_path(&task1, &nonexistent);
        assert!(result.is_err());

        Ok(())
    }
}
