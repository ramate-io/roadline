#[cfg(test)]
pub mod tests {

	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiNode};
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_task_spawner_anchor_relationship() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_task_test_app();

		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 50.0),
			"Anchor Test Task".to_string(),
		);

		// Spawn the task using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check AnchorUiNode components
		let mut anchor_query = world.query::<&AnchorUiNode>();
		let anchors: Vec<_> = anchor_query.iter(world).collect();
		assert_eq!(anchors.len(), 1, "Should have exactly one AnchorUiNode entity");

		// Check AnchorUiConfig components
		let mut anchor_config_query = world.query::<&AnchorUiConfig>();
		let anchor_configs: Vec<_> = anchor_config_query.iter(world).collect();
		assert_eq!(anchor_configs.len(), 1, "Should have exactly one AnchorUiConfig entity");

		let anchor_config = anchor_configs[0];
		assert_eq!(
			anchor_config.anchorpoint,
			AnchorPoint::middle(),
			"Anchor point should be middle"
		);
		assert!(anchor_config.offset.is_none(), "Anchor offset should be None");

		Ok(())
	}
}
