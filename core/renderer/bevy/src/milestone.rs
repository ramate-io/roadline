use crate::components::{MilestoneNode, RenderState};
use crate::RoadlineRenderConfig;
use bevy::prelude::*;
use roadline_representation_core::reified::{DownCell, ReifiedUnit};
use roadline_util::task::Id as TaskId;

/// Represents a milestone sprite in the Bevy world
#[derive(Bundle)]
pub struct MilestoneSprite {
	pub sprite: Sprite,
	pub transform: Transform,
	pub visibility: Visibility,
	pub milestone_node: MilestoneNode,
	pub render_state: RenderState,
}

impl MilestoneSprite {
	/// Create a new milestone sprite from a task's down cell
	pub fn new(task_id: TaskId, down_cell: &DownCell, config: &RoadlineRenderConfig) -> Self {
		let (x, y) = down_cell.outgoing_connection_point();

		// Convert reified units to pixel coordinates
		let pixel_x = x.value() as f32 * config.unit_to_pixel_scale;
		let pixel_y = y.value() as f32 * config.unit_to_pixel_scale;

		Self {
			sprite: Sprite {
				color: config.milestone_color,
				custom_size: Some(Vec2::new(
					config.milestone_radius * 2.0,
					config.milestone_radius * 2.0,
				)),
				..default()
			},
			transform: Transform::from_xyz(pixel_x, pixel_y, 1.0),
			visibility: Visibility::Visible,
			milestone_node: MilestoneNode::new(task_id),
			render_state: RenderState::new(),
		}
	}

	/// Create a milestone sprite with custom position and styling
	pub fn with_custom_style(
		task_id: TaskId,
		position: (ReifiedUnit, ReifiedUnit),
		color: Color,
		radius: f32,
		config: &RoadlineRenderConfig,
	) -> Self {
		let pixel_x = position.0.value() as f32 * config.unit_to_pixel_scale;
		let pixel_y = position.1.value() as f32 * config.unit_to_pixel_scale;

		Self {
			sprite: Sprite {
				color,
				custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
				..default()
			},
			transform: Transform::from_xyz(pixel_x, pixel_y, 1.0),
			visibility: Visibility::Visible,
			milestone_node: MilestoneNode::new(task_id),
			render_state: RenderState::new(),
		}
	}

	/// Create a circular milestone sprite (default shape)
	pub fn circular(task_id: TaskId, down_cell: &DownCell, config: &RoadlineRenderConfig) -> Self {
		Self::new(task_id, down_cell, config)
	}

	/// Create a diamond-shaped milestone sprite
	pub fn diamond(task_id: TaskId, down_cell: &DownCell, config: &RoadlineRenderConfig) -> Self {
		let (x, y) = down_cell.outgoing_connection_point();

		let pixel_x = x.value() as f32 * config.unit_to_pixel_scale;
		let pixel_y = y.value() as f32 * config.unit_to_pixel_scale;

		Self {
			sprite: Sprite {
				color: config.milestone_color,
				custom_size: Some(Vec2::new(
					config.milestone_radius * 2.0,
					config.milestone_radius * 2.0,
				)),
				..default()
			},
			transform: Transform::from_xyz(pixel_x, pixel_y, 1.0)
				.with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)), // 45 degree rotation for diamond
			visibility: Visibility::Visible,
			milestone_node: MilestoneNode::new(task_id),
			render_state: RenderState::new(),
		}
	}
}

/// System to handle milestone sprite animations and interactions
pub fn animate_milestone_sprites(
	time: Res<Time>,
	mut query: Query<(&mut Transform, &mut Sprite), With<MilestoneNode>>,
) {
	for (mut transform, mut sprite) in query.iter_mut() {
		// Simple pulsing animation
		let pulse = (time.elapsed_secs() * 2.0).sin() * 0.1 + 1.0;
		transform.scale = Vec3::splat(pulse);

		// Slight alpha variation for breathing effect
		let alpha = (time.elapsed_secs() * 1.5).sin() * 0.1 + 0.9;
		sprite.color.set_alpha(alpha);
	}
}

/// System to handle milestone hover effects (for future interactivity)
pub fn handle_milestone_interactions(
	mut query: Query<(&Transform, &mut Sprite), With<MilestoneNode>>,
	// TODO: Add cursor position and input handling
) {
	// Placeholder for future interaction handling
	for (_transform, _sprite) in query.iter_mut() {
		// TODO: Implement hover effects, click handling, etc.
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{DateTime, Utc};
	use roadline_representation_core::grid_algebra::cell::Cell;
	use roadline_representation_core::grid_algebra::{lane::LaneId, stretch::Stretch};
	use roadline_representation_core::range_algebra::Date;
	use roadline_representation_core::reified::{DownLane, DownStretch, ReifiedUnit};

	fn create_test_down_cell() -> DownCell {
		// Create test dates (not used in this simplified test helper)
		let _start_date = Date::new(
			DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
				.unwrap()
				.with_timezone(&Utc),
		);
		let _end_date = Date::new(
			DateTime::parse_from_rfc3339("2021-01-10T00:00:00Z")
				.unwrap()
				.with_timezone(&Utc),
		);

		// Create stretch with proper types
		let stretch_range =
			roadline_representation_core::grid_algebra::stretch::StretchRange::new(0, 100);
		let stretch_unit =
			roadline_representation_core::grid_algebra::stretch::StretchUnit::from_average_seconds(
				86400,
			); // 1 day
		let stretch = Stretch::new(stretch_range, stretch_unit);

		let lane_id = LaneId::new(roadline_util::short_id::ShortId::new(0));
		let cell = Cell::new(stretch.clone(), lane_id);

		let down_lane = DownLane::canonical_from_lane(
			lane_id,
			roadline_representation_core::reified::DownLanePadding::new(ReifiedUnit::new(2)),
		);
		let down_stretch = DownStretch::canonical_from_stretch(
			stretch,
			roadline_representation_core::reified::Trim::new(ReifiedUnit::new(10)),
		);

		DownCell::new(cell, down_lane, down_stretch)
	}

	#[test]
	fn test_milestone_sprite_creation() {
		let task_id = TaskId::new(1);
		let down_cell = create_test_down_cell();
		let config = RoadlineRenderConfig::default();

		let milestone = MilestoneSprite::new(task_id, &down_cell, &config);

		assert_eq!(milestone.milestone_node.task_id, task_id);
		assert_eq!(milestone.sprite.color, config.milestone_color);
	}

	#[test]
	fn test_milestone_sprite_custom_style() {
		let task_id = TaskId::new(2);
		let position = (ReifiedUnit::new(50), ReifiedUnit::new(75));
		let custom_color = Color::srgb(1.0, 0.0, 0.0);
		let custom_radius = 15.0;
		let config = RoadlineRenderConfig::default();

		let milestone = MilestoneSprite::with_custom_style(
			task_id,
			position,
			custom_color,
			custom_radius,
			&config,
		);

		assert_eq!(milestone.milestone_node.task_id, task_id);
		assert_eq!(milestone.sprite.color, custom_color);
		assert_eq!(
			milestone.sprite.custom_size,
			Some(Vec2::new(custom_radius * 2.0, custom_radius * 2.0))
		);
	}

	#[test]
	fn test_diamond_milestone_rotation() {
		let task_id = TaskId::new(3);
		let down_cell = create_test_down_cell();
		let config = RoadlineRenderConfig::default();

		let diamond_milestone = MilestoneSprite::diamond(task_id, &down_cell, &config);

		// Check that the diamond has rotation applied
		let rotation = diamond_milestone.transform.rotation;
		assert!((rotation.z - (std::f32::consts::PI / 4.0).sin()).abs() < 0.001);
	}
}
