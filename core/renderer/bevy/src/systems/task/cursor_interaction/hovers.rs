use crate::components::{SelectionState, Task};
use crate::resources::{Roadline, SelectionResource};
use bevy::prelude::*;
use bevy::ui::BorderColor;

#[derive(Debug, Clone)]
pub struct TaskHoverSystem {
	pub task_hover_border_color: Color,
	pub unselected_task_border_color: Color,
}

impl Default for TaskHoverSystem {
	fn default() -> Self {
		Self {
			task_hover_border_color: Color::oklch(0.5, 0.137, 235.06),
			unselected_task_border_color: Color::BLACK,
		}
	}
}

impl TaskHoverSystem {
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
