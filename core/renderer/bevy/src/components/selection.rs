use bevy::prelude::*;

/// Represents the selection state of a task or dependency
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionState {
	/// Not selected
	Unselected,
	/// Directly selected by user click
	Selected,
	/// Selected as a descendant of a directly selected item
	Descendant,
	/// Selected as a parent of a directly selected item
	Parent,
}

impl Default for SelectionState {
	fn default() -> Self {
		Self::Unselected
	}
}

/// Component to track selection state for tasks
#[derive(Component, Debug)]
pub struct TaskSelection {
	pub state: SelectionState,
}

impl Default for TaskSelection {
	fn default() -> Self {
		Self { state: SelectionState::Unselected }
	}
}

/// Component to track selection state for dependencies
#[derive(Component, Debug)]
pub struct DependencySelection {
	pub state: SelectionState,
}

impl Default for DependencySelection {
	fn default() -> Self {
		Self { state: SelectionState::Unselected }
	}
}
