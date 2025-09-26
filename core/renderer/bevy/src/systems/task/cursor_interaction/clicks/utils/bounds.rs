use crate::components::Task;
use crate::resources::Roadline;
use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Helper for checking if a position is within task bounds
pub struct TaskBoundsChecker;

impl TaskBoundsChecker {
	/// Check if a world position is within the bounds of any task
	pub fn find_task_at_position(
		task_query: &Query<(Entity, &Transform, &Task)>,
		roadline: &Roadline,
		world_pos: Vec2,
		pixels_per_unit: f32,
	) -> Option<TaskId> {
		for (_entity, transform, task) in task_query.iter() {
			if Self::is_position_within_task_bounds(
				transform,
				task,
				roadline,
				world_pos,
				pixels_per_unit,
			) {
				return Some(task.task_id);
			}
		}
		None
	}

	/// Check if a position is within the bounds of a specific task
	pub fn is_position_within_task_bounds(
		transform: &Transform,
		task: &Task,
		roadline: &Roadline,
		world_pos: Vec2,
		pixels_per_unit: f32,
	) -> bool {
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

		// Check if position is within task bounds
		world_pos.x >= min_x && world_pos.x <= max_x && world_pos.y >= min_y && world_pos.y <= max_y
	}
}
