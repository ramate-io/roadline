use crate::components::{Dependency, RenderState};
use bevy::prelude::*;
use roadline_util::dependency::Id as DependencyId;

/// Bundle for spawning a dependency edge between tasks
pub struct DependencyBundle {
	pub dependency_id: DependencyId,
	pub start_position: Vec3,
	pub end_position: Vec3,
	pub color: Color,
	pub thickness: f32,
}

impl DependencyBundle {
	pub fn new(dependency_id: DependencyId, start_position: Vec3, end_position: Vec3) -> Self {
		Self {
			dependency_id,
			start_position,
			end_position,
			color: Color::srgb(0.5, 0.5, 0.5),
			thickness: 2.0,
		}
	}

	pub fn with_color(mut self, color: Color) -> Self {
		self.color = color;
		self
	}

	pub fn with_thickness(mut self, thickness: f32) -> Self {
		self.thickness = thickness;
		self
	}

	/// Spawns a dependency edge entity
	pub fn spawn(self, commands: &mut Commands) {
		// Calculate the midpoint and direction for the line
		let midpoint = (self.start_position + self.end_position) / 2.0;
		let direction = self.end_position - self.start_position;
		let length = direction.length();
		let angle = direction.y.atan2(direction.x);

		commands.spawn((
			Sprite {
				color: self.color,
				custom_size: Some(Vec2::new(length, self.thickness)),
				..default()
			},
			Transform {
				translation: midpoint,
				rotation: Quat::from_rotation_z(angle),
				..default()
			},
			Visibility::Visible,
			Dependency::new(self.dependency_id),
			RenderState::new(),
		));
	}
}
