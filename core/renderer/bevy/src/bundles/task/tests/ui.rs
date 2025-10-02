#[cfg(test)]
pub mod tests {

	use super::super::super::TaskNodeMarker;
	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_task_spawner_ui_node_properties() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_task_test_app();

		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 5.0),
			"UI Test Task".to_string(),
		);

		// Spawn the task using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check Node component properties
		let mut node_query = world.query::<&Node>();
		let nodes: Vec<_> = node_query.iter(world).collect();
		assert_eq!(nodes.len(), 4, "Should have exactly four Node entities: one for the task, one for the content, one for the title, and one for the status");

		// Now query specifically for the TaskNodeMarker and Node
		let mut task_node_query = world.query::<(&TaskNodeMarker, &Node)>();
		let task_nodes: Vec<_> = task_node_query.iter(world).collect();
		assert_eq!(task_nodes.len(), 1, "Should have exactly one TaskNodeMarker entity");

		let node = task_nodes[0].1;
		assert_eq!(node.width, Val::Px(200.0), "Node width should match task size");
		assert_eq!(node.height, Val::Px(5.0), "Node height should match task size");
		assert_eq!(node.border, UiRect::all(Val::Px(1.5)), "Node should have 1.5px border");
		assert_eq!(node.align_items, AlignItems::Center, "Node should center align items");
		assert_eq!(
			node.justify_content,
			JustifyContent::Center,
			"Node should center justify content"
		);

		// Check BackgroundColor
		let mut bg_color_query = world.query::<(&TaskNodeMarker, &BackgroundColor)>();
		let bg_colors: Vec<_> = bg_color_query.iter(world).collect();
		assert_eq!(bg_colors.len(), 1, "Should have exactly one BackgroundColor entity");
		assert_eq!(bg_colors[0].1 .0, Color::WHITE, "Background should be white");

		// Check BorderColor
		let mut border_color_query = world.query::<(&TaskNodeMarker, &BorderColor)>();
		let border_colors: Vec<_> = border_color_query.iter(world).collect();
		assert_eq!(border_colors.len(), 1, "Should have exactly one BorderColor entity");
		assert_eq!(border_colors[0].1 .0, Color::BLACK, "Border should be black");

		// Check BorderRadius
		let mut border_radius_query = world.query::<(&TaskNodeMarker, &BorderRadius)>();
		let border_radii: Vec<_> = border_radius_query.iter(world).collect();
		assert_eq!(border_radii.len(), 1, "Should have exactly one BorderRadius entity");
		assert_eq!(border_radii[0].1.top_left, Val::Px(4.0), "Border radius should be 4px");

		Ok(())
	}
}
