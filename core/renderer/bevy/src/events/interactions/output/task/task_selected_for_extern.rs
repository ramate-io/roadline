use crate::components::SelectionState;
use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Event emitted when a task selection changes
#[derive(Event, Debug, Clone)]
pub struct TaskSelectedForExternEvent {
	pub selected_task: TaskId,
	pub renderer_selection_state: SelectionState,
}
