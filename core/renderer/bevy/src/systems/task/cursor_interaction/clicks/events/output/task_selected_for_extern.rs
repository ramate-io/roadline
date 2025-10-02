use crate::components::{SelectionState, Task};
use crate::events::interactions::output::task::TaskSelectedForExternEvent;
use crate::resources::Roadline;
use crate::systems::task::cursor_interaction::clicks::utils::TaskBoundsChecker;
use bevy::input::mouse::{MouseButton, MouseButtonInput};
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::input::ButtonState;
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
	pub pixels_per_x_unit: f32,
	pub pixels_per_y_unit: f32,
}

impl InputMatcher {
	pub fn new(pixels_per_x_unit: f32, pixels_per_y_unit: f32) -> Self {
		Self { pixels_per_x_unit, pixels_per_y_unit }
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
			log::info!("Checking trigger: {:?}", trigger);
			match trigger {
				InputTrigger::ShiftLeftClick => {
					button == MouseButton::Left
						&& input.state.is_pressed()
						&& keyboard_input.pressed(KeyCode::ShiftLeft)
				}
				InputTrigger::RightClick => {
					button == MouseButton::Right && input.state.is_pressed()
				}
				InputTrigger::PressAndHold => false, // touch input only
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
		world_pos: Vec2,
		task_query: &Query<(Entity, &Transform, &Task)>,
		roadline: &Roadline,
		mouse_events: &mut EventReader<MouseButtonInput>,
		keyboard_input: &ButtonInput<KeyCode>,
		emit_fn: &mut dyn FnMut(TaskId, SelectionState),
	) {
		for mouse_event in mouse_events.read() {
			log::info!("Processing mouse event: {:?} {:?}", mouse_event, keyboard_input);
			if self.matches_mouse_input(triggers, mouse_event.button, mouse_event, keyboard_input) {
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
			self.pixels_per_x_unit,
			self.pixels_per_y_unit,
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
	/// Pixels per x unit for bounds checking
	pub pixels_per_x_unit: f32,
	/// Pixels per y unit for bounds checking
	pub pixels_per_y_unit: f32,
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
			pixels_per_x_unit: 10.0,
			pixels_per_y_unit: 75.0,
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
		Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>)>,
		Query<&Window>,
	) {
		move |task_query: Query<(Entity, &Transform, &Task)>,
		      mut mouse_events: EventReader<MouseButtonInput>,
		      mut _touch_events: EventReader<TouchInput>,
		      keyboard_input: Res<ButtonInput<KeyCode>>,
		      roadline: Res<Roadline>,
		      mut _touch_tracker: ResMut<TouchDurationTracker>,
		      mut events: EventWriter<TaskSelectedForExternEvent>,
		      camera_query: Query<
			(&Camera, &GlobalTransform),
			(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
		>,
		      windows: Query<&Window>| {
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

					self.process_single_mouse_event(
						world_pos,
						ev,
						&task_query,
						&keyboard_input,
						&roadline,
						&mut events,
					);
				}
			}
		}
	}

	/// Process a single mouse event and emit task selected events if conditions are met
	pub fn process_single_mouse_event(
		&self,
		world_pos: Vec2,
		mouse_event: &MouseButtonInput,
		task_query: &Query<(Entity, &Transform, &Task)>,
		keyboard_input: &Res<ButtonInput<KeyCode>>,
		roadline: &Roadline,
		events: &mut EventWriter<TaskSelectedForExternEvent>,
	) {
		log::info!("Processing mouse event: {:?}", self);
		if !self.emit_events {
			return;
		}

		let matcher = InputMatcher::new(self.pixels_per_x_unit, self.pixels_per_y_unit);
		let mut emit_fn = |task_id: TaskId, state: SelectionState| {
			log::info!("Emitting task selected for extern event: {:?} {:?}", task_id, state);
			self.emit_task_selected_for_extern(events, task_id, state);
		};

		// Check if this mouse event matches any of our triggers
		if matcher.matches_mouse_input(
			&self.input_triggers,
			mouse_event.button,
			mouse_event,
			keyboard_input,
		) {
			log::info!("Mouse event matches triggers");
			if let Some(task_id) = matcher.find_task_at_position(task_query, roadline, world_pos) {
				emit_fn(task_id, SelectionState::Selected);
			} else {
				log::info!("No task found at position");
			}
		}
	}

	/// Process all input events and emit task selected events
	pub fn process_input_events(
		&self,
		world_pos: Vec2,
		task_query: &Query<(Entity, &Transform, &Task)>,
		mouse_events: &mut EventReader<MouseButtonInput>,
		touch_events: &mut EventReader<TouchInput>,
		keyboard_input: &Res<ButtonInput<KeyCode>>,
		roadline: &Roadline,
		touch_tracker: &mut ResMut<TouchDurationTracker>,
		events: &mut EventWriter<TaskSelectedForExternEvent>,
	) {
		if !self.emit_events {
			return;
		}

		let matcher = InputMatcher::new(self.pixels_per_x_unit, self.pixels_per_y_unit);
		let mut emit_fn = |task_id: TaskId, state: SelectionState| {
			self.emit_task_selected_for_extern(events, task_id, state);
		};

		// Process all input events using the helper
		matcher.process_mouse_events(
			&self.input_triggers,
			world_pos,
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
			let event = TaskSelectedForExternEvent { selected_task, renderer_selection_state };
			println!("Emitting task selected for extern event: {:?}", event);
			events.write(event);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Setup a test app with basic resources for testing the event system
	fn setup_event_test_app() -> bevy::prelude::App {
		let mut app = crate::systems::task::cursor_interaction::clicks::test_utils::setup_cursor_interaction_test_app();
		app.add_event::<TaskSelectedForExternEvent>();
		app.add_event::<TouchInput>();
		app.insert_resource(TouchDurationTracker::new(500)); // 500ms hold duration

		app
	}

	/// Helper function to spawn a test task at the origin
	fn spawn_test_task(
		app: &mut bevy::prelude::App,
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
	) -> Result<(), Box<dyn std::error::Error>> {
		crate::systems::task::cursor_interaction::clicks::test_utils::spawn_test_task(
			app, task_id, position, size, title,
		)?;
		Ok(())
	}

	/// Helper function to simulate a mouse click at world position (0,0)
	fn simulate_mouse_click_at_origin(
		mut windows: Query<(Entity, &mut Window)>,
		mut mouse_events: EventWriter<MouseButtonInput>,
		cameras: Query<(&Camera, &GlobalTransform)>,
	) {
		let (window_entity, mut window) = windows.single_mut().unwrap();
		let (camera, camera_transform) = cameras.single().unwrap();

		// Click at world position (0,0,0)
		let world_pos = Vec3::new(0.0, 0.0, 0.0);

		// Convert world coordinates to screen coordinates
		let screen_pos = camera.world_to_viewport(camera_transform, world_pos).unwrap();

		window.set_cursor_position(Some(screen_pos));
		mouse_events.write(MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: window_entity,
		});
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
		let event_system = TaskSelectedForExternEventSystem {
			input_triggers: vec![InputTrigger::AnyClick],
			emit_events: true,
			pixels_per_x_unit: 10.0,
			pixels_per_y_unit: 75.0,
		};

		// Spawn a test task at origin
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(0.0, 0.0, 0.0), // Center of world
			Vec2::new(20.0, 20.0),    // Reasonable size
			"Test Task".to_string(),
		)?;

		// Build and run the system
		let system_fn = event_system.build();
		app.add_systems(bevy::prelude::Update, system_fn);

		// Simulate a click at the origin where the task is located
		app.add_systems(bevy::prelude::Update, simulate_mouse_click_at_origin);
		app.update();

		// Check that events were emitted
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectedForExternEvent>>();
		let event_count = events.drain().count();
		assert!(event_count > 0, "Should emit events when task is clicked");

		Ok(())
	}

	#[test]
	fn test_event_system_disabled() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_event_test_app();
		let event_system = TaskSelectedForExternEventSystem {
			input_triggers: vec![InputTrigger::AnyClick],
			emit_events: false,
			pixels_per_x_unit: 10.0,
			pixels_per_y_unit: 75.0,
		};

		// Spawn a test task at origin
		spawn_test_task(
			&mut app,
			TaskId::from(1),
			Vec3::new(0.0, 0.0, 0.0), // Center of world
			Vec2::new(20.0, 20.0),    // Reasonable size
			"Test Task".to_string(),
		)?;

		// Build and run the system
		let system_fn = event_system.build();
		app.add_systems(bevy::prelude::Update, system_fn);

		// Simulate a click at the origin where the task is located
		app.add_systems(bevy::prelude::Update, simulate_mouse_click_at_origin);
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
		let matcher = InputMatcher::new(5.0, 75.0);
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
		let matcher = InputMatcher::new(5.0, 75.0);
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
		let matcher = InputMatcher::new(5.0, 75.0);
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
		let matcher = InputMatcher::new(5.0, 75.0);
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
		let matcher = InputMatcher::new(5.0, 75.0);
		let triggers = vec![InputTrigger::PressAndHold];

		// Mock mouse button input for left click (press and hold)
		let mouse_input = MouseButtonInput {
			button: MouseButton::Left,
			state: bevy::input::ButtonState::Pressed,
			window: bevy::prelude::Entity::from_raw(0),
		};

		let keyboard_input = ButtonInput::<KeyCode>::default();

		assert!(!matcher.matches_mouse_input(
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
		let matcher = InputMatcher::new(5.0, 75.0);
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
