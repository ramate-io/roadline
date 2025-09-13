use bevy::prelude::*;
use chrono::{DateTime, Utc};
use roadline_bevy_renderer::{RoadlineRenderConfig, RoadlineRenderer};
use roadline_representation_core::graph::Graph;
use roadline_representation_core::grid_algebra::PreGridAlgebra;
use roadline_representation_core::range_algebra::{Date, PreRangeAlgebra};
use roadline_representation_core::reified::PreReified;
use roadline_util::duration::Duration;
use roadline_util::task::Id as TaskId;
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

fn main() -> Result<(), anyhow::Error> {
	println!("Creating Roadline Bevy Renderer Example");

	// Create a simple graph: Task1 -> Task2 -> Task3
	let mut graph = Graph::new();

	let task1 = create_test_task(1, 1, 0, 5, BTreeSet::new())?;
	let task2 = create_test_task(2, 1, 5, 3, BTreeSet::from_iter([1]))?;
	let task3 = create_test_task(3, 1, 8, 4, BTreeSet::from_iter([2]))?;

	graph.add(task1)?;
	graph.add(task2)?;
	graph.add(task3)?;

	println!("Graph created with {} tasks", graph.task_ids().count());

	// Build the representation layers
	let range_algebra = PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
	println!("Range algebra computed");

	let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;
	println!("Grid algebra computed");

	let reified = PreReified::new(grid_algebra).compute()?;
	println!(
		"Reified representation computed with {} tasks and {} connections",
		reified.task_count(),
		reified.connection_count()
	);

	// Create the Bevy renderer with custom config
	let config = RoadlineRenderConfig {
		unit_to_pixel_scale: 2.0,
		milestone_color: Color::srgb(0.3, 0.8, 0.2), // Green milestones
		edge_color: Color::srgb(0.7, 0.7, 0.9),      // Light blue edges
		milestone_radius: 12.0,
		edge_thickness: 3.0,
		..Default::default()
	};

	let renderer = RoadlineRenderer::with_config(config);
	println!("Renderer created with custom config");

	// Create and configure the Bevy app
	let mut app = renderer.create_app();
	println!("Bevy app created");

	// Render the reified data
	renderer.render(&mut app, reified)?;
	println!("Reified data rendered successfully");

	// Get visual bounds
	if let Some((min_x, max_x, min_y, max_y)) = renderer.get_visual_bounds(&app) {
		println!("Visual bounds: x=[{:.1}, {:.1}], y=[{:.1}, {:.1}]", min_x, max_x, min_y, max_y);
	}

	// Center and fit camera
	renderer.center_camera(&mut app);
	renderer.fit_camera_to_content(&mut app, 0.1); // 10% padding
	println!("Camera positioned and fitted to content");

	println!("Roadline Bevy Renderer example completed successfully!");
	println!("Note: This example demonstrates the renderer setup without running the actual Bevy app loop.");

	Ok(())
}
