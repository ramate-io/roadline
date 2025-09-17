pub mod bundles;
pub mod components;
pub mod milestone;
pub mod resources;
pub mod roadline_renderer;
pub mod systems;

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;

use bevy::prelude::*;

pub use milestone::MilestoneSprite;
pub use roadline_renderer::RoadlineRenderer;

/// Resource to track the root UI node for task positioning
#[derive(Resource, Debug, Clone)]
pub struct RootUiNode(pub Entity);

/// Main plugin for the Roadline Bevy renderer
#[derive(Default)]
pub struct RoadlinePlugin;

impl Plugin for RoadlinePlugin {
	fn build(&self, app: &mut App) {
		app
			// Add core Bevy plugins for 2D rendering
			.add_plugins(DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Roadline Renderer".to_string(),
					canvas: Some("#roadline-canvas".to_string()), // For web integration
					fit_canvas_to_parent: true,
					prevent_default_event_handling: false,
					..default()
				}),
				..default()
			}))
			// Set white background
			.insert_resource(ClearColor(Color::WHITE))
			// Add our custom systems
			.add_systems(Startup, setup_camera)
			.add_systems(
				Update,
				(
					systems::TaskSystemConfig::build(),
					systems::DependencySystemConfig::build(),
					update_ui_positioning,
				),
			);
	}
}

/// Setup the 2D camera for rendering and root UI node
fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2d);

	// Create root UI node that covers the entire screen using flexbox layout
	let root_ui = commands
		.spawn((
			Node {
				position_type: PositionType::Absolute,
				left: Val::Px(0.0),
				top: Val::Px(0.0),
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				align_self: AlignSelf::Stretch,
				justify_self: JustifySelf::Stretch,
				flex_direction: FlexDirection::Column,
				align_items: AlignItems::FlexStart,
				justify_content: JustifyContent::FlexStart,
				..default()
			},
			BackgroundColor(Color::srgb(1.0, 1.0, 1.0)), // White background to see the UI
		))
		.id();

	// Store the root UI node as a resource
	commands.insert_resource(RootUiNode(root_ui));
}

/// System to update UI positioning based on camera movement
fn update_ui_positioning(
	camera_query: Query<&Transform, (With<Camera2d>, Changed<Transform>)>,
	mut ui_query: Query<&mut Node>,
	root_ui: Res<RootUiNode>,
) {
	if let Ok(camera_transform) = camera_query.get_single() {
		if let Ok(mut root_node) = ui_query.get_mut(root_ui.0) {
			// Update the root UI node's margin to offset camera movement
			root_node.margin = UiRect {
				left: Val::Px(-camera_transform.translation.x),
				top: Val::Px(-camera_transform.translation.y),
				right: Val::Px(0.0),
				bottom: Val::Px(0.0),
			};
		}
	}
}

/// Configuration for the renderer
#[derive(Resource, Debug, Clone)]
pub struct RoadlineRenderConfig {
	/// Scale factor for converting reified units to pixels
	pub unit_to_pixel_scale: f32,
	/// Background color
	pub background_color: Color,
	/// Default milestone color
	pub milestone_color: Color,
	/// Default edge color  
	pub edge_color: Color,
	/// Milestone radius in pixels
	pub milestone_radius: f32,
	/// Edge thickness in pixels
	pub edge_thickness: f32,
}

impl Default for RoadlineRenderConfig {
	fn default() -> Self {
		Self {
			unit_to_pixel_scale: 1.0,
			background_color: Color::srgb(1.0, 1.0, 1.0),
			milestone_color: Color::srgb(0.2, 0.7, 1.0),
			edge_color: Color::srgb(0.8, 0.8, 0.8),
			milestone_radius: 8.0,
			edge_thickness: 20.423,
		}
	}
}
