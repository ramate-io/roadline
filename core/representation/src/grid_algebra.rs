pub mod cell;
pub mod lane;
pub mod stretch;

pub use cell::Cell;
pub use lane::LaneId;
pub use stretch::{Stretch, StretchRange, StretchUnit};

use crate::graph::Graph;
use crate::range_algebra::{Date, RangeAlgebra};
use chrono::{DateTime, Utc};
use roadline_util::dependency::{Dependency, Id as DependencyId};
use roadline_util::task::{Id as TaskId, Task};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for GridAlgebra operations
#[derive(Error, Debug)]
pub enum GridAlgebraError {
	#[error("Task {task_id:?} not found in range algebra")]
	TaskNotFound { task_id: TaskId },
	#[error("No tasks found in range algebra")]
	NoTasks,
	#[error("Invalid time range: start date {start:?} is after end date {end:?}")]
	InvalidTimeRange { start: Date, end: Date },
	#[error("Lane assignment failed for task {task_id:?}")]
	LaneAssignmentFailed { task_id: TaskId },
}

/// A mutable structure used to compute the grid layout of a range algebra.
/// Does not provide access to computed cells to prevent modification.
/// Must be consumed to create a `GridAlgebra` for safe access to layout.
#[derive(Debug)]
pub struct PreGridAlgebra {
	range_algebra: RangeAlgebra,
}

impl PreGridAlgebra {
	pub fn new(range_algebra: RangeAlgebra) -> Self {
		Self { range_algebra }
	}

	pub fn range_algebra(&self) -> &RangeAlgebra {
		&self.range_algebra
	}

	/// Computes the grid layout for all tasks and returns an immutable GridAlgebra.
	///
	/// This method consumes the PreGridAlgebra to ensure the layout cannot be modified after computation.
	///
	/// Algorithm:
	/// 1. Determine the optimal time scale unit from task durations
	/// 2. Calculate time boundaries (x-positions) for all tasks
	/// 3. Assign lanes (y-positions) using DFS with dependency locality
	/// 4. Create cells combining time stretches and lane assignments
	pub fn compute(self) -> Result<GridAlgebra, GridAlgebraError> {
		if self.range_algebra.task_count() == 0 {
			return Err(GridAlgebraError::NoTasks);
		}

		// Step 1: Determine time scale unit
		let time_unit = self.determine_time_scale_unit()?;

		// Step 2: Calculate task boundaries (x-positions)
		let task_stretches = self.calculate_task_stretches(time_unit)?;

		// Step 3: Assign lanes (y-positions) using DFS
		let lane_assignments = self.assign_lanes_dfs(&task_stretches)?;

		// Step 4: Create cells
		let mut tasks = HashMap::new();
		for (task_id, stretch) in task_stretches {
			let lane_id = lane_assignments
				.get(&task_id)
				.ok_or(GridAlgebraError::LaneAssignmentFailed { task_id })?;
			tasks.insert(task_id, Cell::new(stretch, *lane_id));
		}

		// Compute max x-axis and y-axis values during construction
		let max_x_axis = tasks.values().map(|cell| cell.stretch().end()).max().unwrap_or(0);

		let max_y_axis = tasks.values().map(|cell| cell.lane_id()).max().unwrap_or(0);

		let total_lanes =
			if max_y_axis == 0 && !tasks.is_empty() { 1 } else { max_y_axis as usize + 1 };

		Ok(GridAlgebra {
			range_algebra: self.range_algebra,
			time_unit,
			tasks,
			total_lanes,
			max_x_axis,
			max_y_axis,
		})
	}

	/// Determines the optimal time scale unit based on the average task duration.
	fn determine_time_scale_unit(&self) -> Result<StretchUnit, GridAlgebraError> {
		let spans = self.range_algebra.spans();
		if spans.is_empty() {
			return Err(GridAlgebraError::NoTasks);
		}

		// Calculate average task duration in seconds
		let total_duration_seconds: u64 = spans
			.values()
			.map(|span| {
				let start = span.start.inner().inner().timestamp();
				let end = span.end.inner().inner().timestamp();
				(end - start) as u64
			})
			.sum();

		let average_duration = total_duration_seconds / spans.len() as u64;
		Ok(StretchUnit::canonical_from_average_seconds(average_duration))
	}

	/// Calculates time boundaries and converts them to stretches.
	fn calculate_task_stretches(
		&self,
		time_unit: StretchUnit,
	) -> Result<HashMap<TaskId, Stretch>, GridAlgebraError> {
		let spans = self.range_algebra.spans();

		// Find the earliest start time to use as reference point
		let reference_time = spans
			.values()
			.map(|span| span.start.inner().inner().timestamp())
			.min()
			.ok_or(GridAlgebraError::NoTasks)?;

		let unit_seconds = time_unit.seconds() as i64;
		let mut task_stretches = HashMap::new();

		for (&task_id, span) in spans {
			let start_timestamp = span.start.inner().inner().timestamp();
			let end_timestamp = span.end.inner().inner().timestamp();
			// Convert to grid units relative to reference time
			let start_unit = ((start_timestamp - reference_time) / unit_seconds) as u8;
			let end_unit =
				((end_timestamp - reference_time + unit_seconds - 1) / unit_seconds) as u8; // Ceiling division

			let stretch_range = StretchRange::new(start_unit, end_unit.max(start_unit + 1)); // Ensure minimum 1 unit duration
			let stretch = Stretch::new(stretch_range, time_unit);

			task_stretches.insert(task_id, stretch);
		}

		Ok(task_stretches)
	}

	/// Assigns lanes using DFS with dependency locality and temporal overlap prevention.
	fn assign_lanes_dfs(
		&self,
		task_stretches: &HashMap<TaskId, Stretch>,
	) -> Result<HashMap<TaskId, LaneId>, GridAlgebraError> {
		let graph = self.range_algebra.graph();
		let mut lane_assignments = HashMap::new();
		let mut lane_occupancy: Vec<Vec<(TaskId, StretchRange)>> = Vec::new();

		// Sort root tasks by start time, then by TaskId for deterministic layout
		let mut roots: Vec<TaskId> = graph.root_tasks();
		roots.sort_by_key(|&task_id| {
			let start_time =
				task_stretches.get(&task_id).map(|stretch| stretch.start()).unwrap_or(0);
			(start_time, task_id) // Secondary sort by TaskId for determinism
		});

		// Calculate maximum width for each root's subtree
		let root_widths: Vec<(TaskId, usize)> = roots
			.iter()
			.map(|&root| {
				let max_width = self.calculate_subtree_max_width(root, task_stretches);
				(root, max_width)
			})
			.collect();

		// Space roots based on their subtree widths
		let mut current_lane = 0;
		for (root, max_width) in root_widths {
			self.dfs_assign_lane(
				root,
				current_lane,
				&mut lane_assignments,
				&mut lane_occupancy,
				task_stretches,
			)?;

			// Leave space for this subtree plus a buffer
			current_lane += max_width + 1; // +1 for buffer between subtrees
		}

		Ok(lane_assignments)
	}

	/// Calculates the maximum width of a subtree rooted at the given task using BFS.
	/// This determines how many lanes the subtree might need at its widest level.
	fn calculate_subtree_max_width(
		&self,
		root: TaskId,
		task_stretches: &HashMap<TaskId, Stretch>,
	) -> usize {
		use std::collections::{BTreeMap, BTreeSet, VecDeque};

		let graph = self.range_algebra.graph();
		let mut queue = VecDeque::new();
		let mut level_counts: BTreeMap<usize, Vec<TaskId>> = BTreeMap::new();
		let mut visited = BTreeSet::new();

		// Start BFS from root
		queue.push_back((root, 0)); // (task_id, depth)
		visited.insert(root);

		while let Some((task_id, depth)) = queue.pop_front() {
			// Add task to its level
			level_counts.entry(depth).or_insert_with(Vec::new).push(task_id);

			// Add children to queue in deterministic order
			let mut dependents = graph.get_dependents(&task_id);
			dependents.sort(); // Sort by TaskId for deterministic order

			for dependent in dependents {
				if !visited.contains(&dependent) {
					visited.insert(dependent);
					queue.push_back((dependent, depth + 1));
				}
			}
		}

		// Calculate maximum width considering temporal overlaps
		let mut max_width = 1; // At least 1 for the root

		// Process levels in deterministic order
		for (_, tasks_at_level) in level_counts {
			if tasks_at_level.is_empty() {
				continue;
			}

			// For each level, count how many tasks overlap temporally
			let overlapping_width = self.count_overlapping_tasks(&tasks_at_level, task_stretches);
			max_width = max_width.max(overlapping_width);
		}

		max_width
	}

	/// Counts how many tasks in a list overlap temporally and would need separate lanes.
	fn count_overlapping_tasks(
		&self,
		tasks: &[TaskId],
		task_stretches: &HashMap<TaskId, Stretch>,
	) -> usize {
		if tasks.len() <= 1 {
			return tasks.len();
		}

		// Sort tasks by start time
		let mut sorted_tasks: Vec<_> = tasks
			.iter()
			.filter_map(|&task_id| task_stretches.get(&task_id).map(|stretch| (task_id, stretch)))
			.collect();
		sorted_tasks.sort_by_key(|(_, stretch)| stretch.start());

		// Use a greedy algorithm to count minimum lanes needed
		let mut lane_end_times: Vec<u8> = Vec::new();

		for (_, stretch) in sorted_tasks {
			let start_time = stretch.start();

			// Find a lane that ends before this task starts
			let mut assigned = false;
			for lane_end in &mut lane_end_times {
				if *lane_end <= start_time {
					*lane_end = stretch.end();
					assigned = true;
					break;
				}
			}

			// If no lane is available, create a new one
			if !assigned {
				lane_end_times.push(stretch.end());
			}
		}

		lane_end_times.len()
	}

	/// DFS lane assignment with preference for parent lanes and spiral search.
	fn dfs_assign_lane(
		&self,
		task_id: TaskId,
		preferred_lane: usize,
		assignments: &mut HashMap<TaskId, LaneId>,
		occupancy: &mut Vec<Vec<(TaskId, StretchRange)>>,
		task_stretches: &HashMap<TaskId, Stretch>,
	) -> Result<(), GridAlgebraError> {
		// Skip if already assigned
		if assignments.contains_key(&task_id) {
			return Ok(());
		}

		let task_stretch =
			task_stretches.get(&task_id).ok_or(GridAlgebraError::TaskNotFound { task_id })?;

		// For tasks with multiple dependencies, prefer median of parent lanes
		let mut dependencies = self.range_algebra.graph().get_dependencies(&task_id);
		dependencies.sort(); // Sort for deterministic behavior

		let preferred_lane = if dependencies.len() > 1 {
			let mut parent_lanes: Vec<usize> = dependencies
				.iter()
				.filter_map(|&dep_id| assignments.get(&dep_id))
				.map(|lane_id| u8::from(*lane_id) as usize)
				.collect();

			if !parent_lanes.is_empty() {
				parent_lanes.sort();
				parent_lanes[parent_lanes.len() / 2] // Median
			} else {
				preferred_lane
			}
		} else {
			preferred_lane
		};

		// Find available lane using spiral search
		let lane_index = self.find_available_lane(preferred_lane, task_stretch.range(), occupancy);
		let lane_id = LaneId::from(lane_index as u8);

		// Ensure we have enough lanes allocated
		while occupancy.len() <= lane_index {
			occupancy.push(Vec::new());
		}

		// Assign task to lane
		assignments.insert(task_id, lane_id);
		occupancy[lane_index].push((task_id, *task_stretch.range()));

		// Recursively assign dependents, preferring this lane
		let mut dependents = self.range_algebra.graph().get_dependents(&task_id);
		dependents.sort(); // Sort for deterministic behavior

		for dependent in dependents {
			self.dfs_assign_lane(dependent, lane_index, assignments, occupancy, task_stretches)?;
		}

		Ok(())
	}

	/// Finds an available lane using spiral search around preferred lane.
	fn find_available_lane(
		&self,
		preferred: usize,
		stretch_range: &StretchRange,
		occupancy: &[Vec<(TaskId, StretchRange)>],
	) -> usize {
		// Check preferred lane first
		if preferred < occupancy.len()
			&& !self.has_temporal_overlap(preferred, stretch_range, occupancy)
		{
			return preferred;
		}

		// Spiral outward: preferred±1, preferred±2, ...
		for offset in 1..=255 {
			// Max lanes limited by u8
			// Try lane below preferred
			if let Some(candidate) = preferred.checked_sub(offset) {
				if candidate < occupancy.len()
					&& !self.has_temporal_overlap(candidate, stretch_range, occupancy)
				{
					return candidate;
				}
			}

			// Try lane above preferred
			let candidate = preferred + offset;
			if candidate < occupancy.len()
				&& !self.has_temporal_overlap(candidate, stretch_range, occupancy)
			{
				return candidate;
			}

			// If we've checked beyond existing lanes, we can use a new lane
			if candidate >= occupancy.len() {
				return candidate;
			}
		}

		// Fallback: create new lane
		occupancy.len()
	}

	/// Checks if a stretch would temporally overlap with existing tasks in a lane.
	fn has_temporal_overlap(
		&self,
		lane_index: usize,
		stretch_range: &StretchRange,
		occupancy: &[Vec<(TaskId, StretchRange)>],
	) -> bool {
		if lane_index >= occupancy.len() {
			return false;
		}

		occupancy[lane_index]
			.iter()
			.any(|(_, existing_range)| stretch_range.overlaps(existing_range))
	}
}

/// An immutable structure containing the computed grid layout of tasks.
/// Provides safe read-only access to computed cells and layout information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridAlgebra {
	range_algebra: RangeAlgebra,
	time_unit: StretchUnit,
	tasks: HashMap<TaskId, Cell>,
	total_lanes: usize,
	max_x_axis: u8,
	max_y_axis: u8,
}

impl GridAlgebra {
	/// Borrows the graph algebra.
	pub fn graph(&self) -> &Graph {
		self.range_algebra.graph()
	}

	/// Get a reference to the underlying range algebra.
	pub fn range_algebra(&self) -> &RangeAlgebra {
		&self.range_algebra
	}

	/// Get the time unit used for the grid.
	pub fn time_unit(&self) -> StretchUnit {
		self.time_unit
	}

	/// Get a reference to all computed cells.
	pub fn tasks(&self) -> &HashMap<TaskId, Cell> {
		&self.tasks
	}

	/// Get the cell for a specific task.
	pub fn task_cell(&self, task_id: &TaskId) -> Option<&Cell> {
		self.tasks.get(task_id)
	}

	/// Get all task IDs that have computed cells.
	pub fn task_ids(&self) -> impl Iterator<Item = &TaskId> {
		self.tasks.keys()
	}

	/// Get the number of tasks with computed cells.
	pub fn task_count(&self) -> usize {
		self.tasks.len()
	}

	/// Get the total number of lanes used.
	pub fn total_lanes(&self) -> usize {
		self.total_lanes
	}

	/// Gets the task for a given task id.
	pub fn task(&self, task_id: &TaskId) -> Option<&Task> {
		self.range_algebra.task(task_id)
	}

	/// Gets the dependency for a given dependency id.
	pub fn dependency(&self, dependency_id: &DependencyId) -> Option<&Dependency> {
		self.range_algebra.dependency(dependency_id)
	}

	/// Check if a task has a computed cell.
	pub fn has_task(&self, task_id: &TaskId) -> bool {
		self.tasks.contains_key(task_id)
	}

	/// Get all tasks in a specific lane.
	pub fn tasks_in_lane(&self, lane_id: u8) -> Vec<(&TaskId, &Cell)> {
		self.tasks.iter().filter(|(_, cell)| cell.lane_id() == lane_id).collect()
	}

	/// Get the maximum time unit used across all tasks.
	/// This is precomputed during grid construction for efficiency.
	pub fn max_time_unit(&self) -> u8 {
		self.max_x_axis
	}

	/// Get the maximum x-axis value (farthest end point of any stretch).
	/// This represents the rightmost edge of the grid.
	/// This is precomputed during grid construction for efficiency.
	pub fn max_x_axis(&self) -> u8 {
		self.max_x_axis
	}

	/// Get the maximum y-axis value (highest lane number used).
	/// This represents the bottom edge of the grid.
	/// This is precomputed during grid construction for efficiency.
	pub fn max_y_axis(&self) -> u8 {
		self.max_y_axis
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::graph::Graph;
	use crate::range_algebra::{Date, PreRangeAlgebra};
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

	/// Creates a simple test graph with dependency chain.
	fn create_simple_test_graph() -> Result<Graph, anyhow::Error> {
		let mut graph = Graph::new();

		// Root task: T1 starts at itself + 0, duration 30 days
		let task1 = create_test_task_with_dependencies(1, 1, 0, 30, BTreeSet::new())?;
		graph.add(task1)?;

		// T2 starts at T1 + 0, duration 15 days
		let task2 = create_test_task_with_dependencies(2, 1, 0, 15, BTreeSet::from_iter([1]))?;
		graph.add(task2)?;

		// T3 starts at T2 + 0, duration 10 days
		let task3 = create_test_task_with_dependencies(3, 2, 0, 10, BTreeSet::from_iter([2]))?;
		graph.add(task3)?;

		Ok(graph)
	}

	/// Creates a test graph with parallel tasks to test lane assignment.
	fn create_parallel_test_graph() -> Result<Graph, anyhow::Error> {
		let mut graph = Graph::new();

		// Root task: T1 starts at itself + 0, duration 30 days
		let task1 = create_test_task_with_dependencies(1, 1, 0, 30, BTreeSet::new())?;
		graph.add(task1)?;

		// T2 and T3 both depend on T1 and overlap in time
		let task2 = create_test_task_with_dependencies(2, 1, 0, 15, BTreeSet::from_iter([1]))?;
		graph.add(task2)?;

		let task3 = create_test_task_with_dependencies(3, 1, 5, 15, BTreeSet::from_iter([1]))?;
		graph.add(task3)?;

		// T4 depends on both T2 and T3, starts after both complete
		let task4 = create_test_task_with_dependencies(4, 3, 0, 10, BTreeSet::from_iter([2, 3]))?;
		graph.add(task4)?;

		Ok(graph)
	}

	#[test]
	fn test_simple_grid_computation() -> Result<(), anyhow::Error> {
		let graph = create_simple_test_graph()?;
		let range_algebra =
			PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
		let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

		// Verify basic properties
		assert_eq!(grid_algebra.task_count(), 3);
		assert!(grid_algebra.has_task(&TaskId::new(1)));
		assert!(grid_algebra.has_task(&TaskId::new(2)));
		assert!(grid_algebra.has_task(&TaskId::new(3)));

		// Verify time unit was determined (should be Weeks with canonical method)
		assert_eq!(grid_algebra.time_unit(), StretchUnit::Weeks);

		Ok(())
	}

	#[test]
	fn test_parallel_lane_assignment() -> Result<(), anyhow::Error> {
		let graph = create_parallel_test_graph()?;
		let range_algebra =
			PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
		let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

		// Verify all tasks are assigned
		assert_eq!(grid_algebra.task_count(), 4);

		// Get lane assignments
		let _task1_lane = grid_algebra.task_cell(&TaskId::new(1)).unwrap().lane_id();
		let task2_lane = grid_algebra.task_cell(&TaskId::new(2)).unwrap().lane_id();
		let task3_lane = grid_algebra.task_cell(&TaskId::new(3)).unwrap().lane_id();
		let task4_lane = grid_algebra.task_cell(&TaskId::new(4)).unwrap().lane_id();

		// T2 and T3 overlap in time and should be in different lanes
		assert_ne!(
			task2_lane, task3_lane,
			"Overlapping tasks T2 and T3 should be in different lanes"
		);

		// T4 depends on both T2 and T3, should be in a lane close to their median
		let parent_lanes = vec![task2_lane, task3_lane];
		let min_parent = *parent_lanes.iter().min().unwrap();
		let max_parent = *parent_lanes.iter().max().unwrap();
		assert!(
			task4_lane >= min_parent.saturating_sub(1) && task4_lane <= max_parent + 1,
			"Task T4 should be near its dependencies"
		);

		// Should have multiple lanes due to overlapping tasks
		assert!(grid_algebra.total_lanes() >= 2);

		Ok(())
	}

	#[test]
	fn test_dependency_locality() -> Result<(), anyhow::Error> {
		let graph = create_simple_test_graph()?;
		let range_algebra =
			PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
		let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

		// Get lane assignments for the dependency chain
		let task1_lane = grid_algebra.task_cell(&TaskId::new(1)).unwrap().lane_id();
		let task2_lane = grid_algebra.task_cell(&TaskId::new(2)).unwrap().lane_id();
		let task3_lane = grid_algebra.task_cell(&TaskId::new(3)).unwrap().lane_id();

		// Since T1->T2->T3 don't overlap in time, they should be in nearby lanes
		// With the new spacing, T1 starts at lane 0, and T2, T3 should follow close by
		assert!(task2_lane <= task1_lane + 1, "T2 should be close to T1");
		assert!(task3_lane <= task2_lane + 1, "T3 should be close to T2");

		// With one root and sequential tasks, should only need a few lanes
		assert!(grid_algebra.total_lanes() <= 3);

		Ok(())
	}

	#[test]
	fn test_time_scale_determination() -> Result<(), anyhow::Error> {
		let graph = create_simple_test_graph()?;
		let range_algebra =
			PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
		let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

		// With tasks of 30, 15, and 10 days, average duration is ~18.3 days
		// Canonical method: largest unit ≤ average is BiWeeks, then move to next smallest = Weeks
		assert_eq!(grid_algebra.time_unit(), StretchUnit::Weeks);

		// Verify stretches are computed correctly (in Week units)
		let task1_stretch = grid_algebra.task_cell(&TaskId::new(1)).unwrap().stretch();
		assert_eq!(task1_stretch.duration(), 5); // 30 days ≈ 5 weeks

		let task2_stretch = grid_algebra.task_cell(&TaskId::new(2)).unwrap().stretch();
		assert_eq!(task2_stretch.duration(), 3); // 15 days ≈ 3 weeks

		Ok(())
	}

	#[test]
	fn test_grid_algebra_api_safety() -> Result<(), anyhow::Error> {
		let graph = create_simple_test_graph()?;
		let range_algebra =
			PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
		let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

		// Verify immutable access works
		assert!(grid_algebra.has_task(&TaskId::new(1)));
		assert!(!grid_algebra.has_task(&TaskId::new(99)));

		// Verify lane queries work
		let lane_0_tasks = grid_algebra.tasks_in_lane(0);
		assert!(lane_0_tasks.len() >= 1); // At least the root task should be in lane 0

		// Verify max time unit, x-axis, and y-axis
		assert!(grid_algebra.max_time_unit() > 0);
		assert_eq!(grid_algebra.max_x_axis(), grid_algebra.max_time_unit());
		assert!(grid_algebra.max_y_axis() < grid_algebra.total_lanes() as u8);

		// Verify iteration
		let task_ids: Vec<_> = grid_algebra.task_ids().cloned().collect();
		assert_eq!(task_ids.len(), 3);

		Ok(())
	}

	#[test]
	fn test_subtree_width_based_spacing() -> Result<(), anyhow::Error> {
		let mut graph = Graph::new();

		// Create a complex graph with different subtree densities
		// Root 1: T1 -> [T2, T3] (wider subtree)
		let task1 = create_test_task_with_dependencies(1, 1, 0, 30, BTreeSet::new())?;
		graph.add(task1)?;

		let task2 = create_test_task_with_dependencies(2, 1, 0, 15, BTreeSet::from_iter([1]))?;
		graph.add(task2)?;

		let task3 = create_test_task_with_dependencies(3, 1, 5, 15, BTreeSet::from_iter([1]))?;
		graph.add(task3)?;

		// Root 2: T4 -> T5 (narrower subtree)
		let task4 = create_test_task_with_dependencies(4, 4, 0, 20, BTreeSet::new())?;
		graph.add(task4)?;

		let task5 = create_test_task_with_dependencies(5, 4, 0, 10, BTreeSet::from_iter([4]))?;
		graph.add(task5)?;

		let range_algebra =
			PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
		let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

		// Verify all tasks are assigned
		assert_eq!(grid_algebra.task_count(), 5);

		// Get lane assignments
		let task1_lane = grid_algebra.task_cell(&TaskId::new(1)).unwrap().lane_id();
		let task2_lane = grid_algebra.task_cell(&TaskId::new(2)).unwrap().lane_id();
		let task3_lane = grid_algebra.task_cell(&TaskId::new(3)).unwrap().lane_id();
		let task4_lane = grid_algebra.task_cell(&TaskId::new(4)).unwrap().lane_id();
		let task5_lane = grid_algebra.task_cell(&TaskId::new(5)).unwrap().lane_id();

		// T2 and T3 overlap and should be in different lanes
		assert_ne!(
			task2_lane, task3_lane,
			"Overlapping tasks T2 and T3 should be in different lanes"
		);

		// T4 (second root) should be spaced away from T1's subtree
		// The gap should account for T1's subtree width (T2 and T3 overlapping = 2 lanes + buffer)
		assert!(task4_lane >= task1_lane + 3, "T4 should be spaced based on T1's subtree width");

		// T5 should be close to its parent T4
		assert!(task5_lane <= task4_lane + 1, "T5 should be close to its parent T4");

		println!(
			"Lane assignments: T1={}, T2={}, T3={}, T4={}, T5={}",
			task1_lane, task2_lane, task3_lane, task4_lane, task5_lane
		);

		Ok(())
	}

	#[test]
	fn test_determinism_stress_test() -> Result<(), anyhow::Error> {
		const ITERATIONS: usize = 50;

		println!("Running determinism stress test with {} iterations...", ITERATIONS);

		// Store results from first iteration to compare against
		let mut baseline_results: Option<(
			Vec<(TaskId, u8)>, // simple grid
			Vec<(TaskId, u8)>, // parallel lanes
			Vec<(TaskId, u8)>, // dependency locality
			Vec<(TaskId, u8)>, // subtree spacing
		)> = None;

		for i in 0..ITERATIONS {
			if i % 10 == 0 {
				println!("  Iteration {}/{}", i + 1, ITERATIONS);
			}

			// Test 1: Simple grid computation
			let simple_results = {
				let mut graph = Graph::new();
				let task1 = create_test_task_with_dependencies(1, 1, 0, 10, BTreeSet::new())?;
				let task2 =
					create_test_task_with_dependencies(2, 1, 5, 10, BTreeSet::from_iter([1]))?;
				graph.add(task1)?;
				graph.add(task2)?;

				let range_algebra =
					PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
				let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

				let mut results = vec![];
				for task_id in [TaskId::new(1), TaskId::new(2)] {
					if let Some(cell) = grid_algebra.task_cell(&task_id) {
						results.push((task_id, u8::from(cell.lane_id())));
					}
				}
				results.sort();
				results
			};

			// Test 2: Parallel lane assignment
			let parallel_results = {
				let mut graph = Graph::new();
				// Root task: T1
				let task1 = create_test_task_with_dependencies(1, 1, 0, 30, BTreeSet::new())?;
				graph.add(task1)?;

				// T2 and T3 both depend on T1 and overlap in time
				let task2 =
					create_test_task_with_dependencies(2, 1, 0, 15, BTreeSet::from_iter([1]))?;
				graph.add(task2)?;

				let task3 =
					create_test_task_with_dependencies(3, 1, 5, 15, BTreeSet::from_iter([1]))?;
				graph.add(task3)?;

				// T4 depends on both T2 and T3, starts after both complete
				let task4 =
					create_test_task_with_dependencies(4, 3, 0, 10, BTreeSet::from_iter([2, 3]))?;
				graph.add(task4)?;

				let range_algebra =
					PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
				let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

				let mut results = vec![];
				for task_id in [TaskId::new(1), TaskId::new(2), TaskId::new(3), TaskId::new(4)] {
					if let Some(cell) = grid_algebra.task_cell(&task_id) {
						results.push((task_id, u8::from(cell.lane_id())));
					}
				}
				results.sort();
				results
			};

			// Test 3: Dependency locality
			let locality_results = {
				let mut graph = Graph::new();
				let task1 = create_test_task_with_dependencies(1, 1, 0, 5, BTreeSet::new())?;
				let task2 =
					create_test_task_with_dependencies(2, 1, 5, 5, BTreeSet::from_iter([1]))?;
				graph.add(task1)?;
				graph.add(task2)?;

				let range_algebra =
					PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
				let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

				let mut results = vec![];
				for task_id in [TaskId::new(1), TaskId::new(2)] {
					if let Some(cell) = grid_algebra.task_cell(&task_id) {
						results.push((task_id, u8::from(cell.lane_id())));
					}
				}
				results.sort();
				results
			};

			// Test 4: Subtree width-based spacing
			let subtree_results = {
				let mut graph = Graph::new();
				let task1 = create_test_task_with_dependencies(1, 1, 0, 30, BTreeSet::new())?;
				let task2 =
					create_test_task_with_dependencies(2, 1, 0, 15, BTreeSet::from_iter([1]))?;
				let task3 =
					create_test_task_with_dependencies(3, 1, 5, 15, BTreeSet::from_iter([1]))?;
				let task4 = create_test_task_with_dependencies(4, 4, 0, 20, BTreeSet::new())?;
				let task5 =
					create_test_task_with_dependencies(5, 4, 0, 10, BTreeSet::from_iter([4]))?;
				graph.add(task1)?;
				graph.add(task2)?;
				graph.add(task3)?;
				graph.add(task4)?;
				graph.add(task5)?;

				let range_algebra =
					PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
				let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

				let mut results = vec![];
				for task_id in
					[TaskId::new(1), TaskId::new(2), TaskId::new(3), TaskId::new(4), TaskId::new(5)]
				{
					if let Some(cell) = grid_algebra.task_cell(&task_id) {
						results.push((task_id, u8::from(cell.lane_id())));
					}
				}
				results.sort();
				results
			};

			let current_results =
				(simple_results, parallel_results, locality_results, subtree_results);

			if let Some(ref baseline) = baseline_results {
				assert_eq!(
					current_results.0, baseline.0,
					"Simple grid results differ at iteration {}: {:?} vs {:?}",
					i, current_results.0, baseline.0
				);
				assert_eq!(
					current_results.1, baseline.1,
					"Parallel lane results differ at iteration {}: {:?} vs {:?}",
					i, current_results.1, baseline.1
				);
				assert_eq!(
					current_results.2, baseline.2,
					"Dependency locality results differ at iteration {}: {:?} vs {:?}",
					i, current_results.2, baseline.2
				);
				assert_eq!(
					current_results.3, baseline.3,
					"Subtree spacing results differ at iteration {}: {:?} vs {:?}",
					i, current_results.3, baseline.3
				);
			} else {
				baseline_results = Some(current_results);
				println!("  Baseline established:");
				println!("    Simple: {:?}", baseline_results.as_ref().unwrap().0);
				println!("    Parallel: {:?}", baseline_results.as_ref().unwrap().1);
				println!("    Locality: {:?}", baseline_results.as_ref().unwrap().2);
				println!("    Subtree: {:?}", baseline_results.as_ref().unwrap().3);
			}
		}

		println!(
			"✅ All {} iterations produced identical results - algorithm is deterministic!",
			ITERATIONS
		);
		Ok(())
	}
}
