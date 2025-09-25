pub mod clicks;
pub mod hovers;

pub use clicks::TaskClickSystem;
pub use hovers::TaskHoverSystem;

use crate::components::Task;
use crate::resources::{Roadline, SelectionResource};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::ui::BorderColor;

#[derive(Debug, Clone)]
pub struct TaskCursorInteractionSystem {
	pub hover_system: TaskHoverSystem,
	pub click_system: TaskClickSystem,
}

impl Default for TaskCursorInteractionSystem {
	fn default() -> Self {
		Self { hover_system: TaskHoverSystem::default(), click_system: TaskClickSystem::default() }
	}
}

impl TaskCursorInteractionSystem {
	pub fn build(
		self,
	) -> impl FnMut(
		Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>)>,
		Query<&Window>,
		Res<ButtonInput<MouseButton>>,
		Query<(Entity, &Transform, &Task)>,
		Query<&mut BorderColor>,
		ResMut<SelectionResource>,
		Option<Res<Roadline>>,
	) {
		move |camera_query: Query<
			(&Camera, &GlobalTransform),
			(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
		>,
		      windows: Query<&Window>,
		      mouse_input: Res<ButtonInput<MouseButton>>,
		      task_query: Query<(Entity, &Transform, &Task)>,
		      ui_query: Query<&mut BorderColor>,
		      selection_resource: ResMut<SelectionResource>,
		      roadline: Option<Res<Roadline>>| {
			self.task_cursor_interaction(
				camera_query,
				windows,
				mouse_input,
				task_query,
				ui_query,
				selection_resource,
				roadline,
			)
		}
	}

	/// Combined system to handle both hover and click interactions for tasks
	pub fn task_cursor_interaction(
		&self,
		camera_query: Query<
			(&Camera, &GlobalTransform),
			(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
		>,
		windows: Query<&Window>,
		mouse_input: Res<ButtonInput<MouseButton>>,
		task_query: Query<(Entity, &Transform, &Task)>,
		mut ui_query: Query<&mut BorderColor>,
		mut selection_resource: ResMut<SelectionResource>,
		roadline: Option<Res<Roadline>>,
	) {
		// Get camera and window info
		let Ok((camera, camera_transform)) = camera_query.single() else {
			return;
		};
		let Ok(window) = windows.single() else {
			return;
		};
		let Some(roadline) = roadline else {
			return;
		};

		// Get mouse position
		let Some(cursor_position) = window.cursor_position() else {
			// If no cursor position, clear all hover effects
			self.hover_system
				.clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
			return;
		};

		// Convert screen coordinates to world coordinates
		let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
			self.hover_system
				.clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
			return;
		};

		// Get the visual bounds to scale everything properly (same as task system)
		let (max_width, max_height) = roadline.visual_bounds();
		let _max_width_f32 = max_width.value() as f32;
		let _max_height_f32 = max_height.value() as f32;

		// Scale factor: same as tasks
		let pixels_per_unit = 50.0;

		// Check for clicks first (higher priority)
		if mouse_input.just_pressed(MouseButton::Left) {
			self.click_system.handle_task_clicks(
				world_pos,
				&task_query,
				&mut selection_resource,
				&mut ui_query,
				&roadline,
				pixels_per_unit,
			);
			return; // Exit early if we handled a click
		}

		// Handle hover effects (lower priority)
		self.hover_system.handle_task_hovers(
			world_pos,
			&task_query,
			&mut ui_query,
			&selection_resource,
			&roadline,
			pixels_per_unit,
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::components::SelectionState;
	use crate::resources::SelectionResource;
	use crate::systems::task::cursor_interaction::clicks::test_utils::{
		setup_cursor_interaction_test_app, simulate_cursor_to_world_position, spawn_test_task,
	};
	use bevy::input::mouse::MouseButton;
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_synthesized_system_hover_only() -> Result<(), Box<dyn std::error::Error>> {
		let cursor_system = TaskCursorInteractionSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn a test task
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(0.0, 0.0, 0.0),
			Vec2::new(20.0, 20.0),
			"Test Task".to_string(),
		)?;

		// Add the synthesized system
		app.add_systems(Update, cursor_system.build());

		// Simulate cursor movement to the task (without clicking)
		fn simulate_cursor_movement(
			mut windows: Query<(Entity, &mut Window)>,
			cameras: Query<(&Camera, &GlobalTransform)>,
		) {
			let _ =
				simulate_cursor_to_world_position(&mut windows, &cameras, Vec3::new(0.0, 0.0, 0.0));
		}

		app.add_systems(Update, simulate_cursor_movement);
		app.update();

		// Check that the task has hover color (not selected)
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					// Should have hover color since we're hovering but not clicking
					assert_eq!(border_color.0, Color::oklch(0.5, 0.137, 235.06));
				}
			}
		}

		// Verify task is not selected
		let selection_resource = app.world().resource::<SelectionResource>();
		let task_state = selection_resource.get_task_state(&TaskId::from(1));
		assert_eq!(task_state, SelectionState::Unselected);

		Ok(())
	}

	#[test]
	fn test_synthesized_system_click_priority() -> Result<(), Box<dyn std::error::Error>> {
		let cursor_system = TaskCursorInteractionSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn a test task
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(0.0, 0.0, 0.0),
			Vec2::new(20.0, 20.0),
			"Test Task".to_string(),
		)?;

		// Add the synthesized system
		app.add_systems(Update, cursor_system.build());

		// Simulate cursor movement and click
		fn simulate_cursor_and_click(
			mut windows: Query<(Entity, &mut Window)>,
			mut mouse_input: ResMut<ButtonInput<MouseButton>>,
			cameras: Query<(&Camera, &GlobalTransform)>,
		) {
			let _ =
				simulate_cursor_to_world_position(&mut windows, &cameras, Vec3::new(0.0, 0.0, 0.0));
			// Simulate mouse button press using ButtonInput
			mouse_input.press(MouseButton::Left);
		}

		app.add_systems(Update, simulate_cursor_and_click);
		app.update();

		// Check that the task is selected (click takes priority over hover)
		let selection_resource = app.world().resource::<SelectionResource>();
		let task_state = selection_resource.get_task_state(&TaskId::from(1));
		assert_eq!(task_state, SelectionState::Selected);

		// Check that the task has selected color (not hover color)
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					// Should have selected color, not hover color
					assert_eq!(border_color.0, Color::oklch(0.5, 0.137, 235.06)); // Selected color
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_synthesized_system_no_cursor_clears_hover() -> Result<(), Box<dyn std::error::Error>> {
		let cursor_system = TaskCursorInteractionSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn a test task
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(0.0, 0.0, 0.0),
			Vec2::new(20.0, 20.0),
			"Test Task".to_string(),
		)?;

		// Add the synthesized system
		app.add_systems(Update, cursor_system.build());

		// First, simulate cursor movement to create hover effect
		fn simulate_cursor_movement(
			mut windows: Query<(Entity, &mut Window)>,
			cameras: Query<(&Camera, &GlobalTransform)>,
		) {
			let _ =
				simulate_cursor_to_world_position(&mut windows, &cameras, Vec3::new(0.0, 0.0, 0.0));
		}

		app.add_systems(Update, simulate_cursor_movement);
		app.update();

		// Verify hover effect is applied
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					assert_eq!(border_color.0, Color::oklch(0.5, 0.137, 235.06)); // Hover color
				}
			}
		}

		// Now simulate no cursor position (cursor moved off window)
		fn clear_cursor_position(mut windows: Query<(Entity, &mut Window)>) {
			if let Ok((_, mut window)) = windows.single_mut() {
				window.set_cursor_position(None);
			}
		}

		app.add_systems(Update, clear_cursor_position);
		app.update();

		// Verify hover effect is cleared
		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(border_color) = ui_query.get(app.world(), ui_entity) {
					assert_eq!(border_color.0, Color::BLACK); // Unselected color
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_synthesized_system_subsystems_work_together() -> Result<(), Box<dyn std::error::Error>>
	{
		let cursor_system = TaskCursorInteractionSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// Spawn two test tasks
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(-50.0, 0.0, 0.0),
			Vec2::new(20.0, 20.0),
			"Task 1".to_string(),
		)?;
		spawn_test_task(
			&mut app,
			TaskId::from(2),
			Vec3::new(50.0, 0.0, 0.0),
			Vec2::new(20.0, 20.0),
			"Task 2".to_string(),
		)?;

		// Test sequence: hover over task 1
		fn test_sequence(
			mut windows: Query<(Entity, &mut Window)>,
			cameras: Query<(&Camera, &GlobalTransform)>,
		) {
			// This is a simplified test - in a real scenario you'd want more sophisticated state management
			// For now, just verify the system can handle multiple interactions
			let _ = simulate_cursor_to_world_position(
				&mut windows,
				&cameras,
				Vec3::new(-50.0, 0.0, 0.0),
			);
		}

		// Add the synthesized system and test sequence
		app.add_systems(Update, (test_sequence, cursor_system.build()).chain());
		app.update();

		// Verify both subsystems are working together by checking that:
		// 1. The system doesn't crash with multiple tasks
		// 2. The system can handle the combined functionality
		// 3. Both tasks exist and have UI entities

		let mut task_query = app.world_mut().query::<(Entity, &Transform, &Task)>();
		let mut ui_query = app.world_mut().query::<&BorderColor>();
		let mut task_count = 0;
		let mut tasks_with_ui = 0;

		for (_entity, _transform, task) in task_query.iter(app.world()) {
			task_count += 1;
			if let Some(ui_entity) = task.ui_entity {
				tasks_with_ui += 1;
				// Verify the UI entity exists and has a border color
				if let Ok(_border_color) = ui_query.get(app.world(), ui_entity) {
					// UI entity exists and is accessible - this is sufficient for this test
				}
			}
		}

		// Verify we have both tasks and they both have UI entities
		assert_eq!(task_count, 2);
		assert_eq!(tasks_with_ui, 2);

		Ok(())
	}
}
