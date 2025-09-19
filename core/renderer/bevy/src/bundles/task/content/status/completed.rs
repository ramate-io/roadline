use bevy::prelude::*;
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct CompletedStatusMarker;

pub struct CompletedStatusSpawner {
	pub completed: u32,
	pub total: u32,
}

impl CompletedStatusSpawner {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn spawn(self, commands: &mut Commands, parent: Entity) {
		let status_entity = commands
			.spawn((
				CompletedStatusMarker,
				Node {
					display: Display::Flex,
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					justify_self: JustifySelf::End,
					align_self: AlignSelf::Center,
					..default()
				},
				Text::new(format!("{}/{}", self.completed, self.total)),
				TextColor(Color::oklch(0.40, 0.08, 149.0)),
				TextFont { font_size: 8.0, ..Default::default() },
				BorderRadius::all(Val::Px(16.0)),
			))
			.id();

		// Attach status to parent
		commands.entity(parent).add_child(status_entity);
	}
}
