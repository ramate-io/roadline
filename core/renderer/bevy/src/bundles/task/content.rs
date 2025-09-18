use bevy::ecs::spawn::SpawnOneRelated;
use bevy::prelude::*;
use bevy::ui::{GridTrack, Node, Val};

pub mod title;

pub type ContentBundle = (Node, BackgroundColor, SpawnOneRelated<ChildOf, title::TitleBundle>);

pub struct ContentPreBundle(ContentBundle);

impl ContentPreBundle {
	pub fn bundle(self) -> ContentBundle {
		self.0
	}
}

pub struct ContentBundler {
	pub title: String,
}

impl ContentBundler {
	pub fn new(title: String) -> Self {
		Self { title }
	}

	pub fn pre_bundle(self) -> ContentPreBundle {
		ContentPreBundle((
			Node {
				width: Val::Percent(100.0),  // Take full width of parent
				height: Val::Percent(100.0), // Take full height of parent
				display: Display::Grid,
				grid_template_columns: vec![GridTrack::fr(2.0), GridTrack::fr(1.0)], // 2fr 1fr grid
				grid_template_rows: vec![GridTrack::fr(1.0)],                        // Single row
				align_content: AlignContent::Center,
				justify_content: JustifyContent::Center,
				padding: UiRect::all(Val::Px(8.0)), // 8px padding inside the content area
				..default()
			},
			BackgroundColor(Color::srgb(0.96, 0.96, 0.96)), // blue background
			Children::spawn_one(title::TitleBundler::new(self.title).pre_bundle().bundle()),
		))
	}
}
