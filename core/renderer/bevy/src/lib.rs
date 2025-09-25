pub mod bundles;
pub mod components;
pub mod resources;
pub mod roadline_renderer;
pub mod systems;

pub mod test_utils;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_ui_anchor::AnchorUiPlugin;

use crate::resources::SelectionResource;

pub use roadline_renderer::RoadlineRenderer;

/// Marker component for the UI camera
#[derive(Component)]
pub struct UiCameraMarker;

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
			// Add bevy_ui_anchor plugin
			.add_plugins(AnchorUiPlugin::<UiCameraMarker>::new())
			// Set white background
			.insert_resource(ClearColor(Color::WHITE))
			// Add selection resource
			.insert_resource(SelectionResource::new())
			// Add our custom systems
			.add_systems(Startup, setup_camera)
			.add_systems(
				Update,
				(
					systems::TaskSpawningSystem::default().build(),
					systems::DependencySystemConfig::build(),
					systems::dependency::dependency_hover_system,
					systems::TaskCursorInteractionSystem::default().build(),
					// systems::click_selection_system,
				),
			);
	}
}

/// Setup the 2D camera for rendering
fn setup_camera(mut commands: Commands) {
	// Spawn the sprite camera
	commands.spawn((
		Camera2d,
		Camera {
			order: 1,
			// Don't draw anything in the background, to see the previous camera.
			clear_color: ClearColorConfig::None,
			..default()
		},
		// This camera will only render entities which are on the same render layer.
		RenderLayers::layer(2),
	));

	commands.spawn((
		Camera2d,
		IsDefaultUiCamera,
		UiCameraMarker, // Mark this camera for bevy_ui_anchor
		RenderLayers::layer(1),
	));
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
