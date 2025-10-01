use crate::test_utils::create_test_roadline;
use crate::RoadlinePlugin;
use crate::{
	resources::{RenderUpdateEvent, Roadline},
	RoadlineRenderConfig,
};
use bevy::prelude::*;
use roadline_representation_core::roadline::Roadline as CoreRoadline;

/// High-level interface for rendering roadline visualizations
pub struct RoadlineRenderer {
	config: RoadlineRenderConfig,
	plugin: RoadlinePlugin,
}

impl RoadlineRenderer {
	/// Create a new RoadlineRenderer with the default configuration
	pub fn new() -> Self {
		Self { config: RoadlineRenderConfig::default(), plugin: RoadlinePlugin::default() }
	}

	pub fn with_config(self, config: RoadlineRenderConfig) -> Self {
		Self { config, ..self }
	}

	pub fn with_plugin(self, plugin: RoadlinePlugin) -> Self {
		Self { plugin, ..self }
	}

	pub fn config(&self) -> &RoadlineRenderConfig {
		&self.config
	}

	pub fn config_mut(&mut self) -> &mut RoadlineRenderConfig {
		&mut self.config
	}

	/// Create a new Bevy App configured for roadline rendering
	pub fn create_app(&self) -> App {
		let mut app = App::new();

		// Add the roadline plugin and configuration
		app.add_plugins(self.plugin.clone())
			.insert_resource(self.config.clone())
			.add_event::<RenderUpdateEvent>();

		app
	}

	/// Creates an app with the test roadline
	pub fn create_app_with_test_roadline(&self) -> Result<App, RoadlineRenderError> {
		let mut app = self.create_app();
		let reified = create_test_roadline().map_err(anyhow::Error::from)?;
		app.insert_resource(Roadline::new(reified));

		if let Some(mut event_writer) =
			app.world_mut().get_resource_mut::<Events<RenderUpdateEvent>>()
		{
			event_writer.send(RenderUpdateEvent);
		} else {
			return Err(RoadlineRenderError::EventSystemNotInitialized);
		}

		Ok(app)
	}

	pub fn create_app_with_roadline(
		&self,
		roadline: CoreRoadline,
	) -> Result<App, RoadlineRenderError> {
		let mut app = self.create_app();

		// remove the existing roadline resource
		app.world_mut().remove_resource::<Roadline>();

		// insert the new roadline resource
		app.insert_resource(Roadline::new(roadline));

		if let Some(mut event_writer) =
			app.world_mut().get_resource_mut::<Events<RenderUpdateEvent>>()
		{
			event_writer.send(RenderUpdateEvent);
		} else {
			return Err(RoadlineRenderError::EventSystemNotInitialized);
		}

		Ok(app)
	}

	/// Render reified data in the given app
	pub fn render(&self, app: &mut App, reified: CoreRoadline) -> Result<(), RoadlineRenderError> {
		// Validate the reified data
		if reified.task_count() == 0 {
			return Err(RoadlineRenderError::EmptyData);
		}

		// Insert the reified data as a resource (wrapped)
		app.insert_resource(Roadline::new(reified));

		// Send render update event
		if let Some(mut event_writer) =
			app.world_mut().get_resource_mut::<Events<RenderUpdateEvent>>()
		{
			event_writer.send(RenderUpdateEvent);
		} else {
			return Err(RoadlineRenderError::EventSystemNotInitialized);
		}

		Ok(())
	}

	/// Update the rendering configuration
	pub fn update_config(&mut self, app: &mut App, new_config: RoadlineRenderConfig) {
		self.config = new_config.clone();
		app.insert_resource(new_config);

		// Trigger a re-render
		if let Some(mut event_writer) =
			app.world_mut().get_resource_mut::<Events<RenderUpdateEvent>>()
		{
			event_writer.send(RenderUpdateEvent);
		}
	}

	/// Get the current visual bounds from the rendered data
	pub fn get_visual_bounds(&self, app: &App) -> Option<(f32, f32, f32, f32)> {
		if let Some(reified) = app.world().get_resource::<Roadline>() {
			let (max_x, max_y) = reified.visual_bounds();
			let pixels_per_unit = 50.0; // Same as in systems.rs
			let pixel_max_x = max_x.value() as f32 * pixels_per_unit;
			let pixel_max_y = max_y.value() as f32 * pixels_per_unit;

			// Return (min_x, max_x, min_y, max_y)
			Some((0.0, pixel_max_x, 0.0, pixel_max_y))
		} else {
			None
		}
	}

	/// Center the camera on the rendered content
	pub fn center_camera(&self, app: &mut App) {
		// Since we now position content around (0,0), just center the camera at origin
		let center_x = 0.0;
		let center_y = 0.0;
		println!("Centering camera at origin: ({:.1}, {:.1})", center_x, center_y);

		// Update camera position
		let mut camera_query = app.world_mut().query_filtered::<&mut Transform, With<Camera2d>>();
		for mut transform in camera_query.iter_mut(app.world_mut()) {
			transform.translation.x = center_x;
			transform.translation.y = center_y;
		}
	}

	/// Fit the camera to show all content with some padding
	pub fn fit_camera_to_content(&self, app: &mut App, _padding_ratio: f32) {
		if let Some((min_x, max_x, min_y, max_y)) = self.get_visual_bounds(app) {
			println!(
				"Visual bounds: min_x: {}, max_x: {}, min_y: {}, max_y: {}",
				min_x, max_x, min_y, max_y
			);

			let content_width = max_x - min_x;
			let content_height = max_y - min_y;

			// Simple scaling: just use a reasonable scale that should show everything
			let scale = 0.5; // Start with a simple fixed scale to see if content appears

			println!(
				"Content size: {}x{}, Using fixed scale: {}",
				content_width, content_height, scale
			);

			// Center camera first
			self.center_camera(app);

			// Apply scale to camera
			let mut camera_query =
				app.world_mut().query_filtered::<&mut Projection, With<Camera2d>>();
			for mut projection in camera_query.iter_mut(app.world_mut()) {
				if let Projection::Orthographic(ref mut orthographic) = *projection {
					orthographic.scale = scale;
				}
			}
		}
	}
}

impl Default for RoadlineRenderer {
	fn default() -> Self {
		Self::new()
	}
}

/// Errors that can occur during roadline rendering
#[derive(Debug, thiserror::Error)]
pub enum RoadlineRenderError {
	#[error("Cannot render empty data")]
	EmptyData,
	#[error("Event system not properly initialized")]
	EventSystemNotInitialized,
	#[error("Bevy app not properly configured")]
	AppNotConfigured,
	#[error("Roadline renderer encountered an internal error: {0}")]
	Internal(#[from] anyhow::Error),
}
