#[cfg(test)]
pub mod tests {

	use super::super::super::{TaskHoverable, TaskSize};
	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use crate::components::{RenderState, Task};
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_task_spawner_spawns_entities() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_task_test_app();

		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 50.0),
			"Test Task".to_string(),
		);

		// Spawn the task using the builder
		app.world_mut().run_system_once(params.build())?;

		// Check that entities were spawned
		let world = app.world_mut();

		// Check for TaskHoverable entities
		let hoverable_count = world.query::<&TaskHoverable>().iter(world).count();
		assert_eq!(hoverable_count, 1, "Should spawn exactly one TaskHoverable entity");

		// Check for TaskSize entities
		let task_size_count = world.query::<&TaskSize>().iter(world).count();
		assert_eq!(task_size_count, 1, "Should spawn exactly one TaskSize entity");

		// Check for Task entities
		let task_count = world.query::<&Task>().iter(world).count();
		assert_eq!(task_count, 1, "Should spawn exactly one Task entity");

		// Check for RenderState entities
		let render_state_count = world.query::<&RenderState>().iter(world).count();
		assert_eq!(render_state_count, 1, "Should spawn exactly one RenderState entity");

		Ok(())
	}

	#[test]
	fn test_task_spawner_sets_correct_components() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_task_test_app();

		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(42),
			Vec3::new(150.0, 250.0, 0.5),
			Vec2::new(300.0, 75.0),
			"Specific Task".to_string(),
		);

		// Spawn the task using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check Task component values
		let mut task_query = world.query::<&Task>();
		let tasks: Vec<_> = task_query.iter(world).collect();
		assert_eq!(tasks.len(), 1, "Should have exactly one Task entity");

		let task = tasks[0];
		assert_eq!(task.task_id, TaskId::from(42), "Task ID should match");
		assert!(task.ui_entity.is_some(), "Task should have a UI entity reference");

		// Check TaskSize component values
		let mut task_size_query = world.query::<&TaskSize>();
		let task_sizes: Vec<_> = task_size_query.iter(world).collect();
		assert_eq!(task_sizes.len(), 1, "Should have exactly one TaskSize entity");

		let task_size = task_sizes[0];
		assert_eq!(task_size.size, Vec2::new(300.0, 75.0), "Task size should match");

		// Check Transform component values
		let mut transform_query = world.query::<(&Task, &Transform)>();
		let transforms: Vec<_> = transform_query.iter(world).collect();
		assert_eq!(transforms.len(), 1, "Should have exactly one Transform entity");

		let transform = transforms[0].1;
		assert_eq!(
			transform.translation,
			Vec3::new(150.0, 250.0, 0.5),
			"Transform position should match"
		);

		Ok(())
	}
}
