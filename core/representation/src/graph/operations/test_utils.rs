//! Common test utilities for graph tests.
//! 
//! This module provides helper functions for creating test data
//! to avoid repetitive error handling in test code.

use crate::graph::Graph;
use roadline_util::task::Id as TaskId;
use roadline_util::dependency::id::Id as DependencyId;

/// Helper function for creating a basic linear graph for testing.
/// Creates: task1 -> task2 -> task3 -> task4
pub fn create_linear_graph() -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let task1 = TaskId::new(1);
    let task2 = TaskId::new(2);
    let task3 = TaskId::new(3);
    let task4 = TaskId::new(4);
    let dep = DependencyId::new(1);
    
    graph.add_dependency(task1, dep, task2).expect("Failed to add dependency");
    graph.add_dependency(task2, dep, task3).expect("Failed to add dependency");
    graph.add_dependency(task3, dep, task4).expect("Failed to add dependency");
    
    Ok(graph)
}

/// Helper function for creating a branched graph for testing.
/// Creates: task1 -> [task2, task3] -> task4, task1 -> task5
pub fn create_branched_graph() -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let task1 = TaskId::new(1);
    let task2 = TaskId::new(2);
    let task3 = TaskId::new(3);
    let task4 = TaskId::new(4);
    let task5 = TaskId::new(5);
    let dep = DependencyId::new(1);
    
    graph.add_dependency(task1, dep, task2).expect("Failed to add dependency");
    graph.add_dependency(task1, dep, task3).expect("Failed to add dependency");
    graph.add_dependency(task1, dep, task5).expect("Failed to add dependency");
    graph.add_dependency(task2, dep, task4).expect("Failed to add dependency");
    graph.add_dependency(task3, dep, task4).expect("Failed to add dependency");
    
    Ok(graph)
}

/// Helper function for creating a complex test graph.
/// Creates a graph with multiple levels and branches for comprehensive testing.
pub fn create_complex_graph() -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    
    // Create tasks
    let tasks: Vec<TaskId> = (1..=10)
        .map(|i| TaskId::new(i))
        .collect::<Vec<_>>();
    
    let dep = DependencyId::new(1);
    
    // Create a complex dependency structure:
    // task1 -> [task2, task3]
    // task2 -> [task4, task5]  
    // task3 -> [task5, task6]
    // task4 -> task7
    // task5 -> [task7, task8]
    // task6 -> task8
    // task7 -> task9
    // task8 -> task9
    // task9 -> task10
    
    let dependencies = vec![
        (0, 1), (0, 2),  // task1 -> task2, task3
        (1, 3), (1, 4),  // task2 -> task4, task5
        (2, 4), (2, 5),  // task3 -> task5, task6
        (3, 6),          // task4 -> task7
        (4, 6), (4, 7),  // task5 -> task7, task8
        (5, 7),          // task6 -> task8
        (6, 8),          // task7 -> task9
        (7, 8),          // task8 -> task9
        (8, 9),          // task9 -> task10
    ];
    
    for (from_idx, to_idx) in dependencies {
        graph.add_dependency(tasks[from_idx], dep, tasks[to_idx])
            .expect("Failed to add dependency");
    }
    
    Ok(graph)
}

pub fn create_acyclic_graph() -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let task1 = TaskId::new(1);
    let task2 = TaskId::new(2);
    let task3 = TaskId::new(3);
    let task4 = TaskId::new(4);
    let  dep = DependencyId::new(1);
    
    // Create DAG: task1 -> task2 -> task4, task1 -> task3 -> task4
    graph.add_dependency(task1,  dep, task2)?;
    graph.add_dependency(task1,  dep, task3)?;
    graph.add_dependency(task2,  dep, task4)?;
    graph.add_dependency(task3,  dep, task4)?;
    
    Ok(graph)
}

pub fn create_cyclic_graph() -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let task1 = TaskId::new(1);
    let task2 = TaskId::new(2);
    let task3 = TaskId::new(3);
    let  dep = DependencyId::new(1);
    
    // Create cycle: task1 -> task2 -> task3 -> task1
    graph.add_dependency(task1,  dep, task2)?;
    graph.add_dependency(task2,  dep, task3)?;
    graph.add_dependency(task3,  dep, task1)?;
    
    Ok(graph)
}

pub fn create_test_graph() -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let task1 = TaskId::new(1);
    let task2 = TaskId::new(2);
    let task3 = TaskId::new(3);
    let task4 = TaskId::new(4);
    let  dep = DependencyId::new(1);
    
    // Create graph: task1 -> task2 -> task3, task4 (isolated)
    graph.add_dependency(task1,  dep, task2)?;
    graph.add_dependency(task2,  dep, task3)?;
    graph.add_task(task4);
    
    Ok(graph)
}