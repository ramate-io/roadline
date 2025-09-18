use super::{StatusBundle, StatusPreBundle};
use bevy::prelude::*;
use bevy::ui::{Node, Val};

pub type CompletedStatusBundle = (Node, BackgroundColor, Sprite);

pub struct CompletedStatusPreBundle(CompletedStatusBundle);

impl CompletedStatusPreBundle {
	pub fn bundle(self) -> CompletedStatusBundle {
		self.0
	}
}

pub struct CompletedStatus {
	pub completed: u32,
	pub total: u32,
}

impl CompletedStatus {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn pre_bundle(self) -> CompletedStatusPreBundle {
		let color = Color::srgb(0.0, 1.0, 0.0); // Green for completed

		CompletedStatusPreBundle((
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
