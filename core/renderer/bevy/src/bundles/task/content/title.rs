use bevy::prelude::*;
use bevy::ui::Node;

#[derive(Component)]
pub struct TitleMarker;

#[derive(Bundle)]
pub struct TitleBundle {
	pub marker: TitleMarker,
	pub node: Node,
	pub text: Text,
	pub text_color: TextColor,
	pub text_font: TextFont,
	pub children: Children,
}

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
		TitlePreBundle(TitleBundle {
			marker: TitleMarker,
			node: Node {
				display: Display::Flex,
				align_items: AlignItems::Center,
				align_content: AlignContent::Center,
				justify_content: JustifyContent::Start, // Left-align the text
				align_self: AlignSelf::Center,
				..default()
			},
			text: Text::new(self.title),
			text_color: TextColor(Color::BLACK),
			text_font: TextFont { font_size: 6.0, ..Default::default() },
			children: Children::default(),
		})
	}
}
