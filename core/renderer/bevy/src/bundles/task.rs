pub mod content;
pub use content::{
	status::{
		CompletedStatusBundle, InProgressStatusBundle, MissedStatusBundle, NotStartedStatusBundle,
		StatusBundlable,
	},
	ContentBundle, ContentSpawner,
};

use crate::components::{RenderState, Task};
use bevy::prelude::*;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiNode, AnchoredUiNodes};
use roadline_util::task::Id as TaskId;
use std::marker::PhantomData;

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
pub struct TaskSpawner<T: StatusBundlable> {
	pub data: TaskSpawnerData,
	pub phantom: PhantomData<T>,
}

impl<T> From<TaskSpawnerData> for TaskSpawner<T>
where
	T: StatusBundlable,
{
	fn from(data: TaskSpawnerData) -> Self {
		Self { data, phantom: PhantomData }
	}
}

impl<T> TaskSpawner<T>
where
	T: StatusBundlable,
{
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
			phantom: PhantomData,
		}
	}

	pub fn with_font_size(mut self, font_size: f32) -> Self {
		self.data.font_size = font_size;
		self
	}

	pub fn spawn(self, commands: &mut Commands) {
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

		let content_bundle = ContentSpawner::<T>::new(self.data.title).pre_bundle().bundle();

		// Spawn child content imperatively
		let content_entity = commands.spawn(content_bundle).id();

		// Attach child to parent
		commands.entity(parent_entity).add_child(content_entity);
	}
}
