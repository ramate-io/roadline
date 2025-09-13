use crate::{
	resources::{ReifiedData, RenderUpdateEvent},
	RoadlineRenderConfig,
};
use bevy::prelude::*;
use roadline_representation_core::reified::Reified as CoreReified;

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
	pub fn render(&self, app: &mut App, reified: CoreReified) -> Result<(), RoadlineRenderError> {
		// Validate the reified data
		if reified.task_count() == 0 {
			return Err(RoadlineRenderError::EmptyData);
		}

		// Insert the reified data as a resource (wrapped)
		app.insert_resource(ReifiedData::new(reified));

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
		if let Some(reified) = app.world().get_resource::<ReifiedData>() {
			let (max_x, max_y) = reified.visual_bounds();
			let pixel_max_x = max_x.value() as f32 * self.config.unit_to_pixel_scale;
			let pixel_max_y = max_y.value() as f32 * self.config.unit_to_pixel_scale;

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
			let content_width = max_x - min_x;
			let content_height = max_y - min_y;

			// Calculate required scale to fit content with padding
			let padded_width = content_width * (1.0 + padding_ratio);
			let padded_height = content_height * (1.0 + padding_ratio);

			// Assume a default viewport size for scaling calculation
			let viewport_width = 1024.0;
			let viewport_height = 768.0;

			let scale_x = viewport_width / padded_width;
			let scale_y = viewport_height / padded_height;
			let scale = scale_x.min(scale_y);

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

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{DateTime, Utc};
	use roadline_representation_core::graph::Graph;
	use roadline_representation_core::grid_algebra::PreGridAlgebra;
	use roadline_representation_core::range_algebra::{Date, PreRangeAlgebra};
	use roadline_representation_core::reified::PreReified;
	use roadline_util::duration::Duration;
	use roadline_util::task::Id as TaskId;
	use roadline_util::task::{
		range::{End, PointOfReference, Start, TargetDate},
		Task,
	};
	use std::collections::BTreeSet;
	use std::time::Duration as StdDuration;

	fn test_date(iso_string: &str) -> Date {
		let datetime = DateTime::parse_from_rfc3339(iso_string)
			.expect("Valid datetime string")
			.with_timezone(&Utc);
		Date::new(datetime)
	}

	fn create_test_task(
		id: u8,
		reference_id: u8,
		offset_days: u64,
		duration_days: u64,
	) -> Result<Task, anyhow::Error> {
		let id = TaskId::new(id);
		let reference_id = TaskId::new(reference_id);

		let start = Start::from(TargetDate {
			point_of_reference: PointOfReference::from(reference_id),
			duration: Duration::from(StdDuration::from_secs(offset_days * 24 * 60 * 60)),
		});

		let end = End::from(Duration::from(StdDuration::from_secs(duration_days * 24 * 60 * 60)));
		let range = roadline_util::task::Range::new(start, end);

		Ok(Task::new(
			id,
			roadline_util::task::Title::new_test(),
			BTreeSet::new(),
			BTreeSet::new(),
			roadline_util::task::Summary::new_test(),
			range,
		))
	}

	fn create_test_reified() -> Result<CoreReified, anyhow::Error> {
		let mut graph = Graph::new();

		let task1 = create_test_task(1, 1, 0, 10)?;
		let task2 = create_test_task(2, 1, 5, 10)?;

		graph.add(task1)?;
		graph.add(task2)?;

		let range_algebra =
			PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
		let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;
		let reified = PreReified::new(grid_algebra).compute()?;

		Ok(reified)
	}

	#[test]
	fn test_renderer_creation() {
		let renderer = RoadlineRenderer::new();
		let app = renderer.create_app();

		assert!(app.world().contains_resource::<RoadlineRenderConfig>());
	}

	#[test]
	fn test_render_with_data() -> Result<(), anyhow::Error> {
		let renderer = RoadlineRenderer::new();
		let mut app = renderer.create_app();
		let reified = create_test_reified()?;

		let result = renderer.render(&mut app, reified);
		assert!(result.is_ok());

		// Verify reified data was inserted
		assert!(app.world().contains_resource::<ReifiedData>());

		Ok(())
	}

	#[test]
	fn test_render_empty_data_error() {
		// Skip this test for now since we need a proper empty constructor
		// TODO: Implement proper empty reified data creation for testing
	}

	#[test]
	fn test_visual_bounds() -> Result<(), anyhow::Error> {
		let renderer = RoadlineRenderer::new();
		let mut app = renderer.create_app();
		let reified = create_test_reified()?;

		renderer.render(&mut app, reified)?;

		let bounds = renderer.get_visual_bounds(&app);
		assert!(bounds.is_some());

		let (min_x, max_x, min_y, max_y) = bounds.unwrap();
		assert!(max_x > min_x);
		assert!(max_y >= min_y);

		Ok(())
	}
}
