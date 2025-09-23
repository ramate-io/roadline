pub mod clicks;
pub use clicks::handle_task_clicks;
pub mod hovers;
pub use hovers::{clear_hover_effects, handle_task_hovers};

use crate::components::{SelectionState, Task};
use crate::resources::{Roadline, SelectionResource};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::ui::BorderColor;
use roadline_util::task::Id as TaskId;

#[derive(Debug, Clone)]
pub struct TaskCursorInteractionSystem {
	pub parent_task_border_color: Color,
	pub descendant_task_border_color: Color,
	pub unselected_task_border_color: Color,
	pub selected_task_border_color: Color,
	pub task_hover_border_color: Color,
	pub parent_dependency_color: Color,
	pub descendant_dependency_color: Color,
	pub unselected_dependency_color: Color,
	pub selected_dependency_color: Color,
	pub dependency_hover_color: Color,
}

impl Default for TaskCursorInteractionSystem {
	fn default() -> Self {
		Self {
			parent_task_border_color: Color::oklch(0.5, 0.137, 0.0),
			descendant_task_border_color: Color::oklch(0.5, 0.137, 235.06),
			unselected_task_border_color: Color::BLACK,
			selected_task_border_color: Color::oklch(0.5, 0.137, 235.06),
			task_hover_border_color: Color::oklch(0.5, 0.137, 235.06),
			parent_dependency_color: Color::oklch(0.5, 0.137, 0.0),
			descendant_dependency_color: Color::oklch(0.5, 0.137, 235.06),
			unselected_dependency_color: Color::BLACK,
			selected_dependency_color: Color::oklch(0.5, 0.137, 235.06),
			dependency_hover_color: Color::oklch(0.5, 0.137, 235.06),
		}
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
			clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
			return;
		};

		// Convert screen coordinates to world coordinates
		let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
			clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
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
			handle_task_clicks(
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
		handle_task_hovers(
			world_pos,
			&task_query,
			&mut ui_query,
			&selection_resource,
			&roadline,
			pixels_per_unit,
		);
	}
}

/// Combined system to handle both hover and click interactions for tasks
pub fn task_cursor_interaction_system(
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
		clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
		return;
	};

	// Convert screen coordinates to world coordinates
	let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
		clear_hover_effects(&task_query, &mut ui_query, &selection_resource);
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
		handle_task_clicks(
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
	handle_task_hovers(
		world_pos,
		&task_query,
		&mut ui_query,
		&selection_resource,
		&roadline,
		pixels_per_unit,
	);
}
