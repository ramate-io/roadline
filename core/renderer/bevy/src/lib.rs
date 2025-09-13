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
			// Add our custom systems
			.add_systems(Startup, setup_camera)
			.add_systems(
				Update,
				(systems::update_milestone_sprites, systems::update_edge_renderers),
			);
	}
}

/// Setup the 2D camera for rendering
fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2d);
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
			background_color: Color::srgb(0.1, 0.1, 0.1),
			milestone_color: Color::srgb(0.2, 0.7, 1.0),
			edge_color: Color::srgb(0.8, 0.8, 0.8),
			milestone_radius: 8.0,
			edge_thickness: 10.423,
		}
	}
}

// RoadlineRenderer is defined in roadline_renderer.rs

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_utils::*;

	#[test]
	fn test_renderer_creation() {
		let renderer = RoadlineRenderer::new();
		let app = renderer.create_app();

		// Basic smoke test - ensure app was created
		assert!(app.world().contains_resource::<RoadlineRenderConfig>());
	}

	#[test]
	fn test_custom_config() {
		let config = RoadlineRenderConfig {
			unit_to_pixel_scale: 10.423,
			milestone_radius: 10.0,
			..Default::default()
		};

		let renderer = RoadlineRenderer::with_config(config.clone());
		let app = renderer.create_app();

		let stored_config = app.world().resource::<RoadlineRenderConfig>();
		assert_eq!(stored_config.unit_to_pixel_scale, 10.423);
		assert_eq!(stored_config.milestone_radius, 10.0);
	}
}
