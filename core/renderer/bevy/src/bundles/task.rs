pub mod content;
#[cfg(test)]
pub mod tests;
pub use content::ContentSpawner;

use crate::components::{RenderState, Task};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiNode};
use roadline_util::task::Id as TaskId;

#[derive(Debug, Component)]
pub struct TaskNodeMarker;

#[derive(Debug, Component)]
pub struct TaskHoverable;

#[derive(Debug, Component)]
pub struct TaskSize {
	pub size: Vec2,
}

#[derive(Debug, Clone)]
pub struct TaskSpawnerData {
	pub task_id: TaskId,
	pub position: Vec3,
	pub size: Vec2,
	pub title: String,
	pub font_size: f32,
	pub in_future: bool,
	pub completed: u32,
	pub total: u32,
}

/// Helper struct for spawning all task entities
#[derive(Debug, Clone)]
pub struct TaskSpawner {
	pub data: TaskSpawnerData,
}

impl TaskSpawner {
	pub fn new(
		task_id: TaskId,
		position: Vec3,
		size: Vec2,
		title: String,
		in_future: bool,
		completed: u32,
		total: u32,
	) -> Self {
		Self {
			data: TaskSpawnerData {
				task_id,
				position,
				size,
				title,
				font_size: 6.0,
				in_future,
				completed,
				total,
			},
		}
	}

	pub fn with_font_size(mut self, font_size: f32) -> Self {
		self.data.font_size = font_size;
		self
	}

	pub fn spawn(
		self,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<ColorMaterial>>,
	) {
		let parent_entity = commands
			.spawn((
				TaskNodeMarker,
				TaskHoverable,
				TaskSize { size: self.data.size },
				Node {
					width: Val::Px(self.data.size.x),
					height: Val::Px(self.data.size.y),
					border: UiRect::all(Val::Px(1.5)),
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				BackgroundColor(Color::WHITE),
				BorderColor(Color::BLACK),
				BorderRadius::all(Val::Px(4.0)),
			))
			.id();

		let task_entity = commands
			.spawn((
				Task::new(self.data.task_id).with_ui_entity(parent_entity),
				RenderState::new(),
				Transform::from_translation(self.data.position),
				Visibility::Visible,
				RenderLayers::layer(2),
			))
			.id();

		// Add the anchor relationship
		commands.entity(parent_entity).insert((
			AnchorUiNode::to_entity(task_entity),
			AnchorUiConfig { anchorpoint: AnchorPoint::middle(), offset: None },
		));

		// Spawn content using the new imperative spawner
		ContentSpawner::new(
			self.data.title,
			self.data.in_future,
			self.data.completed,
			self.data.total,
		)
		.spawn(
			commands,
			meshes,
			materials,
			parent_entity,
			self.data.position, // Pass the world position
			self.data.size,     // Pass the task size
		);
	}
}
