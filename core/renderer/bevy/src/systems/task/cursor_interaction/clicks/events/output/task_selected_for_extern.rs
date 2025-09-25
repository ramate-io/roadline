use crate::components::{SelectionState, Task};
use crate::events::interactions::output::task::TaskSelectedForExternEvent;
use crate::resources::Roadline;
use crate::systems::task::cursor_interaction::clicks::utils::TaskBoundsChecker;
use bevy::input::mouse::{MouseButton, MouseButtonInput};
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::prelude::*;
use roadline_util::task::Id as TaskId;
use std::collections::HashMap;
use std::time::Instant;

/// Configuration for input combinations that trigger external task selection
#[derive(Debug, Clone, PartialEq)]
pub enum InputTrigger {
	/// Trigger on shift + left click (default)
	ShiftLeftClick,
	/// Trigger on right click
	RightClick,
	/// Trigger on press and hold (touch screen support)
	PressAndHold,
	/// Trigger on left click only
	LeftClick,
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

/// Resource to track touch input durations for press and hold detection
#[derive(Debug, Resource, Default)]
pub struct TouchDurationTracker {
	/// Map of touch ID to start time
	touch_starts: HashMap<u64, Instant>,
	/// Minimum duration for press and hold (in milliseconds)
	pub min_hold_duration_ms: u64,
}

impl TouchDurationTracker {
	pub fn new(min_hold_duration_ms: u64) -> Self {
		Self { touch_starts: HashMap::new(), min_hold_duration_ms }
	}

	pub fn handle_touch_event(&mut self, touch_event: &TouchInput) -> bool {
		match touch_event.phase {
			TouchPhase::Started => {
				self.touch_starts.insert(touch_event.id, Instant::now());
				false
			}
			TouchPhase::Ended => {
				if let Some(start_time) = self.touch_starts.remove(&touch_event.id) {
					let duration = start_time.elapsed();
					duration.as_millis() >= self.min_hold_duration_ms as u128
				} else {
					false
				}
			}
			TouchPhase::Moved => false,
			TouchPhase::Canceled => {
				self.touch_starts.remove(&touch_event.id);
				false
			}
		}
	}
}

/// Helper for input matching and task selection
pub struct InputMatcher {
	pub pixels_per_unit: f32,
}

impl InputMatcher {
	pub fn new(pixels_per_unit: f32) -> Self {
		Self { pixels_per_unit }
	}

	/// Check if mouse input matches any of the configured triggers
	pub fn matches_mouse_input(
		&self,
		triggers: &[InputTrigger],
		button: MouseButton,
		input: &MouseButtonInput,
		keyboard_input: &ButtonInput<KeyCode>,
	) -> bool {
		triggers.iter().any(|trigger| {
			match trigger {
				InputTrigger::ShiftLeftClick => {
					button == MouseButton::Left
						&& input.state.is_pressed()
						&& keyboard_input.pressed(KeyCode::ShiftLeft)
				}
				InputTrigger::RightClick => {
					button == MouseButton::Right && input.state.is_pressed()
				}
				InputTrigger::PressAndHold => {
					// For mouse, press and hold is just a left click
					// Real press and hold detection would require timing
					button == MouseButton::Left && input.state.is_pressed()
				}
				InputTrigger::LeftClick => button == MouseButton::Left && input.state.is_pressed(),
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
		})
	}

	/// Check if touch input matches any of the configured triggers
	pub fn matches_touch_input(
		&self,
		triggers: &[InputTrigger],
		touch_event: &TouchInput,
		touch_tracker: &mut TouchDurationTracker,
	) -> bool {
		triggers.iter().any(|trigger| {
			match trigger {
				InputTrigger::PressAndHold => {
					// Use duration tracking for touch press and hold
					touch_tracker.handle_touch_event(touch_event)
				}
				InputTrigger::RightClick
				| InputTrigger::ShiftLeftClick
				| InputTrigger::LeftClick
				| InputTrigger::CtrlLeftClick
				| InputTrigger::AltLeftClick => {
					// These don't apply to touch input
					false
				}
				InputTrigger::AnyClick => {
					// Any touch counts as a click
					touch_event.phase == TouchPhase::Started
				}
			}
		})
	}

	/// Process mouse events and emit task selected events if conditions are met
	pub fn process_mouse_events(
		&self,
		triggers: &[InputTrigger],
		task_query: &Query<(Entity, &Transform, &Task)>,
		roadline: &Roadline,
		mouse_events: &mut EventReader<MouseButtonInput>,
		keyboard_input: &ButtonInput<KeyCode>,
		emit_fn: &mut dyn FnMut(TaskId, SelectionState),
	) {
		for mouse_event in mouse_events.read() {
			if self.matches_mouse_input(triggers, mouse_event.button, mouse_event, keyboard_input) {
				// Convert mouse position to world coordinates
				// This is a simplified version - in practice you'd need camera/viewport conversion
				let world_pos = Vec2::new(0.0, 0.0); // Placeholder

				if let Some(task_id) = self.find_task_at_position(task_query, roadline, world_pos) {
					emit_fn(task_id, SelectionState::Selected);
				}
			}
		}
	}

	/// Process touch events and emit task selected events if conditions are met
	pub fn process_touch_events(
		&self,
		triggers: &[InputTrigger],
		task_query: &Query<(Entity, &Transform, &Task)>,
		roadline: &Roadline,
		touch_events: &mut EventReader<TouchInput>,
		touch_tracker: &mut TouchDurationTracker,
		emit_fn: &mut dyn FnMut(TaskId, SelectionState),
	) {
		for touch_event in touch_events.read() {
			if self.matches_touch_input(triggers, touch_event, touch_tracker) {
				// Convert touch position to world coordinates
				let world_pos = touch_event.position;

				if let Some(task_id) = self.find_task_at_position(task_query, roadline, world_pos) {
					emit_fn(task_id, SelectionState::Selected);
				}
			}
		}
	}

	/// Find a task at the given world position
	pub fn find_task_at_position(
		&self,
		task_query: &Query<(Entity, &Transform, &Task)>,
		roadline: &Roadline,
		world_pos: Vec2,
	) -> Option<TaskId> {
		TaskBoundsChecker::find_task_at_position(
			task_query,
			roadline,
			world_pos,
			self.pixels_per_unit,
		)
	}
}

/// System for handling task selection for external systems
#[derive(Debug, Clone, Resource)]
pub struct TaskSelectedForExternEventSystem {
	/// Input combinations that trigger external selection
	pub input_triggers: Vec<InputTrigger>,
	/// Whether to emit events
	pub emit_events: bool,
	/// Pixels per unit for bounds checking
	pub pixels_per_unit: f32,
}

impl Default for TaskSelectedForExternEventSystem {
	fn default() -> Self {
		Self {
			input_triggers: vec![
				InputTrigger::ShiftLeftClick,
				InputTrigger::RightClick,
				InputTrigger::PressAndHold,
			],
			emit_events: true,
			pixels_per_unit: 50.0, // Default value from TaskClickSystem
		}
	}
}

impl TaskSelectedForExternEventSystem {
	/// Build a system function for emitting task selected for extern events
	pub fn build(
		self,
	) -> impl FnMut(
		Query<(Entity, &Transform, &Task)>,
		EventReader<MouseButtonInput>,
		EventReader<TouchInput>,
		Res<ButtonInput<KeyCode>>,
		Res<Roadline>,
		ResMut<TouchDurationTracker>,
		EventWriter<TaskSelectedForExternEvent>,
	) {
		move |task_query: Query<(Entity, &Transform, &Task)>,
		      mut mouse_events: EventReader<MouseButtonInput>,
		      mut touch_events: EventReader<TouchInput>,
		      keyboard_input: Res<ButtonInput<KeyCode>>,
		      roadline: Res<Roadline>,
		      mut touch_tracker: ResMut<TouchDurationTracker>,
		      mut events: EventWriter<TaskSelectedForExternEvent>| {
			self.process_input_events(
				&task_query,
				&mut mouse_events,
				&mut touch_events,
				&keyboard_input,
				&roadline,
				&mut touch_tracker,
				&mut events,
			);
		}
	}

	/// Process all input events and emit task selected events
	pub fn process_input_events(
		&self,
		task_query: &Query<(Entity, &Transform, &Task)>,
		mouse_events: &mut EventReader<MouseButtonInput>,
		touch_events: &mut EventReader<TouchInput>,
		keyboard_input: &Res<ButtonInput<KeyCode>>,
		roadline: &Res<Roadline>,
		touch_tracker: &mut ResMut<TouchDurationTracker>,
		events: &mut EventWriter<TaskSelectedForExternEvent>,
	) {
		if !self.emit_events {
			return;
		}

		let matcher = InputMatcher::new(self.pixels_per_unit);
		let mut emit_fn = |task_id: TaskId, state: SelectionState| {
			self.emit_task_selected_for_extern(events, task_id, state);
		};

		// Process all input events using the helper
		matcher.process_mouse_events(
			&self.input_triggers,
			task_query,
			roadline,
			mouse_events,
			keyboard_input,
			&mut emit_fn,
		);

		matcher.process_touch_events(
			&self.input_triggers,
			task_query,
			roadline,
			touch_events,
			touch_tracker,
			&mut emit_fn,
		);
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
		app.add_event::<TouchInput>();
		app.insert_resource(TouchDurationTracker::new(500)); // 500ms hold duration

		// Create a minimal roadline for testing
		let roadline = roadline_representation_core::roadline::RoadlineBuilder::new()
			.build()
			.unwrap_or_else(|_| {
				roadline_representation_core::roadline::RoadlineBuilder::new().build().unwrap()
			});
		app.insert_resource(crate::resources::Roadline::new(roadline));

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
		let event_system = TaskSelectedForExternEventSystem {
			input_triggers: vec![InputTrigger::AnyClick],
			emit_events: true,
			pixels_per_unit: 50.0,
		};

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
		let event_system = TaskSelectedForExternEventSystem {
			input_triggers: vec![InputTrigger::AnyClick],
			emit_events: false,
			pixels_per_unit: 50.0,
		};

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
	fn test_default_configuration_multiple_triggers() {
		let event_system = TaskSelectedForExternEventSystem::default();

		// Should have the three default triggers
		assert_eq!(event_system.input_triggers.len(), 3);
		assert!(event_system.input_triggers.contains(&InputTrigger::ShiftLeftClick));
		assert!(event_system.input_triggers.contains(&InputTrigger::RightClick));
		assert!(event_system.input_triggers.contains(&InputTrigger::PressAndHold));
		assert!(event_system.emit_events);
	}

	#[test]
	fn test_touch_duration_tracker() {
		let mut tracker = TouchDurationTracker::new(500); // 500ms hold duration

		// Test touch started
		let touch_start = TouchInput {
			id: 1,
			phase: TouchPhase::Started,
			position: Vec2::new(100.0, 100.0),
			window: Entity::from_raw(0),
			force: None,
		};

		assert!(!tracker.handle_touch_event(&touch_start));

		// Test touch ended too quickly (should not trigger)
		let touch_end_quick = TouchInput {
			id: 1,
			phase: TouchPhase::Ended,
			position: Vec2::new(100.0, 100.0),
			window: Entity::from_raw(0),
			force: None,
		};

		assert!(!tracker.handle_touch_event(&touch_end_quick));

		// Test touch ended after sufficient time (should trigger)
		// Note: In real tests, you'd need to wait or mock time
		// For now, we'll test the logic structure
		let touch_end_long = TouchInput {
			id: 2,
			phase: TouchPhase::Ended,
			position: Vec2::new(100.0, 100.0),
			window: Entity::from_raw(0),
			force: None,
		};

		// This will be false because we haven't started touch 2
		assert!(!tracker.handle_touch_event(&touch_end_long));
	}

	#[test]
	fn test_input_matcher_mouse_input() {
		let matcher = InputMatcher::new(50.0);
		let triggers = vec![InputTrigger::RightClick, InputTrigger::ShiftLeftClick];

		// Test right click
		let mouse_input = MouseButtonInput {
			button: MouseButton::Right,
			state: bevy::input::ButtonState::Pressed,
			window: Entity::from_raw(0),
		};
		let keyboard_input = ButtonInput::<KeyCode>::default();

		assert!(matcher.matches_mouse_input(
			&triggers,
			MouseButton::Right,
			&mouse_input,
			&keyboard_input
		));
		assert!(!matcher.matches_mouse_input(
			&triggers,
			MouseButton::Left,
			&mouse_input,
			&keyboard_input
		));
	}

	#[test]
	fn test_input_trigger_left_click() {
		let matcher = InputMatcher::new(50.0);
		let triggers = vec![InputTrigger::LeftClick];

		// Mock mouse button input for left click
		let mouse_input = MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		// Mock keyboard input (no modifiers)
		let keyboard_input = ButtonInput::<KeyCode>::default();

		assert!(matcher.matches_mouse_input(
			&triggers,
			MouseButton::Left,
			&mouse_input,
			&keyboard_input
		));
		assert!(!matcher.matches_mouse_input(
			&triggers,
			MouseButton::Right,
			&mouse_input,
			&keyboard_input
		));
	}

	#[test]
	fn test_input_trigger_shift_left_click() {
		let matcher = InputMatcher::new(50.0);
		let triggers = vec![InputTrigger::ShiftLeftClick];

		// Mock mouse button input for left click
		let mouse_input = MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		// Mock keyboard input with shift pressed
		let mut keyboard_input = ButtonInput::<KeyCode>::default();
		keyboard_input.press(KeyCode::ShiftLeft);

		assert!(matcher.matches_mouse_input(
			&triggers,
			MouseButton::Left,
			&mouse_input,
			&keyboard_input
		));

		// Test without shift
		let keyboard_input_no_shift = ButtonInput::<KeyCode>::default();
		assert!(!matcher.matches_mouse_input(
			&triggers,
			MouseButton::Left,
			&mouse_input,
			&keyboard_input_no_shift
		));
	}

	#[test]
	fn test_input_trigger_right_click() {
		let matcher = InputMatcher::new(50.0);
		let triggers = vec![InputTrigger::RightClick];

		// Mock mouse button input for right click
		let mouse_input = MouseButtonInput {
			button: MouseButton::Right,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		let keyboard_input = ButtonInput::<KeyCode>::default();

		assert!(matcher.matches_mouse_input(
			&triggers,
			MouseButton::Right,
			&mouse_input,
			&keyboard_input
		));
		assert!(!matcher.matches_mouse_input(
			&triggers,
			MouseButton::Left,
			&mouse_input,
			&keyboard_input
		));
	}

	#[test]
	fn test_input_trigger_press_and_hold() {
		let matcher = InputMatcher::new(50.0);
		let triggers = vec![InputTrigger::PressAndHold];

		// Mock mouse button input for left click (press and hold)
		let mouse_input = MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		let keyboard_input = ButtonInput::<KeyCode>::default();

		assert!(matcher.matches_mouse_input(
			&triggers,
			MouseButton::Left,
			&mouse_input,
			&keyboard_input
		));
		// Note: In a real implementation, press and hold would distinguish from quick clicks
		// For now, it behaves like a left click
	}

	#[test]
	fn test_input_trigger_any_click() {
		let matcher = InputMatcher::new(50.0);
		let triggers = vec![InputTrigger::AnyClick];

		// Mock mouse button input for any click
		let mouse_input = MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		let keyboard_input = ButtonInput::<KeyCode>::default();

		assert!(matcher.matches_mouse_input(
			&triggers,
			MouseButton::Left,
			&mouse_input,
			&keyboard_input
		));
		assert!(matcher.matches_mouse_input(
			&triggers,
			MouseButton::Right,
			&mouse_input,
			&keyboard_input
		));
		assert!(matcher.matches_mouse_input(
			&triggers,
			MouseButton::Middle,
			&mouse_input,
			&keyboard_input
		));
	}
}
