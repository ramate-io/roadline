use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct NotStartedStatusMarker;

pub struct NotStartedStatusSpawner {
	pub total: u32,
}

impl NotStartedStatusSpawner {
	pub fn new(total: u32) -> Self {
		Self { total }
	}

	pub fn spawn(self, commands: &mut Commands, parent: Entity) {
		let status_entity = commands
			.spawn((
				NotStartedStatusMarker,
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
				BackgroundColor(Color::srgb(0.3, 0.5, 0.9)), // Nice blue
				Text::new(format!("0/{}", self.total)),
				TextColor(Color::WHITE),
				TextFont { font_size: 8.0, ..Default::default() },
			))
			.id();

		// Attach status to parent
		commands.entity(parent).add_child(status_entity);
	}
}
