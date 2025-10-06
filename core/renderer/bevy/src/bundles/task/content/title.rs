use bevy::prelude::*;
use bevy::ui::Node;
use roadline_util::task::Id as TaskId;

#[derive(Component)]
pub struct TitleMarker;

pub struct TitleSpawner {
	/// The id of the task.
	pub task_id: TaskId,
	/// The title of the task.
	pub title: String,
	/// The size of of the task.
	pub size: Vec2,
}

impl TitleSpawner {
	pub fn new(task_id: TaskId, title: String, available_width: f32) -> Self {
		Self { task_id, title, size: Vec2::new(available_width, 0.0) }
	}

	pub fn elided_title(&self) -> String {
		let base_title = format!("T{}: {}", self.task_id.value(), self.title);

		// Estimate character width based on font size (12px) and typical character width
		// Assuming roughly 0.6 * font_size for average character width
		const FONT_SIZE: f32 = 12.0;
		const CHAR_WIDTH_RATIO: f32 = 0.6;
		const ELLIPSIS_WIDTH: f32 = 3.0 * FONT_SIZE * CHAR_WIDTH_RATIO; // Width of "..."

		let available_width = self.size.x;
		let char_width = FONT_SIZE * CHAR_WIDTH_RATIO;

		// Calculate how many characters we can fit
		let max_chars = ((available_width - ELLIPSIS_WIDTH) / char_width) as usize;

		if base_title.len() <= max_chars {
			base_title
		} else {
			let truncated_len = max_chars.saturating_sub(3); // Leave room for "..."
			if truncated_len > 0 {
				format!("{}...", &base_title[..truncated_len])
			} else {
				"...".to_string()
			}
		}
	}

	pub fn spawn(self, commands: &mut Commands, parent: Entity) {
		let elided_title = self.elided_title();

		let title_entity = commands
			.spawn((
				TitleMarker,
				Node {
					display: Display::Flex,
					align_items: AlignItems::Center,
					align_content: AlignContent::Center,
					justify_content: JustifyContent::Start, // Left-align the text
					align_self: AlignSelf::Center,
					overflow: Overflow::hidden(),
					..default()
				},
				Text::new(elided_title),
				TextColor(Color::BLACK),
				TextFont { font_size: 12.0, ..Default::default() },
			))
			.id();

		// Attach title to parent
		commands.entity(parent).add_child(title_entity);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bevy::ecs::system::RunSystemOnce;

	#[test]
	fn test_title_spawner_creation() -> Result<(), Box<dyn std::error::Error>> {
		let title = "Test Title".to_string();

		let spawner = TitleSpawner::new(
			TaskId::new(1),
			title.clone(),
			100.0, // 100px width for testing
		);

		assert_eq!(spawner.title, title);

		Ok(())
	}

	#[test]
	fn test_title_spawner_spawns_title_entity() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestTitleParams { title_text: "Spawn Test Title".to_string() };

		// Spawn the title using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that TitleMarker was spawned
		let mut title_marker_query = world.query::<&TitleMarker>();
		let title_markers: Vec<_> = title_marker_query.iter(world).collect();
		assert_eq!(title_markers.len(), 1, "Should spawn exactly one TitleMarker entity");

		Ok(())
	}

	#[derive(Clone)]
	struct TestTitleParams {
		title_text: String,
	}

	impl TestTitleParams {
		fn new() -> Self {
			Self { title_text: "Specific Title Text".to_string() }
		}

		fn build(&self) -> impl FnMut(Commands) {
			let title_text = self.title_text.clone();
			move |mut commands: Commands| {
				let spawner = TitleSpawner::new(
					TaskId::new(1),
					title_text.clone(),
					100.0, // 100px width for testing
				);
				let parent_entity = commands.spawn_empty().id();
				spawner.spawn(&mut commands, parent_entity);
			}
		}
	}

	#[test]
	fn test_title_spawner_sets_correct_text() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestTitleParams::new();

		// Spawn the title using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that Text component has correct content
		let mut text_query = world.query::<&Text>();
		let texts: Vec<_> = text_query.iter(world).collect();
		assert_eq!(texts.len(), 1, "Should spawn exactly one Text entity");

		let text = texts[0];
		assert_eq!(text.0, params.title_text, "Text content should match title");

		Ok(())
	}

	#[test]
	fn test_title_spawner_sets_correct_styling() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestTitleParams { title_text: "Styling Test Title".to_string() };

		// Spawn the title using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check TextColor component
		let mut text_color_query = world.query::<&TextColor>();
		let text_colors: Vec<_> = text_color_query.iter(world).collect();
		assert_eq!(text_colors.len(), 1, "Should spawn exactly one TextColor entity");
		assert_eq!(text_colors[0].0, Color::BLACK, "Text color should be black");

		// Check TextFont component
		let mut text_font_query = world.query::<&TextFont>();
		let text_fonts: Vec<_> = text_font_query.iter(world).collect();
		assert_eq!(text_fonts.len(), 1, "Should spawn exactly one TextFont entity");
		assert_eq!(text_fonts[0].font_size, 12.0, "Font size should be 12.0");

		Ok(())
	}

	#[test]
	fn test_title_spawner_sets_correct_node_layout() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestTitleParams { title_text: "Layout Test Title".to_string() };

		// Spawn the title using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check Node component properties
		let mut node_query = world.query::<&Node>();
		let nodes: Vec<_> = node_query.iter(world).collect();
		assert_eq!(nodes.len(), 1, "Should spawn exactly one Node entity");

		let node = nodes[0];
		assert_eq!(node.display, Display::Flex, "Node should use flex display");
		assert_eq!(node.align_items, AlignItems::Center, "Node should center align items");
		assert_eq!(node.align_content, AlignContent::Center, "Node should center align content");
		assert_eq!(
			node.justify_content,
			JustifyContent::Start,
			"Node should start justify content"
		);
		assert_eq!(node.align_self, AlignSelf::Center, "Node should center align self");

		Ok(())
	}

	#[test]
	fn test_title_spawner_attaches_to_parent() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestTitleParams { title_text: "Parent Attachment Test".to_string() };

		// Spawn the title using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that the parent entity has children
		let mut children_query = world.query::<&Children>();
		let children_components: Vec<_> = children_query.iter(world).collect();

		// Find the parent's children component
		let parent_children = children_components.iter().find(|children| {
			children.iter().any(|child| {
				// Check if this child has a title marker
				world.get::<TitleMarker>(child).is_some()
			})
		});

		assert!(parent_children.is_some(), "Parent should have a child with TitleMarker");

		Ok(())
	}

	#[test]
	fn test_title_spawner_empty_title() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestTitleParams {
			title_text: "".to_string(), // Empty title
		};

		// Spawn the title using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that TitleMarker was still spawned
		let mut title_marker_query = world.query::<&TitleMarker>();
		let title_markers: Vec<_> = title_marker_query.iter(world).collect();
		assert_eq!(title_markers.len(), 1, "Should spawn TitleMarker even with empty title");

		// Check that Text component has empty content
		let mut text_query = world.query::<&Text>();
		let texts: Vec<_> = text_query.iter(world).collect();
		assert_eq!(texts.len(), 1, "Should spawn Text entity even with empty title");
		assert_eq!(texts[0].0, "", "Text content should be empty");

		Ok(())
	}

	#[test]
	fn test_title_spawner_long_title() -> Result<(), Box<dyn std::error::Error>> {
		// Setup app
		let mut app = App::new();
		app.add_plugins(MinimalPlugins);

		let params = TestTitleParams {
			title_text: "This is a very long title that might wrap or overflow depending on the container size and styling".to_string(),
		};

		// Spawn the title using the builder
		app.world_mut().run_system_once(params.build())?;

		let world = app.world_mut();

		// Check that TitleMarker was spawned
		let mut title_marker_query = world.query::<&TitleMarker>();
		let title_markers: Vec<_> = title_marker_query.iter(world).collect();
		assert_eq!(title_markers.len(), 1, "Should spawn TitleMarker for long title");

		// Check that Text component has correct content
		let mut text_query = world.query::<&Text>();
		let texts: Vec<_> = text_query.iter(world).collect();
		assert_eq!(texts.len(), 1, "Should spawn Text entity for long title");
		assert_eq!(texts[0].0, params.title_text, "Text content should match long title");

		Ok(())
	}
}
