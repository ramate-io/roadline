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
	pub fn new(completed: u32, total: u32) -> Self {
		// Determine status type based on completion
		if completed == 0 {
			// Not started
			Self::NotStarted(NotStartedStatusSpawner::new(total))
		} else if completed == total {
			// Completed
			Self::Completed(CompletedStatusSpawner::new(completed, total))
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

	#[test]
	fn test_status_spawner_not_started() -> Result<(), Box<dyn std::error::Error>> {
		let completed = 0;
		let total = 5;

		let spawner = StatusSpawner::new(completed, total);

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

		let spawner = StatusSpawner::new(completed, total);

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

		let spawner = StatusSpawner::new(completed, total);

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
		let spawner = StatusSpawner::new(0, 0);
		match spawner {
			StatusSpawner::NotStarted(_) => {}
			_ => return Err("Expected NotStarted for 0/0".into()),
		}

		// Test edge case: completed = 1, total = 1 (should be completed)
		let spawner = StatusSpawner::new(1, 1);
		match spawner {
			StatusSpawner::Completed(_) => {}
			_ => return Err("Expected Completed for 1/1".into()),
		}

		// Test edge case: completed = 1, total = 2 (should be in progress)
		let spawner = StatusSpawner::new(1, 2);
		match spawner {
			StatusSpawner::InProgress(_) => {}
			_ => return Err("Expected InProgress for 1/2".into()),
		}

		Ok(())
	}

	#[test]
	fn test_status_spawner_spawns_not_started() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let completed = 0;
		let total = 3;
		let world_position = Vec3::new(100.0, 200.0, 0.0);
		let task_size = Vec2::new(200.0, 50.0);

		let spawner = StatusSpawner::new(completed, total);

		// Create a parent entity
		let parent_entity = app.world_mut().spawn_empty().id();

		// Spawn the status
		app.world_mut().run_system_once(
			|mut commands: Commands,
			 mut meshes: ResMut<Assets<Mesh>>,
			 mut materials: ResMut<Assets<ColorMaterial>>| {
				spawner.spawn(
					&mut commands,
					&mut meshes,
					&mut materials,
					parent_entity,
					world_position,
					task_size,
				);
			},
		);

		let world = app.world();

		// Check that NotStartedStatusMarker was spawned
		let not_started_query = world.query::<&NotStartedStatusMarker>();
		let not_started_markers: Vec<_> = not_started_query.iter(world).collect();
		assert_eq!(
			not_started_markers.len(),
			1,
			"Should spawn exactly one NotStartedStatusMarker entity"
		);

		// Check that StatusMarker was spawned
		let status_marker_query = world.query::<&StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 1, "Should spawn exactly one StatusMarker entity");

		Ok(())
	}

	#[test]
	fn test_status_spawner_spawns_in_progress() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let completed = 2;
		let total = 5;
		let world_position = Vec3::new(100.0, 200.0, 0.0);
		let task_size = Vec2::new(200.0, 50.0);

		let spawner = StatusSpawner::new(completed, total);

		// Create a parent entity
		let parent_entity = app.world_mut().spawn_empty().id();

		// Spawn the status
		app.world_mut().run_system_once(
			|mut commands: Commands,
			 mut meshes: ResMut<Assets<Mesh>>,
			 mut materials: ResMut<Assets<ColorMaterial>>| {
				spawner.spawn(
					&mut commands,
					&mut meshes,
					&mut materials,
					parent_entity,
					world_position,
					task_size,
				);
			},
		);

		let world = app.world();

		// Check that InProgressStatusMarker was spawned
		let in_progress_query = world.query::<&InProgressStatusMarker>();
		let in_progress_markers: Vec<_> = in_progress_query.iter(world).collect();
		assert_eq!(
			in_progress_markers.len(),
			1,
			"Should spawn exactly one InProgressStatusMarker entity"
		);

		// Check that StatusMarker was spawned
		let status_marker_query = world.query::<&StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 1, "Should spawn exactly one StatusMarker entity");

		Ok(())
	}

	#[test]
	fn test_status_spawner_spawns_completed() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let completed = 3;
		let total = 3;
		let world_position = Vec3::new(100.0, 200.0, 0.0);
		let task_size = Vec2::new(200.0, 50.0);

		let spawner = StatusSpawner::new(completed, total);

		// Create a parent entity
		let parent_entity = app.world_mut().spawn_empty().id();

		// Spawn the status
		app.world_mut().run_system_once(
			|mut commands: Commands,
			 mut meshes: ResMut<Assets<Mesh>>,
			 mut materials: ResMut<Assets<ColorMaterial>>| {
				spawner.spawn(
					&mut commands,
					&mut meshes,
					&mut materials,
					parent_entity,
					world_position,
					task_size,
				);
			},
		);

		let world = app.world();

		// Check that CompletedStatusMarker was spawned
		let completed_query = world.query::<&CompletedStatusMarker>();
		let completed_markers: Vec<_> = completed_query.iter(world).collect();
		assert_eq!(
			completed_markers.len(),
			1,
			"Should spawn exactly one CompletedStatusMarker entity"
		);

		// Check that StatusMarker was spawned
		let status_marker_query = world.query::<&StatusMarker>();
		let status_markers: Vec<_> = status_marker_query.iter(world).collect();
		assert_eq!(status_markers.len(), 1, "Should spawn exactly one StatusMarker entity");

		// Check that CheckMarkMesh was spawned (for completed status)
		let check_mark_query =
			world.query::<&crate::bundles::task::content::status::completed::CheckMarkMesh>();
		let check_marks: Vec<_> = check_mark_query.iter(world).collect();
		assert_eq!(check_marks.len(), 1, "Should spawn exactly one CheckMarkMesh entity");

		Ok(())
	}

	#[test]
	fn test_status_spawner_attaches_to_parent() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let completed = 1;
		let total = 4;
		let world_position = Vec3::new(100.0, 200.0, 0.0);
		let task_size = Vec2::new(200.0, 50.0);

		let spawner = StatusSpawner::new(completed, total);

		// Create a parent entity
		let parent_entity = app.world_mut().spawn_empty().id();

		// Spawn the status
		app.world_mut().run_system_once(
			|mut commands: Commands,
			 mut meshes: ResMut<Assets<Mesh>>,
			 mut materials: ResMut<Assets<ColorMaterial>>| {
				spawner.spawn(
					&mut commands,
					&mut meshes,
					&mut materials,
					parent_entity,
					world_position,
					task_size,
				);
			},
		);

		let world = app.world();

		// Check that the parent entity has children
		let children_query = world.query::<&Children>();
		let children_components: Vec<_> = children_query.iter(world).collect();

		// Find the parent's children component
		let parent_children = children_components.iter().find(|children| {
			children.iter().any(|&child| {
				// Check if this child has a status marker
				world.get::<StatusMarker>(child).is_some()
			})
		});

		assert!(parent_children.is_some(), "Parent should have a child with StatusMarker");

		Ok(())
	}
}
