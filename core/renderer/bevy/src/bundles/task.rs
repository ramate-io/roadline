use crate::components::{RenderState, Task};
use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Bundle for spawning a complete task entity with all its visual components
pub struct TaskBundle {
	pub task_id: TaskId,
	pub position: Vec3,
	pub size: Vec2,
	pub title: String,
	pub font_size: f32,
}

impl TaskBundle {
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2, title: String) -> Self {
		Self { task_id, position, size, title, font_size: 6.0 }
	}

	pub fn with_font_size(mut self, font_size: f32) -> Self {
		self.font_size = font_size;
		self
	}

	/// Spawns all entities needed for a task: border, background, and text
	pub fn spawn(self, commands: &mut Commands) {
		let border_size = Vec2::new(self.size.x + 2.0, self.size.y + 2.0);

		// Spawn the background with border (black border)
		commands.spawn((
			Sprite { color: Color::BLACK, custom_size: Some(border_size), ..default() },
			Transform::from_xyz(self.position.x, self.position.y, 1.0),
			Visibility::Visible,
		));

		// Spawn the inner background (white)
		commands.spawn((
			Sprite {
				color: Color::srgb(0.96, 0.96, 0.96),
				custom_size: Some(self.size),
				..default()
			},
			Transform::from_xyz(self.position.x, self.position.y, 1.1), // Slightly higher z to appear on top of border
			Visibility::Visible,
			Task::new(self.task_id),
			RenderState::new(),
		));

		// Spawn the text within the sprite bounds
		commands.spawn((
			Text2d::new(self.title),
			TextFont { font_size: self.font_size, ..default() },
			TextColor(Color::BLACK),
			Transform::from_xyz(self.position.x, self.position.y, 2.0), // Higher z-index to appear on top
			Visibility::Visible,
		));
	}
}
