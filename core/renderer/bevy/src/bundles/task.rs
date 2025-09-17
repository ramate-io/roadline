use crate::components::{RenderState, Task};
use bevy::prelude::*;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
use roadline_util::task::Id as TaskId;

/// Bundle for the main task entity with built-in border
#[derive(Bundle)]
pub struct TaskBundle {
	pub node: Node,
	pub background_color: BackgroundColor,
	pub border_color: BorderColor,
	pub border_radius: BorderRadius,
	pub transform: Transform,
	pub visibility: Visibility,
	pub task: Task,
	pub render_state: RenderState,
}

impl TaskBundle {
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2) -> Self {
		Self {
			node: Node {
				width: Val::Px(size.x),
				height: Val::Px(size.y),
				margin: UiRect {
					left: Val::Px(position.x - size.x / 2.0),
					top: Val::Px(position.y - size.y / 2.0),
					right: Val::Px(0.0),
					bottom: Val::Px(0.0),
				},
				border: UiRect::all(Val::Px(2.0)), // 2px border on all sides
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			background_color: BackgroundColor(Color::srgb(0.96, 0.96, 0.96)),
			border_color: BorderColor(Color::BLACK),
			border_radius: BorderRadius::all(Val::Px(4.0)), // Rounded corners with 4px radius
			transform: Transform::from_xyz(position.x, position.y, 1.0),
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
	pub transform: Transform,
	pub visibility: Visibility,
}

impl TaskTextBundle {
	pub fn new(title: String, position: Vec3, font_size: f32) -> Self {
		Self {
			text: Text::new(title),
			text_layout: TextLayout::default(),
			node: Node {
				width: Val::Px(200.0),            // Reasonable width for text
				height: Val::Px(font_size * 1.5), // Height based on font size
				margin: UiRect {
					left: Val::Px(position.x),
					top: Val::Px(position.y),
					right: Val::Px(0.0),
					bottom: Val::Px(0.0),
				},
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
	pub fn spawn(self, commands: &mut Commands, root_ui: Entity) {
		// Spawn the main task node with built-in border
		let task_entity = commands.spawn(TaskBundle::new(self.task_id, self.position, self.size)).id();
		
		// Spawn the text
		let text_entity = commands.spawn(TaskTextBundle::new(self.title, self.position, self.font_size)).id();
		
		// Add both entities as children of the root UI node
		commands.entity(root_ui).add_child(task_entity);
		commands.entity(root_ui).add_child(text_entity);
	}
}
