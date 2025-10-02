#[cfg(test)]
pub mod tests {

	use super::super::super::{TaskHoverable, TaskSize};
	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use crate::components::Task;
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_multiple_task_spawning() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_task_test_app();

		let params = TestTasksParams::new()
			.with_basic_task(
				TaskId::from(1),
				Vec3::new(100.0, 100.0, 0.0),
				Vec2::new(200.0, 75.0),
				"Task 1".to_string(),
			)
			.with_basic_task(
				TaskId::from(2),
				Vec3::new(200.0, 200.0, 0.0),
				Vec2::new(275.0, 60.0),
				"Task 2".to_string(),
			)
			.with_basic_task(
				TaskId::from(3),
				Vec3::new(300.0, 300.0, 0.0),
				Vec2::new(300.0, 70.0),
				"Task 3".to_string(),
			);

		// Spawn multiple tasks
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that all tasks were spawned
		let mut task_query = world.query::<&Task>();
		let tasks: Vec<_> = task_query.iter(world).collect();
		assert_eq!(tasks.len(), 3, "Should spawn exactly 3 Task entities");

		// Check that all UI components were spawned
		let mut task_hoverable_query = world.query::<&TaskHoverable>();
		let hoverables: Vec<_> = task_hoverable_query.iter(world).collect();
		assert_eq!(hoverables.len(), 3, "Should spawn exactly 3 TaskHoverable entities");

		let mut task_size_query = world.query::<&TaskSize>();
		let task_sizes: Vec<_> = task_size_query.iter(world).collect();
		assert_eq!(task_sizes.len(), 3, "Should spawn exactly 3 TaskSize entities");

		// Check that all content components were spawned
		let mut title_marker_query =
			world.query::<&crate::bundles::task::content::title::TitleMarker>();
		let title_markers: Vec<_> = title_marker_query.iter(world).collect();
		assert_eq!(title_markers.len(), 3, "Should spawn exactly 3 TitleMarker entities");

		let mut status_marker_query =
			world.query::<&crate::bundles::task::content::status::StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 3, "Should spawn exactly 3 StatusMarker entities");

		// Check that all text components were spawned
		let mut text_query = world.query::<&Text>();
		let texts: Vec<_> = text_query.iter(world).collect();
		assert_eq!(texts.len(), 6, "Should spawn exactly 6 Text entities (3 titles + 3 statuses)");

		Ok(())
	}
}
