use bevy::prelude::*;
use bevy::ui::{GridTrack, Node, Val};

pub mod status;
pub use status::StatusSpawner;
pub mod title;
pub use title::TitleSpawner;

pub struct ContentSpawner {
	pub title: String,
	pub completed: u32,
	pub total: u32,
}

impl ContentSpawner {
	pub fn new(title: String, completed: u32, total: u32) -> Self {
		Self { title, completed, total }
	}

	pub fn spawn(self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>, parent: Entity) {
		let content_entity = commands
			.spawn(Node {
				width: Val::Percent(100.0),  // Take full width of parent
				height: Val::Percent(100.0), // Take full height of parent
				display: Display::Grid,
				grid_template_columns: vec![GridTrack::fr(2.0), GridTrack::fr(1.0)], // 2fr 1fr grid
				grid_template_rows: vec![GridTrack::fr(1.0)],                        // Single row
				align_content: AlignContent::Center,
				justify_content: JustifyContent::Center,
				justify_self: JustifySelf::Center,
				padding: UiRect::all(Val::Px(8.0)), // 8px padding inside the content area
				..default()
			})
			.id();

		// Spawn title
		TitleSpawner::new(self.title).spawn(commands, content_entity);

		// Spawn status
		StatusSpawner::new(self.completed, self.total).spawn(commands, meshes, materials, content_entity);

		// Attach content to parent
		commands.entity(parent).add_child(content_entity);
	}
}
