pub mod completed;
pub mod in_progress;
pub mod missed;
pub mod not_started;

pub use completed::CompletedStatusSpawner;
pub use in_progress::InProgressStatusSpawner;
pub use missed::MissedStatusSpawner;
pub use not_started::NotStartedStatusSpawner;

use bevy::prelude::*;

#[derive(Component)]
pub struct StatusMarker;

pub enum StatusSpawner {
	NotStarted(NotStartedStatusSpawner),
	InProgress(InProgressStatusSpawner),
	Completed(CompletedStatusSpawner),
	Missed(MissedStatusSpawner),
}

impl StatusSpawner {
	pub fn new(in_future: bool, completed: u32, total: u32) -> Self {
		if completed == total {
			// Completed
			Self::Completed(CompletedStatusSpawner::new(completed, total))
		} else if !in_future && (completed != total) {
			// Missed
			Self::Missed(MissedStatusSpawner::new(completed, total))
		} else if completed == 0 {
			// Not started
			Self::NotStarted(NotStartedStatusSpawner::new(total))
		} else {
			// In progress
			Self::InProgress(InProgressStatusSpawner::new(completed, total))
		}
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
		match self {
			StatusSpawner::NotStarted(spawner) => spawner.spawn(commands, parent),
			StatusSpawner::InProgress(spawner) => spawner.spawn(commands, parent),
			StatusSpawner::Completed(spawner) => {
				spawner.spawn(commands, meshes, materials, parent, world_position, task_size)
			}
			StatusSpawner::Missed(spawner) => spawner.spawn(commands, parent),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::bundles::task::content::status::{
		completed::CompletedStatusMarker, in_progress::InProgressStatusMarker,
		not_started::NotStartedStatusMarker,
	};
	use bevy::ecs::system::RunSystemOnce;
	use bevy::render::mesh::MeshPlugin;
	use bevy::render::view::VisibilityPlugin;
	use bevy::scene::ScenePlugin;
	use bevy::transform::TransformPlugin;

	/// Helper function to set up an app with minimal plugins for status spawning
	fn setup_status_test_app() -> App {
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
	fn test_status_spawner_not_started() -> Result<(), Box<dyn std::error::Error>> {
		let completed = 0;
		let total = 5;

		let spawner = StatusSpawner::new(true, completed, total);

		match spawner {
			StatusSpawner::NotStarted(not_started_spawner) => {
				assert_eq!(
					not_started_spawner.total, total,
					"NotStarted spawner should have correct total"
				);
			}
			_ => return Err("Expected NotStarted status spawner".into()),
		}

		Ok(())
	}

	#[test]
	fn test_status_spawner_in_progress() -> Result<(), Box<dyn std::error::Error>> {
		let completed = 2;
		let total = 5;

		let spawner = StatusSpawner::new(true, completed, total);

		match spawner {
			StatusSpawner::InProgress(in_progress_spawner) => {
				assert_eq!(
					in_progress_spawner.completed, completed,
					"InProgress spawner should have correct completed count"
				);
				assert_eq!(
					in_progress_spawner.total, total,
					"InProgress spawner should have correct total"
				);
			}
			_ => return Err("Expected InProgress status spawner".into()),
		}

		Ok(())
	}

	#[test]
	fn test_status_spawner_completed() -> Result<(), Box<dyn std::error::Error>> {
		let completed = 5;
		let total = 5;

		let spawner = StatusSpawner::new(true, completed, total);

		match spawner {
			StatusSpawner::Completed(completed_spawner) => {
				assert_eq!(
					completed_spawner.completed, completed,
					"Completed spawner should have correct completed count"
				);
				assert_eq!(
					completed_spawner.total, total,
					"Completed spawner should have correct total"
				);
			}
			_ => return Err("Expected Completed status spawner".into()),
		}

		Ok(())
	}

	#[test]
	fn test_status_spawner_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
		// Test edge case: completed = 0, total = 0 (should be not started)
		let spawner = StatusSpawner::new(true, 0, 0);
		match spawner {
			StatusSpawner::Completed(_) => {}
			_ => return Err("Expected Completed for 0/0".into()),
		}

		// Test edge case: completed = 1, total = 1 (should be completed)
		let spawner = StatusSpawner::new(true, 1, 1);
		match spawner {
			StatusSpawner::Completed(_) => {}
			_ => return Err("Expected Completed for 1/1".into()),
		}

		// Test edge case: completed = 1, total = 2 (should be in progress)
		let spawner = StatusSpawner::new(true, 1, 2);
		match spawner {
			StatusSpawner::InProgress(_) => {}
			_ => return Err("Expected InProgress for 1/2".into()),
		}

		Ok(())
	}

	#[derive(Clone)]
	struct TestStatusParams {
		completed: u32,
		total: u32,
		world_position: Vec3,
		task_size: Vec2,
	}

	impl TestStatusParams {
		fn new() -> Self {
			Self {
				completed: 0,
				total: 3,
				world_position: Vec3::new(100.0, 200.0, 0.0),
				task_size: Vec2::new(200.0, 5.0),
			}
		}

		fn build(
			&self,
		) -> impl FnMut(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>) {
			let completed = self.completed;
			let total = self.total;
			let world_position = self.world_position;
			let task_size = self.task_size;
			move |mut commands: Commands,
			      mut meshes: ResMut<Assets<Mesh>>,
			      mut materials: ResMut<Assets<ColorMaterial>>| {
				let spawner = StatusSpawner::new(true, completed, total);
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
	fn test_status_spawner_spawns_not_started() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_status_test_app();

		let params = TestStatusParams::new();

		// Spawn the status using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that NotStartedStatusMarker was spawned
		let mut not_started_query = world.query::<&NotStartedStatusMarker>();
		let not_started_markers: Vec<_> = not_started_query.iter(world).collect();
		assert_eq!(
			not_started_markers.len(),
			1,
			"Should spawn exactly one NotStartedStatusMarker entity"
		);

		// Check that StatusMarker was spawned
		let mut status_marker_query = world.query::<&StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 1, "Should spawn exactly one StatusMarker entity");

		Ok(())
	}

	#[test]
	fn test_status_spawner_spawns_in_progress() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_status_test_app();

		let params = TestStatusParams {
			completed: 2,
			total: 5,
			world_position: Vec3::new(100.0, 200.0, 0.0),
			task_size: Vec2::new(200.0, 5.0),
		};

		// Spawn the status using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that InProgressStatusMarker was spawned
		let mut in_progress_query = world.query::<&InProgressStatusMarker>();
		let in_progress_markers: Vec<_> = in_progress_query.iter(world).collect();
		assert_eq!(
			in_progress_markers.len(),
			1,
			"Should spawn exactly one InProgressStatusMarker entity"
		);

		// Check that StatusMarker was spawned
		let mut status_marker_query = world.query::<&StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 1, "Should spawn exactly one StatusMarker entity");

		Ok(())
	}

	#[test]
	fn test_status_spawner_spawns_completed() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_status_test_app();

		let params = TestStatusParams {
			completed: 3,
			total: 3,
			world_position: Vec3::new(100.0, 200.0, 0.0),
			task_size: Vec2::new(200.0, 5.0),
		};

		// Spawn the status using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that CompletedStatusMarker was spawned
		let mut completed_query = world.query::<&CompletedStatusMarker>();
		let completed_markers: Vec<_> = completed_query.iter(world).collect();
		assert_eq!(
			completed_markers.len(),
			1,
			"Should spawn exactly one CompletedStatusMarker entity"
		);

		// Check that StatusMarker was spawned
		let mut status_marker_query = world.query::<&StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 1, "Should spawn exactly one StatusMarker entity");

		// Check that CheckMarkMesh was spawned (for completed status)
		let mut check_mark_query =
			world.query::<&crate::bundles::task::content::status::completed::CheckMarkMesh>();
		let check_marks: Vec<_> = check_mark_query.iter(world).collect();
		assert_eq!(check_marks.len(), 1, "Should spawn exactly one CheckMarkMesh entity");

		Ok(())
	}

	#[test]
	fn test_status_spawner_attaches_to_parent() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = setup_status_test_app();

		let params = TestStatusParams {
			completed: 1,
			total: 4,
			world_position: Vec3::new(100.0, 200.0, 0.0),
			task_size: Vec2::new(200.0, 5.0),
		};

		// Spawn the status using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that the parent entity has children
		let mut children_query = world.query::<&Children>();
		let children_components: Vec<_> = children_query.iter(world).collect();

		// Find the parent's children component
		let parent_children = children_components.iter().find(|children| {
			children.iter().any(|child| {
				// Check if this child has a status marker
				world.get::<StatusMarker>(child).is_some()
			})
		});

		assert!(parent_children.is_some(), "Parent should have a child with StatusMarker");

		Ok(())
	}
}
