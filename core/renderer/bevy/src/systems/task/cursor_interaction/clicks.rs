use crate::components::{SelectionState, Task};
use crate::resources::{Roadline, SelectionResource};
use bevy::prelude::*;
use bevy::ui::BorderColor;
use roadline_util::task::Id as TaskId;

#[derive(Debug, Clone)]
pub struct TaskClickSystem {
	pub parent_task_border_color: Color,
	pub descendant_task_border_color: Color,
	pub unselected_task_border_color: Color,
	pub selected_task_border_color: Color,
	pub parent_dependency_color: Color,
	pub descendant_dependency_color: Color,
	pub unselected_dependency_color: Color,
	pub selected_dependency_color: Color,
}

impl Default for TaskClickSystem {
	fn default() -> Self {
		Self {
			parent_task_border_color: Color::oklch(0.5, 0.137, 0.0),
			descendant_task_border_color: Color::oklch(0.5, 0.137, 235.06),
			unselected_task_border_color: Color::BLACK,
			selected_task_border_color: Color::oklch(0.5, 0.137, 235.06),
			parent_dependency_color: Color::oklch(0.5, 0.137, 0.0),
			descendant_dependency_color: Color::oklch(0.5, 0.137, 235.06),
			unselected_dependency_color: Color::BLACK,
			selected_dependency_color: Color::oklch(0.5, 0.137, 235.06),
		}
	}
}

impl TaskClickSystem {
	/// Handle click detection and selection
	pub fn handle_task_clicks(
		&self,
		world_pos: Vec2,
		task_query: &Query<(Entity, &Transform, &Task)>,
		selection_resource: &mut ResMut<SelectionResource>,
		ui_query: &mut Query<&mut BorderColor>,
		roadline: &Roadline,
		pixels_per_unit: f32,
	) {
		for (_entity, transform, task) in task_query.iter() {
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

			if in_bounds {
				self.handle_task_click(
					task.task_id,
					selection_resource,
					ui_query,
					roadline,
					task_query,
				);
				return; // Exit early if we clicked on a task
			}
		}
	}

	/// Handle clicking on a task
	fn handle_task_click(
		&self,
		task_id: TaskId,
		selection_resource: &mut ResMut<SelectionResource>,
		ui_query: &mut Query<&mut BorderColor>,
		roadline: &Roadline,
		task_query: &Query<(Entity, &Transform, &Task)>,
	) {
		// Get current selection state
		let current_state = selection_resource.get_task_state(&task_id);

		// Toggle selection
		let new_state = match current_state {
			SelectionState::Unselected => SelectionState::Selected,
			SelectionState::Selected => SelectionState::Unselected,
			SelectionState::Descendant => SelectionState::Selected, // Now the descendant is selected it will have to be manually unselected.
			SelectionState::Parent => SelectionState::Unselected,
		};

		selection_resource.set_task_state(task_id, new_state);

		// Update visual feedback
		self.update_task_visual_feedback(task_id, new_state, ui_query, task_query);

		// Update descendant and parent states based on current selections
		self.update_selection_states_persistent(selection_resource, ui_query, roadline, task_query);
	}

	/// Update visual feedback for a task
	fn update_task_visual_feedback(
		&self,
		task_id: TaskId,
		state: SelectionState,
		ui_query: &mut Query<&mut BorderColor>,
		task_query: &Query<(Entity, &Transform, &Task)>,
	) {
		// Find the task entity and get its UI entity
		if let Some((_, _, task)) = task_query.iter().find(|(_, _, t)| t.task_id == task_id) {
			if let Some(ui_entity) = task.ui_entity {
				if let Ok(mut border_color) = ui_query.get_mut(ui_entity) {
					match state {
						SelectionState::Unselected => {
							border_color.0 = self.unselected_task_border_color;
						}
						SelectionState::Selected => {
							border_color.0 = self.selected_task_border_color;
						}
						SelectionState::Descendant => {
							border_color.0 = self.descendant_task_border_color;
						}
						SelectionState::Parent => {
							border_color.0 = self.parent_task_border_color;
						}
					}
				}
			}
		}
	}

	/// Update descendant and parent states based on selected tasks
	fn update_selection_states_persistent(
		&self,
		selection_resource: &mut ResMut<SelectionResource>,
		ui_query: &mut Query<&mut BorderColor>,
		roadline: &Roadline,
		task_query: &Query<(Entity, &Transform, &Task)>,
	) {
		// Clear all descendant and parent states first
		let mut tasks_to_clear = Vec::new();
		let mut dependencies_to_clear = Vec::new();

		for (task_id, state) in &selection_resource.tasks {
			if *state == SelectionState::Descendant || *state == SelectionState::Parent {
				tasks_to_clear.push(*task_id);
			}
		}

		for (dependency_id, state) in &selection_resource.dependencies {
			if *state == SelectionState::Descendant || *state == SelectionState::Parent {
				dependencies_to_clear.push(*dependency_id);
			}
		}

		// Clear descendant and parent states
		for task_id in tasks_to_clear {
			selection_resource.set_task_state(task_id, SelectionState::Unselected);
			self.update_task_visual_feedback(
				task_id,
				SelectionState::Unselected,
				ui_query,
				task_query,
			);
		}

		for dependency_id in dependencies_to_clear {
			selection_resource.set_dependency_state(dependency_id, SelectionState::Unselected);
		}

		// Now mark descendants and parents for all selected tasks
		for (task_id, state) in &selection_resource.tasks.clone() {
			if *state == SelectionState::Selected {
				self.mark_descendants_persistent(
					task_id,
					selection_resource,
					ui_query,
					roadline,
					task_query,
				);
				self.mark_parents_persistent(
					task_id,
					selection_resource,
					ui_query,
					roadline,
					task_query,
				);
			}
		}
	}

	/// Mark all descendants of a task using DFS (persistent version)
	fn mark_descendants_persistent(
		&self,
		start_task_id: &TaskId,
		selection_resource: &mut ResMut<SelectionResource>,
		ui_query: &mut Query<&mut BorderColor>,
		roadline: &Roadline,
		task_query: &Query<(Entity, &Transform, &Task)>,
	) {
		// Use DFS to traverse the graph
		let result = roadline.dfs(start_task_id, |task_id, _depth| {
			// Skip the start task (it's already selected)
			if task_id == start_task_id {
				return Ok(());
			}

			// Only mark as descendant if not already selected
			let current_state = selection_resource.get_task_state(task_id);
			if current_state == SelectionState::Unselected {
				selection_resource.set_task_state(*task_id, SelectionState::Descendant);
				self.update_task_visual_feedback(
					*task_id,
					SelectionState::Descendant,
					ui_query,
					task_query,
				);
			}

			// Find and mark dependencies that lead to this task
			for (dependency_id, _) in roadline.connections() {
				if &dependency_id.to() == task_id {
					let dep_state = selection_resource.get_dependency_state(&dependency_id);
					if dep_state == SelectionState::Unselected {
						selection_resource
							.set_dependency_state(*dependency_id, SelectionState::Descendant);
					}
				}
			}

			Ok(())
		});

		if let Err(e) = result {
			eprintln!("Error during DFS traversal: {:?}", e);
		}
	}

	/// Mark all parents of a task using reverse DFS (persistent version)
	fn mark_parents_persistent(
		&self,
		start_task_id: &TaskId,
		selection_resource: &mut ResMut<SelectionResource>,
		ui_query: &mut Query<&mut BorderColor>,
		roadline: &Roadline,
		task_query: &Query<(Entity, &Transform, &Task)>,
	) {
		// Use reverse DFS to traverse the graph backwards
		let result = roadline.graph().rev_dfs(start_task_id, |task_id, _depth| {
			// Skip the start task (it's already selected)
			if task_id == start_task_id {
				return Ok(());
			}

			// Only mark as parent if not already selected or descendant
			let current_state = selection_resource.get_task_state(task_id);
			if current_state == SelectionState::Unselected {
				selection_resource.set_task_state(*task_id, SelectionState::Parent);
				self.update_task_visual_feedback(
					*task_id,
					SelectionState::Parent,
					ui_query,
					task_query,
				);
			}

			// Find and mark dependencies that lead to this task
			for (dependency_id, _) in roadline.connections() {
				if &dependency_id.to() == task_id {
					let dep_state = selection_resource.get_dependency_state(&dependency_id);
					if dep_state == SelectionState::Unselected {
						selection_resource
							.set_dependency_state(*dependency_id, SelectionState::Parent);
					}
				}
			}

			Ok(())
		});

		if let Err(e) = result {
			eprintln!("Error during reverse DFS traversal: {:?}", e);
		}
	}
}
