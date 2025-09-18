use bevy::prelude::*;
use bevy::ui::{Node, Val};

pub type TitleBundle = (Node, Text, TextColor, Children);

pub struct TitlePreBundle(TitleBundle);

impl TitlePreBundle {
	pub fn bundle(self) -> TitleBundle {
		self.0
	}
}

pub struct TitleBundler {
	pub title: String,
}

impl TitleBundler {
	pub fn new(title: String) -> Self {
		Self { title }
	}

	pub fn pre_bundle(self) -> TitlePreBundle {
		TitlePreBundle((
			Node {
				width: Val::Percent(100.0),  // Take full width of grid cell
				height: Val::Percent(100.0), // Take full height of grid cell
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Start, // Left-align the text
				padding: UiRect::all(Val::Px(4.0)),     // Small padding around text
				..default()
			},
			Text::new(self.title),
			TextColor(Color::BLACK),
			Children::default(),
		))
	}
}
