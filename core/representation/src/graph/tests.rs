//! Integration tests for the graph module.
//! 
//! These tests verify that all modules work together correctly and cover 
//! complex scenarios involving multiple graph operations.

#[cfg(test)]
mod integration_tests {
    use super::super::Graph;
    use roadline_util::task::Task;
    use roadline_util::dependency::Dependency;
    use crate::graph::test_utils::*;


    #[test]
    fn test_complex_graph_creation_and_queries() -> Result<(), anyhow::Error> {
        let frame = create_complex_frame()?;
        let graph = create_complex_graph(&frame)?;
        
        // Test basic counts
        assert_eq!(graph.task_count(), 10);
        assert_eq!(graph.dependency_count(), 13);
        
        // Test root and leaf identification
        let roots = graph.root_tasks();
        let leaves = graph.leaf_tasks();
        
        assert_eq!(roots.len(), 1);
        assert!(roots.contains(&&Task::test_from_id_string("task1")?));
        
        assert_eq!(leaves.len(), 1);
        assert!(leaves.contains(&&Task::test_from_id_string("task10")?));
        
        // Test specific dependencies
        let task1 = Task::test_from_id_string("task1")?;
        let task5 = Task::test_from_id_string("task5")?;
        
        let task1_deps = graph.get_dependencies(&task1);
        assert_eq!(task1_deps.len(), 2);
        
        let task5_dependents = graph.get_dependents(&task5);
        assert_eq!(task5_dependents.len(), 2); // task2 and task3
        
        Ok(())
    }

    #[test]
    fn test_traversal_completeness() -> Result<(), anyhow::Error> {
        let frame = create_complex_frame()?;
        let graph = create_complex_graph(&frame)?;
        let start_task = Task::test_from_id_string("task1")?;
        
        // Test DFS visits all reachable nodes
        let mut dfs_visited = Vec::new();
        graph.dfs(&start_task, |task_id, _depth| {
            dfs_visited.push(*task_id);
            Ok(())
        })?;
        
        assert_eq!(dfs_visited.len(), 10); // All tasks should be reachable
        
        // Test BFS visits all reachable nodes
        let mut bfs_visited = Vec::new();
        graph.bfs(&start_task, |task_id, _depth| {
            bfs_visited.push(*task_id);
            Ok(())
        })?;
        
        assert_eq!(bfs_visited.len(), 10);
        
        // BFS should visit by depth level
        let mut bfs_depths = Vec::new();
        graph.bfs(&start_task, |_task_id, depth| {
            bfs_depths.push(depth);
            Ok(())
        })?;
        
        // Should be non-decreasing (each depth level before the next)
        for window in bfs_depths.windows(2) {
            assert!(window[0] <= window[1]);
        }
        
        Ok(())
    }

    #[test]
    fn test_path_finding() -> Result<(), anyhow::Error> {
        let frame = create_complex_frame()?;
        let graph = create_complex_graph(&frame)?;
        let task1 = Task::test_from_id_string("task1")?;
        let task10 = Task::test_from_id_string("task10")?;
        let task5 = Task::test_from_id_string("task5")?;
        
        // Test path from start to end
        let path = graph.shortest_path(&task1, &task10)?.unwrap();
        assert!(path.len() >= 5); // At least 5 steps from task1 to task10
        assert_eq!(path[0], task1);
        assert_eq!(path[path.len() - 1], task10);
        
        // Test path to intermediate node
        let path_to_5 = graph.shortest_path(&task1, &task5)?.unwrap();
        assert!(path_to_5.len() >= 3); // At least 3 steps
        
        // Test no reverse path (DAG property)
        let reverse_path = graph.shortest_path(&task10, &task1)?;
        assert!(reverse_path.is_none());
        
        Ok(())
    }

    #[test]
    fn test_cycle_detection_and_topological_sort() -> Result<(), anyhow::Error> {
        let frame = create_complex_frame()?;
        let graph = create_complex_graph(&frame)?;
        
        // Graph should be acyclic
        assert!(!graph.has_cycles()?);
        assert!(graph.is_dag()?);
        
        // Should be able to get topological ordering
        let topo_order = graph.topological_sort()?;
        assert_eq!(topo_order.len(), 10);
        
        // task1 should come before all others  
        let task1 = Task::test_from_id_string("task1")?;
        let task1_pos = topo_order.iter().position(|t| **t == task1)
            .ok_or(anyhow::anyhow!("task1 should be in topological sort"))?;
        assert_eq!(task1_pos, 0);

        // task10 should come last
        let task10 = Task::test_from_id_string("task10")?;
        let task10_pos = topo_order.iter().position(|t| **t == task10)
            .ok_or(anyhow::anyhow!("task10 should be in topological sort"))?;
        assert_eq!(task10_pos, 9);
        
        // Verify topological property: if A -> B, then A comes before B in ordering
        for (i, &task_a) in topo_order.iter().enumerate() {
            for &task_b in topo_order.iter().skip(i + 1) {
                assert!(!graph.has_dependency(&task_b, &task_a));
            }
        }
        
        Ok(())
    }

    #[test]
    fn test_mutation_operations() -> Result<(), anyhow::Error> {
        let frame = create_complex_frame()?;
        let mut graph = create_complex_graph(&frame)?;
        let initial_count = graph.task_count();
        
        // Test adding a new task
        let new_task = Task::test_from_id_string("new_task")?;
        graph.add_task(new_task);
        assert_eq!(graph.task_count(), initial_count + 1);
        assert!(graph.contains_task(&new_task));
        
        // Test adding dependency to new task
        let task1 = Task::test_from_id_string("task1")?;
        let dep_id = Dependency::test_from_id_string("new_dep")?;
        graph.add_dependency(task1, dep_id, new_task).unwrap();
        assert!(graph.has_dependency(&task1, &new_task));
        
        // Test removing dependency
        let task2 = Task::test_from_id_string("task2")?;
        let old_dep_id = Dependency::test_from_id_string("dep1")?;
        let removed = graph.remove_dependency(&task1, &old_dep_id, &task2).unwrap();
        assert!(removed);
        assert!(!graph.has_dependency(&task1, &task2));
        
        // Test removing task
        let task5 = Task::test_from_id_string("task5")?;
        let dependents_before = graph.get_dependents(&task5);
        assert!(!dependents_before.is_empty());
        
        let removed = graph.remove_task(&task5).unwrap();
        assert!(removed);
        assert!(!graph.contains_task(&task5));
        
        // All references to task5 should be gone
        for task_id in graph.task_ids() {
            let deps = graph.get_dependencies(task_id);
            assert!(!deps.contains(&task5));
        }

        Ok(())
    }

    #[test]
    fn test_strongly_connected_components() -> Result<(), anyhow::Error> {
        let graph = create_complex_graph()?;
        
        // DAG should have each node as its own SCC
        let components = graph.strongly_connected_components()?;
        assert_eq!(components.len(), 10);
        
        for component in &components {
            assert_eq!(component.len(), 1);
        }
        
        // Test with a graph that has cycles
        let mut cyclic_graph = Graph::new();
        let tasks: Vec<Task> = (1..=4)
            .map(|i| Task::test_from_id_string(&format!("task{}", i))?)
            .collect::<Result<Vec<_>, _>>()?;
        let dep_id = Dependency::test_from_id_string("dep1")?;
        
        // Create cycle: task1 -> task2 -> task3 -> task1, plus task4 isolated
        cyclic_graph.add_dependency(tasks[0], dep_id, tasks[1])?;
        cyclic_graph.add_dependency(tasks[1], dep_id, tasks[2])?;
        cyclic_graph.add_dependency(tasks[2], dep_id, tasks[0])?;
        cyclic_graph.add_task(tasks[3]);
        
        let cyclic_components = cyclic_graph.strongly_connected_components()?;
        assert_eq!(cyclic_components.len(), 2); // One SCC with 3 nodes, one with 1 node
        
        let cycle_found = cyclic_components.iter().any(|comp| comp.len() == 3);
        assert!(cycle_found);

        Ok(())
    }

    #[test]
    fn test_error_handling() -> Result<(), anyhow::Error> {
        let graph = create_complex_graph()?;
        let nonexistent = TaskId::from_string("nonexistent")?;
        let task1 = TaskId::from_string("task1")?;
        
        // Test DFS with nonexistent task
        let result = graph.dfs(&nonexistent, |_task_id, _depth| Ok(()));
        assert!(result.is_err());
        
        // Test BFS with nonexistent task
        let result = graph.bfs(&nonexistent, |_task_id, _depth| Ok(()));
        assert!(result.is_err());
        
        // Test shortest path with nonexistent task
        let result = graph.shortest_path(&task1, &nonexistent);
        assert!(result.is_err());
        
        // Test visitor function error propagation
        let result = graph.dfs(&task1, |_task_id, _depth| {
            Err("Test error".into())
        });
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_reachability() -> Result<(), anyhow::Error> {
        let graph = create_complex_graph()?;
        let task1 = Task::test_from_id_string("task1")?;
        
        let reachable = graph.reachable_tasks(&task1)?;
        assert_eq!(reachable.len(), 10); // All tasks reachable from task1
        
        // Test reachability from intermediate node
        let task5 = Task::test_from_id_string("task5")?;
        let reachable_from_5 = graph.reachable_tasks(&task5)?;
        assert!(reachable_from_5.len() < 10); // Fewer tasks reachable from task5
        
        // task1 should not be reachable from task5 (DAG property)
        assert!(!reachable_from_5.contains(&task1));

        Ok(())
    }

    #[test]
    fn test_graph_properties() -> Result<(), anyhow::Error> {
        let graph = create_complex_graph()?;
        
        // Test that predicates are consistent
        for task in graph.tasks() {
            let predicates = graph.get_predicates(task);
            if let Some(preds) = predicates {
                for predicate in preds {
                    // Each predicate target should exist in the graph
                    assert!(graph.contains_task(&predicate.task));
                    
                    // The dependency should be detectable via has_dependency
                    assert!(graph.has_dependency(task, &predicate.task));
                }
            }
        }

        // Test dependency/dependent symmetry
        for task in graph.tasks() {
            let dependencies = graph.get_dependencies(task);
            for dep_task in dependencies {
                let dependents = graph.get_dependents(&dep_task);
                assert!(dependents.contains(&task));
            }
        }

        Ok(())
    }

    #[test]
    fn test_empty_graph() -> Result<(), anyhow::Error> {
        let graph = Graph::new();
        
        assert_eq!(graph.task_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.root_tasks().is_empty());
        assert!(graph.leaf_tasks().is_empty());
        assert!(!graph.has_cycles().unwrap());
        assert!(graph.is_dag().unwrap());
        
        let topo_order = graph.topological_sort()?;
        assert!(topo_order.is_empty());
        
        let components = graph.strongly_connected_components()?;
        assert!(components.is_empty());

        Ok(())
    }

    #[test]
    fn test_single_task_graph() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        let task = Task::test_from_id_string("single_task")?;
        graph.add_task(task);
        
        assert_eq!(graph.task_count(), 1);
        assert_eq!(graph.dependency_count(), 0);
        assert_eq!(graph.root_tasks(), vec![task]);
        assert_eq!(graph.leaf_tasks(), vec![task]);
        assert!(!graph.has_cycles()?);
        assert!(graph.is_dag()?);
        
        let topo_order = graph.topological_sort()?;
        assert_eq!(topo_order, vec![task]);
        
        let reachable = graph.reachable_tasks(&task)?;
        assert_eq!(reachable, vec![task]);

        Ok(())
    }
}
