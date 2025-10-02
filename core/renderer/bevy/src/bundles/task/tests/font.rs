#[cfg(test)]
pub mod tests {

	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use crate::components::Task;
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_task_spawning_with_custom_font_size() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_task_test_app();

		let params = TestTasksParams::new().with_custom_font_task(
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 5.0),
			"Custom Font Size Task".to_string(),
			16.0,
		);

		// Spawn the task using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Verify the task was spawned correctly
		let mut task_query = world.query::<&Task>();
		let tasks: Vec<_> = task_query.iter(world).collect();
		assert_eq!(tasks.len(), 1, "Should spawn exactly one Task entity");
		assert_eq!(tasks[0].task_id, TaskId::from(1), "Task ID should match");

		Ok(())
	}
}
