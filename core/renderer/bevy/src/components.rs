use bevy::prelude::*;
use roadline_util::dependency::Id as DependencyId;
use roadline_util::task::Id as TaskId;
pub mod dependency;
pub mod task;

/// Component that marks an entity as a milestone node
#[derive(Component, Debug)]
pub struct MilestoneNode {
	pub task_id: TaskId,
}

impl MilestoneNode {
	pub fn new(task_id: TaskId) -> Self {
		Self { task_id }
	}
}

/// Component that marks an entity as an edge/connection between tasks
#[derive(Component, Debug)]
pub struct TaskEdge {
	pub dependency_id: DependencyId,
}

impl TaskEdge {
	pub fn new(dependency_id: DependencyId) -> Self {
		Self { dependency_id }
	}
}

/// Component for tracking render state
#[derive(Component, Debug)]
pub struct RenderState {
	pub needs_update: bool,
}

impl RenderState {
	pub fn new() -> Self {
		Self { needs_update: true }
	}

	pub fn mark_dirty(&mut self) {
		self.needs_update = true;
	}

	pub fn mark_clean(&mut self) {
		self.needs_update = false;
	}
}

impl Default for RenderState {
	fn default() -> Self {
		Self::new()
	}
}
