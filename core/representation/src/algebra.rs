pub mod span;

pub use span::Span;
pub use span::Date;

use crate::graph::Graph;
use roadline_util::task::{Task, id::Id as TaskId};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for Algebra operations
#[derive(Error, Debug)]
pub enum AlgebraError {
    #[error("Graph error: {0}")]
    Graph(#[from] crate::graph::GraphError),
    #[error("Task {task_id:?} not found in graph")]
    TaskNotFound {  task_id: TaskId },
    #[error("Task {task_id:?} has invalid range specification")]
    InvalidRange {  task_id: TaskId },
    #[error("Task {task_id:?} references non-existent task {reference_id:?} in its range")]
    InvalidReference {  task_id: TaskId, reference_id: TaskId },
    #[error("Root task {task_id:?} must reference itself with +0 offset")]
    InvalidRootRange {  task_id: TaskId },
    #[error("Task {task_id:?} dependency not satisfied: dependency {dependency_id:?} must end before task starts")]
    DependencyNotSatisfied {  task_id: TaskId, dependency_id: TaskId },
}

/// Adds a duration to a date, returning a new date.
fn add_duration_to_date(date: Date, duration: std::time::Duration) -> Date {
    let datetime = date.inner();
    let duration_chrono = chrono::Duration::from_std(duration)
        .unwrap_or(chrono::Duration::zero()); // Fallback for invalid duration
    Date::new(datetime + duration_chrono)
}

/// A structure used to compute and store  the span algebra of a graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Algebra {
    /// The graph of the algebra is the graph of the tasks and dependencies.
    pub graph: Graph,
    /// The spans of the algebra are the spans of time for the tasks in the graph.
    pub spans: HashMap<TaskId, Span>,
}

impl Algebra {
    pub fn new(graph: Graph) -> Self {
        Self { 
            graph, 
            spans: HashMap::new(), 
        }
    }

    pub fn with_capacity(graph: Graph, capacity: usize) -> Self {
        Self { 
            graph, 
            spans: HashMap::with_capacity(capacity), 
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    /// Fills the spans for all tasks in the graph based on their range specifications.
    /// 
    /// Algorithm:
    /// 1. Check that the graph is a DAG (no cycles)
    /// 2. Get topological ordering of tasks
    /// 3. For each task in topological order:
    ///    a. Validate range specification
    ///    b. Compute start date based on reference task
    ///    c. Compute end date by adding duration to start date
    ///    d. Validate that all dependencies end before this task starts
    /// 4. Store computed spans
    pub fn fill_spans(&mut self, root_date: Date) -> Result<(), AlgebraError> {
        // Clear existing spans
        self.spans.clear();
        
        // Ensure graph is a DAG
        if !self.graph.is_dag()? {
            return Err(AlgebraError::Graph(crate::graph::GraphError::Internal(
                "Cannot compute spans for graph with cycles".into()
            )));
        }
        
        // Get topological ordering 
        let topo_order = self.graph.topological_sort()?;
        
        // Process tasks in topological order
        for  task_id in topo_order {
            self.compute_task_span( task_id, root_date)?;
        }
        
        Ok(())
    }
    
    /// Computes the span for a single task based on its range specification.
    fn compute_task_span(&mut self,  task_id: TaskId, root_date: Date) -> Result<(), AlgebraError> {
        let task = self.graph().arena().tasks().get(& task_id)
            .ok_or(AlgebraError::TaskNotFound {  task_id })?;
        
        // Extract range components
        let start_target_date = task.range.start.clone().into(); // Convert Start to TargetDate
        let end_duration: roadline_util::duration::Duration = task.range.end.clone().into(); // Convert End to Duration
        let end_duration: std::time::Duration = end_duration.into(); // Convert to std Duration
        
        // Compute start date
        let start_date = self.compute_start_date(&start_target_date, & task_id, root_date)?;
        
        // Compute end date by adding duration to start date
        let end_date = add_duration_to_date(start_date, end_duration);
        
        // Validate dependencies are satisfied
        self.validate_dependencies(task, start_date)?;
        
        // Store the computed span
        let span = Span::new(
            span::Start::new(start_date),
            span::End::new(end_date),
        );
        self.spans.insert( task_id, span);
        
        Ok(())
    }
    
    /// Computes the start date for a task based on its TargetDate specification.
    fn compute_start_date(
        &self, 
        target_date: &roadline_util::task::range::TargetDate, 
         task_id: &TaskId,
        root_date: Date
    ) -> Result<Date, AlgebraError> {
        let reference_id: TaskId = target_date.point_of_reference.clone().into();
        let duration: roadline_util::duration::Duration = target_date.duration.clone().into(); // Convert to Duration
        let duration: std::time::Duration = duration.into(); // Convert to std::time::Duration
        
        // Handle root tasks (self-reference)
        if reference_id == * task_id {
            // For root tasks, must be +0 offset
            if duration != std::time::Duration::from_secs(0) {
                return Err(AlgebraError::InvalidRootRange {  task_id: * task_id });
            }
            return Ok(root_date);
        }
        
        // For non-root tasks, find the referenced task's end date
        let reference_span = self.spans.get(&reference_id)
            .ok_or(AlgebraError::InvalidReference { 
                 task_id: * task_id, 
                reference_id 
            })?;
        
        // Start date = reference task's end date + offset duration
        let reference_end_date = reference_span.end.inner(); 
        Ok(add_duration_to_date(reference_end_date, duration))
    }
    
    /// Validates that all dependencies of a task end before the task starts.
    fn validate_dependencies(&self, task: &Task, task_start_date: Date) -> Result<(), AlgebraError> {
        let  task_id = *task.id();
        
        // Check dependencies from the graph
        let dependencies = self.graph.get_dependencies(& task_id);
        
        for  dep_id in dependencies {
            let dep_span = self.spans.get(& dep_id)
                .ok_or(AlgebraError::TaskNotFound {  task_id:  dep_id })?;
            
            // Dependency must end before or at the same time as task starts
            if dep_span.end.inner().inner() > task_start_date.inner() {
                return Err(AlgebraError::DependencyNotSatisfied { 
                     task_id, 
                    dependency_id:  dep_id 
                });
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use roadline_util::dependency::id::Id as DependencyId;
    use crate::graph::Graph;
    use roadline_util::task::{Task, range::{Start, End, PointOfReference, TargetDate}};
    use roadline_util::duration::Duration;
    use chrono::{DateTime, Utc};
    use std::collections::{HashMap, BTreeSet};
    use std::time::Duration as StdDuration;

    /// Creates a test date from an ISO string.
    fn test_date(iso_string: &str) -> Date {
        let datetime = DateTime::parse_from_rfc3339(iso_string)
            .expect("Valid datetime string")
            .with_timezone(&Utc);
        Date::new(datetime)
    }

    /// Creates a test task with the specified parameters.
    fn create_test_task(
        id: u8, 
        reference_id: u8, 
        offset_days: u64, 
        duration_days: u64
    ) -> Result<Task, Box<dyn std::error::Error>> {
        let id = TaskId::new(id);
        let reference_id = TaskId::new(reference_id);
        
        let start = Start::from(TargetDate {
            point_of_reference: PointOfReference::from(reference_id),
            duration: Duration::from(StdDuration::from_secs(offset_days * 24 * 60 * 60)),
        });
        
        let end = End::from(Duration::from(StdDuration::from_secs(duration_days * 24 * 60 * 60)));
        
        let range = roadline_util::task::Range::new(start, end);
        
        Ok(Task::new(
            id,
            roadline_util::task::Title::new_test(),
            BTreeSet::new(),
            BTreeSet::new(),
            roadline_util::task::Summary::new_test(),
            range,
        ))
    }

    /// Creates a simple test graph with one root task and one dependent task.
    fn create_simple_test_graph() -> Result<Graph, Box<dyn std::error::Error>> {
        let mut graph = Graph::new();
        
        // Root task: T1 starts at itself + 0, duration 30 days
        let task1 = create_test_task(1, 1, 0, 30)?;
        graph.add(task1)?;
        
        // Dependent task: T2 starts at T1 + 0, duration 15 days  
        let task2 = create_test_task(2, 1, 0, 15)?;
        graph.add(task2)?;
        
        Ok(graph)
    }

    /// Creates a complex test graph with multiple dependencies.
    fn create_complex_test_graph() -> Result<Graph, Box<dyn std::error::Error>> {
        let mut graph = Graph::new();
        
        // Root task: T1 starts at itself + 0, duration 30 days
        let task1 = create_test_task(1, 1, 0, 30)?;
        graph.add(task1)?;
        
        // T2 starts at T1 + 10 days, duration 20 days
        let task2 = create_test_task(2, 1, 10, 20)?;
        graph.add(task2)?;
        
        // T3 starts at T2 + 5 days, duration 15 days  
        let task3 = create_test_task(3, 2, 5, 15)?;
        graph.add(task3)?;
        
        Ok(graph)
    }

    #[test]
    fn test_simple_algebra_computation() -> Result<(), Box<dyn std::error::Error>> {
        let graph = create_simple_test_graph()?;
        let mut algebra = Algebra::new(graph);
        
        let root_date = test_date("2024-01-01T00:00:00Z");
        algebra.fill_spans(root_date)?;
        
        // Check task1 span
        let task1_id = TaskId::new(1);
        let task1_span = algebra.spans.get(&task1_id)
            .ok_or("task1 span should exist")?;
        
        assert_eq!(task1_span.start.inner(), root_date);
        
        // Task1 should end 30 days after start
        let expected_task1_end = test_date("2024-01-31T00:00:00Z");
        assert_eq!(task1_span.end.inner(), expected_task1_end);
        
        // Check task2 span  
        let task2_id = TaskId::new(2);
        let task2_span = algebra.spans.get(&task2_id)
            .ok_or("task2 span should exist")?;
        
        // Task2 starts at task1 end + 0 = 2024-01-31
        assert_eq!(task2_span.start.inner(), expected_task1_end);
        
        // Task2 should end 15 days after start
        let expected_task2_end = test_date("2024-02-15T00:00:00Z");
        assert_eq!(task2_span.end.inner(), expected_task2_end);
        
        Ok(())
    }

    #[test]
    fn test_complex_algebra_computation() -> Result<(), Box<dyn std::error::Error>> {
        let graph = create_complex_test_graph()?;
        let mut algebra = Algebra::new(graph);
        
        let root_date = test_date("2024-01-01T00:00:00Z");
        algebra.fill_spans(root_date)?;
        
        // Check all spans are computed
        assert_eq!(algebra.spans.len(), 3);
        
        let task1_id = TaskId::new(1);
        let task2_id = TaskId::new(2);
        let task3_id = TaskId::new(3);
        
        let task1_span = algebra.spans.get(&task1_id).unwrap();
        let task2_span = algebra.spans.get(&task2_id).unwrap();
        let task3_span = algebra.spans.get(&task3_id).unwrap();
        
        // Task1: 2024-01-01 to 2024-01-31
        assert_eq!(task1_span.start.inner(), test_date("2024-01-01T00:00:00Z"));
        assert_eq!(task1_span.end.inner(), test_date("2024-01-31T00:00:00Z"));
        
        // Task2: starts T1 end + 10 days = 2024-02-10, duration 20 days = ends 2024-03-01
        assert_eq!(task2_span.start.inner(), test_date("2024-02-10T00:00:00Z"));
        assert_eq!(task2_span.end.inner(), test_date("2024-03-01T00:00:00Z"));
        
        // Task3: starts T2 end + 5 days = 2024-03-06, duration 15 days = ends 2024-03-21
        assert_eq!(task3_span.start.inner(), test_date("2024-03-06T00:00:00Z"));
        assert_eq!(task3_span.end.inner(), test_date("2024-03-21T00:00:00Z"));
        
        Ok(())
    }

    #[test]
    fn test_root_task_invalid_offset() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = Graph::new();
        
        // Root task with non-zero offset (invalid)
        let bad_task = create_test_task(1, 1, 5, 30)?;
        graph.add(bad_task)?;
        
        let mut algebra = Algebra::new(graph);
        let root_date = test_date("2024-01-01T00:00:00Z");
        
        let result = algebra.fill_spans(root_date);
        assert!(result.is_err());
        
        if let Err(AlgebraError::InvalidRootRange {  task_id: err_task_id }) = result {
            assert_eq!(err_task_id,  TaskId::new(1));
        } else {
            panic!("Expected InvalidRootRange error");
        }
        
        Ok(())
    }

    #[test]
    fn test_cyclic_graph_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut tasks = HashMap::new();
        let mut graph = Graph::new();
        
        // Create two tasks that depend on each other (cycle)
        let task1 = create_test_task(1, 2, 0, 30)?;
        let task2 = create_test_task(2, 1, 0, 30)?;
        
        let task1_id = *task1.id();
        let task2_id = *task2.id();
        
        tasks.insert(task1_id, task1);
        tasks.insert(task2_id, task2);
        
        // Create cycle: task1 -> task2 -> task1
        let dep1_id = DependencyId::new(1);
        let dep2_id = DependencyId::new(2);
        
        graph.add_dependency(task1_id, dep1_id, task2_id)?;
        graph.add_dependency(task2_id, dep2_id, task1_id)?;
        
        let mut algebra = Algebra::new(graph);
        let root_date = test_date("2024-01-01T00:00:00Z");
        
        let result = algebra.fill_spans(root_date);
        assert!(result.is_err());
        
        // Should get a graph error about cycles
        assert!(matches!(result, Err(AlgebraError::Graph(_))));
        
        Ok(())
    }

    #[test]
    fn test_task_not_found_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = Graph::new();
        
        // Create task that references non-existent task
        let task1 = create_test_task(1, 100, 0, 30)?;
        graph.add(task1)?;
        
        // Add task to graph so it gets processed
        graph.add_task(TaskId::new(1));
        
        let mut algebra = Algebra::new(graph);
        let root_date = test_date("2024-01-01T00:00:00Z");
        
        let result = algebra.fill_spans(root_date);
        assert!(result.is_err());
        
        // Should get an InvalidReference error
        if let Err(AlgebraError::InvalidReference {  task_id, reference_id }) = result {
            assert_eq!( task_id, TaskId::new(1));
            assert_eq!(reference_id, TaskId::new(100));
        } else {
            panic!("Expected InvalidReference error, got: {:?}", result);
        }
        
        Ok(())
    }

    #[test] 
    fn test_dependency_not_satisfied() -> Result<(), Box<dyn std::error::Error>> {
        let mut tasks = HashMap::new();
        let mut graph = Graph::new();
        
        // Task1: duration 10 days  
        let task1 = create_test_task(1, 1, 0, 10)?;
        let task1_id = *task1.id();
        tasks.insert(task1_id, task1);
        
        // Task2: starts at task1 + 0 (so starts when task1 ends), duration 5 days
        let task2 = create_test_task(2, 1, 0, 5)?;
        let task2_id = *task2.id();
        tasks.insert(task2_id, task2);
        
        // Task3: starts at task1 + 5 days (so starts before task2 ends!), duration 5 days  
        let task3 = create_test_task(3, 1, 5, 5)?;
        let task3_id = *task3.id();
        tasks.insert(task3_id, task3);
        
        // Create dependencies: task3 depends on task2, but task3 starts before task2 ends
        let dep1_id = DependencyId::new(1);
        let dep2_id = DependencyId::new(2);
        
        graph.add_dependency(task2_id, dep1_id, task1_id)?;
        graph.add_dependency(task3_id, dep2_id, task2_id)?; // This should fail validation
        
        let mut algebra = Algebra::new(graph);
        let root_date = test_date("2024-01-01T00:00:00Z");
        
        let result = algebra.fill_spans(root_date);
        assert!(result.is_err());
        
        // Should get a DependencyNotSatisfied error
        if let Err(AlgebraError::DependencyNotSatisfied {  task_id, dependency_id }) = result {
            assert_eq!( task_id, task3_id);
            assert_eq!(dependency_id, task2_id);
        } else {
            panic!("Expected DependencyNotSatisfied error, got: {:?}", result);
        }
        
        Ok(())
    }

}