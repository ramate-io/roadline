use crate::components::{SelectionState, Task};
use crate::events::interactions::output::task::TaskSelectedForExternEvent;
use bevy::input::mouse::{MouseButton, MouseButtonInput};
use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Configuration for input combinations that trigger external task selection
#[derive(Debug, Clone)]
pub enum InputTrigger {
	/// Trigger on left click only
	LeftClick,
	/// Trigger on right click only
	RightClick,
	/// Trigger on shift + left click (default)
	ShiftLeftClick,
	/// Trigger on ctrl + left click
	CtrlLeftClick,
	/// Trigger on alt + left click
	AltLeftClick,
	/// Trigger on any click (for testing)
	AnyClick,
}

impl Default for InputTrigger {
	fn default() -> Self {
		Self::ShiftLeftClick
	}
}

impl InputTrigger {
	/// Check if the current input state matches this trigger
	pub fn matches(
		&self,
		button: MouseButton,
		input: &MouseButtonInput,
		keyboard_input: &ButtonInput<KeyCode>,
	) -> bool {
		match self {
			InputTrigger::LeftClick => button == MouseButton::Left && input.state.is_pressed(),
			InputTrigger::RightClick => button == MouseButton::Right && input.state.is_pressed(),
			InputTrigger::ShiftLeftClick => {
				button == MouseButton::Left
					&& input.state.is_pressed()
					&& keyboard_input.pressed(KeyCode::ShiftLeft)
			}
			InputTrigger::CtrlLeftClick => {
				button == MouseButton::Left
					&& input.state.is_pressed()
					&& keyboard_input.pressed(KeyCode::ControlLeft)
			}
			InputTrigger::AltLeftClick => {
				button == MouseButton::Left
					&& input.state.is_pressed()
					&& keyboard_input.pressed(KeyCode::AltLeft)
			}
			InputTrigger::AnyClick => input.state.is_pressed(),
		}
	}
}

/// System for handling task selection for external systems
#[derive(Debug, Clone, Resource)]
pub struct TaskSelectedForExternEventSystem {
	/// Input combination that triggers external selection
	pub input_trigger: InputTrigger,
	/// Whether to emit events
	pub emit_events: bool,
}

impl Default for TaskSelectedForExternEventSystem {
	fn default() -> Self {
		Self { input_trigger: InputTrigger::default(), emit_events: true }
	}
}

impl TaskSelectedForExternEventSystem {
	/// Build a system function for emitting task selected for extern events
	pub fn build(
		self,
	) -> impl FnMut(
		Query<(Entity, &Transform, &Task)>,
		EventReader<MouseButtonInput>,
		Res<ButtonInput<KeyCode>>,
		EventWriter<TaskSelectedForExternEvent>,
	) {
		move |task_query: Query<(Entity, &Transform, &Task)>,
		      mut mouse_events: EventReader<MouseButtonInput>,
		      keyboard_input: Res<ButtonInput<KeyCode>>,
		      mut events: EventWriter<TaskSelectedForExternEvent>| {
			if !self.emit_events {
				return;
			}

			// Process mouse events
			for mouse_event in mouse_events.read() {
				if self.input_trigger.matches(mouse_event.button, mouse_event, &keyboard_input) {
					// Find the task that was clicked
					for (_entity, _transform, task) in task_query.iter() {
						// Simple bounding box check - in a real implementation, this would be more sophisticated
						// For now, we'll emit an event for any task when the trigger is activated
						// This is a placeholder for proper hit detection
						self.emit_task_selected_for_extern(
							&mut events,
							task.task_id,
							SelectionState::Selected,
						);
					}
				}
			}
		}
	}

	/// Emit a task selected for extern event
	pub fn emit_task_selected_for_extern(
		&self,
		events: &mut EventWriter<TaskSelectedForExternEvent>,
		selected_task: TaskId,
		renderer_selection_state: SelectionState,
	) {
		if self.emit_events {
			events.write(TaskSelectedForExternEvent { selected_task, renderer_selection_state });
		}
	}

	/// Create a system that emits events for testing purposes
	pub fn build_test_system(
		self,
	) -> impl FnMut(Query<(Entity, &Transform, &Task)>, EventWriter<TaskSelectedForExternEvent>) {
		move |task_query: Query<(Entity, &Transform, &Task)>,
		      mut events: EventWriter<TaskSelectedForExternEvent>| {
			if !self.emit_events {
				return;
			}

			// For testing, emit events for all tasks
			for (_entity, _transform, task) in task_query.iter() {
				self.emit_task_selected_for_extern(
					&mut events,
					task.task_id,
					SelectionState::Selected,
				);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use bevy::ecs::system::RunSystemOnce;

	/// Setup a test app with basic resources for testing the event system
	fn setup_event_test_app() -> bevy::prelude::App {
		let mut app = setup_task_test_app();
		app.add_plugins(bevy::input::InputPlugin);
		app.add_event::<TaskSelectedForExternEvent>();
		app.add_event::<MouseButtonInput>();
		app
	}

	#[test]
	fn test_event_system_builds_correctly() {
		let mut app = setup_event_test_app();
		let event_system = TaskSelectedForExternEventSystem::default();

		// The system should build without panicking
		let system_fn = event_system.build();
		app.add_systems(bevy::prelude::Update, system_fn);

		// Run the app to ensure it doesn't panic
		app.update();
	}

	#[test]
	fn test_event_system_with_tasks() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_event_test_app();

		// Spawn a test task
		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(1),
			bevy::math::Vec3::new(0.0, 0.0, 0.0),
			bevy::math::Vec2::new(100.0, 50.0),
			"Test Task".to_string(),
		);
		app.world_mut().run_system_once(params.build())?;

		// Create event system with events enabled
		let event_system =
			TaskSelectedForExternEventSystem { emit_events: true, ..Default::default() };

		// Build and run the test system
		let system_fn = event_system.build_test_system();
		app.add_systems(bevy::prelude::Update, system_fn);
		app.update();

		// Check that events were emitted
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectedForExternEvent>>();
		let event_count = events.drain().count();
		assert!(event_count > 0, "Should emit events when emit_events is true");

		Ok(())
	}

	#[test]
	fn test_event_system_disabled() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_event_test_app();

		// Spawn a test task
		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(1),
			bevy::math::Vec3::new(0.0, 0.0, 0.0),
			bevy::math::Vec2::new(100.0, 50.0),
			"Test Task".to_string(),
		);
		app.world_mut().run_system_once(params.build())?;

		// Create event system with events disabled
		let event_system =
			TaskSelectedForExternEventSystem { emit_events: false, ..Default::default() };

		// Build and run the test system
		let system_fn = event_system.build_test_system();
		app.add_systems(bevy::prelude::Update, system_fn);
		app.update();

		// Check that no events were emitted
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectedForExternEvent>>();
		let event_count = events.drain().count();
		assert_eq!(event_count, 0, "Should not emit events when emit_events is false");

		Ok(())
	}

	#[test]
	fn test_event_system_direct_emission() {
		let mut app = setup_event_test_app();
		let event_system = TaskSelectedForExternEventSystem::default();

		// Test direct emission method by running a system
		let test_system = move |mut events: EventWriter<TaskSelectedForExternEvent>| {
			event_system.emit_task_selected_for_extern(
				&mut events,
				TaskId::from(1),
				SelectionState::Selected,
			);
		};

		app.add_systems(bevy::prelude::Update, test_system);
		app.update();

		// Check that event was emitted
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectedForExternEvent>>();
		let event_count = events.drain().count();
		assert_eq!(event_count, 1, "Should emit one event when called directly");
	}

	#[test]
	fn test_input_trigger_left_click() {
		let trigger = InputTrigger::LeftClick;

		// Mock mouse button input for left click
		let mouse_input = MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		// Mock keyboard input (no modifiers)
		let keyboard_input = ButtonInput::<KeyCode>::default();

		assert!(trigger.matches(MouseButton::Left, &mouse_input, &keyboard_input));
		assert!(!trigger.matches(MouseButton::Right, &mouse_input, &keyboard_input));
	}

	#[test]
	fn test_input_trigger_shift_left_click() {
		let trigger = InputTrigger::ShiftLeftClick;

		// Mock mouse button input for left click
		let mouse_input = MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		// Mock keyboard input with shift pressed
		let mut keyboard_input = ButtonInput::<KeyCode>::default();
		keyboard_input.press(KeyCode::ShiftLeft);

		assert!(trigger.matches(MouseButton::Left, &mouse_input, &keyboard_input));

		// Test without shift
		let keyboard_input_no_shift = ButtonInput::<KeyCode>::default();
		assert!(!trigger.matches(MouseButton::Left, &mouse_input, &keyboard_input_no_shift));
	}

	#[test]
	fn test_input_trigger_any_click() {
		let trigger = InputTrigger::AnyClick;

		// Mock mouse button input for any click
		let mouse_input = MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		let keyboard_input = ButtonInput::<KeyCode>::default();

		assert!(trigger.matches(MouseButton::Left, &mouse_input, &keyboard_input));
		assert!(trigger.matches(MouseButton::Right, &mouse_input, &keyboard_input));
		assert!(trigger.matches(MouseButton::Middle, &mouse_input, &keyboard_input));
	}
}
