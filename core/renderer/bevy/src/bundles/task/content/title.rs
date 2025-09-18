use bevy::prelude::*;
use bevy::ui::Node;

pub type TitleBundle = (Node, BackgroundColor, Text, TextColor, TextFont, Children);

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
				display: Display::Flex,
				align_items: AlignItems::Center,
				align_content: AlignContent::Center,
				justify_content: JustifyContent::Start, // Left-align the text
				align_self: AlignSelf::Center,
				..default()
			},
			BackgroundColor(Color::WHITE),
			Text::new(self.title),
			TextColor(Color::BLACK),
			TextFont { font_size: 8.0, ..Default::default() },
			Children::default(),
		))
	}
}
