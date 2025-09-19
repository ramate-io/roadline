use super::StatusBundlable;
use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct CompletedStatusMarker;

#[derive(Bundle)]
pub struct CompletedStatusBundle {
	pub marker: CompletedStatusMarker,
	pub node: Node,
	pub background_color: BackgroundColor,
	pub text: Text,
}

impl StatusBundlable for CompletedStatusBundle {
	fn new_status_bundle(completed: u32, total: u32) -> Self {
		Self {
			marker: CompletedStatusMarker,
			node: Node::default(),
			background_color: BackgroundColor::default(),
			text: Text::new("Completed"),
		}
	}
}

pub struct CompletedStatusPreBundle(CompletedStatusBundle);

impl CompletedStatusPreBundle {
	pub fn bundle(self) -> CompletedStatusBundle {
		self.0
	}
}

pub struct CompletedStatusBundler {
	pub completed: u32,
	pub total: u32,
}

impl CompletedStatusBundler {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn pre_bundle(self) -> CompletedStatusPreBundle {
		let color = Color::srgb(0.0, 1.0, 0.0); // Green for completed

		CompletedStatusPreBundle(CompletedStatusBundle {
			marker: CompletedStatusMarker,
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
