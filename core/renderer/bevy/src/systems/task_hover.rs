use crate::components::Task;
use crate::resources::Roadline;
use bevy::prelude::*;
use bevy::ui::BorderColor;

/// System to handle hover effects on task entities
pub fn task_hover_system(
	camera_query: Query<
		(&Camera, &GlobalTransform),
		(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
	>,
	windows: Query<&Window>,
	task_query: Query<(&Transform, &Task)>,
	mut ui_query: Query<&mut BorderColor>,
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
	if let Some(cursor_position) = window.cursor_position() {
		// Convert screen coordinates to world coordinates
		let world_pos_result = camera.viewport_to_world_2d(camera_transform, cursor_position);
		if let Ok(world_pos) = world_pos_result {
			// Get the visual bounds to scale everything properly (same as task system)
			let (max_width, max_height) = roadline.visual_bounds();
			let max_width_f32 = max_width.value() as f32;
			let max_height_f32 = max_height.value() as f32;

			// Scale factor: same as tasks
			let pixels_per_unit = 50.0;

			// Calculate offsets to center the content around (0,0) (same as task system)
			let content_width_pixels = max_width_f32 * pixels_per_unit;
			let content_height_pixels = max_height_f32 * pixels_per_unit;
			let _offset_x = -content_width_pixels / 2.0;
			let _offset_y = -content_height_pixels / 2.0;

			// Check each task for hover
			for (transform, task) in task_query.iter() {
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

				// Change UI colors if hovering
				if let Some(ui_entity) = task.ui_entity {
					if let Ok(mut border_color) = ui_query.get_mut(ui_entity) {
						if in_bounds {
							// Change colors to indicate hover
							border_color.0 = Color::oklch(0.5, 0.137, 235.06); // Dark blue border
						} else {
							// Change back to default colors
							border_color.0 = Color::BLACK;
						}
					}
				}
			}
		}
	}
}
