#[cfg(test)]
pub mod tests {

	use super::super::super::{TaskHoverable, TaskSize};
	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use crate::components::Task;
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use bevy_ui_anchor::AnchorUiNode;
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_complete_task_spawning_pipeline() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_task_test_app();

		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(42),
			Vec3::new(150.0, 250.0, 0.5),
			Vec2::new(300.0, 75.0),
			"Complete Pipeline Test".to_string(),
		);

		// Spawn the complete task
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Verify the complete hierarchy was created correctly

		// 1. Check main task components
		let mut task_query = world.query::<&Task>();
		let tasks: Vec<_> = task_query.iter(world).collect();
		assert_eq!(tasks.len(), 1, "Should spawn exactly one Task entity");

		// 2. Check UI components
		let mut task_hoverable_query = world.query::<&TaskHoverable>();
		let hoverables: Vec<_> = task_hoverable_query.iter(world).collect();
		assert_eq!(hoverables.len(), 1, "Should spawn exactly one TaskHoverable entity");

		let mut task_size_query = world.query::<&TaskSize>();
		let task_sizes: Vec<_> = task_size_query.iter(world).collect();
		assert_eq!(task_sizes.len(), 1, "Should spawn exactly one TaskSize entity");

		// 3. Check content hierarchy
		let mut title_marker_query =
			world.query::<&crate::bundles::task::content::title::TitleMarker>();
		let title_markers: Vec<_> = title_marker_query.iter(world).collect();
		assert_eq!(title_markers.len(), 1, "Should spawn exactly one TitleMarker entity");

		let mut status_marker_query =
			world.query::<&crate::bundles::task::content::status::StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 1, "Should spawn exactly one StatusMarker entity");

		// 4. Check text content
		let mut text_query = world.query::<&Text>();
		let texts: Vec<_> = text_query.iter(world).collect();
		assert_eq!(texts.len(), 2, "Should spawn exactly two Text entities (title and status)");

		// 5. Check node hierarchy
		let mut node_query = world.query::<&Node>();
		let nodes: Vec<_> = node_query.iter(world).collect();
		assert!(
			nodes.len() >= 3,
			"Should spawn at least 3 Node entities (main, content, title, status)"
		);

		// 6. Check anchor relationship
		let mut anchor_query = world.query::<&AnchorUiNode>();
		let anchors: Vec<_> = anchor_query.iter(world).collect();
		assert_eq!(anchors.len(), 1, "Should spawn exactly one AnchorUiNode entity");

		Ok(())
	}
}
