use super::StatusMarker;
use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct MissedStatusMarker;

pub struct MissedStatusSpawner {
	pub completed: u32,
	pub total: u32,
}

impl MissedStatusSpawner {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn spawn(self, commands: &mut Commands, parent: Entity) {
		let status_entity = commands
			.spawn((
				StatusMarker,
				MissedStatusMarker,
				Node {
					display: Display::Flex,
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					align_self: AlignSelf::Center,
					width: Val::Px(32.0),
					height: Val::Px(32.0),
					..default()
				},
				BorderRadius::all(Val::Px(16.0)),
				BackgroundColor(Color::srgb(1.0, 0.0, 0.0)), // Red for missed
				Text::new(format!("{}/{}", self.completed, self.total)),
				TextColor(Color::WHITE),
				TextFont { font_size: 8.0, ..Default::default() },
			))
			.id();

		// Attach status to parent
		commands.entity(parent).add_child(status_entity);
	}
}
