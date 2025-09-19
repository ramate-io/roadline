pub mod content;
pub use content::{
	status::{
		CompletedStatusBundle, InProgressStatusBundle, MissedStatusBundle, NotStartedStatusBundle,
		StatusBundlable,
	},
	ContentBundle,
};

use crate::components::{RenderState, Task};
use bevy::ecs::spawn::SpawnOneRelated;
use bevy::prelude::*;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiNode, AnchoredUiNodes};
use roadline_util::task::Id as TaskId;
use std::marker::PhantomData;

pub type TaskBundle<T> = (
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
			SpawnOneRelated<ChildOf, ContentBundle<T>>,
		),
	>,
);

pub struct TaskPreBundle<T: StatusBundlable>(TaskBundle<T>);

impl<T> TaskPreBundle<T>
where
	T: StatusBundlable,
{
	pub fn bundle(self) -> TaskBundle<T> {
		self.0
	}
}

pub struct TaskBundlerData {
	pub task_id: TaskId,
	pub position: Vec3,
	pub size: Vec2,
	pub title: String,
	pub font_size: f32,
	pub completed: u32,
	pub total: u32,
}

impl TaskBundlerData {
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2, title: String) -> Self {
		Self { task_id, position, size, title, font_size: 6.0, completed: 3, total: 3 }
	}

	pub fn spawn(self, commands: &mut Commands) {
		// match the completions to the status bundlable
		if self.completed == self.total {
			let bundler = TaskBundler::<CompletedStatusBundle>::from(self);
			let bundle = bundler.pre_bundle().bundle();
			commands.spawn(bundle);
		} else if self.completed == 0 {
			let bundler = TaskBundler::<NotStartedStatusBundle>::from(self);
			let bundle = bundler.pre_bundle().bundle();
			commands.spawn(bundle);
		} else {
			let bundler = TaskBundler::<InProgressStatusBundle>::from(self);
			let bundle = bundler.pre_bundle().bundle();
			commands.spawn(bundle);
		}
	}
}

/// Helper struct for spawning all task entities
pub struct TaskBundler<T: StatusBundlable> {
	pub data: TaskBundlerData,
	pub phantom: PhantomData<T>,
}

impl<T> From<TaskBundlerData> for TaskBundler<T>
where
	T: StatusBundlable,
{
	fn from(data: TaskBundlerData) -> Self {
		Self { data, phantom: PhantomData }
	}
}

impl<T> TaskBundler<T>
where
	T: StatusBundlable,
{
	pub fn new(task_id: TaskId, position: Vec3, size: Vec2, title: String) -> Self {
		Self {
			data: TaskBundlerData {
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

	pub fn pre_bundle(self) -> TaskPreBundle<T> {
		let content_bundle = content::ContentBundler::new(self.data.title).pre_bundle().bundle();

		TaskPreBundle((
			Task::new(self.data.task_id),
			RenderState::new(),
			Transform::from_translation(self.data.position),
			Visibility::Visible,
			AnchoredUiNodes::spawn_one((
				AnchorUiConfig {
					anchorpoint: AnchorPoint::middle(),
					offset: None, // No offset since we're using the entity's transform
				},
				Node {
					width: Val::Px(self.data.size.x),
					height: Val::Px(self.data.size.y),
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
}
