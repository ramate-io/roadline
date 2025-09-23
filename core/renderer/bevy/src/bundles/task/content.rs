use bevy::prelude::*;
use bevy::ui::{GridTrack, Node, Val};

pub mod status;
pub use status::StatusSpawner;
pub mod title;
pub use title::TitleSpawner;

pub struct ContentSpawner {
	pub title: String,
	pub in_future: bool,
	pub completed: u32,
	pub total: u32,
}

impl ContentSpawner {
	pub fn new(title: String, in_future: bool, completed: u32, total: u32) -> Self {
		Self { title, in_future, completed, total }
	}

	pub fn spawn(
		self,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<ColorMaterial>>,
		parent: Entity,
		world_position: Vec3,
		task_size: Vec2,
	) {
		let content_entity = commands
			.spawn(Node {
				width: Val::Percent(100.0),  // Take full width of parent
				height: Val::Percent(100.0), // Take full height of parent
				display: Display::Grid,
				grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()], // 2fr 1fr grid
				grid_template_rows: vec![GridTrack::fr(1.0)],
				column_gap: Val::Px(8.0), // Single row
				align_content: AlignContent::Center,
				justify_content: JustifyContent::Start,
				justify_self: JustifySelf::Center,
				padding: UiRect::all(Val::Px(8.0)), // 8px padding inside the content area
				..default()
			})
			.id();

		// Spawn title
		TitleSpawner::new(self.title).spawn(commands, content_entity);

		// Spawn status
		StatusSpawner::new(self.in_future, self.completed, self.total).spawn(
			commands,
			meshes,
			materials,
			content_entity,
			world_position,
			task_size,
		);

		// Attach content to parent
		commands.entity(parent).add_child(content_entity);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::bundles::task::content::{status::StatusMarker, title::TitleMarker};
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use bevy::render::mesh::MeshPlugin;
	use bevy::render::view::VisibilityPlugin;
	use bevy::scene::ScenePlugin;
	use bevy::transform::TransformPlugin;

	/// Helper function to set up an app with minimal plugins for content spawning
	fn setup_content_test_app() -> App {
		let mut app = App::new();
		app.add_plugins((
			MinimalPlugins,
			AssetPlugin::default(),
			ScenePlugin,
			MeshPlugin,
			TransformPlugin,
			VisibilityPlugin,
		))
		.init_asset::<ColorMaterial>()
		.init_asset::<Mesh>()
		.register_type::<Visibility>()
		.register_type::<InheritedVisibility>()
		.register_type::<ViewVisibility>()
		.register_type::<MeshMaterial2d<ColorMaterial>>();
		app
	}

	#[test]
	fn test_content_spawner_creation() -> Result<(), Box<dyn std::error::Error>> {
		let title = "Test Content".to_string();
		let completed = 2;
		let total = 5;

		let spawner = ContentSpawner::new(title.clone(), true, completed, total);

		assert_eq!(spawner.title, title);
		assert_eq!(spawner.completed, completed);
		assert_eq!(spawner.total, total);

		Ok(())
	}

	#[derive(Clone)]
	struct TestContentParams {
		title: String,
		completed: u32,
		total: u32,
		world_position: Vec3,
		task_size: Vec2,
	}

	impl TestContentParams {
		fn new() -> Self {
			Self {
				title: "Content Test".to_string(),
				completed: 3,
				total: 7,
				world_position: Vec3::new(100.0, 200.0, 0.0),
				task_size: Vec2::new(200.0, 50.0),
			}
		}

		fn build(
			&self,
		) -> impl FnMut(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>) {
			let title = self.title.clone();
			let completed = self.completed;
			let total = self.total;
			let world_position = self.world_position;
			let task_size = self.task_size;
			move |mut commands: Commands,
			      mut meshes: ResMut<Assets<Mesh>>,
			      mut materials: ResMut<Assets<ColorMaterial>>| {
				let spawner = ContentSpawner::new(title.clone(), true, completed, total);
				let parent_entity = commands.spawn_empty().id();
				spawner.spawn(
					&mut commands,
					&mut meshes,
					&mut materials,
					parent_entity,
					world_position,
					task_size,
				);
			}
		}
	}

	#[test]
	fn test_content_spawner_spawns_content_node() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_content_test_app();

		let params = TestContentParams::new();

		// Spawn the content using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that content node was spawned with correct properties
		let mut node_query = world.query::<&Node>();
		let nodes: Vec<_> = node_query.iter(world).collect();

		// Should have at least one node (the content node)
		assert!(nodes.len() >= 1, "Should spawn at least one Node entity");

		// Find the content node (the one with grid display)
		let content_node = nodes.iter().find(|node| node.display == Display::Grid);
		assert!(content_node.is_some(), "Should spawn a content node with grid display");

		let content_node = content_node.unwrap();
		assert_eq!(content_node.width, Val::Percent(100.0), "Content node should take full width");
		assert_eq!(
			content_node.height,
			Val::Percent(100.0),
			"Content node should take full height"
		);
		assert_eq!(content_node.display, Display::Grid, "Content node should use grid display");
		assert_eq!(
			content_node.column_gap,
			Val::Px(8.0),
			"Content node should have 8px column gap"
		);
		assert_eq!(
			content_node.align_content,
			AlignContent::Center,
			"Content node should center align content"
		);
		assert_eq!(
			content_node.justify_content,
			JustifyContent::Start,
			"Content node should start justify content"
		);
		assert_eq!(
			content_node.justify_self,
			JustifySelf::Center,
			"Content node should center justify self"
		);
		assert_eq!(
			content_node.padding,
			UiRect::all(Val::Px(8.0)),
			"Content node should have 8px padding"
		);

		Ok(())
	}

	#[test]
	fn test_content_spawner_spawns_title_and_status() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_content_test_app();

		let params = TestContentParams {
			title: "Title and Status Test".to_string(),
			completed: 1,
			total: 3,
			world_position: Vec3::new(100.0, 200.0, 0.0),
			task_size: Vec2::new(200.0, 50.0),
		};

		// Spawn the content using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that title marker was spawned
		let mut title_marker_query = world.query::<&TitleMarker>();
		let title_markers: Vec<_> = title_marker_query.iter(world).collect();
		assert_eq!(title_markers.len(), 1, "Should spawn exactly one TitleMarker entity");

		// Check that status marker was spawned
		let mut status_marker_query = world.query::<&StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 1, "Should spawn exactly one StatusMarker entity");

		Ok(())
	}

	#[test]
	fn test_content_spawner_attaches_to_parent() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_content_test_app();

		let params = TestContentParams {
			title: "Parent Attachment Test".to_string(),
			completed: 0,
			total: 1,
			world_position: Vec3::new(100.0, 200.0, 0.0),
			task_size: Vec2::new(200.0, 50.0),
		};

		// Spawn the content using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that the parent entity has children
		let mut children_query = world.query::<&Children>();
		let children_components: Vec<_> = children_query.iter(world).collect();

		// Find the parent's children component
		let parent_children = children_components.iter().find(|children| {
			children.iter().any(|child| {
				// Check if this child has a grid node (our content node)
				if let Some(node) = world.get::<Node>(child) {
					node.display == Display::Grid
				} else {
					false
				}
			})
		});

		assert!(
			parent_children.is_some(),
			"Parent should have a child with grid display (content node)"
		);

		Ok(())
	}

	#[test]
	fn test_content_spawner_grid_layout() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_content_test_app();

		let params = TestContentParams {
			title: "Grid Layout Test".to_string(),
			completed: 2,
			total: 4,
			world_position: Vec3::new(100.0, 200.0, 0.0),
			task_size: Vec2::new(200.0, 50.0),
		};

		// Spawn the content using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check grid template columns
		let mut node_query = world.query::<&Node>();
		let nodes: Vec<_> = node_query.iter(world).collect();

		let content_node = nodes.iter().find(|node| node.display == Display::Grid);
		assert!(content_node.is_some(), "Should have a grid content node");

		let content_node = content_node.unwrap();
		assert_eq!(content_node.grid_template_columns.len(), 2, "Grid should have 2 columns");
		assert_eq!(
			content_node.grid_template_columns[0],
			GridTrack::auto(),
			"First column should be auto"
		);
		assert_eq!(
			content_node.grid_template_columns[1],
			GridTrack::auto(),
			"Second column should be auto"
		);

		assert_eq!(content_node.grid_template_rows.len(), 1, "Grid should have 1 row");
		assert_eq!(content_node.grid_template_rows[0], GridTrack::fr(1.0), "Row should be 1fr");

		Ok(())
	}
}
