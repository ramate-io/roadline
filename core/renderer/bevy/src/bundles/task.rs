pub mod content;
pub use content::ContentBundle;

use crate::components::{RenderState, Task};
use bevy::ecs::spawn::SpawnOneRelated;
use bevy::prelude::*;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiNode, AnchoredUiNodes};
use roadline_util::task::Id as TaskId;

pub type TaskBundle = (
	Task,
	RenderState,
	bevy::prelude::Transform,
	bevy::prelude::Visibility,
	SpawnOneRelated<
		AnchorUiNode,
		(
			AnchorUiConfig,
			bevy::prelude::Node,
			BackgroundColor,
			BorderColor,
			BorderRadius,
			SpawnOneRelated<ChildOf, ContentBundle>,
		),
	>,
);

pub struct TaskPreBundle(TaskBundle);

impl TaskPreBundle {
	pub fn bundle(self) -> TaskBundle {
		self.0
	}
}

/// Helper struct for spawning all task entities
pub struct TaskBundler {
	pub task_id: TaskId,
	pub position: Vec3,
	pub size: Vec2,
	pub title: String,
	pub font_size: f32,
}

impl TaskBundler {
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2, title: String) -> Self {
		Self { task_id, position, size, title, font_size: 6.0 }
	}

	pub fn with_font_size(mut self, font_size: f32) -> Self {
		self.font_size = font_size;
		self
	}

	pub fn pre_bundle(self) -> TaskPreBundle {
		let content_bundle = content::ContentBundler::new(self.title).pre_bundle().bundle();

		TaskPreBundle((
			Task::new(self.task_id),
			RenderState::new(),
			Transform::from_translation(self.position),
			Visibility::Visible,
			AnchoredUiNodes::spawn_one((
				AnchorUiConfig {
					anchorpoint: AnchorPoint::middle(),
					offset: None, // No offset since we're using the entity's transform
				},
				Node {
					width: Val::Px(self.size.x),
					height: Val::Px(self.size.y),
					border: UiRect::all(Val::Px(1.0)), // 2px border on all sides
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				BackgroundColor(Color::WHITE),
				BorderColor(Color::BLACK),
				BorderRadius::all(Val::Px(4.0)), // Rounded corners with 4px radius
				Children::spawn_one(content_bundle),
			)),
		))
	}

	pub fn spawn(self, commands: &mut Commands) {

		/*let anchor_ui_node = Anch

		let task_bundle = commands
			.spawn((
				Task::new(self.task_id),
				RenderState::new(),
				Transform::from_translation(self.position),
				Visibility::Visible,
			))
			.id();*/
	}
}
