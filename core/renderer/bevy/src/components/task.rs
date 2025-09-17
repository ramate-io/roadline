use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Component that marks an entity as a task
#[derive(Component, Debug)]
pub struct Task {
	pub task_id: TaskId,
}

impl Task {
	pub fn new(task_id: TaskId) -> Self {
		Self { task_id }
	}
}
