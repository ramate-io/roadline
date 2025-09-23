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
