use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use bevy::ui::{GridTrack, Node, Val};

pub mod status;
pub use status::{StatusBundlable, StatusBundle};
pub mod title;
use std::marker::PhantomData;
pub use title::TitleBundle;

pub type ContentBundle<T> =
	(Node, SpawnRelatedBundle<ChildOf, (Spawn<TitleBundle>, Spawn<StatusBundle<T>>)>);

pub struct ContentPreBundle<T: StatusBundlable>(ContentBundle<T>);

impl<T> ContentPreBundle<T>
where
	T: StatusBundlable,
{
	pub fn bundle(self) -> ContentBundle<T> {
		self.0
	}
}

pub struct ContentBundler<T: StatusBundlable> {
	pub title: String,
	pub phantom: PhantomData<T>,
}

impl<T> ContentBundler<T>
where
	T: StatusBundlable,
{
	pub fn new(title: String) -> Self {
		Self { title, phantom: PhantomData }
	}

	pub fn pre_bundle(self) -> ContentPreBundle<T> {
		let title_bundle = title::TitleBundler::new(self.title).pre_bundle().bundle();
		let status_bundle = status::StatusBundler::new(1, 1).pre_bundle().bundle();

		ContentPreBundle((
			Node {
				width: Val::Percent(100.0),  // Take full width of parent
				height: Val::Percent(100.0), // Take full height of parent
				display: Display::Grid,
				grid_template_columns: vec![GridTrack::fr(2.0), GridTrack::fr(1.0)], // 2fr 1fr grid
				grid_template_rows: vec![GridTrack::fr(1.0)],                        // Single row
				align_content: AlignContent::Center,
				justify_content: JustifyContent::Center,
				justify_self: JustifySelf::Center,
				padding: UiRect::all(Val::Px(8.0)), // 8px padding inside the content area
				..default()
			},
			children![title_bundle, status_bundle],
		))
	}
}
