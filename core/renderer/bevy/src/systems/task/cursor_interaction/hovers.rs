pub mod test_utils;

use crate::components::{SelectionState, Task};
use crate::resources::{Roadline, SelectionResource};
use bevy::prelude::*;
use bevy::ui::BorderColor;

#[derive(Debug, Clone, Resource)]
pub struct TaskHoverSystem {
	pub task_hover_border_color: Color,
	pub unselected_task_border_color: Color,
	pub pixels_per_unit: f32,
}

impl Default for TaskHoverSystem {
	fn default() -> Self {
		Self {
			task_hover_border_color: Color::oklch(0.5, 0.137, 235.06),
			unselected_task_border_color: Color::BLACK,
			pixels_per_unit: 75.0,
		}
	}
}

impl TaskHoverSystem {
	/// Build a system function for task hover handling
	pub fn build(
		self,
	) -> impl FnMut(
		Query<(Entity, &Transform, &Task)>,
		ResMut<SelectionResource>,
		Query<&mut BorderColor>,
		Res<Roadline>,
		Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>)>,
		Query<&Window>,
	) {
		move |task_query: Query<(Entity, &Transform, &Task)>,
		      selection_resource: ResMut<SelectionResource>,
		      mut ui_query: Query<&mut BorderColor>,
		      roadline: Res<Roadline>,
		      camera_query: Query<
			(&Camera, &GlobalTransform),
			(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
		>,
		      windows: Query<&Window>| {
			// Get camera and window info
			let Ok((camera, camera_transform)) = camera_query.single() else {
				return; // No camera found, skip hover processing
			};

			let Ok(window) = windows.single() else {
				return; // No window found, skip hover processing
			};

			// Get mouse position
			let Some(cursor_position) = window.cursor_position() else {
				// No cursor position, clear hover effects
				self.clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
				return;
			};

			// Convert screen coordinates to world coordinates
			let world_pos = match camera.viewport_to_world_2d(camera_transform, cursor_position) {
				Ok(world_pos) => world_pos,
				Err(_) => {
					// Failed to convert, clear hover effects
					self.clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
					return;
				}
			};

			self.handle_task_hovers(
				world_pos,
				&task_query,
				&mut ui_query,
				&selection_resource,
				&roadline,
				self.pixels_per_unit,
			);
		}
	}

	/// Handle hover effects
	pub fn handle_task_hovers(
		&self,
		world_pos: Vec2,
		task_query: &Query<(Entity, &Transform, &Task)>,
		ui_query: &mut Query<&mut BorderColor>,
		selection_resource: &SelectionResource,
		roadline: &Roadline,
		pixels_per_unit: f32,
	) {
		for (_entity, transform, task) in task_query.iter() {
			let selection_state = selection_resource.get_task_state(&task.task_id);

			// Skip hover effects for selected/descendant/parent tasks - don't override selection colors
			if selection_state == SelectionState::Selected
				|| selection_state == SelectionState::Descendant
				|| selection_state == SelectionState::Parent
			{
				continue;
			}

			// Get task position from transform
			let task_pos = transform.translation.truncate();

			// Get actual task bounds from roadline
			let (start_x, start_y, end_x, end_y) = roadline.task_bounds(&task.task_id);
			let width = end_x - start_x;
			let height = end_y - start_y;

			// Convert reified units to pixel coordinates using same scaling as task system
			let sprite_width = width as f32 * pixels_per_unit;
			let sprite_height = height as f32 * pixels_per_unit;

			let min_x = task_pos.x - sprite_width / 2.0;
			let max_x = task_pos.x + sprite_width / 2.0;
			let min_y = task_pos.y - sprite_height / 2.0;
			let max_y = task_pos.y + sprite_height / 2.0;

			// Check if mouse is within task bounds
			let in_bounds = world_pos.x >= min_x
				&& world_pos.x <= max_x
				&& world_pos.y >= min_y
				&& world_pos.y <= max_y;

			// Update visual feedback for unselected tasks only
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(mut border_color) = ui_query.get_mut(ui_entity) {
					if in_bounds {
						border_color.0 = self.task_hover_border_color;
					} else {
						border_color.0 = self.unselected_task_border_color;
					}
				}
			}
		}
	}

	/// Clear hover effects when cursor is not over any task
	pub fn clear_hover_effects(
		&self,
		task_query: &Query<(Entity, &Transform, &Task)>,
		ui_query: &mut Query<&mut BorderColor>,
		selection_resource: &SelectionResource,
	) {
		for (_entity, _transform, task) in task_query.iter() {
			let selection_state = selection_resource.get_task_state(&task.task_id);

			// Skip clearing hover effects for selected/descendant/parent tasks - don't override selection colors
			if selection_state == SelectionState::Selected
				|| selection_state == SelectionState::Descendant
				|| selection_state == SelectionState::Parent
			{
				continue;
			}

			if let Some(ui_entity) = task.ui_entity {
				if let Ok(mut border_color) = ui_query.get_mut(ui_entity) {
					// Only clear hover for unselected tasks
					border_color.0 = self.unselected_task_border_color;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::components::SelectionState;
	use crate::resources::{Roadline, SelectionResource};
	use crate::systems::task::cursor_interaction::hovers::test_utils::{
		setup_cursor_interaction_test_app, spawn_test_task,
	};
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_compatible_with_spawned_tasks() -> Result<(), Box<dyn std::error::Error>> {
		let hover_system = TaskHoverSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn tasks
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 5.0),
			"UI Test Task".to_string(),
		)?;

		// Test the hover logic directly without coordinate conversion
		fn test_hover_logic(
			hover_system: Res<TaskHoverSystem>,
			task_query: Query<(Entity, &Transform, &Task)>,
			selection_resource: ResMut<SelectionResource>,
			mut ui_query: Query<&mut BorderColor>,
			roadline: Res<Roadline>,
		) {
			// Test with world coordinates that should hit the task
			// Task is at Vec3(100.0, 200.0, 0.0) with actual bounds min=(75, 175), max=(125, 225)
			// So let's hover at the center: (100, 200)
			let world_pos = Vec2::new(100.0, 200.0);

			hover_system.handle_task_hovers(
				world_pos,
				&task_query,
				&mut ui_query,
				&selection_resource,
				&roadline,
				hover_system.pixels_per_unit,
			);
		}

		// Add the hover system as a resource and the test system
		app.insert_resource(hover_system);
		app.add_systems(Update, test_hover_logic);

		// Run the test system
		app.update();

		// Check that the task now has hover color
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					// Should have hover color since we're hovering over it
					assert_eq!(border_color.0, Color::oklch(0.5, 0.137, 235.06));
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_hover_outside_bounds() -> Result<(), Box<dyn std::error::Error>> {
		let hover_system = TaskHoverSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn tasks
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 5.0),
			"UI Test Task".to_string(),
		)?;

		// Test the hover logic with coordinates outside the task bounds
		fn test_hover_logic(
			hover_system: Res<TaskHoverSystem>,
			task_query: Query<(Entity, &Transform, &Task)>,
			selection_resource: ResMut<SelectionResource>,
			mut ui_query: Query<&mut BorderColor>,
			roadline: Res<Roadline>,
		) {
			// Test with world coordinates that should NOT hit the task
			// Task is at Vec3(100.0, 200.0, 0.0) with actual bounds min=(75, 175), max=(125, 225)
			// So let's hover far away: (0, 0)
			let world_pos = Vec2::new(0.0, 0.0);

			hover_system.handle_task_hovers(
				world_pos,
				&task_query,
				&mut ui_query,
				&selection_resource,
				&roadline,
				hover_system.pixels_per_unit,
			);
		}

		// Add the hover system as a resource and the test system
		app.insert_resource(hover_system);
		app.add_systems(Update, test_hover_logic);

		// Run the test system
		app.update();

		// Check that the task has unselected color (no hover)
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					// Should have unselected color since we're not hovering over it
					assert_eq!(border_color.0, Color::BLACK);
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_hover_skips_selected_tasks() -> Result<(), Box<dyn std::error::Error>> {
		let hover_system = TaskHoverSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn tasks
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 5.0),
			"UI Test Task".to_string(),
		)?;

		// Set the task as selected
		let mut selection_resource = app.world_mut().resource_mut::<SelectionResource>();
		selection_resource.set_task_state(TaskId::from(1), SelectionState::Selected);

		// Test the hover logic with coordinates that should hit the task
		fn test_hover_logic(
			hover_system: Res<TaskHoverSystem>,
			task_query: Query<(Entity, &Transform, &Task)>,
			selection_resource: ResMut<SelectionResource>,
			mut ui_query: Query<&mut BorderColor>,
			roadline: Res<Roadline>,
		) {
			// Test with world coordinates that should hit the task
			let world_pos = Vec2::new(100.0, 200.0);

			hover_system.handle_task_hovers(
				world_pos,
				&task_query,
				&mut ui_query,
				&selection_resource,
				&roadline,
				hover_system.pixels_per_unit,
			);
		}

		// Add the hover system as a resource and the test system
		app.insert_resource(hover_system);
		app.add_systems(Update, test_hover_logic);

		// Run the test system
		app.update();

		// Check that the task still has its original color (hover should be skipped for selected tasks)
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					// Should NOT have hover color since the task is selected
					// The color should remain whatever it was set to by the selection system
					assert_ne!(border_color.0, Color::oklch(0.5, 0.137, 235.06));
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_clear_hover_effects() -> Result<(), Box<dyn std::error::Error>> {
		let hover_system = TaskHoverSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn tasks
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 5.0),
			"UI Test Task".to_string(),
		)?;

		// Test the clear hover effects logic
		fn test_clear_logic(
			hover_system: Res<TaskHoverSystem>,
			task_query: Query<(Entity, &Transform, &Task)>,
			selection_resource: ResMut<SelectionResource>,
			mut ui_query: Query<&mut BorderColor>,
		) {
			hover_system.clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
		}

		// Add the hover system as a resource and the test system
		app.insert_resource(hover_system);
		app.add_systems(Update, test_clear_logic);

		// Run the test system
		app.update();

		// Check that the task has unselected color
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					// Should have unselected color after clearing hover effects
					assert_eq!(border_color.0, Color::BLACK);
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_with_camera_and_window() -> Result<(), Box<dyn std::error::Error>> {
		let hover_system = TaskHoverSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn tasks
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(0.0, 0.0, 0.0), // Center of world
			Vec2::new(20.0, 20.0),    // Reasonable size
			"UI Test Task".to_string(),
		)?;

		fn simulate_cursor_movement(
			mut windows: Query<(Entity, &mut Window)>,
			cameras: Query<(&Camera, &GlobalTransform)>,
		) {
			let (_window_entity, mut window) = windows.single_mut().unwrap();
			let (camera, camera_transform) = cameras.single().unwrap();

			// Task is at Vec3(0.0, 0.0, 0.0) with size Vec2(20.0, 20.0)
			let world_pos = Vec3::new(0.0, 0.0, 0.0);

			// Convert world coordinates to screen coordinates
			let screen_pos = camera.world_to_viewport(camera_transform, world_pos).unwrap();

			window.set_cursor_position(Some(screen_pos));
		}

		// Systems need to be chained to avoid first registration bug.
		app.add_systems(Update, (simulate_cursor_movement, hover_system.clone().build()).chain());
		app.update();

		// Check that the task now has hover color
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					// Should have hover color since cursor is over the task
					assert_eq!(border_color.0, hover_system.task_hover_border_color);
				}
			}
		}

		Ok(())
	}
}
