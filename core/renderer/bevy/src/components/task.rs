use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Component that marks an entity as a task
#[derive(Component, Debug)]
pub struct Task {
	pub task_id: TaskId,
	pub ui_entity: Option<Entity>,
}

impl Task {
	pub fn new(task_id: TaskId) -> Self {
		Self { task_id, ui_entity: None }
	}

	pub fn with_ui_entity(mut self, ui_entity: Entity) -> Self {
		self.ui_entity = Some(ui_entity);
		self
	}
}
