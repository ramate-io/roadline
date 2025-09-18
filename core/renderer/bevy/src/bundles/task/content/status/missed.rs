use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct MissedStatusMarker;

pub type MissedStatusBundle = (MissedStatusMarker, Node, BackgroundColor, Sprite);

pub struct MissedStatusPreBundle(MissedStatusBundle);

impl MissedStatusPreBundle {
	pub fn bundle(self) -> MissedStatusBundle {
		self.0
	}
}

pub struct MissedStatusBundler {
	pub completed: u32,
	pub total: u32,
}

impl MissedStatusBundler {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn pre_bundle(self) -> MissedStatusPreBundle {
		let color = Color::srgb(1.0, 0.0, 0.0); // Red for missed

		MissedStatusPreBundle((
			MissedStatusMarker,
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
