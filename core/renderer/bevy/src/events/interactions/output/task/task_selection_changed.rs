use crate::components::SelectionState;
use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Event emitted when a task selection changes
#[derive(Event, Debug, Clone)]
pub struct TaskSelectionChangedEvent {
	pub selected_task: TaskId,
	pub previous_selection_state: SelectionState,
	pub new_selection_state: SelectionState,
}
