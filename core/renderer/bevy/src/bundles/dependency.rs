use crate::components::{Dependency, RenderState};
use bevy::prelude::*;
use roadline_util::dependency::Id as DependencyId;

/// Bundle for spawning a dependency edge between tasks
#[derive(Bundle)]
pub struct DependencyBundle {
	pub sprite: Sprite,
	pub transform: Transform,
	pub visibility: Visibility,
	pub dependency: Dependency,
	pub render_state: RenderState,
}

impl DependencyBundle {
	pub fn new(dependency_id: DependencyId, start_position: Vec3, end_position: Vec3) -> Self {
		// Calculate the midpoint and direction for the line
		let midpoint = (start_position + end_position) / 2.0;
		let direction = end_position - start_position;
		let length = direction.length();
		let angle = direction.y.atan2(direction.x);

		Self {
			sprite: Sprite {
				color: Color::srgb(0.5, 0.5, 0.5),
				custom_size: Some(Vec2::new(length, 2.0)),
				..default()
			},
			transform: Transform {
				translation: midpoint,
				rotation: Quat::from_rotation_z(angle),
				..default()
			},
			visibility: Visibility::Visible,
			dependency: Dependency::new(dependency_id),
			render_state: RenderState::new(),
		}
	}

	pub fn with_color(mut self, color: Color) -> Self {
		self.sprite.color = color;
		self
	}

	pub fn with_thickness(mut self, thickness: f32) -> Self {
		if let Some(size) = self.sprite.custom_size {
			self.sprite.custom_size = Some(Vec2::new(size.x, thickness));
		}
		self
	}
}
