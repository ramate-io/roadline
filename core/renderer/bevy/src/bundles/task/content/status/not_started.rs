use super::StatusBundlable;

use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct NotStartedStatusMarker;

#[derive(Bundle)]
pub struct NotStartedStatusBundle {
	pub marker: NotStartedStatusMarker,
	pub node: Node,
	pub background_color: BackgroundColor,
	pub text: Text,
}

impl StatusBundlable for NotStartedStatusBundle {
	fn new_status_bundle(completed: u32, total: u32) -> Self {
		Self {
			marker: NotStartedStatusMarker,
			node: Node::default(),
			background_color: BackgroundColor::default(),
			text: Text::new("Not Started"),
		}
	}
}

pub struct NotStartedStatusPreBundle(NotStartedStatusBundle);

impl NotStartedStatusPreBundle {
	pub fn bundle(self) -> NotStartedStatusBundle {
		self.0
	}
}

pub struct NotStartedStatusBundler {
	pub total: u32,
}

impl NotStartedStatusBundler {
	pub fn new(total: u32) -> Self {
		Self { total }
	}

	pub fn pre_bundle(self) -> NotStartedStatusPreBundle {
		let color = Color::srgb(0.0, 0.0, 1.0); // Blue for not started

		NotStartedStatusPreBundle(NotStartedStatusBundle {
			marker: NotStartedStatusMarker,
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
