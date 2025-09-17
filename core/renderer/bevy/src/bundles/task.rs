use crate::components::{RenderState, Task};
use bevy::prelude::*;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchoredUiNodes};
use roadline_util::task::Id as TaskId;

/// Spawns a task with anchored UI and text
pub fn spawn_task_with_ui(
	commands: &mut Commands,
	task_id: TaskId,
	position: Vec3,
	size: Vec2,
	title: String,
) -> Entity {
	commands.spawn((
		Task::new(task_id),
		RenderState::new(),
		Transform::from_translation(position),
		Visibility::Visible,
		AnchoredUiNodes::spawn_one((
			AnchorUiConfig {
				anchorpoint: AnchorPoint::middle(),
				offset: None, // No offset since we're using the entity's transform
			},
			Node {
				width: Val::Px(size.x),
				height: Val::Px(size.y),
				border: UiRect::all(Val::Px(2.0)), // 2px border on all sides
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			BackgroundColor(Color::srgb(0.96, 0.96, 0.96)),
			BorderColor(Color::BLACK),
			BorderRadius::all(Val::Px(4.0)), // Rounded corners with 4px radius
			Children::spawn_one(
				Text::new(title),
			),
		)),
	)).id()
}

/// Helper struct for spawning all task entities
pub struct TaskSpawner {
	pub task_id: TaskId,
	pub position: Vec3,
	pub size: Vec2,
	pub title: String,
	pub font_size: f32,
}

impl TaskSpawner {
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2, title: String) -> Self {
		Self { task_id, position, size, title, font_size: 6.0 }
	}

	pub fn with_font_size(mut self, font_size: f32) -> Self {
		self.font_size = font_size;
		self
	}

	/// Spawns all entities needed for a task: main task node and text
	pub fn spawn(self, commands: &mut Commands) {
		// Spawn the main task node with anchored UI and text
		spawn_task_with_ui(commands, self.task_id, self.position, self.size, self.title);
	}
}
