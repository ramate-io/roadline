// Test utilities for creating roadline test data
use chrono::{DateTime, Utc};
use roadline_representation_core::graph::Graph;
use roadline_representation_core::grid_algebra::PreGridAlgebra;
use roadline_representation_core::range_algebra::{Date, PreRangeAlgebra};
use roadline_representation_core::reified::{PreReified, Reified as CoreReified, ReifiedConfig};
use roadline_util::duration::Duration;
use roadline_util::task::Id as TaskId;
use roadline_util::task::{
	range::{End, PointOfReference, Start, TargetDate},
	Task,
};
use std::collections::BTreeSet;
use std::time::Duration as StdDuration;

/// Creates a test date from an ISO string.
pub fn test_date(iso_string: &str) -> Date {
	let datetime = DateTime::parse_from_rfc3339(iso_string)
		.expect("Valid datetime string")
		.with_timezone(&Utc);
	Date::new(datetime)
}

/// Creates a test task with the specified parameters.
pub fn create_test_task(
	id: u8,
	reference_id: u8,
	offset_days: u64,
	duration_days: u64,
	dependencies: BTreeSet<u8>,
) -> Result<Task, anyhow::Error> {
	let id = TaskId::new(id);
	let reference_id = TaskId::new(reference_id);

	let start = Start::from(TargetDate {
		point_of_reference: PointOfReference::from(reference_id),
		duration: Duration::from(StdDuration::from_secs(offset_days * 24 * 60 * 60)),
	});

	let end = End::from(Duration::from(StdDuration::from_secs(duration_days * 24 * 60 * 60)));
	let range = roadline_util::task::Range::new(start, end);

	let mut task = Task::new(
		id,
		roadline_util::task::Title::new_test(),
		BTreeSet::new(),
		BTreeSet::new(),
		roadline_util::task::Summary::new_test(),
		range,
	);

	task.dependencies_mut()
		.extend(dependencies.into_iter().map(|id| TaskId::new(id)));
	Ok(task)
}

/// Creates a simple test graph with 3 connected tasks: Task1 -> Task2 -> Task3
pub fn create_simple_test_graph() -> Result<Graph, anyhow::Error> {
	let mut graph = Graph::new();

	let task1 = create_test_task(1, 1, 0, 5, BTreeSet::new())?;
	let task2 = create_test_task(2, 1, 5, 3, BTreeSet::from_iter([1]))?;
	let task3 = create_test_task(3, 1, 8, 4, BTreeSet::from_iter([2]))?;

	graph.add(task1)?;
	graph.add(task2)?;
	graph.add(task3)?;

	Ok(graph)
}

/// Creates a more complex test graph with parallel tasks and multiple dependencies
pub fn create_complex_test_graph() -> Result<Graph, anyhow::Error> {
	let mut graph = Graph::new();

	// Create a more complex dependency structure:
	// Task1 (foundation)
	// Task2 and Task3 depend on Task1 (parallel)
	// Task4 depends on both Task2 and Task3 (convergence)
	// Task5 depends on Task4 (final)
	let task1 = create_test_task(1, 1, 0, 3, BTreeSet::new())?;
	let task2 = create_test_task(2, 1, 3, 4, BTreeSet::from_iter([1]))?;
	let task3 = create_test_task(3, 1, 3, 5, BTreeSet::from_iter([1]))?;
	let task4 = create_test_task(4, 1, 8, 3, BTreeSet::from_iter([2, 3]))?;
	let task5 = create_test_task(5, 1, 11, 2, BTreeSet::from_iter([4]))?;

	graph.add(task1)?;
	graph.add(task2)?;
	graph.add(task3)?;
	graph.add(task4)?;
	graph.add(task5)?;

	Ok(graph)
}

/// Creates a reified representation from a graph with default configuration
pub fn create_reified_from_graph(graph: Graph) -> Result<CoreReified, anyhow::Error> {
	use roadline_representation_core::reified::{DownLanePadding, ReifiedUnit, Trim};

	// Use smaller trim values to avoid overflow
	let config = ReifiedConfig::default_config()
		.with_connection_trim(Trim::new(ReifiedUnit::new(1)))
		.with_inter_lane_padding(DownLanePadding::new(ReifiedUnit::new(1)));
	create_reified_from_graph_with_config(graph, config)
}

/// Creates a reified representation from a graph with custom configuration
pub fn create_reified_from_graph_with_config(
	graph: Graph,
	config: ReifiedConfig,
) -> Result<CoreReified, anyhow::Error> {
	let range_algebra = PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
	let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;
	let reified = PreReified::new(grid_algebra).with_config(config).compute()?;
	Ok(reified)
}

/// Creates a simple test reified representation for testing
pub fn create_simple_test_reified() -> Result<CoreReified, anyhow::Error> {
	let graph = create_simple_test_graph()?;
	create_reified_from_graph(graph)
}

/// Creates a complex test reified representation for testing
pub fn create_complex_test_reified() -> Result<CoreReified, anyhow::Error> {
	let graph = create_complex_test_graph()?;
	use roadline_representation_core::reified::{DownLanePadding, ReifiedUnit, Trim};

	let config = ReifiedConfig::default_config()
		.with_connection_trim(Trim::new(ReifiedUnit::new(1)))
		.with_inter_lane_padding(DownLanePadding::new(ReifiedUnit::new(1)));
	create_reified_from_graph_with_config(graph, config)
}

/// Validates that a reified representation has the expected structure
pub fn validate_reified_structure(
	reified: &CoreReified,
	expected_task_count: usize,
	expected_connection_count: usize,
) -> Result<(), anyhow::Error> {
	if reified.task_count() != expected_task_count {
		return Err(anyhow::anyhow!(
			"Expected {} tasks, found {}",
			expected_task_count,
			reified.task_count()
		));
	}

	if reified.connection_count() != expected_connection_count {
		return Err(anyhow::anyhow!(
			"Expected {} connections, found {}",
			expected_connection_count,
			reified.connection_count()
		));
	}

	// Validate that visual bounds are reasonable
	let (max_x, max_y) = reified.visual_bounds();
	if max_x.value() == 0 || max_y.value() == 0 {
		return Err(anyhow::anyhow!("Visual bounds should be non-zero"));
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_simple_graph_creation() -> Result<(), anyhow::Error> {
		let graph = create_simple_test_graph()?;
		assert_eq!(graph.task_ids().count(), 3);
		Ok(())
	}

	#[test]
	fn test_complex_graph_creation() -> Result<(), anyhow::Error> {
		let graph = create_complex_test_graph()?;
		assert_eq!(graph.task_ids().count(), 5);
		Ok(())
	}

	#[test]
	fn test_simple_reified_creation() -> Result<(), anyhow::Error> {
		let reified = create_simple_test_reified()?;
		validate_reified_structure(&reified, 3, 2)?;
		Ok(())
	}

	#[test]
	fn test_complex_reified_creation() -> Result<(), anyhow::Error> {
		let reified = create_complex_test_reified()?;
		validate_reified_structure(&reified, 5, 5)?; // 5 tasks, 5 dependencies total (1->2, 1->3, 2->4, 3->4, 4->5)
		Ok(())
	}
}
