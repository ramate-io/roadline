use bevy::prelude::*;
use roadline_util::dependency::Id as DependencyId;
use roadline_util::task::Id as TaskId;
use std::collections::HashMap;

use crate::components::SelectionState;

/// Resource to track selection states for tasks and dependencies
#[derive(Resource, Default)]
pub struct SelectionResource {
	pub tasks: HashMap<TaskId, SelectionState>,
	pub dependencies: HashMap<DependencyId, SelectionState>,
}

impl SelectionResource {
	pub fn new() -> Self {
		Self { tasks: HashMap::new(), dependencies: HashMap::new() }
	}

	pub fn get_task_state(&self, task_id: &TaskId) -> SelectionState {
		self.tasks.get(task_id).copied().unwrap_or(SelectionState::Unselected)
	}

	pub fn set_task_state(&mut self, task_id: TaskId, state: SelectionState) {
		self.tasks.insert(task_id, state);
	}

	pub fn get_dependency_state(&self, dependency_id: &DependencyId) -> SelectionState {
		self.dependencies
			.get(dependency_id)
			.copied()
			.unwrap_or(SelectionState::Unselected)
	}

	pub fn set_dependency_state(&mut self, dependency_id: DependencyId, state: SelectionState) {
		self.dependencies.insert(dependency_id, state);
	}

	pub fn clear_all(&mut self) {
		self.tasks.clear();
		self.dependencies.clear();
	}
}
