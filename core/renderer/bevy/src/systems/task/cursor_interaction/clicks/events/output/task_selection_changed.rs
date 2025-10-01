use crate::components::{SelectionState, Task};
use crate::events::interactions::TaskSelectionChangedEvent;
use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Wrapper components for testing
#[derive(Component)]
pub struct PreviousSelectionState(pub SelectionState);

#[derive(Component)]
pub struct NewSelectionState(pub SelectionState);

#[derive(Debug, Clone)]
pub enum EventEmission {
	/// Don't emit any events
	None,
	/// Emit events when selection changes
	Changed,
	/// Emit events regardless of whether selection changes
	Always,
}

/// System for handling task selection changed events
#[derive(Debug, Clone, Resource)]
pub struct TaskSelectionChangedEventSystem {
	/// Whether to emit events when selection changes
	pub emit_events: EventEmission,
}

impl Default for TaskSelectionChangedEventSystem {
	fn default() -> Self {
		Self { emit_events: EventEmission::Changed }
	}
}

impl TaskSelectionChangedEventSystem {
	/// Build a system function for emitting task selection changed events
	pub fn build(
		self,
	) -> impl FnMut(
		Query<(Entity, &Transform, &Task, &PreviousSelectionState, &NewSelectionState)>,
		EventWriter<TaskSelectionChangedEvent>,
	) {
		move |task_query: Query<(
			Entity,
			&Transform,
			&Task,
			&PreviousSelectionState,
			&NewSelectionState,
		)>,
		      mut events: EventWriter<TaskSelectionChangedEvent>| {
			// This system can be used for testing by querying task entities
			for (_entity, _transform, task, previous_state, new_state) in task_query.iter() {
				self.emit_task_selection_changed(
					&mut events,
					task.task_id,
					previous_state.0,
					new_state.0,
				);
			}
		}
	}

	/// Emit a task selection changed event
	pub fn emit_task_selection_changed(
		&self,
		events: &mut EventWriter<TaskSelectionChangedEvent>,
		selected_task: TaskId,
		previous_selection_state: SelectionState,
		new_selection_state: SelectionState,
	) {
		match self.emit_events {
			EventEmission::None => {}
			EventEmission::Changed => {
				if previous_selection_state != new_selection_state {
					events.write(TaskSelectionChangedEvent {
						selected_task,
						previous_selection_state,
						new_selection_state,
					});
				}
			}
			EventEmission::Always => {
				events.write(TaskSelectionChangedEvent {
					selected_task,
					previous_selection_state,
					new_selection_state,
				});
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
		app.add_event::<TaskSelectionChangedEvent>();
		app
	}

	#[test]
	fn test_event_system_builds_correctly() {
		let mut app = setup_event_test_app();
		let event_system = TaskSelectionChangedEventSystem::default();

		// The system should build without panicking
		let system_fn = event_system.build();
		app.add_systems(bevy::prelude::Update, system_fn);

		// Run the app to ensure it doesn't panic
		app.update();
	}

	#[test]
	fn test_event_system_with_tasks() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_event_test_app();

		// Spawn a test task with SelectionState components
		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(1),
			bevy::math::Vec3::new(0.0, 0.0, 0.0),
			bevy::math::Vec2::new(100.0, 75.0),
			"Test Task".to_string(),
		);
		app.world_mut().run_system_once(params.build())?;

		// Add SelectionState components to the spawned task for testing
		let world = app.world_mut();
		let mut task_query = world.query::<(Entity, &Task)>();
		let entities: Vec<Entity> = task_query.iter(world).map(|(entity, _)| entity).collect();
		for entity in entities {
			world.entity_mut(entity).insert((
				PreviousSelectionState(SelectionState::Unselected), // previous_state
				NewSelectionState(SelectionState::Selected),        // new_state
			));
		}

		// Create event system with Always emission for testing
		let event_system = TaskSelectionChangedEventSystem { emit_events: EventEmission::Always };

		// Build and run the system
		let system_fn = event_system.build();
		app.add_systems(bevy::prelude::Update, system_fn);
		app.update();

		// Check that events were emitted
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectionChangedEvent>>();
		let event_count = events.drain().count();
		assert!(event_count > 0, "Should emit events when EventEmission::Always is set");

		Ok(())
	}

	#[test]
	fn test_event_system_none_emission() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_event_test_app();

		// Spawn a test task with SelectionState components
		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(1),
			bevy::math::Vec3::new(0.0, 0.0, 0.0),
			bevy::math::Vec2::new(100.0, 75.0),
			"Test Task".to_string(),
		);
		app.world_mut().run_system_once(params.build())?;

		// Add SelectionState components to the spawned task for testing
		let world = app.world_mut();
		let mut task_query = world.query::<(Entity, &Task)>();
		let entities: Vec<Entity> = task_query.iter(world).map(|(entity, _)| entity).collect();
		for entity in entities {
			world.entity_mut(entity).insert((
				PreviousSelectionState(SelectionState::Unselected), // previous_state
				NewSelectionState(SelectionState::Selected),        // new_state
			));
		}

		// Create event system with None emission
		let event_system = TaskSelectionChangedEventSystem { emit_events: EventEmission::None };

		// Build and run the system
		let system_fn = event_system.build();
		app.add_systems(bevy::prelude::Update, system_fn);
		app.update();

		// Check that no events were emitted
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectionChangedEvent>>();
		let event_count = events.drain().count();
		assert_eq!(event_count, 0, "Should not emit events when EventEmission::None is set");

		Ok(())
	}

	#[test]
	fn test_event_system_changed_emission() -> Result<(), Box<dyn std::error::Error>> {
		let mut app = setup_event_test_app();

		// Spawn a test task with different SelectionState components
		let params = TestTasksParams::new().with_basic_task(
			TaskId::from(1),
			bevy::math::Vec3::new(0.0, 0.0, 0.0),
			bevy::math::Vec2::new(100.0, 75.0),
			"Test Task".to_string(),
		);
		app.world_mut().run_system_once(params.build())?;

		// Add SelectionState components with different states (should emit)
		let world = app.world_mut();
		let mut task_query = world.query::<(Entity, &Task)>();
		let entities: Vec<Entity> = task_query.iter(world).map(|(entity, _)| entity).collect();
		for entity in entities {
			world.entity_mut(entity).insert((
				PreviousSelectionState(SelectionState::Unselected), // previous_state
				NewSelectionState(SelectionState::Selected),        // new_state (different!)
			));
		}

		// Create event system with Changed emission
		let event_system = TaskSelectionChangedEventSystem { emit_events: EventEmission::Changed };

		// Build and run the system
		let system_fn = event_system.build();
		app.add_systems(bevy::prelude::Update, system_fn);
		app.update();

		// Check that events were emitted (because states are different)
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectionChangedEvent>>();
		let event_count = events.drain().count();
		assert_eq!(event_count, 1, "Should emit events when states are different");

		// Now test with same states (should not emit)
		let world = app.world_mut();
		let mut task_query = world.query::<(Entity, &Task)>();
		let entities: Vec<Entity> = task_query.iter(world).map(|(entity, _)| entity).collect();
		for entity in entities {
			world.entity_mut(entity).insert((
				PreviousSelectionState(SelectionState::Selected), // previous_state
				NewSelectionState(SelectionState::Selected),      // new_state (same!)
			));
		}

		app.update();

		// Check that no events were emitted (because states are the same)
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectionChangedEvent>>();
		let event_count = events.drain().count();
		assert_eq!(event_count, 0, "Should not emit events when states are the same");

		Ok(())
	}

	#[test]
	fn test_event_system_direct_emission() {
		let mut app = setup_event_test_app();
		let event_system = TaskSelectionChangedEventSystem::default();

		// Test direct emission method by running a system
		let test_system = move |mut events: EventWriter<TaskSelectionChangedEvent>| {
			event_system.emit_task_selection_changed(
				&mut events,
				TaskId::from(1),
				SelectionState::Unselected,
				SelectionState::Selected,
			);
		};

		app.add_systems(bevy::prelude::Update, test_system);
		app.update();

		// Check that event was emitted
		let mut events = app
			.world_mut()
			.resource_mut::<bevy::ecs::event::Events<TaskSelectionChangedEvent>>();
		let event_count = events.drain().count();
		assert_eq!(event_count, 1, "Should emit one event when called directly");
	}
}
