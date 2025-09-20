use crate::components::{Dependency, SelectionState, Task};
use crate::resources::{Roadline, SelectionResource};
use bevy::prelude::*;
use bevy::ui::BorderColor;
use roadline_util::dependency::Id as DependencyId;
use roadline_util::task::Id as TaskId;

/// System to handle click selection for tasks and dependencies
pub fn click_selection_system(
	camera_query: Query<
		(&Camera, &GlobalTransform),
		(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
	>,
	windows: Query<&Window>,
	mouse_input: Res<ButtonInput<MouseButton>>,
	task_query: Query<(Entity, &Transform, &Task)>,
	dependency_query: Query<(
		Entity,
		&Dependency,
		&crate::systems::dependency::DependencyCurveData,
	)>,
	mut ui_query: Query<&mut BorderColor>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut selection_resource: ResMut<SelectionResource>,
	roadline: Option<Res<Roadline>>,
) {
	// Only process clicks on left mouse button
	if !mouse_input.just_pressed(MouseButton::Left) {
		return;
	}

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
		return;
	};

	// Convert screen coordinates to world coordinates
	let world_pos_result = camera.viewport_to_world_2d(camera_transform, cursor_position);
	let Ok(world_pos) = world_pos_result else {
		return;
	};

	// Get the visual bounds to scale everything properly (same as task system)
	let (max_width, max_height) = roadline.visual_bounds();
	let _max_width_f32 = max_width.value() as f32;
	let _max_height_f32 = max_height.value() as f32;

	// Scale factor: same as tasks
	let pixels_per_unit = 50.0;

	// Check for task clicks first (higher priority)
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
			handle_task_click(
				&task.task_id,
				&mut selection_resource,
				&mut ui_query,
				&mut materials,
				&roadline,
				&task_query,
			);
			return; // Exit early if we clicked on a task
		}
	}

	// Check for dependency clicks
	for (_entity, dependency, curve_data) in dependency_query.iter() {
		// Calculate distance to the bezier curve
		let mouse_pos_3d = Vec3::new(world_pos.x, world_pos.y, 0.0);
		let distance_to_curve = distance_to_bezier_curve(
			mouse_pos_3d,
			curve_data.start,
			curve_data.control1,
			curve_data.control2,
			curve_data.end,
		);

		// If mouse is within 30 pixels of the curve
		if distance_to_curve < 30.0 {
			handle_dependency_click(
				&dependency.dependency_id,
				&mut selection_resource,
				&mut ui_query,
				&mut materials,
				&roadline,
				&task_query,
			);
			return; // Exit early if we clicked on a dependency
		}
	}
}

/// Handle clicking on a task
fn handle_task_click(
	task_id: &TaskId,
	selection_resource: &mut ResMut<SelectionResource>,
	ui_query: &mut Query<&mut BorderColor>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	roadline: &Roadline,
	task_query: &Query<(Entity, &Transform, &Task)>,
) {
	// Get current selection state
	let current_state = selection_resource.get_task_state(task_id);

	// Toggle selection
	let new_state = match current_state {
		SelectionState::Unselected => SelectionState::Selected,
		SelectionState::Selected => SelectionState::Unselected,
		SelectionState::Descendant => SelectionState::Selected, // Now the descendant is selected it will have to be manually unselected.
	};

	selection_resource.set_task_state(*task_id, new_state);

	// Update visual feedback
	update_task_visual_feedback(*task_id, new_state, ui_query, task_query);

	// If selecting, mark all descendants
	if new_state == SelectionState::Selected {
		mark_descendants(task_id, selection_resource, ui_query, materials, roadline, task_query);
	} else {
		// If deselecting, clear all descendants
		clear_all_selections(selection_resource, ui_query, materials, task_query);
	}
}

/// Handle clicking on a dependency
fn handle_dependency_click(
	dependency_id: &DependencyId,
	selection_resource: &mut ResMut<SelectionResource>,
	ui_query: &mut Query<&mut BorderColor>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	roadline: &Roadline,
	task_query: &Query<(Entity, &Transform, &Task)>,
) {
	// Get current selection state
	let current_state = selection_resource.get_dependency_state(dependency_id);

	// Toggle selection
	let new_state = match current_state {
		SelectionState::Unselected => SelectionState::Selected,
		SelectionState::Selected => SelectionState::Unselected,
		SelectionState::Descendant => SelectionState::Unselected, // Can't directly select descendants
	};

	selection_resource.set_dependency_state(*dependency_id, new_state);

	// Update visual feedback
	update_dependency_visual_feedback(*dependency_id, new_state, materials);

	// If selecting, mark all descendants from the "to" task
	if new_state == SelectionState::Selected {
		// Get the "to" task from the dependency
		let to_task_id = &dependency_id.to();
		mark_descendants(to_task_id, selection_resource, ui_query, materials, roadline, task_query);
	} else {
		// If deselecting, clear all descendants
		clear_all_selections(selection_resource, ui_query, materials, task_query);
	}
}

/// Mark all descendants of a task using DFS
fn mark_descendants(
	start_task_id: &TaskId,
	selection_resource: &mut ResMut<SelectionResource>,
	ui_query: &mut Query<&mut BorderColor>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	roadline: &Roadline,
	task_query: &Query<(Entity, &Transform, &Task)>,
) {
	// Use DFS to traverse the graph
	let result = roadline.dfs(start_task_id, |task_id, _depth| {
		// Skip the start task (it's already selected)
		if task_id == start_task_id {
			return Ok(());
		}

		// Mark the task as descendant
		selection_resource.set_task_state(*task_id, SelectionState::Descendant);
		update_task_visual_feedback(*task_id, SelectionState::Descendant, ui_query, task_query);

		// Find and mark dependencies that lead to this task
		// We need to iterate through all dependencies to find ones that lead to this task
		for (dependency_id, _) in roadline.connections() {
			if &dependency_id.to() == task_id {
				selection_resource.set_dependency_state(*dependency_id, SelectionState::Descendant);
				update_dependency_visual_feedback(
					*dependency_id,
					SelectionState::Descendant,
					materials,
				);
			}
		}

		Ok(())
	});

	if let Err(e) = result {
		eprintln!("Error during DFS traversal: {:?}", e);
	}
}

/// Clear all selections
fn clear_all_selections(
	selection_resource: &mut ResMut<SelectionResource>,
	ui_query: &mut Query<&mut BorderColor>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	task_query: &Query<(Entity, &Transform, &Task)>,
) {
	// Clear all task selections
	let task_ids: Vec<TaskId> = selection_resource.tasks.keys().copied().collect();
	for task_id in task_ids {
		selection_resource.set_task_state(task_id, SelectionState::Unselected);
		update_task_visual_feedback(task_id, SelectionState::Unselected, ui_query, task_query);
	}

	// Clear all dependency selections
	let dependency_ids: Vec<DependencyId> =
		selection_resource.dependencies.keys().copied().collect();
	for dependency_id in dependency_ids {
		selection_resource.set_dependency_state(dependency_id, SelectionState::Unselected);
		update_dependency_visual_feedback(dependency_id, SelectionState::Unselected, materials);
	}
}

/// Update visual feedback for a task
fn update_task_visual_feedback(
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
						println!("Setting to BLACK");
						border_color.0 = Color::BLACK;
					}
					SelectionState::Selected => {
						println!("Setting to DARK BLUE");
						border_color.0 = Color::oklch(0.5, 0.137, 235.06); // Dark blue
					}
					SelectionState::Descendant => {
						println!("Setting to DARK BLUE");
						border_color.0 = Color::oklch(0.5, 0.137, 235.06); // Same as selected for now
					}
				}
			}
		}
	}
}

/// Update visual feedback for a dependency
fn update_dependency_visual_feedback(
	_dependency_id: DependencyId,
	_state: SelectionState,
	_materials: &mut ResMut<Assets<ColorMaterial>>,
) {
	// This would need to be implemented based on how we access the material
	// For now, we'll implement this when we have the material handle
}

/// Calculate the distance from a point to a cubic bezier curve
fn distance_to_bezier_curve(point: Vec3, p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> f32 {
	// Sample points along the curve and find the minimum distance
	let mut min_distance = f32::INFINITY;
	let steps = 32; // Number of samples along the curve

	for i in 0..=steps {
		let t = i as f32 / steps as f32;
		let curve_point = cubic_bezier(p0, p1, p2, p3, t);
		let distance = point.distance(curve_point);
		min_distance = min_distance.min(distance);
	}

	min_distance
}

/// Cubic bezier helper function
fn cubic_bezier(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
	let u = 1.0 - t;
	u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}
