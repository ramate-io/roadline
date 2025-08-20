//! Common test utilities for graph tests.
//! 
//! This module provides helper functions for creating test data
//! to avoid repetitive error handling in test code.

use crate::graph::Graph;
use crate::frame::Frame;
use roadline_util::task::Task;
use roadline_util::dependency::Dependency;

/// Test frame with basic linear graph data.
/// Creates: task1 -> task2 -> task3 -> task4
pub fn create_linear_frame() -> Result<Frame, anyhow::Error> {
    let mut frame = Frame::new();
    
    // Add tasks
    frame.add_task(Task::test_from_id_string("task1")?);
    frame.add_task(Task::test_from_id_string("task2")?);
    frame.add_task(Task::test_from_id_string("task3")?);
    frame.add_task(Task::test_from_id_string("task4")?);
    
    // Add dependency
    frame.add_dependency(Dependency::test_from_id_string("dep1")?);
    
    Ok(frame)
}

/// Creates a linear graph from a frame.
/// Creates: task1 -> task2 -> task3 -> task4
pub fn create_linear_graph(frame: &Frame) -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let dep = &frame.dependencies[0];
    
    // task1 -> task2 -> task3 -> task4
    graph.add_dependency(&frame.tasks[0], dep, &frame.tasks[1])?;
    graph.add_dependency(&frame.tasks[1], dep, &frame.tasks[2])?;
    graph.add_dependency(&frame.tasks[2], dep, &frame.tasks[3])?;
    
    Ok(graph)
}

/// Test frame with branched graph data.
/// Creates: task1 -> [task2, task3] -> task4, task1 -> task5
pub fn create_branched_frame() -> Result<Frame, anyhow::Error> {
    let mut frame = Frame::new();
    
    // Add tasks
    frame.add_task(Task::test_from_id_string("task1")?);
    frame.add_task(Task::test_from_id_string("task2")?);
    frame.add_task(Task::test_from_id_string("task3")?);
    frame.add_task(Task::test_from_id_string("task4")?);
    frame.add_task(Task::test_from_id_string("task5")?);
    
    // Add dependency
    frame.add_dependency(Dependency::test_from_id_string("dep1")?);
    
    Ok(frame)
}

/// Creates a branched graph from a frame.
/// Creates: task1 -> [task2, task3] -> task4, task1 -> task5
pub fn create_branched_graph(frame: &Frame) -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let dep = &frame.dependencies[0];
    
    // task1 -> [task2, task3, task5]
    graph.add_dependency(&frame.tasks[0], dep, &frame.tasks[1])?;
    graph.add_dependency(&frame.tasks[0], dep, &frame.tasks[2])?;
    graph.add_dependency(&frame.tasks[0], dep, &frame.tasks[4])?;
    
    // [task2, task3] -> task4
    graph.add_dependency(&frame.tasks[1], dep, &frame.tasks[3])?;
    graph.add_dependency(&frame.tasks[2], dep, &frame.tasks[3])?;
    
    Ok(graph)
}

/// Test frame with complex graph data.
/// Creates a graph with multiple levels and branches for comprehensive testing.
pub fn create_complex_frame() -> Result<Frame, anyhow::Error> {
    let mut frame = Frame::new();
    
    // Add 10 tasks
    for i in 1..=10 {
        frame.add_task(Task::test_from_id_string(&format!("task{}", i))?);
    }
    
    // Add dependency
    frame.add_dependency(Dependency::test_from_id_string("dep1")?);
    
    Ok(frame)
}

/// Creates a complex graph from a frame.
/// Creates a graph with multiple levels and branches for comprehensive testing.
pub fn create_complex_graph(frame: &Frame) -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let dep = &frame.dependencies[0];
    
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
        graph.add_dependency(&frame.tasks[from_idx], dep, &frame.tasks[to_idx])?;
    }
    
    Ok(graph)
}

/// Test frame for acyclic graph testing.
pub fn create_acyclic_frame() -> Result<Frame, anyhow::Error> {
    let mut frame = Frame::new();
    
    // Add tasks
    frame.add_task(Task::test_from_id_string("task1")?);
    frame.add_task(Task::test_from_id_string("task2")?);
    frame.add_task(Task::test_from_id_string("task3")?);
    frame.add_task(Task::test_from_id_string("task4")?);
    
    // Add dependency
    frame.add_dependency(Dependency::test_from_id_string("dep1")?);
    
    Ok(frame)
}

/// Creates an acyclic graph from a frame.
pub fn create_acyclic_graph(frame: &Frame) -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let dep = &frame.dependencies[0];
    
    // Create DAG: task1 -> task2 -> task4, task1 -> task3 -> task4
    graph.add_dependency(&frame.tasks[0], dep, &frame.tasks[1])?;
    graph.add_dependency(&frame.tasks[0], dep, &frame.tasks[2])?;
    graph.add_dependency(&frame.tasks[1], dep, &frame.tasks[3])?;
    graph.add_dependency(&frame.tasks[2], dep, &frame.tasks[3])?;
    
    Ok(graph)
}

/// Test frame for cyclic graph testing.
pub fn create_cyclic_frame() -> Result<Frame, anyhow::Error> {
    let mut frame = Frame::new();
    
    // Add tasks
    frame.add_task(Task::test_from_id_string("task1")?);
    frame.add_task(Task::test_from_id_string("task2")?);
    frame.add_task(Task::test_from_id_string("task3")?);
    
    // Add dependency
    frame.add_dependency(Dependency::test_from_id_string("dep1")?);
    
    Ok(frame)
}

/// Creates a cyclic graph from a frame.
pub fn create_cyclic_graph(frame: &Frame) -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let dep = &frame.dependencies[0];
    
    // Create cycle: task1 -> task2 -> task3 -> task1
    graph.add_dependency(&frame.tasks[0], dep, &frame.tasks[1])?;
    graph.add_dependency(&frame.tasks[1], dep, &frame.tasks[2])?;
    graph.add_dependency(&frame.tasks[2], dep, &frame.tasks[0])?;
    
    Ok(graph)
}

/// Test frame for basic graph testing.
pub fn create_test_frame() -> Result<Frame, anyhow::Error> {
    let mut frame = Frame::new();
    
    // Add tasks
    frame.add_task(Task::test_from_id_string("task1")?);
    frame.add_task(Task::test_from_id_string("task2")?);
    frame.add_task(Task::test_from_id_string("task3")?);
    frame.add_task(Task::test_from_id_string("task4")?);
    
    // Add dependency
    frame.add_dependency(Dependency::test_from_id_string("dep1")?);
    
    Ok(frame)
}

/// Creates a test graph from a frame.
/// Creates: task1 -> task2 -> task3, task4 (isolated)
pub fn create_test_graph(frame: &Frame) -> Result<Graph, anyhow::Error> {
    let mut graph = Graph::new();
    let dep = &frame.dependencies[0];
    
    // Create graph: task1 -> task2 -> task3, task4 (isolated)
    graph.add_dependency(&frame.tasks[0], dep, &frame.tasks[1])?;
    graph.add_dependency(&frame.tasks[1], dep, &frame.tasks[2])?;
    graph.add_task(&frame.tasks[3]);
    
    Ok(graph)
}