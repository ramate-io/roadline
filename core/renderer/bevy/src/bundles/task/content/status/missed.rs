use super::StatusBundlable;

use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct MissedStatusMarker;

#[derive(Bundle)]
pub struct MissedStatusBundle {
	pub marker: MissedStatusMarker,
	pub node: Node,
	pub background_color: BackgroundColor,
	pub text: Text,
}

impl StatusBundlable for MissedStatusBundle {
	fn new_status_bundle(completed: u32, total: u32) -> Self {
		Self {
			marker: MissedStatusMarker,
			node: Node::default(),
			background_color: BackgroundColor::default(),
			text: Text::new("Missed"),
		}
	}
}

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

		MissedStatusPreBundle(MissedStatusBundle {
			marker: MissedStatusMarker,
			node: Node {
				display: Display::Flex,
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				align_self: AlignSelf::Center,
				width: Val::Px(24.0),  // Fixed width for status indicator
				height: Val::Px(24.0), // Fixed height for status indicator
				..default()
			},
			background_color: BackgroundColor(color),
			text: Text::new("Hello"),
		})
	}
}
