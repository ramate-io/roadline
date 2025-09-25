use crate::bundles::TaskSpawner;
use crate::components::Task;
use crate::resources::{RenderUpdateEvent, Roadline};
use crate::RoadlineRenderConfig;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::sprite::ColorMaterial;

/// Configuration for task spawning systems
#[derive(Debug, Clone, Resource)]
pub struct TaskSpawningSystem {
	pub pixels_per_unit: f32,
}

impl Default for TaskSpawningSystem {
	fn default() -> Self {
		Self {
			pixels_per_unit: 50.0,
		}
	}
}

impl TaskSpawningSystem {
	/// Builds a system function for updating task sprites
	pub fn build(
		self,
	) -> impl FnMut(
		Commands,
		EventReader<RenderUpdateEvent>,
		Option<Res<Roadline>>,
		Res<RoadlineRenderConfig>,
		Query<Entity, With<Task>>,
		ResMut<Assets<Mesh>>,
		ResMut<Assets<ColorMaterial>>,
	) {
		move |mut commands: Commands,
		      render_events: EventReader<RenderUpdateEvent>,
		      reified_opt: Option<Res<Roadline>>,
		      _config: Res<RoadlineRenderConfig>,
		      existing_tasks: Query<Entity, With<Task>>,
		      mut meshes: ResMut<Assets<Mesh>>,
		      mut materials: ResMut<Assets<ColorMaterial>>| {
			self.update_task_sprites(
				&mut commands,
				&render_events,
				&reified_opt,
				&existing_tasks,
				&mut meshes,
				&mut materials,
			);
		}
	}

	/// Update task sprites based on render events and roadline data
	pub fn update_task_sprites(
		&self,
		commands: &mut Commands,
		render_events: &EventReader<RenderUpdateEvent>,
		reified_opt: &Option<Res<Roadline>>,
		existing_tasks: &Query<Entity, With<Task>>,
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<ColorMaterial>>,
	) {
		// Only update if we received a render event and have reified data
		if render_events.is_empty() || reified_opt.is_none() {
			return;
		}
		// Events are automatically cleared after being read

		let reified = reified_opt.as_ref().unwrap();

		// Clear existing task entities
		for entity in existing_tasks.iter() {
			commands.entity(entity).despawn();
		}

		// Get the visual bounds to scale everything properly
		let (max_width, max_height) = reified.visual_bounds();
		let max_width_f32 = max_width.value() as f32;
		let max_height_f32 = max_height.value() as f32;

		// Calculate offsets to center the content around (0,0)
		let content_width_pixels = max_width_f32 * self.pixels_per_unit;
		let content_height_pixels = max_height_f32 * self.pixels_per_unit;
		let offset_x = -content_width_pixels / 2.0;
		let offset_y = -content_height_pixels / 2.0;

		// Create new task sprites for each task
		for (task_id, start_x, start_y, end_x, end_y) in reified.task_rectangles() {
			println!(
				"task_id: {:?}, start_x: {}, start_y: {}, end_x: {}, end_y: {}",
				task_id, start_x, start_y, end_x, end_y
			);
			println!("Max bounds: width={}, height={}", max_width_f32, max_height_f32);

			let (x, y) = (start_x, start_y);
			let height = end_y - start_y;
			let width = end_x - start_x;

			// Convert reified units to pixel coordinates using proper scaling
			let pixel_x = x as f32 * self.pixels_per_unit + offset_x;
			let pixel_y = y as f32 * self.pixels_per_unit + offset_y;
			let sprite_width = width as f32 * self.pixels_per_unit;
			let sprite_height = height as f32 * self.pixels_per_unit;

			// Adjust for left justification (Bevy positions by center, so move right by half width)
			let left_justified_x = pixel_x + (sprite_width / 2.0);

			println!(
				"Rendering: pixel_pos=({:.1}, {:.1}), size=({:.1}x{:.1}), left_justified_x={:.1}",
				pixel_x, pixel_y, sprite_width, sprite_height, left_justified_x
			);

			let task = reified.task(task_id);
			if task.is_none() {
				continue;
			}
			let task = task.unwrap();
			let title = task.title();

			// Use TaskSpawner to spawn all task entities
			let task_spawner = TaskSpawner::new(
				*task_id,
				Vec3::new(left_justified_x, pixel_y, 0.0),
				Vec2::new(sprite_width, sprite_height),
				title.text.clone(),
				false,
				3,
				3,
			);

			task_spawner.spawn(commands, meshes, materials);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use crate::resources::{Roadline, RenderUpdateEvent};
	use crate::test_utils::create_test_roadline;
	use bevy::ecs::system::RunSystemOnce;
	use roadline_util::task::Id as TaskId;

	/// Helper function to set up an app with all plugins and resources needed for spawning testing
	fn setup_spawning_test_app() -> App {
		let mut app = setup_task_test_app();

		// Add required resources
		let core_roadline = create_test_roadline().expect("Failed to create test roadline");
		app.insert_resource(Roadline::from(core_roadline));
		app.insert_resource(RoadlineRenderConfig::default());

		// Add the RenderUpdateEvent
		app.add_event::<RenderUpdateEvent>();

		app
	}

	/// Helper function to spawn a basic test task
	fn spawn_test_task(
		app: &mut App,
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
	) -> Result<(), Box<dyn std::error::Error>> {
		let params = TestTasksParams::new().with_basic_task(task_id, position, size, title);
		app.world_mut().run_system_once(params.build())?;
		Ok(())
	}

	/// Helper function to send a render update event
	fn send_render_update_event(app: &mut App) {
		let mut events = app.world_mut().resource_mut::<Events<RenderUpdateEvent>>();
		events.send(RenderUpdateEvent);
	}

	#[test]
	fn test_spawning_system_builds_correctly() -> Result<(), Box<dyn std::error::Error>> {
		let spawning_system = TaskSpawningSystem::default();

		// Setup app with all required resources
		let mut app = setup_spawning_test_app();

		// Add the spawning system
		app.add_systems(Update, spawning_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that tasks were spawned
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let task_count = task_query.iter(app.world()).count();

		// Should have spawned tasks from the test roadline
		assert!(task_count > 0, "Expected tasks to be spawned, but found {}", task_count);

		Ok(())
	}

	#[test]
	fn test_spawning_system_clears_existing_tasks() -> Result<(), Box<dyn std::error::Error>> {
		let spawning_system = TaskSpawningSystem::default();

		// Setup app with all required resources
		let mut app = setup_spawning_test_app();

		// Spawn some initial tasks manually
		spawn_test_task(
			&mut app,
			TaskId::from(255),
			Vec3::new(0.0, 0.0, 0.0),
			Vec2::new(20.0, 20.0),
			"Manual Task".to_string(),
		)?;

		// Verify initial task exists
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let initial_count = task_query.iter(app.world()).count();
		assert_eq!(initial_count, 1);

		// Add the spawning system
		app.add_systems(Update, spawning_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that the manual task was cleared and new tasks were spawned
		let final_count = task_query.iter(app.world()).count();
		assert!(final_count > 0, "Expected tasks to be spawned after clearing");
		assert_ne!(final_count, initial_count, "Task count should have changed");

		// Verify the manual task is gone
		let manual_task_exists = task_query
			.iter(app.world())
			.any(|(_, _, task)| task.task_id == TaskId::from(255));
		assert!(!manual_task_exists, "Manual task should have been cleared");

		Ok(())
	}

	#[test]
	fn test_spawning_system_without_render_event() -> Result<(), Box<dyn std::error::Error>> {
		let spawning_system = TaskSpawningSystem::default();

		// Setup app with all required resources
		let mut app = setup_spawning_test_app();

		// Add the spawning system
		app.add_systems(Update, spawning_system.build());

		// Run the system WITHOUT sending a render update event
		app.update();

		// Verify that no tasks were spawned
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let task_count = task_query.iter(app.world()).count();

		assert_eq!(task_count, 0, "Expected no tasks to be spawned without render event");

		Ok(())
	}

	#[test]
	fn test_spawning_system_without_roadline() -> Result<(), Box<dyn std::error::Error>> {
		let spawning_system = TaskSpawningSystem::default();

		// Setup app WITHOUT roadline resource
		let mut app = setup_task_test_app();
		app.insert_resource(RoadlineRenderConfig::default());
		app.add_event::<RenderUpdateEvent>();

		// Add the spawning system
		app.add_systems(Update, spawning_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that no tasks were spawned
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let task_count = task_query.iter(app.world()).count();

		assert_eq!(task_count, 0, "Expected no tasks to be spawned without roadline");

		Ok(())
	}

	#[test]
	fn test_spawning_system_custom_pixels_per_unit() -> Result<(), Box<dyn std::error::Error>> {
		let custom_pixels_per_unit = 100.0;
		let spawning_system = TaskSpawningSystem {
			pixels_per_unit: custom_pixels_per_unit,
		};

		// Setup app with all required resources
		let mut app = setup_spawning_test_app();

		// Add the spawning system
		app.add_systems(Update, spawning_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that tasks were spawned with custom scaling
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let task_count = task_query.iter(app.world()).count();

		assert!(task_count > 0, "Expected tasks to be spawned with custom scaling");

		// Verify that the scaling was applied correctly by checking task positions
		// (This is a basic check - in a real scenario you'd want more sophisticated verification)
		for (_, transform, _) in task_query.iter(app.world()) {
			// Tasks should be positioned based on the custom pixels_per_unit
			// The exact values depend on the test roadline data
			assert!(transform.translation.x.is_finite());
			assert!(transform.translation.y.is_finite());
		}

		Ok(())
	}

	#[test]
	fn test_spawning_system_multiple_render_events() -> Result<(), Box<dyn std::error::Error>> {
		let spawning_system = TaskSpawningSystem::default();

		// Setup app with all required resources
		let mut app = setup_spawning_test_app();

		// Add the spawning system
		app.add_systems(Update, spawning_system.build());

		// Send multiple render update events
		send_render_update_event(&mut app);
		send_render_update_event(&mut app);
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that tasks were spawned (should only spawn once per update cycle)
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let task_count = task_query.iter(app.world()).count();

		assert!(task_count > 0, "Expected tasks to be spawned with multiple render events");

		Ok(())
	}
}
