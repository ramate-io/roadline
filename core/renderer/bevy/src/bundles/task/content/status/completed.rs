use super::StatusBundlable;
use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct CompletedStatusMarker;

#[derive(Bundle)]
pub struct CompletedStatusBundle {
	pub marker: CompletedStatusMarker,
	pub node: Node,
	pub text: Text,
	pub text_color: TextColor,
	pub text_font: TextFont,
	pub border_radius: BorderRadius,
}

impl StatusBundlable for CompletedStatusBundle {
	fn new_status_bundle(completed: u32, total: u32) -> Self {
		Self {
			marker: CompletedStatusMarker,
			node: Node {
				display: Display::Flex,
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				justify_self: JustifySelf::End,
				align_self: AlignSelf::Center,
				..default()
			},
			border_radius: BorderRadius::all(Val::Px(16.0)),
			text: Text::new(format!("{}/{}", completed, total)), // Filled circle + fraction
			text_font: TextFont { font_size: 8.0, ..Default::default() },
			text_color: TextColor(Color::oklch(0.40, 0.08, 149.0)),
		}
	}
}

pub struct CompletedStatusPreBundle(CompletedStatusBundle);

impl CompletedStatusPreBundle {
	pub fn bundle(self) -> CompletedStatusBundle {
		self.0
	}
}

pub struct CompletedStatusSpawner {
	pub completed: u32,
	pub total: u32,
}

impl CompletedStatusSpawner {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn pre_bundle(self) -> CompletedStatusPreBundle {
		CompletedStatusPreBundle(CompletedStatusBundle::new_status_bundle(
			self.completed,
			self.total,
		))
	}
}
