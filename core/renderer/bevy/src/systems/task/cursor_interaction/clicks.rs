pub mod events;
pub mod test_utils;
pub mod utils;

use crate::components::{SelectionState, Task};
use crate::events::interactions::output::task::TaskSelectedForExternEvent;
use crate::events::interactions::TaskSelectionChangedEvent;
use crate::resources::{Roadline, SelectionResource};
use crate::systems::task::cursor_interaction::clicks::events::output::task_selected_for_extern::TouchDurationTracker;
use crate::systems::task::cursor_interaction::clicks::events::output::TaskSelectedForExternEventSystem;
use crate::systems::task::cursor_interaction::clicks::events::TaskSelectionChangedEventSystem;
use crate::systems::task::cursor_interaction::clicks::utils::TaskBoundsChecker;
use bevy::input::mouse::{MouseButton, MouseButtonInput};
use bevy::prelude::*;
use bevy::ui::BorderColor;
use roadline_util::task::Id as TaskId;

#[derive(Debug, Clone, Resource)]
pub struct TaskClickSystem {
	pub parent_task_border_color: Color,
	pub descendant_task_border_color: Color,
	pub unselected_task_border_color: Color,
	pub selected_task_border_color: Color,
	pub parent_dependency_color: Color,
	pub descendant_dependency_color: Color,
	pub unselected_dependency_color: Color,
	pub selected_dependency_color: Color,
	pub pixels_per_unit: f32,
	pub extern_event_system: TaskSelectedForExternEventSystem,
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
			pixels_per_unit: 75.0,
			extern_event_system: TaskSelectedForExternEventSystem::default(),
		}
	}
}

impl TaskClickSystem {
	/// Build a system function for task click handling
	pub fn build(
		self,
	) -> impl FnMut(
		Query<(Entity, &Transform, &Task)>,
		ResMut<SelectionResource>,
		Query<&mut BorderColor>,
		Res<Roadline>,
		EventReader<MouseButtonInput>,
		Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>)>,
		Query<&Window>,
		EventWriter<TaskSelectionChangedEvent>,
		Res<TaskSelectionChangedEventSystem>,
		EventWriter<TaskSelectedForExternEvent>,
		EventReader<TouchInput>,
		Res<ButtonInput<KeyCode>>,
		ResMut<TouchDurationTracker>,
	) {
		move |task_query: Query<(Entity, &Transform, &Task)>,
		      mut selection_resource: ResMut<SelectionResource>,
		      mut ui_query: Query<&mut BorderColor>,
		      roadline: Res<Roadline>,
		      mut mouse_events: EventReader<MouseButtonInput>,
		      camera_query: Query<
			(&Camera, &GlobalTransform),
			(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
		>,
		      windows: Query<&Window>,
		      mut task_selection_changed_events: EventWriter<TaskSelectionChangedEvent>,
		      task_selection_event_system: Res<TaskSelectionChangedEventSystem>,
		      mut task_extern_events: EventWriter<TaskSelectedForExternEvent>,
		      mut touch_events: EventReader<TouchInput>,
		      keyboard_input: Res<ButtonInput<KeyCode>>,
		      mut touch_tracker: ResMut<TouchDurationTracker>| {
			use bevy::input::ButtonState;

			// Process mouse button events
			for ev in mouse_events.read() {
				if ev.button == MouseButton::Left && ev.state == ButtonState::Pressed {
					// Get camera and window info
					let Ok((camera, camera_transform)) = camera_query.single() else {
						panic!("No camera found");
					};

					let Ok(window) = windows.single() else {
						panic!("No window found");
					};

					// Get mouse position
					let Some(cursor_position) = window.cursor_position() else {
						panic!("No cursor position found");
					};

					// Convert screen coordinates to world coordinates
					let world_pos = match camera
						.viewport_to_world_2d(camera_transform, cursor_position)
					{
						Ok(world_pos) => world_pos,
						Err(e) => {
							panic!("Failed to convert cursor position to world position: {:?}", e);
						}
					};

					self.handle_task_clicks(
						world_pos,
						ev,
						&task_query,
						&mut selection_resource,
						&mut ui_query,
						&roadline,
						self.pixels_per_unit,
						&mut task_selection_changed_events,
						&task_selection_event_system,
						&mut task_extern_events,
						&mut touch_events,
						&keyboard_input,
						&mut touch_tracker,
					);
				}
			}
		}
	}

	/// Handle click detection and selection
	pub fn handle_task_clicks(
		&self,
		world_pos: Vec2,
		mouse_event: &MouseButtonInput,
		task_query: &Query<(Entity, &Transform, &Task)>,
		selection_resource: &mut ResMut<SelectionResource>,
		ui_query: &mut Query<&mut BorderColor>,
		roadline: &Roadline,
		pixels_per_unit: f32,
		task_selection_changed_events: &mut EventWriter<TaskSelectionChangedEvent>,
		task_selection_event_system: &TaskSelectionChangedEventSystem,
		task_extern_events: &mut EventWriter<TaskSelectedForExternEvent>,
		_touch_events: &mut EventReader<TouchInput>,
		keyboard_input: &Res<ButtonInput<KeyCode>>,
		_touch_tracker: &mut ResMut<TouchDurationTracker>,
	) {
		log::info!("Handling task clicks: {:?} {:?}", mouse_event, keyboard_input);
		// Run the extern event system for this specific mouse event
		self.extern_event_system.process_single_mouse_event(
			world_pos,
			mouse_event,
			task_query,
			&keyboard_input,
			&roadline,
			task_extern_events,
		);

		// Use TaskBoundsChecker to find the clicked task
		if let Some(task_id) = TaskBoundsChecker::find_task_at_position(
			task_query,
			roadline,
			world_pos,
			pixels_per_unit,
		) {
			self.handle_task_click(
				task_id,
				selection_resource,
				ui_query,
				roadline,
				task_query,
				task_selection_changed_events,
				task_selection_event_system,
			);
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
		task_selection_changed_events: &mut EventWriter<TaskSelectionChangedEvent>,
		task_selection_event_system: &TaskSelectionChangedEventSystem,
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

		// Emit selection changed event
		task_selection_event_system.emit_task_selection_changed(
			task_selection_changed_events,
			task_id,
			current_state,
			new_state,
		);

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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::components::SelectionState;
	use crate::resources::{Roadline, SelectionResource};
	use crate::systems::task::cursor_interaction::clicks::test_utils::{
		setup_cursor_interaction_test_app, spawn_test_task,
	};
	use bevy::input::ButtonState;
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_compatible_with_spawned_tasks() -> Result<(), Box<dyn std::error::Error>> {
		let click_system = TaskClickSystem::default();

		// Setup app with all required resources
		let mut app = setup_cursor_interaction_test_app();

		// app.add_systems(Update, click_system.build());

		// Spawn tasks
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(100.0, 200.0, 0.0),
			Vec2::new(200.0, 5.0),
			"UI Test Task".to_string(),
		)?;

		// Test the click logic directly without coordinate conversion
		fn test_click_logic(
			click_system: Res<TaskClickSystem>,
			task_query: Query<(Entity, &Transform, &Task)>,
			mut selection_resource: ResMut<SelectionResource>,
			mut ui_query: Query<&mut BorderColor>,
			roadline: Res<Roadline>,
			mut task_selection_changed_events: EventWriter<TaskSelectionChangedEvent>,
			task_selection_event_system: Res<TaskSelectionChangedEventSystem>,
			mut task_extern_events: EventWriter<TaskSelectedForExternEvent>,
			mut touch_events: EventReader<TouchInput>,
			keyboard_input: Res<ButtonInput<KeyCode>>,
			mut touch_tracker: ResMut<TouchDurationTracker>,
		) {
			// Test with world coordinates that should hit the task
			// Task is at Vec3(100.0, 200.0, 0.0) with actual bounds min=(75, 175), max=(125, 225)
			// So let's click at the center: (100, 200)
			let world_pos = Vec2::new(100.0, 200.0);

			// Create a synthetic mouse event for testing
			let mouse_event = MouseButtonInput {
				button: MouseButton::Left,
				state: bevy::input::ButtonState::Pressed,
				window: Entity::from_raw(0), // Use a dummy entity for testing
			};

			click_system.handle_task_clicks(
				world_pos,
				&mouse_event,
				&task_query,
				&mut selection_resource,
				&mut ui_query,
				&roadline,
				click_system.pixels_per_unit,
				&mut task_selection_changed_events,
				&task_selection_event_system,
				&mut task_extern_events,
				&mut touch_events,
				&keyboard_input,
				&mut touch_tracker,
			);
		}

		// Add the click system as a resource and the test system
		app.insert_resource(click_system);
		app.insert_resource(crate::systems::task::cursor_interaction::clicks::events::TaskSelectionChangedEventSystem::default());
		app.insert_resource(TouchDurationTracker::default());
		app.add_event::<TaskSelectedForExternEvent>();
		app.add_systems(Update, test_click_logic);

		// Run the test system
		app.update();

		// Check that the task is now selected
		let selection_resource = app.world().resource::<SelectionResource>();
		let task_state = selection_resource.get_task_state(&TaskId::from(1));
		assert_eq!(task_state, SelectionState::Selected);

		Ok(())
	}

	#[test]
	fn test_with_camera_and_window() -> Result<(), Box<dyn std::error::Error>> {
		let click_system = TaskClickSystem::default();

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

		fn simulate_click(
			mut windows: Query<(Entity, &mut Window)>,
			mut mouse_events: EventWriter<MouseButtonInput>,
			cameras: Query<(&Camera, &GlobalTransform)>,
		) {
			let (window_entity, mut window) = windows.single_mut().unwrap();
			let (camera, camera_transform) = cameras.single().unwrap();

			// Click at 0,0 to avoid viewport issues
			let world_pos = Vec3::new(0.0, 0.0, 0.0); // Halfway between camera and origin

			// Convert world coordinates to screen coordinates
			let screen_pos = camera.world_to_viewport(camera_transform, world_pos).unwrap();

			window.set_cursor_position(Some(screen_pos));
			mouse_events.write(MouseButtonInput {
				button: MouseButton::Left,
				state: ButtonState::Pressed,
				window: window_entity,
			});
		}

		// Add the extern event
		app.add_event::<TaskSelectedForExternEvent>();

		// Systems need to be chained to avoid first registration bug.
		app.add_systems(Update, (simulate_click, click_system.build()).chain());
		app.update();

		// Check that the task is now selected
		let selection_resource = app.world().resource::<SelectionResource>();
		let task_state = selection_resource.get_task_state(&TaskId::from(1));
		assert_eq!(task_state, SelectionState::Selected);

		Ok(())
	}
}
