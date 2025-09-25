use super::super::TaskSpawner;
use crate::UiCameraMarker;
use bevy::prelude::*;
use bevy::render::mesh::MeshPlugin;
use bevy::render::view::VisibilityPlugin;
use bevy::scene::ScenePlugin;
use bevy::transform::TransformPlugin;
use bevy_ui_anchor::AnchorUiPlugin;
use roadline_util::task::Id as TaskId;

/// Helper function to set up an app with minimal plugins needed for task spawning
pub fn setup_task_test_app() -> App {
	let mut app = App::new();
	app.add_plugins((
		MinimalPlugins,
		AssetPlugin::default(),
		ScenePlugin,
		MeshPlugin,
		TransformPlugin,
		VisibilityPlugin,
		AnchorUiPlugin::<UiCameraMarker>::new(),
	))
	.init_asset::<ColorMaterial>()
	.init_asset::<Mesh>()
	.register_type::<Visibility>()
	.register_type::<InheritedVisibility>()
	.register_type::<ViewVisibility>()
	.register_type::<MeshMaterial2d<ColorMaterial>>();
	app
}

/// Unified test parameters builder for task spawning tests
#[derive(Clone)]
pub struct TestTasksParams {
	spawners: Vec<TaskSpawner>,
}

impl TestTasksParams {
	/// Constructs a new TestTasksParams with no spawners.
	pub fn new() -> Self {
		Self { spawners: vec![] }
	}

	/// Adds a spawner to the TestTasksParams.
	pub fn with_spawner(mut self, spawner: TaskSpawner) -> Self {
		self.spawners.push(spawner);
		self
	}

	/// Adds a basic task spawner with default parameters
	pub fn with_basic_task(
		self,
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
	) -> Self {
		let spawner = TaskSpawner::new(task_id, position, size, title, false, 3, 3);
		self.with_spawner(spawner)
	}

	/// Adds a task spawner with custom font size
	pub fn with_custom_font_task(
		self,
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
		font_size: f32,
	) -> Self {
		let spawner =
			TaskSpawner::new(task_id, position, size, title, false, 3, 3).with_font_size(font_size);
		self.with_spawner(spawner)
	}

	/// Adds a task spawner with specific status (completed/total counts)
	pub fn with_status_task(
		self,
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
		is_completed: bool,
		completed: u32,
		total: u32,
	) -> Self {
		let spawner =
			TaskSpawner::new(task_id, position, size, title, is_completed, completed, total);
		self.with_spawner(spawner)
	}

	/// Builds a closure that will spawn the tasks.
	pub fn build(
		self,
	) -> impl FnMut(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>) {
		move |mut commands: Commands,
		      mut meshes: ResMut<Assets<Mesh>>,
		      mut materials: ResMut<Assets<ColorMaterial>>| {
			// Because this is not fn once, each time this is called we need to clone the spawners.
			for spawner in self.spawners.to_owned() {
				spawner.spawn(&mut commands, &mut meshes, &mut materials);
			}
		}
	}

	/// Builds a closure that will spawn the tasks from a reference.
	pub fn as_build(
		&self,
	) -> impl FnMut(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>) {
		self.clone().build()
	}
}

impl Default for TestTasksParams {
	fn default() -> Self {
		Self::new()
	}
}
