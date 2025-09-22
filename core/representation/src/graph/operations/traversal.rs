use crate::graph::{Graph, GraphError};
use roadline_util::task::Id as TaskId;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

impl Graph {
	/// Builds a reverse adjacency list: task_id -> Vec<parent_task_ids>
	/// This maps each task to all tasks that have dependencies pointing to it.
	fn build_reverse_adjacency(&self) -> HashMap<TaskId, Vec<TaskId>> {
		let mut reverse_adjacency: HashMap<TaskId, Vec<TaskId>> = HashMap::new();
		for (parent_task_id, predicates) in &self.facts {
			for predicate in predicates {
				reverse_adjacency
					.entry(predicate.task_id)
					.or_insert_with(Vec::new)
					.push(*parent_task_id);
			}
		}
		reverse_adjacency
	}

	/// Performs a depth-first search starting from the given task.
	/// The closure is called for each visited task with the task ID and current depth.
	pub fn dfs<F>(&self, start_task: &TaskId, mut visit: F) -> Result<(), GraphError>
	where
		F: FnMut(&TaskId, usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
	{
		if !self.contains_task(start_task) {
			return Err(GraphError::Internal(
				format!("Task {:?} not found in graph", start_task).into(),
			));
		}

		let mut visited = HashSet::new();
		let mut stack = vec![(*start_task, 0)];

		while let Some((task_id, depth)) = stack.pop() {
			if visited.insert(task_id) {
				visit(&task_id, depth).map_err(|e| GraphError::Internal(e))?;

				// Add dependencies to stack in reverse order to maintain left-to-right traversal
				if let Some(predicates) = self.facts.get(&task_id) {
					for predicate in predicates.iter().rev() {
						if !visited.contains(&predicate.task_id) {
							stack.push((predicate.task_id, depth + 1));
						}
					}
				}
			}
		}

		Ok(())
	}

	/// Performs a depth-first search in reverse order, i.e., descendants are now parents
	pub fn rev_dfs<F>(&self, start_task: &TaskId, mut visit: F) -> Result<(), GraphError>
	where
		F: FnMut(&TaskId, usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
	{
		if !self.contains_task(start_task) {
			return Err(GraphError::Internal(
				format!("Task {:?} not found in graph", start_task).into(),
			));
		}

		let reverse_adjacency = self.build_reverse_adjacency();
		let mut visited = HashSet::new();
		let mut stack = vec![(*start_task, 0)];

		while let Some((task_id, depth)) = stack.pop() {
			if visited.insert(task_id) {
				visit(&task_id, depth).map_err(|e| GraphError::Internal(e))?;

				// Add parents to stack in reverse order to maintain left-to-right traversal
				if let Some(parents) = reverse_adjacency.get(&task_id) {
					for parent_task_id in parents.iter().rev() {
						if !visited.contains(parent_task_id) {
							stack.push((*parent_task_id, depth + 1));
						}
					}
				}
			}
		}

		Ok(())
	}

	/// Performs a breadth-first search starting from the given task.
	/// The closure is called for each visited task with the task ID and current depth.
	pub fn bfs<F>(&self, start_task: &TaskId, mut visit: F) -> Result<(), GraphError>
	where
		F: FnMut(&TaskId, usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
	{
		if !self.contains_task(start_task) {
			return Err(GraphError::Internal(
				format!("Task {:?} not found in graph", start_task).into(),
			));
		}

		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();
		queue.push_back((*start_task, 0));

		while let Some((task_id, depth)) = queue.pop_front() {
			if visited.insert(task_id) {
				visit(&task_id, depth).map_err(|e| GraphError::Internal(e))?;

				// Add dependencies to queue
				if let Some(predicates) = self.facts.get(&task_id) {
					for predicate in predicates {
						if !visited.contains(&predicate.task_id) {
							queue.push_back((predicate.task_id, depth + 1));
						}
					}
				}
			}
		}

		Ok(())
	}

	/// Performs a breadth-first search in reverse order, i.e., descendants are now parents
	pub fn rev_bfs<F>(&self, start_task: &TaskId, mut visit: F) -> Result<(), GraphError>
	where
		F: FnMut(&TaskId, usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
	{
		if !self.contains_task(start_task) {
			return Err(GraphError::Internal(
				format!("Task {:?} not found in graph", start_task).into(),
			));
		}

		let reverse_adjacency = self.build_reverse_adjacency();
		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();
		queue.push_back((*start_task, 0));

		while let Some((task_id, depth)) = queue.pop_front() {
			if visited.insert(task_id) {
				visit(&task_id, depth).map_err(|e| GraphError::Internal(e))?;

				// Add parents to queue
				if let Some(parents) = reverse_adjacency.get(&task_id) {
					for parent_task_id in parents {
						if !visited.contains(parent_task_id) {
							queue.push_back((*parent_task_id, depth + 1));
						}
					}
				}
			}
		}

		Ok(())
	}

	/// Performs DFS and collects all reachable tasks from the start task.
	pub fn reachable_tasks(&self, start_task: &TaskId) -> Result<Vec<TaskId>, GraphError> {
		let mut reachable = Vec::new();
		self.dfs(start_task, |task_id, _depth| {
			reachable.push(*task_id);
			Ok(())
		})?;
		Ok(reachable)
	}

	/// Finds the shortest path between two tasks using BFS.
	/// Returns None if no path exists.
	pub fn shortest_path(
		&self,
		from: &TaskId,
		to: &TaskId,
	) -> Result<Option<Vec<TaskId>>, GraphError> {
		if !self.contains_task(from) || !self.contains_task(to) {
			return Err(GraphError::Internal("One or both tasks not found in graph".into()));
		}

		if from == to {
			return Ok(Some(vec![*from]));
		}

		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();
		let mut parent: std::collections::HashMap<TaskId, TaskId> =
			std::collections::HashMap::new();

		queue.push_back(*from);
		visited.insert(*from);

		while let Some(current) = queue.pop_front() {
			if let Some(predicates) = self.facts.get(&current) {
				for predicate in predicates {
					let next_task = predicate.task_id;

					if next_task == *to {
						// Found the target, reconstruct path
						let mut path = vec![*to];
						let mut current = current;

						while current != *from {
							path.push(current);
							current = parent[&current];
						}
						path.push(*from);
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
	use crate::graph::operations::test_utils::*;

	#[test]
	fn test_dfs_linear_graph() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task1 = TaskId::new(1);

		let mut visited = Vec::new();
		let mut depths = Vec::new();

		graph.dfs(&task1, |task_id, depth| {
			visited.push(*task_id);
			depths.push(depth);
			Ok(())
		})?;

		assert_eq!(visited.len(), 4);
		assert_eq!(visited[0], task1); // Should start with task1
		assert_eq!(depths[0], 0);
		assert_eq!(depths[3], 3); // Last task should be at depth 3

		Ok(())
	}

	#[test]
	fn test_bfs_linear_graph() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task1 = TaskId::new(1);

		let mut visited = Vec::new();
		let mut depths = Vec::new();

		graph.bfs(&task1, |task_id, depth| {
			visited.push(*task_id);
			depths.push(depth);
			Ok(())
		})?;

		assert_eq!(visited.len(), 4);
		assert_eq!(visited[0], task1); // Should start with task1
		assert_eq!(depths, vec![0, 1, 2, 3]); // BFS should visit in order of depth

		Ok(())
	}

	#[test]
	fn test_dfs_branched_graph() -> Result<(), anyhow::Error> {
		let graph = create_branched_graph()?;
		let task1 = TaskId::new(1);

		let mut visited = Vec::new();

		graph.dfs(&task1, |task_id, _depth| {
			visited.push(*task_id);
			Ok(())
		})?;

		assert_eq!(visited.len(), 5);
		assert_eq!(visited[0], task1); // Should start with task1

		Ok(())
	}

	#[test]
	fn test_bfs_branched_graph() -> Result<(), anyhow::Error> {
		let graph = create_branched_graph()?;
		let task1 = TaskId::new(1);

		let mut visited = Vec::new();
		let mut depths = Vec::new();

		graph.bfs(&task1, |task_id, depth| {
			visited.push(*task_id);
			depths.push(depth);
			Ok(())
		})?;

		assert_eq!(visited.len(), 5);
		assert_eq!(visited[0], task1); // Should start with task1

		// task2, task3, task5 should all be at depth 1
		let depth_1_count = depths.iter().filter(|&&d| d == 1).count();
		assert_eq!(depth_1_count, 3);

		Ok(())
	}

	#[test]
	fn test_dfs_nonexistent_task() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let nonexistent = TaskId::new(100);

		let result = graph.dfs(&nonexistent, |_task_id, _depth| Ok(()));
		assert!(result.is_err());

		Ok(())
	}

	#[test]
	fn test_reachable_tasks() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task1 = TaskId::new(1);

		let reachable = graph.reachable_tasks(&task1)?;
		assert_eq!(reachable.len(), 4);

		Ok(())
	}

	#[test]
	fn test_shortest_path_linear() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task1 = TaskId::new(1);
		let task4 = TaskId::new(4);

		let path = graph.shortest_path(&task1, &task4).unwrap().unwrap();
		assert_eq!(path.len(), 4);
		assert_eq!(path[0], task1);
		assert_eq!(path[3], task4);

		Ok(())
	}

	#[test]
	fn test_shortest_path_same_task() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task1 = TaskId::new(1);

		let path = graph.shortest_path(&task1, &task1).unwrap().unwrap();
		assert_eq!(path, vec![task1]);

		Ok(())
	}

	#[test]
	fn test_shortest_path_no_path() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task1 = TaskId::new(1);
		let task4 = TaskId::new(4);

		// Try to find path in reverse direction (should fail)
		let path = graph.shortest_path(&task4, &task1).unwrap();
		assert!(path.is_none());

		Ok(())
	}

	#[test]
	fn test_shortest_path_nonexistent_task() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task1 = TaskId::new(1);
		let nonexistent = TaskId::new(100);

		let result = graph.shortest_path(&task1, &nonexistent);
		assert!(result.is_err());

		Ok(())
	}

	// === Reverse DFS Tests ===

	#[test]
	fn test_rev_dfs_linear_graph() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task4 = TaskId::new(4); // Start from the end

		let mut visited = Vec::new();
		let mut depths = Vec::new();

		graph.rev_dfs(&task4, |task_id, depth| {
			visited.push(*task_id);
			depths.push(depth);
			Ok(())
		})?;

		assert_eq!(visited.len(), 4);
		assert_eq!(visited[0], task4); // Should start with task4
		assert_eq!(depths[0], 0);
		assert_eq!(depths[3], 3); // task1 should be at depth 3

		// Verify we visited all tasks in reverse order
		assert_eq!(visited, vec![task4, TaskId::new(3), TaskId::new(2), TaskId::new(1)]);
		assert_eq!(depths, vec![0, 1, 2, 3]);

		Ok(())
	}

	#[test]
	fn test_rev_dfs_branched_graph() -> Result<(), anyhow::Error> {
		let graph = create_branched_graph()?;
		let task4 = TaskId::new(4); // End of both branches

		let mut visited = Vec::new();

		graph.rev_dfs(&task4, |task_id, _depth| {
			visited.push(*task_id);
			Ok(())
		})?;

		assert_eq!(visited.len(), 4); // task4, task2, task3, task1
		assert_eq!(visited[0], task4); // Should start with task4

		// Should include all tasks in the path to task4
		assert!(visited.contains(&TaskId::new(1))); // Root
		assert!(visited.contains(&TaskId::new(2))); // Parent via one branch
		assert!(visited.contains(&TaskId::new(3))); // Parent via other branch

		Ok(())
	}

	#[test]
	fn test_rev_dfs_complex_graph() -> Result<(), anyhow::Error> {
		let graph = create_complex_graph()?;
		let task10 = TaskId::new(10); // Final task

		let mut visited = Vec::new();
		let mut depths = Vec::new();

		graph.rev_dfs(&task10, |task_id, depth| {
			visited.push(*task_id);
			depths.push(depth);
			Ok(())
		})?;

		assert_eq!(visited.len(), 10); // Should visit all tasks
		assert_eq!(visited[0], task10); // Should start with task10

		// Verify depth progression
		assert_eq!(depths[0], 0); // task10 at depth 0
		assert_eq!(depths[1], 1); // task9 at depth 1
		assert_eq!(depths[2], 2); // task7 or task8 at depth 2

		// Should include the root task1
		assert!(visited.contains(&TaskId::new(1)));

		Ok(())
	}

	#[test]
	fn test_rev_dfs_isolated_task() -> Result<(), anyhow::Error> {
		let graph = create_test_graph()?;
		let task4 = TaskId::new(4); // Isolated task

		let mut visited = Vec::new();

		graph.rev_dfs(&task4, |task_id, _depth| {
			visited.push(*task_id);
			Ok(())
		})?;

		assert_eq!(visited.len(), 1);
		assert_eq!(visited[0], task4); // Should only visit itself

		Ok(())
	}

	#[test]
	fn test_rev_dfs_cyclic_graph() -> Result<(), anyhow::Error> {
		let graph = create_cyclic_graph()?;
		let task1 = TaskId::new(1);

		let mut visited = Vec::new();

		graph.rev_dfs(&task1, |task_id, _depth| {
			visited.push(*task_id);
			Ok(())
		})?;

		assert_eq!(visited.len(), 3); // Should visit all tasks in the cycle
		assert_eq!(visited[0], task1); // Should start with task1

		// Should include all tasks in the cycle
		assert!(visited.contains(&TaskId::new(1)));
		assert!(visited.contains(&TaskId::new(2)));
		assert!(visited.contains(&TaskId::new(3)));

		Ok(())
	}

	#[test]
	fn test_rev_dfs_nonexistent_task() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let nonexistent = TaskId::new(100);

		let result = graph.rev_dfs(&nonexistent, |_task_id, _depth| Ok(()));
		assert!(result.is_err());

		Ok(())
	}

	#[test]
	fn test_rev_dfs_error_propagation() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task4 = TaskId::new(4);

		let result = graph.rev_dfs(&task4, |task_id, _depth| {
			if *task_id == TaskId::new(2) {
				Err("Test error".into())
			} else {
				Ok(())
			}
		});

		assert!(result.is_err());

		Ok(())
	}

	#[test]
	fn test_rev_dfs_vs_dfs_complement() -> Result<(), anyhow::Error> {
		let graph = create_branched_graph()?;
		let task1 = TaskId::new(1);
		let task4 = TaskId::new(4);

		// Forward DFS from task1
		let mut forward_visited = Vec::new();
		graph.dfs(&task1, |task_id, _depth| {
			forward_visited.push(*task_id);
			Ok(())
		})?;

		// Reverse DFS from task4
		let mut reverse_visited = Vec::new();
		graph.rev_dfs(&task4, |task_id, _depth| {
			reverse_visited.push(*task_id);
			Ok(())
		})?;

		// The intersection should include task1 and task4
		assert!(forward_visited.contains(&task1));
		assert!(forward_visited.contains(&task4));
		assert!(reverse_visited.contains(&task1));
		assert!(reverse_visited.contains(&task4));

		Ok(())
	}

	// === Reverse BFS Tests ===

	#[test]
	fn test_rev_bfs_linear_graph() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task4 = TaskId::new(4); // Start from the end

		let mut visited = Vec::new();
		let mut depths = Vec::new();

		graph.rev_bfs(&task4, |task_id, depth| {
			visited.push(*task_id);
			depths.push(depth);
			Ok(())
		})?;

		assert_eq!(visited.len(), 4);
		assert_eq!(visited[0], task4); // Should start with task4
		assert_eq!(depths[0], 0);
		assert_eq!(depths[1], 1); // task3 at depth 1
		assert_eq!(depths[2], 2); // task2 at depth 2
		assert_eq!(depths[3], 3); // task1 at depth 3

		// BFS should visit in order of depth
		assert_eq!(visited, vec![task4, TaskId::new(3), TaskId::new(2), TaskId::new(1)]);
		assert_eq!(depths, vec![0, 1, 2, 3]);

		Ok(())
	}

	#[test]
	fn test_rev_bfs_branched_graph() -> Result<(), anyhow::Error> {
		let graph = create_branched_graph()?;
		let task4 = TaskId::new(4); // End of both branches

		let mut visited = Vec::new();
		let mut depths = Vec::new();

		graph.rev_bfs(&task4, |task_id, depth| {
			visited.push(*task_id);
			depths.push(depth);
			Ok(())
		})?;

		assert_eq!(visited.len(), 4); // task4, task2, task3, task1
		assert_eq!(visited[0], task4); // Should start with task4

		// BFS should visit all tasks at depth 1 before depth 2
		assert_eq!(depths[0], 0); // task4 at depth 0
		assert!(depths[1] == 1 && depths[2] == 1); // task2 and task3 at depth 1
		assert_eq!(depths[3], 2); // task1 at depth 2

		// Should include all tasks in the path to task4
		assert!(visited.contains(&TaskId::new(1))); // Root
		assert!(visited.contains(&TaskId::new(2))); // Parent via one branch
		assert!(visited.contains(&TaskId::new(3))); // Parent via other branch

		Ok(())
	}

	#[test]
	fn test_rev_bfs_complex_graph() -> Result<(), anyhow::Error> {
		let graph = create_complex_graph()?;
		let task10 = TaskId::new(10); // Final task

		let mut visited = Vec::new();
		let mut depths = Vec::new();

		graph.rev_bfs(&task10, |task_id, depth| {
			visited.push(*task_id);
			depths.push(depth);
			Ok(())
		})?;

		assert_eq!(visited.len(), 10); // Should visit all tasks
		assert_eq!(visited[0], task10); // Should start with task10

		// Verify BFS depth ordering
		assert_eq!(depths[0], 0); // task10 at depth 0
		assert_eq!(depths[1], 1); // task9 at depth 1

		// Should include the root task1
		assert!(visited.contains(&TaskId::new(1)));

		Ok(())
	}

	#[test]
	fn test_rev_bfs_isolated_task() -> Result<(), anyhow::Error> {
		let graph = create_test_graph()?;
		let task4 = TaskId::new(4); // Isolated task

		let mut visited = Vec::new();

		graph.rev_bfs(&task4, |task_id, _depth| {
			visited.push(*task_id);
			Ok(())
		})?;

		assert_eq!(visited.len(), 1);
		assert_eq!(visited[0], task4); // Should only visit itself

		Ok(())
	}

	#[test]
	fn test_rev_bfs_cyclic_graph() -> Result<(), anyhow::Error> {
		let graph = create_cyclic_graph()?;
		let task1 = TaskId::new(1);

		let mut visited = Vec::new();

		graph.rev_bfs(&task1, |task_id, _depth| {
			visited.push(*task_id);
			Ok(())
		})?;

		assert_eq!(visited.len(), 3); // Should visit all tasks in the cycle
		assert_eq!(visited[0], task1); // Should start with task1

		// Should include all tasks in the cycle
		assert!(visited.contains(&TaskId::new(1)));
		assert!(visited.contains(&TaskId::new(2)));
		assert!(visited.contains(&TaskId::new(3)));

		Ok(())
	}

	#[test]
	fn test_rev_bfs_nonexistent_task() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let nonexistent = TaskId::new(100);

		let result = graph.rev_bfs(&nonexistent, |_task_id, _depth| Ok(()));
		assert!(result.is_err());

		Ok(())
	}

	#[test]
	fn test_rev_bfs_error_propagation() -> Result<(), anyhow::Error> {
		let graph = create_linear_graph()?;
		let task4 = TaskId::new(4);

		let result = graph.rev_bfs(&task4, |task_id, _depth| {
			if *task_id == TaskId::new(2) {
				Err("Test error".into())
			} else {
				Ok(())
			}
		});

		assert!(result.is_err());

		Ok(())
	}

	#[test]
	fn test_rev_bfs_vs_bfs_complement() -> Result<(), anyhow::Error> {
		let graph = create_branched_graph()?;
		let task1 = TaskId::new(1);
		let task4 = TaskId::new(4);

		// Forward BFS from task1
		let mut forward_visited = Vec::new();
		graph.bfs(&task1, |task_id, _depth| {
			forward_visited.push(*task_id);
			Ok(())
		})?;

		// Reverse BFS from task4
		let mut reverse_visited = Vec::new();
		graph.rev_bfs(&task4, |task_id, _depth| {
			reverse_visited.push(*task_id);
			Ok(())
		})?;

		// The intersection should include task1 and task4
		assert!(forward_visited.contains(&task1));
		assert!(forward_visited.contains(&task4));
		assert!(reverse_visited.contains(&task1));
		assert!(reverse_visited.contains(&task4));

		Ok(())
	}

	#[test]
	fn test_rev_bfs_vs_rev_dfs_same_nodes() -> Result<(), anyhow::Error> {
		let graph = create_complex_graph()?;
		let task10 = TaskId::new(10);

		// Reverse BFS from task10
		let mut bfs_visited = Vec::new();
		graph.rev_bfs(&task10, |task_id, _depth| {
			bfs_visited.push(*task_id);
			Ok(())
		})?;

		// Reverse DFS from task10
		let mut dfs_visited = Vec::new();
		graph.rev_dfs(&task10, |task_id, _depth| {
			dfs_visited.push(*task_id);
			Ok(())
		})?;

		// Both should visit the same set of nodes (but in different order)
		assert_eq!(bfs_visited.len(), dfs_visited.len());

		// Convert to sets to compare
		let bfs_set: std::collections::HashSet<_> = bfs_visited.into_iter().collect();
		let dfs_set: std::collections::HashSet<_> = dfs_visited.into_iter().collect();
		assert_eq!(bfs_set, dfs_set);

		Ok(())
	}
}
