use bevy::prelude::*;
use bevy::ui::Node;

#[derive(Component)]
pub struct TitleMarker;

pub struct TitleSpawnerData {
	pub title: String,
}

pub struct TitleSpawner {
	pub data: TitleSpawnerData,
}

impl TitleSpawner {
	pub fn new(title: String) -> Self {
		Self { data: TitleSpawnerData { title } }
	}

	pub fn spawn(self, commands: &mut Commands, parent: Entity) {
		let title_entity = commands
			.spawn((
				TitleMarker,
				Node {
					display: Display::Flex,
					align_items: AlignItems::Center,
					align_content: AlignContent::Center,
					justify_content: JustifyContent::Start, // Left-align the text
					align_self: AlignSelf::Center,
					..default()
				},
				Text::new(self.data.title),
				TextColor(Color::BLACK),
				TextFont { font_size: 8.0, ..Default::default() },
			))
			.id();

		// Attach title to parent
		commands.entity(parent).add_child(title_entity);
	}
}
