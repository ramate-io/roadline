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
    TooEarlyForDependency {  task_id: TaskId, dependency_id: TaskId },
    #[error("No root tasks found in graph")]
    NoRootTasks,
    #[error("Root task {task_id:?} has invalid offset: {offset:?}. Only root tasks can self-reference their start date")]
    OnlyRootTasksCanSelfReference {  task_id: TaskId, offset: std::time::Duration },
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
        let start_duration: roadline_util::duration::Duration = task.range.start.duration().clone().into(); // Convert Start to Duration
        let end_duration: roadline_util::duration::Duration = task.range.end.clone().into(); // Convert End to Duration
        let end_duration: std::time::Duration = end_duration.into(); // Convert to std Duration
        
        // Compute start date
        let start_date = if task.is_root() {
            // Root tasks will ignore the reference and simply offset from the root date
            // This has the sid-effect of allowing self-reference, which some users may prefer. 
            add_duration_to_date(root_date, start_duration.into())
        } else {
            // For non-root tasks, use the reference and offset
            self.compute_non_root_start_date(&start_target_date, & task_id)?
        };
        
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
    fn compute_non_root_start_date(
        &self, 
        target_date: &roadline_util::task::range::TargetDate, 
        task_id: &TaskId,
    ) -> Result<Date, AlgebraError> {
        let reference_id: TaskId = target_date.point_of_reference.clone().into();
        let duration: roadline_util::duration::Duration = target_date.duration.clone().into(); // Convert to Duration
        let duration: std::time::Duration = duration.into(); // Convert to std::time::Duration
        
        // Handle root tasks with zero offset
        if reference_id == * task_id {
            return Err(AlgebraError::OnlyRootTasksCanSelfReference {  task_id: * task_id, offset: duration });
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
                return Err(AlgebraError::TooEarlyForDependency { 
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

    use crate::graph::Graph;
    use roadline_util::task::{Task, range::{Start, End, PointOfReference, TargetDate}};
    use roadline_util::duration::Duration;
    use chrono::{DateTime, Utc};
    use std::collections::BTreeSet;
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
    ) -> Result<Task, anyhow::Error> {
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

    fn create_test_task_with_dependencies(
        id: u8, 
        reference_id: u8, 
        offset_days: u64, 
        duration_days: u64,
        dependencies: BTreeSet<u8>
    ) -> Result<Task, anyhow::Error> {
        let mut task = create_test_task(id, reference_id, offset_days, duration_days)?;
        task.dependencies_mut().extend(dependencies.into_iter().map(|id| TaskId::new(id)));
        Ok(task)
    }

    /// Creates a simple test graph with one root task and one dependent task.
    fn create_simple_valid_test_graph() -> Result<Graph, anyhow::Error> {
        let mut graph = Graph::new();
        
        // Root task: T1 starts at itself + 0, duration 30 days
        let task1 = create_test_task_with_dependencies(1, 1, 0, 30, BTreeSet::new())?;
        graph.add(task1)?;
        
        // Dependent task: T2 starts at T1 + 0, duration 15 days  
        let task2 = create_test_task_with_dependencies(2, 1, 0, 15, BTreeSet::from_iter([1]))?;
        graph.add(task2)?;
        
        Ok(graph)
    }

    /// Creates a complex test graph with multiple dependencies.
    fn create_complex_valid_test_graph() -> Result<Graph, anyhow::Error> {
        let mut graph = Graph::new();
        
        // Root task: T1 starts at itself + 0, duration 30 days
        let task1 = create_test_task_with_dependencies(1, 1, 0, 30, BTreeSet::new())?;
        graph.add(task1)?;
        
        // T2 starts at T1 + 10 days, duration 20 days
        let task2 = create_test_task_with_dependencies(2, 1, 10, 20, BTreeSet::from_iter([1]))?;
        graph.add(task2)?;
        
        // T3 starts at T2 + 5 days, duration 15 days  
        let task3 = create_test_task_with_dependencies(3, 2, 5, 15, BTreeSet::from_iter([2]))?;
        graph.add(task3)?;

        // T4 starts at T1 + 5 days, duration 15 days  
        let task4 = create_test_task_with_dependencies(4, 1, 5, 15, BTreeSet::from_iter([1]))?;
        graph.add(task4)?;
        
        Ok(graph)
    }

    fn create_simple_invalid_test_graph() -> Result<Graph, anyhow::Error> {
        let mut graph = Graph::new();
        
        // task1 is a root task
        let task1 = create_test_task_with_dependencies(1, 1, 0, 30, BTreeSet::new())?;
        graph.add(task1)?;

        // task2 depends on task1 and is placed after task1
        let task2 = create_test_task_with_dependencies(2, 1, 0, 15, BTreeSet::from_iter([1]))?;
        graph.add(task2)?;

        // task3 depends on task2 but is place relative to task1, beginning before task2 ends
        let task3 = create_test_task_with_dependencies(3, 1, 5, 15, BTreeSet::from_iter([2]))?;
        graph.add(task3)?;
        
        Ok(graph)
    }

    #[test]
    fn test_simple_valid_graph() -> Result<(), anyhow::Error> {
        let graph = create_simple_valid_test_graph()?;
        let mut algebra = Algebra::new(graph);
        let root_date = test_date("2021-01-01T00:00:00Z");
        algebra.fill_spans(root_date)?;

        Ok(())
    }

    #[test]
    fn test_complex_valid_graph() -> Result<(), anyhow::Error> {
        let graph = create_complex_valid_test_graph()?;
        let mut algebra = Algebra::new(graph);
        let root_date = test_date("2021-01-01T00:00:00Z");
        algebra.fill_spans(root_date)?;

        Ok(())
    }

    #[test]
    fn test_simple_invalid_graph() -> Result<(), anyhow::Error> {
        let graph = create_simple_invalid_test_graph()?;
        let mut algebra = Algebra::new(graph);
        let root_date = test_date("2021-01-01T00:00:00Z");
        match algebra.fill_spans(root_date) {
            Ok(_) => panic!("Expected error, got Ok"),
            Err(AlgebraError::TooEarlyForDependency { task_id, dependency_id }) => {
                assert_eq!(task_id, TaskId::new(3));
                assert_eq!(dependency_id, TaskId::new(2));
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        Ok(())
    }
   

}