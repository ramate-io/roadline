use super::StatusBundlable;

use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct InProgressStatusMarker;

#[derive(Bundle)]
pub struct InProgressStatusBundle {
	pub marker: InProgressStatusMarker,
	pub node: Node,
	pub background_color: BackgroundColor,
	pub text: Text,
}

impl StatusBundlable for InProgressStatusBundle {
	fn new_status_bundle(completed: u32, total: u32) -> Self {
		Self {
			marker: InProgressStatusMarker,
			node: Node::default(),
			background_color: BackgroundColor::default(),
			text: Text::new("In Progress"),
		}
	}
}

pub struct InProgressStatusPreBundle(InProgressStatusBundle);

impl InProgressStatusPreBundle {
	pub fn bundle(self) -> InProgressStatusBundle {
		self.0
	}
}

pub struct InProgressStatusBundler {
	pub completed: u32,
	pub total: u32,
}

impl InProgressStatusBundler {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn pre_bundle(self) -> InProgressStatusPreBundle {
		let color = Color::srgb(1.0, 1.0, 0.0); // Yellow for in progress

		InProgressStatusPreBundle(InProgressStatusBundle {
			marker: InProgressStatusMarker,
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
