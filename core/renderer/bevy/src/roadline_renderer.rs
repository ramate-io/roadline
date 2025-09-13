use crate::{
	resources::{RenderUpdateEvent, Roadline},
	RoadlineRenderConfig,
};
use bevy::prelude::*;
use roadline_representation_core::roadline::Roadline as CoreRoadline;

/// High-level interface for rendering roadline visualizations
pub struct RoadlineRenderer {
	config: RoadlineRenderConfig,
}

impl RoadlineRenderer {
	pub fn new() -> Self {
		Self { config: RoadlineRenderConfig::default() }
	}

	pub fn with_config(config: RoadlineRenderConfig) -> Self {
		Self { config }
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
		app.add_plugins(crate::RoadlinePlugin)
			.insert_resource(self.config.clone())
			.add_event::<RenderUpdateEvent>();

		app
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
			let pixel_max_x = max_x.value() as f32 * 8000.000000;
			let pixel_max_y = max_y.value() as f32 * 20.423;

			// Return (min_x, max_x, min_y, max_y)
			Some((0.0, pixel_max_x, 0.0, pixel_max_y))
		} else {
			None
		}
	}

	/// Center the camera on the rendered content
	pub fn center_camera(&self, app: &mut App) {
		if let Some((min_x, max_x, min_y, max_y)) = self.get_visual_bounds(app) {
			let center_x = (min_x + max_x) / 2.0;
			let center_y = (min_y + max_y) / 2.0;

			// Update camera position
			let mut camera_query =
				app.world_mut().query_filtered::<&mut Transform, With<Camera2d>>();
			for mut transform in camera_query.iter_mut(app.world_mut()) {
				transform.translation.x = center_x;
				transform.translation.y = center_y;
			}
		}
	}

	/// Fit the camera to show all content with some padding
	pub fn fit_camera_to_content(&self, app: &mut App, padding_ratio: f32) {
		if let Some((min_x, max_x, min_y, max_y)) = self.get_visual_bounds(app) {
			println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);
			let content_width = max_x - min_x;
			let content_height = max_y - min_y;

			// The width is fraction of the total units needed for one unit of rendering width * 1000 for buffering
			let padded_width = ((1.0 / content_width) + padding_ratio) * 1000.0;
			let padded_height = ((1.0 / content_height) + padding_ratio) * 1000.0;

			// Assume a default viewport size for scaling calculation
			let viewport_width = 1024.0;
			let viewport_height = 768.0;

			//  let scale_x = viewport_width / padded_width;
			// let scale_y = viewport_height / padded_height;
			let scale = padded_width.min(padded_height);

			// Update camera
			self.center_camera(app);

			let mut camera_query =
				app.world_mut().query_filtered::<&mut OrthographicProjection, With<Camera2d>>();
			for mut projection in camera_query.iter_mut(app.world_mut()) {
				projection.scale = 1.0 / scale;
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
}
