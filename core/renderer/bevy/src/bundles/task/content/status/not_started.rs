use super::{StatusBundle, StatusPreBundle};
use bevy::prelude::*;
use bevy::ui::{Node, Val};

pub type NotStartedStatusBundle = (Node, BackgroundColor, Sprite);

pub struct NotStartedStatusPreBundle(NotStartedStatusBundle);

impl NotStartedStatusPreBundle {
	pub fn bundle(self) -> NotStartedStatusBundle {
		self.0
	}
}

pub struct NotStartedStatus {
	pub total: u32,
}

impl NotStartedStatus {
	pub fn new(total: u32) -> Self {
		Self { total }
	}

	pub fn pre_bundle(self) -> NotStartedStatusPreBundle {
		let color = Color::srgb(0.0, 0.0, 1.0); // Blue for not started

		NotStartedStatusPreBundle((
			Node {
				display: Display::Flex,
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				align_self: AlignSelf::Center,
				width: Val::Px(24.0),  // Fixed width for status indicator
				height: Val::Px(24.0), // Fixed height for status indicator
				..default()
			},
			BackgroundColor(color),
			Sprite { color: color, custom_size: Some(Vec2::new(24.0, 24.0)), ..default() },
		))
	}
}
