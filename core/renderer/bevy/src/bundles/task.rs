pub mod content;
pub use content::ContentSpawner;

use crate::components::{RenderState, Task};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiNode};
use roadline_util::task::Id as TaskId;

#[derive(Component)]
pub struct TaskHoverable;

#[derive(Component)]
pub struct TaskSize {
	pub size: Vec2,
}
pub struct TaskSpawnerData {
	pub task_id: TaskId,
	pub position: Vec3,
	pub size: Vec2,
	pub title: String,
	pub font_size: f32,
	pub completed: u32,
	pub total: u32,
}

/// Helper struct for spawning all task entities
pub struct TaskSpawner {
	pub data: TaskSpawnerData,
}

impl TaskSpawner {
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2, title: String) -> Self {
		Self {
			data: TaskSpawnerData {
				task_id,
				position,
				size,
				title,
				font_size: 6.0,
				completed: 3,
				total: 3,
			},
		}
	}

	pub fn with_font_size(mut self, font_size: f32) -> Self {
		self.data.font_size = font_size;
		self
	}

	pub fn spawn(
		self,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<ColorMaterial>>,
	) {
		let parent_entity = commands
			.spawn((
				TaskHoverable,
				TaskSize { size: self.data.size },
				Node {
					width: Val::Px(self.data.size.x),
					height: Val::Px(self.data.size.y),
					border: UiRect::all(Val::Px(1.5)),
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				BackgroundColor(Color::WHITE),
				BorderColor(Color::BLACK),
				BorderRadius::all(Val::Px(4.0)),
			))
			.id();

		let task_entity = commands
			.spawn((
				Task::new(self.data.task_id).with_ui_entity(parent_entity),
				RenderState::new(),
				Transform::from_translation(self.data.position),
				Visibility::Visible,
				RenderLayers::layer(2),
			))
			.id();

		// Add the anchor relationship
		commands.entity(parent_entity).insert((
			AnchorUiNode::to_entity(task_entity),
			AnchorUiConfig { anchorpoint: AnchorPoint::middle(), offset: None },
		));

		// Spawn content using the new imperative spawner
		ContentSpawner::new(self.data.title, self.data.completed, self.data.total).spawn(
			commands,
			meshes,
			materials,
			parent_entity,
			self.data.position, // Pass the world position
			self.data.size,     // Pass the task size
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;

	#[test]
	fn test_task_spawner_creation() -> Result<(), Box<dyn std::error::Error>> {
		let task_id = TaskId::from(1);
		let position = Vec3::new(100.0, 200.0, 0.0);
		let size = Vec2::new(200.0, 50.0);
		let title = "Test Task".to_string();

		let spawner = TaskSpawner::new(task_id, position, size, title.clone());

		assert_eq!(spawner.data.task_id, task_id);
		assert_eq!(spawner.data.position, position);
		assert_eq!(spawner.data.size, size);
		assert_eq!(spawner.data.title, title);
		assert_eq!(spawner.data.font_size, 6.0);
		assert_eq!(spawner.data.completed, 3);
		assert_eq!(spawner.data.total, 3);

		Ok(())
	}

	#[test]
	fn test_task_spawner_with_font_size() -> Result<(), Box<dyn std::error::Error>> {
		let task_id = TaskId::from(1);
		let position = Vec3::new(100.0, 200.0, 0.0);
		let size = Vec2::new(200.0, 50.0);
		let title = "Test Task".to_string();

		let spawner = TaskSpawner::new(task_id, position, size, title).with_font_size(12.0);

		assert_eq!(spawner.data.font_size, 12.0);

		Ok(())
	}

	struct TestTaskParams {
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
	}

	impl TestTaskParams {
		fn new() -> Self {
			Self {
				task_id: TaskId::from(1),
				position: Vec3::new(100.0, 200.0, 0.0),
				size: Vec2::new(200.0, 50.0),
				title: "Test Task".to_string(),
			}
		}

		fn spawn_system(
			mut commands: Commands,
			mut meshes: ResMut<Assets<Mesh>>,
			mut materials: ResMut<Assets<ColorMaterial>>,
		) {
			let params = Self::new();
			let spawner = TaskSpawner::new(params.task_id, params.position, params.size, params.title);
			spawner.spawn(&mut commands, &mut meshes, &mut materials);
		}
	}

	#[test]
	fn test_task_spawner_spawns_entities() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		// Spawn the task
		app.world_mut().run_system_once(TestTaskParams::spawn_system);

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

	struct TestComponentParams {
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
	}

	impl TestComponentParams {
		fn new() -> Self {
			Self {
				task_id: TaskId::from(42),
				position: Vec3::new(150.0, 250.0, 0.5),
				size: Vec2::new(300.0, 75.0),
				title: "Specific Task".to_string(),
			}
		}

		fn spawn_system(
			mut commands: Commands,
			mut meshes: ResMut<Assets<Mesh>>,
			mut materials: ResMut<Assets<ColorMaterial>>,
		) {
			let params = Self::new();
			let spawner = TaskSpawner::new(params.task_id, params.position, params.size, params.title);
			spawner.spawn(&mut commands, &mut meshes, &mut materials);
		}
	}

	#[test]
	fn test_task_spawner_sets_correct_components() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestComponentParams::new();

		// Spawn the task
		app.world_mut().run_system_once(TestComponentParams::spawn_system);

		let world = app.world_mut();

		// Check Task component values
		let mut task_query = world.query::<&Task>();
		let tasks: Vec<_> = task_query.iter(world).collect();
		assert_eq!(tasks.len(), 1, "Should have exactly one Task entity");

		let task = tasks[0];
		assert_eq!(task.task_id, params.task_id, "Task ID should match");
		assert!(task.ui_entity.is_some(), "Task should have a UI entity reference");

		// Check TaskSize component values
		let mut task_size_query = world.query::<&TaskSize>();
		let task_sizes: Vec<_> = task_size_query.iter(world).collect();
		assert_eq!(task_sizes.len(), 1, "Should have exactly one TaskSize entity");

		let task_size = task_sizes[0];
		assert_eq!(task_size.size, params.size, "Task size should match");

		// Check Transform component values
		let mut transform_query = world.query::<&Transform>();
		let transforms: Vec<_> = transform_query.iter(world).collect();
		assert_eq!(transforms.len(), 1, "Should have exactly one Transform entity");

		let transform = transforms[0];
		assert_eq!(transform.translation, params.position, "Transform position should match");

		Ok(())
	}

	struct TestUINodeParams {
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
	}

	impl TestUINodeParams {
		fn new() -> Self {
			Self {
				task_id: TaskId::from(1),
				position: Vec3::new(100.0, 200.0, 0.0),
				size: Vec2::new(200.0, 50.0),
				title: "UI Test Task".to_string(),
			}
		}

		fn spawn_system(
			mut commands: Commands,
			mut meshes: ResMut<Assets<Mesh>>,
			mut materials: ResMut<Assets<ColorMaterial>>,
		) {
			let params = Self::new();
			let spawner = TaskSpawner::new(params.task_id, params.position, params.size, params.title);
			spawner.spawn(&mut commands, &mut meshes, &mut materials);
		}
	}

	#[test]
	fn test_task_spawner_ui_node_properties() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestUINodeParams::new();

		// Spawn the task
		app.world_mut().run_system_once(TestUINodeParams::spawn_system);

		let world = app.world_mut();

		// Check Node component properties
		let mut node_query = world.query::<&Node>();
		let nodes: Vec<_> = node_query.iter(world).collect();
		assert_eq!(nodes.len(), 1, "Should have exactly one Node entity");

		let node = nodes[0];
		assert_eq!(node.width, Val::Px(params.size.x), "Node width should match task size");
		assert_eq!(node.height, Val::Px(params.size.y), "Node height should match task size");
		assert_eq!(node.border, UiRect::all(Val::Px(1.5)), "Node should have 1.5px border");
		assert_eq!(node.align_items, AlignItems::Center, "Node should center align items");
		assert_eq!(
			node.justify_content,
			JustifyContent::Center,
			"Node should center justify content"
		);

		// Check BackgroundColor
		let mut bg_color_query = world.query::<&BackgroundColor>();
		let bg_colors: Vec<_> = bg_color_query.iter(world).collect();
		assert_eq!(bg_colors.len(), 1, "Should have exactly one BackgroundColor entity");
		assert_eq!(bg_colors[0].0, Color::WHITE, "Background should be white");

		// Check BorderColor
		let mut border_color_query = world.query::<&BorderColor>();
		let border_colors: Vec<_> = border_color_query.iter(world).collect();
		assert_eq!(border_colors.len(), 1, "Should have exactly one BorderColor entity");
		assert_eq!(border_colors[0].0, Color::BLACK, "Border should be black");

		// Check BorderRadius
		let mut border_radius_query = world.query::<&BorderRadius>();
		let border_radii: Vec<_> = border_radius_query.iter(world).collect();
		assert_eq!(border_radii.len(), 1, "Should have exactly one BorderRadius entity");
		assert_eq!(border_radii[0].top_left, Val::Px(4.0), "Border radius should be 4px");

		Ok(())
	}

	struct TestAnchorParams {
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
	}

	impl TestAnchorParams {
		fn new() -> Self {
			Self {
				task_id: TaskId::from(1),
				position: Vec3::new(100.0, 200.0, 0.0),
				size: Vec2::new(200.0, 50.0),
				title: "Anchor Test Task".to_string(),
			}
		}

		fn spawn_system(
			mut commands: Commands,
			mut meshes: ResMut<Assets<Mesh>>,
			mut materials: ResMut<Assets<ColorMaterial>>,
		) {
			let params = Self::new();
			let spawner = TaskSpawner::new(params.task_id, params.position, params.size, params.title);
			spawner.spawn(&mut commands, &mut meshes, &mut materials);
		}
	}

	#[test]
	fn test_task_spawner_anchor_relationship() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestAnchorParams::new();

		// Spawn the task
		app.world_mut().run_system_once(TestAnchorParams::spawn_system);

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

	#[test]
	fn test_complete_task_spawning_pipeline() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		fn spawner_system(
			mut commands: Commands,
			mut meshes: ResMut<Assets<Mesh>>,
			mut materials: ResMut<Assets<ColorMaterial>>,
		) {
			let task_id = TaskId::from(42);
			let position = Vec3::new(150.0, 250.0, 0.5);
			let size = Vec2::new(300.0, 75.0);
			let title = "Complete Pipeline Test".to_string();

			let spawner = TaskSpawner::new(task_id, position, size, title);

			spawner.spawn(&mut commands, &mut meshes, &mut materials);
		}

		// Spawn the complete task
		app.world_mut().run_system_once(spawner_system);

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

	fn spawn_not_started_task_system(
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<ColorMaterial>>,
	) {
		let spawner = TaskSpawner::new(
			TaskId::from(1),
			Vec3::new(100.0, 100.0, 0.0),
			Vec2::new(200.0, 50.0),
			"Not Started Task".to_string(),
		);
		spawner.spawn(&mut commands, &mut meshes, &mut materials);
	}

	fn spawn_in_progress_task_system(
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<ColorMaterial>>,
	) {
		let spawner = TaskSpawner::new(
			TaskId::from(2),
			Vec3::new(200.0, 200.0, 0.0),
			Vec2::new(200.0, 50.0),
			"In Progress Task".to_string(),
		);
		spawner.spawn(&mut commands, &mut meshes, &mut materials);
	}

	fn spawn_completed_task_system(
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<ColorMaterial>>,
	) {
		let spawner = TaskSpawner::new(
			TaskId::from(3),
			Vec3::new(300.0, 300.0, 0.0),
			Vec2::new(200.0, 50.0),
			"Completed Task".to_string(),
		);
		spawner.spawn(&mut commands, &mut meshes, &mut materials);
	}

	#[test]
	fn test_task_spawning_with_different_statuses() -> Result<(), Box<dyn std::error::Error>> {
		// Test not started status
		let mut app1 = App::new();
		app1.add_plugins(MinimalPlugins);
		app1.world_mut().run_system_once(spawn_not_started_task_system);

		let world1 = app1.world_mut();
		let mut not_started_query =
			world1
				.query::<&crate::bundles::task::content::status::not_started::NotStartedStatusMarker>(
				);
		let not_started_markers: Vec<_> = not_started_query.iter(world1).collect();
		assert_eq!(
			not_started_markers.len(),
			1,
			"Should spawn NotStartedStatusMarker for not started task"
		);

		// Test in progress status
		let mut app2 = App::new();
		app2.add_plugins(MinimalPlugins);
		app2.world_mut().run_system_once(spawn_in_progress_task_system);

		let world2 = app2.world_mut();
		let mut in_progress_query =
			world2
				.query::<&crate::bundles::task::content::status::in_progress::InProgressStatusMarker>(
				);
		let in_progress_markers: Vec<_> = in_progress_query.iter(world2).collect();
		assert_eq!(
			in_progress_markers.len(),
			1,
			"Should spawn InProgressStatusMarker for in progress task"
		);

		// Test completed status
		let mut app3 = App::new();
		app3.add_plugins(MinimalPlugins);
		app3.world_mut().run_system_once(spawn_completed_task_system);

		let world3 = app3.world_mut();
		let mut completed_query =
			world3
				.query::<&crate::bundles::task::content::status::completed::CompletedStatusMarker>(
				);
		let completed_markers: Vec<_> = completed_query.iter(world3).collect();
		assert_eq!(
			completed_markers.len(),
			1,
			"Should spawn CompletedStatusMarker for completed task"
		);

		let mut check_mark_query =
			world3.query::<&crate::bundles::task::content::status::completed::CheckMarkMesh>();
		let check_marks: Vec<_> = check_mark_query.iter(world3).collect();
		assert_eq!(check_marks.len(), 1, "Should spawn CheckMarkMesh for completed task");

		Ok(())
	}

	fn spawn_multiple_tasks_system(
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<ColorMaterial>>,
	) {
		let tasks_data = vec![
			(
				TaskId::from(1),
				Vec3::new(100.0, 100.0, 0.0),
				Vec2::new(200.0, 50.0),
				"Task 1".to_string(),
			),
			(
				TaskId::from(2),
				Vec3::new(200.0, 200.0, 0.0),
				Vec2::new(250.0, 60.0),
				"Task 2".to_string(),
			),
			(
				TaskId::from(3),
				Vec3::new(300.0, 300.0, 0.0),
				Vec2::new(300.0, 70.0),
				"Task 3".to_string(),
			),
		];

		for (task_id, position, size, title) in tasks_data {
			let spawner = TaskSpawner::new(task_id, position, size, title);
			spawner.spawn(&mut commands, &mut meshes, &mut materials);
		}
	}

	#[test]
	fn test_multiple_task_spawning() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		// Spawn multiple tasks
		app.world_mut().run_system_once(spawn_multiple_tasks_system);

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

	fn spawn_custom_font_size_system(
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<ColorMaterial>>,
	) {
		let task_id = TaskId::from(1);
		let position = Vec3::new(100.0, 200.0, 0.0);
		let size = Vec2::new(200.0, 50.0);
		let title = "Custom Font Size Task".to_string();
		let custom_font_size = 16.0;

		let spawner =
			TaskSpawner::new(task_id, position, size, title).with_font_size(custom_font_size);
		spawner.spawn(&mut commands, &mut meshes, &mut materials);
	}

	struct TestCustomFontParams {
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
		font_size: f32,
	}

	impl TestCustomFontParams {
		fn new() -> Self {
			Self {
				task_id: TaskId::from(1),
				position: Vec3::new(100.0, 200.0, 0.0),
				size: Vec2::new(200.0, 50.0),
				title: "Custom Font Size Task".to_string(),
				font_size: 16.0,
			}
		}

		fn spawn_system(
			mut commands: Commands,
			mut meshes: ResMut<Assets<Mesh>>,
			mut materials: ResMut<Assets<ColorMaterial>>,
		) {
			let params = Self::new();
			let spawner = TaskSpawner::new(params.task_id, params.position, params.size, params.title)
				.with_font_size(params.font_size);
			spawner.spawn(&mut commands, &mut meshes, &mut materials);
		}
	}

	#[test]
	fn test_task_spawning_with_custom_font_size() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestCustomFontParams::new();

		// Spawn the task
		app.world_mut().run_system_once(TestCustomFontParams::spawn_system);

		let world = app.world_mut();

		// Verify the task was spawned correctly
		let mut task_query = world.query::<&Task>();
		let tasks: Vec<_> = task_query.iter(world).collect();
		assert_eq!(tasks.len(), 1, "Should spawn exactly one Task entity");
		assert_eq!(tasks[0].task_id, params.task_id, "Task ID should match");

		Ok(())
	}
}
