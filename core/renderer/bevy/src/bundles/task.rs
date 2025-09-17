use crate::components::{RenderState, Task};
use bevy::prelude::*;
use roadline_util::task::Id as TaskId;

/// Bundle for the task border (black border around the task)
#[derive(Bundle)]
pub struct TaskBorderBundle {
	pub sprite: Sprite,
	pub transform: Transform,
	pub visibility: Visibility,
}

impl TaskBorderBundle {
	pub fn new(position: Vec3, size: Vec2) -> Self {
		let border_size = Vec2::new(size.x + 2.0, size.y + 2.0);
		Self {
			node: Node::default(),
			background_color: BackgroundColor(Color::BLACK),
			border_radius: BorderRadius::all(Val::Px(5.0)), // Slightly larger radius for border
			style: Style {
				position_type: PositionType::Absolute,
				left: Val::Px(position.x - border_size.x / 2.0),
				top: Val::Px(-position.y - border_size.y / 2.0), // Flip Y for UI coordinates
				width: Val::Px(border_size.x),
				height: Val::Px(border_size.y),
				..default()
			},
			transform: Transform::from_xyz(position.x, position.y, 1.0),
			visibility: Visibility::Visible,
		}
	}
}

/// Bundle for the main task entity (background with Task component)
#[derive(Bundle)]
pub struct TaskBackgroundBundle {
	pub node: Node,
	pub background_color: BackgroundColor,
	pub border_radius: BorderRadius,
	pub style: Style,
	pub transform: Transform,
	pub visibility: Visibility,
	pub task: Task,
	pub render_state: RenderState,
}

impl TaskBackgroundBundle {
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2) -> Self {
		Self {
			node: Node::default(),
			background_color: BackgroundColor(Color::srgb(0.96, 0.96, 0.96)),
			border_radius: BorderRadius::all(Val::Px(4.0)), // Rounded corners with 4px radius
			style: Style {
				position_type: PositionType::Absolute,
				left: Val::Px(position.x - size.x / 2.0),
				top: Val::Px(-position.y - size.y / 2.0), // Flip Y for UI coordinates
				width: Val::Px(size.x),
				height: Val::Px(size.y),
				..default()
			},
			transform: Transform::from_xyz(position.x, position.y, 1.1), // Slightly higher z to appear on top of border
			visibility: Visibility::Visible,
			task: Task::new(task_id),
			render_state: RenderState::new(),
		}
	}
}

/// Bundle for the task text
#[derive(Bundle)]
pub struct TaskTextBundle {
	pub text: Text,
	pub text_layout: TextLayout,
	pub node: Node,
	pub style: Style,
	pub transform: Transform,
	pub visibility: Visibility,
}

impl TaskTextBundle {
	pub fn new(title: String, position: Vec3, font_size: f32) -> Self {
		Self {
			text: Text::new(title),
			text_layout: TextLayout::default(),
			node: Node::default(),
			style: Style {
				position_type: PositionType::Absolute,
				left: Val::Px(position.x),
				top: Val::Px(-position.y),        // Flip Y for UI coordinates
				width: Val::Px(200.0),            // Reasonable width for text
				height: Val::Px(font_size * 1.5), // Height based on font size
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..default()
			},
			transform: Transform::from_xyz(position.x, position.y, 2.0), // Higher z-index to appear on top
			visibility: Visibility::Visible,
		}
	}
}

/// Helper struct for spawning all task entities
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
		// Spawn the border
		commands.spawn(TaskBorderBundle::new(self.position, self.size));

		// Spawn the background with Task component
		commands.spawn(TaskBackgroundBundle::new(self.task_id, self.position, self.size));

		// Spawn the text
		commands.spawn(TaskTextBundle::new(self.title, self.position, self.font_size));
	}
}
