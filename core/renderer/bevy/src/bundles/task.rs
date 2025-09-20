pub mod content;
pub use content::ContentSpawner;

use crate::components::{RenderState, Task};
use bevy::prelude::*;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiNode};
use roadline_util::task::Id as TaskId;
pub struct TaskSpawnerData {
	pub task_id: TaskId,
	pub position: Vec3,
	pub size: Vec2,
	pub title: String,
	pub font_size: f32,
	pub completed: u32,
	pub total: u32,
}

/// Helper struct for spawning all task entities
pub struct TaskSpawner {
	pub data: TaskSpawnerData,
}

impl TaskSpawner {
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2, title: String) -> Self {
		Self {
			data: TaskSpawnerData {
				task_id,
				position,
				size,
				title,
				font_size: 6.0,
				completed: 3,
				total: 3,
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
		let task_entity = commands
			.spawn((
				Task::new(self.data.task_id),
				RenderState::new(),
				Transform::from_translation(self.data.position),
				Visibility::Visible,
			))
			.id();

		let parent_entity = commands
			.spawn((
				AnchorUiNode::to_entity(task_entity),
				AnchorUiConfig { anchorpoint: AnchorPoint::middle(), offset: None },
				Node {
					width: Val::Px(self.data.size.x),
					height: Val::Px(self.data.size.y),
					border: UiRect::all(Val::Px(1.0)),
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				BackgroundColor(Color::WHITE),
				BorderColor(Color::BLACK),
				BorderRadius::all(Val::Px(4.0)),
			))
			.id();

		// Spawn content using the new imperative spawner
		ContentSpawner::new(self.data.title, self.data.completed, self.data.total).spawn(
			commands,
			meshes,
			materials,
			parent_entity,
			self.data.position, // Pass the world position
		);
	}
}
