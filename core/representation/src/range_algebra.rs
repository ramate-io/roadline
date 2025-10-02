pub mod span;

pub use span::Date;
pub use span::Span;

use crate::graph::Graph;
use roadline_util::dependency::{Dependency, Id as DependencyId};
use roadline_util::task::{id::Id as TaskId, Task};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for RangeAlgebra operations
#[derive(Error, Debug)]
pub enum RangeAlgebraError {
	#[error("Graph error: {0}")]
	Graph(#[from] crate::graph::GraphError),
	#[error("Task {task_id:?} not found in graph")]
	TaskNotFound { task_id: TaskId },
	#[error("Task {task_id:?} has invalid range specification")]
	InvalidRange { task_id: TaskId },
	#[error("Task {task_id:?} references non-existent task {reference_id:?} in its range")]
	InvalidReference { task_id: TaskId, reference_id: TaskId },
	#[error("Root task {task_id:?} must reference itself with +0 offset")]
	InvalidRootRange { task_id: TaskId },
	#[error("Task {task_id:?} dependency not satisfied: dependency {dependency_id:?} must end before task starts")]
	TooEarlyForDependency { task_id: TaskId, dependency_id: TaskId },
	#[error("No root tasks found in graph")]
	NoRootTasks,
	#[error("Root task {task_id:?} has invalid offset: {offset:?}. Only root tasks can self-reference their start date")]
	OnlyRootTasksCanSelfReference { task_id: TaskId, offset: std::time::Duration },
	#[error("Multiple errors occurred: {}", format_multiple_errors(.errors))]
	Multiple { errors: Vec<RangeAlgebraError> },
	#[error("Graph contains cycles: {}", format_cycles(.cycles))]
	GraphHasCycles { cycles: Vec<Vec<TaskId>> },
	#[error("Invalid date: {date:?}")]
	InvalidDate { date: String },
}

fn format_multiple_errors(errors: &[RangeAlgebraError]) -> String {
	errors
		.iter()
		.enumerate()
		.map(|(i, e)| format!("{}. {}", i + 1, e))
		.collect::<Vec<_>>()
		.join("; ")
}

fn format_cycles(cycles: &[Vec<TaskId>]) -> String {
	cycles
		.iter()
		.enumerate()
		.map(|(i, cycle)| format!("Cycle {}: {:?}", i + 1, cycle))
		.collect::<Vec<_>>()
		.join("; ")
}

/// Adds a duration to a date, returning a new date.
fn add_duration_to_date(date: Date, duration: std::time::Duration) -> Date {
	let datetime = date.inner();
	let duration_chrono = chrono::Duration::from_std(duration).unwrap_or(chrono::Duration::zero()); // Fallback for invalid duration
	Date::new(datetime + duration_chrono)
}

/// A mutable structure used to compute the range algebra of a graph.
/// Does not provide access to computed spans to prevent modification.
/// Must be consumed to create a `RangeAlgebra` for safe access to spans.
#[derive(Debug)]
pub struct PreRangeAlgebra {
	/// The graph of tasks and dependencies.
	graph: Graph,
	/// Internal spans storage - not accessible externally.
	spans: HashMap<TaskId, Span>,
}

impl PreRangeAlgebra {
	pub fn new(graph: Graph) -> Self {
		Self { graph, spans: HashMap::new() }
	}

	pub fn with_capacity(graph: Graph, capacity: usize) -> Self {
		Self { graph, spans: HashMap::with_capacity(capacity) }
	}

	pub fn graph(&self) -> &Graph {
		&self.graph
	}

	pub fn graph_mut(&mut self) -> &mut Graph {
		&mut self.graph
	}

	/// Computes the spans for all tasks in the graph and returns an immutable RangeAlgebra.
	///
	/// This method consumes the PreRangeAlgebra to ensure spans cannot be modified after computation.
	///
	/// Algorithm:
	/// 1. Check that the graph is a DAG (no cycles)
	/// 2. Get topological ordering of tasks
	/// 3. For each task in topological order:
	///    a. Validate range specification
	///    b. Compute start date based on reference task
	///    c. Compute end date by adding duration to start date
	///    d. Validate that all dependencies end before this task starts
	/// 4. Store computed spans and return immutable RangeAlgebra
	pub fn compute(mut self, root_date: Date) -> Result<RangeAlgebra, RangeAlgebraError> {
		// Clear existing spans
		self.spans.clear();

		// Ensure graph is a DAG by checking for cycles
		let cycles = self.graph.find_cycles()?;
		if !cycles.is_empty() {
			return Err(RangeAlgebraError::GraphHasCycles { cycles });
		}

		// Get topological ordering
		let topo_order = self.graph.topological_sort()?;

		// Process tasks in topological order, collecting all errors
		let mut errors = Vec::new();
		for task_id in topo_order {
			if let Err(e) = self.compute_task_span(task_id, root_date) {
				errors.push(e);
			}
		}

		// Return all collected errors if any occurred
		if !errors.is_empty() {
			return Err(RangeAlgebraError::Multiple { errors });
		}

		// Consume self and return immutable RangeAlgebra
		Ok(RangeAlgebra { graph: self.graph, spans: self.spans })
	}

	/// Computes the span for a single task based on its range specification.
	fn compute_task_span(
		&mut self,
		task_id: TaskId,
		root_date: Date,
	) -> Result<(), RangeAlgebraError> {
		let task = self
			.graph()
			.arena()
			.tasks()
			.get(&task_id)
			.ok_or(RangeAlgebraError::TaskNotFound { task_id })?;

		println!("task: {:?}", task);

		// Extract range components
		// Start duration
		let start_target_date = task.range.start.clone().into(); // Convert Start to TargetDate
		let start_duration: roadline_util::duration::Duration =
			task.range.start.duration().clone().into(); // Convert Start to Duration

		// End duration
		let end_duration: roadline_util::duration::Duration = task.range.end.clone().into(); // Convert End to Duration
		let end_duration: std::time::Duration = end_duration.into(); // Convert to std Duration

		println!("task_id: {:?}", task_id);
		println!("start_target_date: {:?}", start_target_date);
		println!("start_duration: {:?}", start_duration);
		println!("end_duration: {:?}", end_duration);

		// Compute start date
		let start_date = if task.is_root() {
			// Root tasks will ignore the reference and simply offset from the root date
			// This has the side-effect of allowing self-reference, which some users may prefer.
			add_duration_to_date(root_date, start_duration.into())
		} else {
			// For non-root tasks, use the reference and offset
			self.compute_non_root_start_date(&start_target_date, &task_id)?
		};

		// Compute end date by adding duration to start date
		let end_date = add_duration_to_date(start_date, end_duration);

		// Validate dependencies are satisfied
		self.validate_dependencies(task, start_date)?;

		// Store the computed span
		let span = Span::new(span::Start::new(start_date), span::End::new(end_date));
		self.spans.insert(task_id, span);

		Ok(())
	}

	/// Computes the start date for a task based on its TargetDate specification.
	fn compute_non_root_start_date(
		&self,
		target_date: &roadline_util::task::range::TargetDate,
		task_id: &TaskId,
	) -> Result<Date, RangeAlgebraError> {
		println!("target_date: {:?}", target_date);
		let reference_id: TaskId = target_date.point_of_reference.clone().into();
		let duration: roadline_util::duration::Duration = target_date.duration.clone().into(); // Convert to Duration
		let duration: std::time::Duration = duration.into(); // Convert to std::time::Duration

		// Handle root tasks with zero offset
		if reference_id == *task_id {
			return Err(RangeAlgebraError::OnlyRootTasksCanSelfReference {
				task_id: *task_id,
				offset: duration,
			});
		}

		// For non-root tasks, find the referenced task's end date
		let reference_span = self
			.spans
			.get(&reference_id)
			.ok_or(RangeAlgebraError::InvalidReference { task_id: *task_id, reference_id })?;

		println!("reference_id: {:?}", reference_id);
		println!("task_id: {:?}", task_id);
		println!("reference_span: {:?}", reference_span);
		println!("duration: {:?}", duration);

		// Start date = reference task's end date + offset duration
		let reference_end_date = reference_span.end.inner();
		Ok(add_duration_to_date(reference_end_date, duration))
	}

	/// Validates that all dependencies of a task end before the task starts.
	fn validate_dependencies(
		&self,
		task: &Task,
		task_start_date: Date,
	) -> Result<(), RangeAlgebraError> {
		let task_id = *task.id();

		// Check dependencies from the graph, collecting all errors
		let dependencies = self.graph.get_dependencies(&task_id);
		let mut errors = Vec::new();

		for dep_id in dependencies {
			let dep_span = match self.spans.get(&dep_id) {
				Some(span) => span,
				None => {
					errors.push(RangeAlgebraError::TaskNotFound { task_id: dep_id });
					continue;
				}
			};

			// Dependency must end before or at the same time as task starts
			if dep_span.end.inner().inner() > task_start_date.inner() {
				errors.push(RangeAlgebraError::TooEarlyForDependency {
					task_id,
					dependency_id: dep_id,
				});
			}
		}

		// Return all collected errors if any occurred
		if !errors.is_empty() {
			return Err(RangeAlgebraError::Multiple { errors });
		}

		Ok(())
	}
}

/// An immutable structure containing the computed range algebra of a graph.
/// Provides safe read-only access to computed spans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeAlgebra {
	/// The graph of tasks and dependencies.
	graph: Graph,
	/// The computed spans for all tasks in the graph.
	spans: HashMap<TaskId, Span>,
}

impl RangeAlgebra {
	/// Get a reference to the graph.
	pub fn graph(&self) -> &Graph {
		&self.graph
	}

	/// Get a reference to all computed spans.
	pub fn spans(&self) -> &HashMap<TaskId, Span> {
		&self.spans
	}

	/// Get the span for a specific task.
	pub fn span(&self, task_id: &TaskId) -> Option<&Span> {
		self.spans.get(task_id)
	}

	/// Get all task IDs that have computed spans.
	pub fn task_ids(&self) -> impl Iterator<Item = &TaskId> {
		self.spans.keys()
	}

	/// Get the number of tasks with computed spans.
	pub fn task_count(&self) -> usize {
		self.spans.len()
	}

	/// Check if a task has a computed span.
	pub fn has_span(&self, task_id: &TaskId) -> bool {
		self.spans.contains_key(task_id)
	}

	/// Gets the task for a given task id.
	pub fn task(&self, task_id: &TaskId) -> Option<&Task> {
		self.graph.task(task_id)
	}

	/// Gets the dependency for a given dependency id.
	pub fn dependency(&self, dependency_id: &DependencyId) -> Option<&Dependency> {
		self.graph.dependency(dependency_id)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::graph::Graph;
	use chrono::{DateTime, Utc};
	use roadline_util::duration::Duration;
	use roadline_util::task::{
		range::{End, PointOfReference, Start, TargetDate},
		Task,
	};
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
		duration_days: u64,
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
		dependencies: BTreeSet<u8>,
	) -> Result<Task, anyhow::Error> {
		let mut task = create_test_task(id, reference_id, offset_days, duration_days)?;
		task.dependencies_mut()
			.extend(dependencies.into_iter().map(|id| TaskId::new(id)));
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
		let pre_algebra = PreRangeAlgebra::new(graph);
		let root_date = test_date("2021-01-01T00:00:00Z");
		let algebra = pre_algebra.compute(root_date)?;

		// Verify we can access spans
		assert_eq!(algebra.task_count(), 2);
		assert!(algebra.has_span(&TaskId::new(1)));
		assert!(algebra.has_span(&TaskId::new(2)));

		Ok(())
	}

	#[test]
	fn test_complex_valid_graph() -> Result<(), anyhow::Error> {
		let graph = create_complex_valid_test_graph()?;
		let pre_algebra = PreRangeAlgebra::new(graph);
		let root_date = test_date("2021-01-01T00:00:00Z");
		let algebra = pre_algebra.compute(root_date)?;

		// Verify we can access spans
		assert_eq!(algebra.task_count(), 4);
		for i in 1..=4 {
			assert!(algebra.has_span(&TaskId::new(i)));
		}

		Ok(())
	}

	#[test]
	fn test_simple_invalid_graph() -> Result<(), anyhow::Error> {
		let graph = create_simple_invalid_test_graph()?;
		let pre_algebra = PreRangeAlgebra::new(graph);
		let root_date = test_date("2021-01-01T00:00:00Z");
		match pre_algebra.compute(root_date) {
			Ok(_) => panic!("Expected error, got Ok"),
			Err(RangeAlgebraError::Multiple { errors }) => {
				assert_eq!(errors.len(), 1);
				match &errors[0] {
					RangeAlgebraError::Multiple { errors: inner_errors } => {
						assert_eq!(inner_errors.len(), 1);
						match &inner_errors[0] {
							RangeAlgebraError::TooEarlyForDependency { task_id, dependency_id } => {
								assert_eq!(*task_id, TaskId::new(3));
								assert_eq!(*dependency_id, TaskId::new(2));
							}
							e => panic!("Unexpected inner error: {:?}", e),
						}
					}
					RangeAlgebraError::TooEarlyForDependency { task_id, dependency_id } => {
						assert_eq!(*task_id, TaskId::new(3));
						assert_eq!(*dependency_id, TaskId::new(2));
					}
					e => panic!("Unexpected error: {:?}", e),
				}
			}
			Err(e) => panic!("Unexpected error: {:?}", e),
		}

		Ok(())
	}

	#[test]
	fn test_multiple_dependency_errors() -> Result<(), anyhow::Error> {
		let mut graph = Graph::new();

		// Root task: T1 starts at itself + 0, duration 10 days
		let task1 = create_test_task_with_dependencies(1, 1, 0, 10, BTreeSet::new())?;
		graph.add(task1)?;

		// T2 starts at T1 + 0, duration 5 days
		let task2 = create_test_task_with_dependencies(2, 1, 0, 5, BTreeSet::from_iter([1]))?;
		graph.add(task2)?;

		// T3 starts at T1 + 0, duration 5 days - should conflict with T2 dependency
		let task3 = create_test_task_with_dependencies(3, 1, 0, 5, BTreeSet::from_iter([2]))?;
		graph.add(task3)?;

		// T4 starts at T1 + 2, duration 5 days - should also conflict with T2 dependency
		let task4 = create_test_task_with_dependencies(4, 1, 2, 5, BTreeSet::from_iter([2]))?;
		graph.add(task4)?;

		let pre_algebra = PreRangeAlgebra::new(graph);
		let root_date = test_date("2021-01-01T00:00:00Z");

		match pre_algebra.compute(root_date) {
			Ok(_) => panic!("Expected errors, got Ok"),
			Err(RangeAlgebraError::Multiple { errors }) => {
				// Should have multiple task errors (T3 and T4 both failing)
				assert_eq!(errors.len(), 2);

				// Each should be a dependency validation error
				for error in &errors {
					match error {
						RangeAlgebraError::Multiple { errors: inner_errors } => {
							assert_eq!(inner_errors.len(), 1);
							match &inner_errors[0] {
								RangeAlgebraError::TooEarlyForDependency {
									task_id,
									dependency_id,
								} => {
									assert!(
										*task_id == TaskId::new(3) || *task_id == TaskId::new(4)
									);
									assert_eq!(*dependency_id, TaskId::new(2));
								}
								e => panic!("Unexpected inner error: {:?}", e),
							}
						}
						e => panic!("Unexpected error: {:?}", e),
					}
				}
			}
			Err(e) => panic!("Unexpected error: {:?}", e),
		}

		Ok(())
	}

	#[test]
	fn test_cycle_detection_reports_all_cycles() -> Result<(), anyhow::Error> {
		let mut graph = Graph::new();

		// Create two separate cycles:
		// Cycle 1: T1 -> T2 -> T3 -> T1
		let task1 = create_test_task_with_dependencies(1, 2, 0, 10, BTreeSet::from_iter([3]))?;
		let task2 = create_test_task_with_dependencies(2, 3, 0, 10, BTreeSet::from_iter([1]))?;
		let task3 = create_test_task_with_dependencies(3, 1, 0, 10, BTreeSet::from_iter([2]))?;

		// Cycle 2: T4 -> T5 -> T4
		let task4 = create_test_task_with_dependencies(4, 5, 0, 10, BTreeSet::from_iter([5]))?;
		let task5 = create_test_task_with_dependencies(5, 4, 0, 10, BTreeSet::from_iter([4]))?;

		graph.add(task1)?;
		graph.add(task2)?;
		graph.add(task3)?;
		graph.add(task4)?;
		graph.add(task5)?;

		let pre_algebra = PreRangeAlgebra::new(graph);
		let root_date = test_date("2021-01-01T00:00:00Z");

		match pre_algebra.compute(root_date) {
			Ok(_) => panic!("Expected cycle error, got Ok"),
			Err(RangeAlgebraError::GraphHasCycles { cycles }) => {
				// Should detect cycles (exact number depends on cycle detection algorithm)
				assert!(!cycles.is_empty());
				println!("Detected cycles: {:?}", cycles);
			}
			Err(e) => panic!("Unexpected error: {:?}", e),
		}

		Ok(())
	}

	#[test]
	fn test_immutable_api_safety() -> Result<(), anyhow::Error> {
		let graph = create_simple_valid_test_graph()?;
		let pre_algebra = PreRangeAlgebra::new(graph);
		let root_date = test_date("2021-01-01T00:00:00Z");
		let algebra = pre_algebra.compute(root_date)?;

		// Verify immutable access works
		let task1_span = algebra.span(&TaskId::new(1)).unwrap();
		let task2_span = algebra.span(&TaskId::new(2)).unwrap();

		// Verify spans are computed correctly (T2 starts after T1 ends)
		assert!(task2_span.start.inner().inner() >= task1_span.end.inner().inner());

		// Test iteration over task IDs
		let task_ids: Vec<_> = algebra.task_ids().cloned().collect();
		assert_eq!(task_ids.len(), 2);
		assert!(task_ids.contains(&TaskId::new(1)));
		assert!(task_ids.contains(&TaskId::new(2)));

		Ok(())
	}
}
