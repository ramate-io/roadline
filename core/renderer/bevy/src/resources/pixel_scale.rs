use bevy::prelude::*;

/// Resource that controls the pixel scaling for rendering tasks and dependencies.
/// This allows for zoom control by adjusting the pixels per unit values.
#[derive(Debug, Clone, Resource)]
pub struct PixelScale {
	/// Pixels per unit for the X-axis (time axis)
	pub pixels_per_x_unit: f32,
	/// Pixels per unit for the Y-axis (lane axis)
	pub pixels_per_y_unit: f32,
}

impl Default for PixelScale {
	fn default() -> Self {
		Self { pixels_per_x_unit: 10.0, pixels_per_y_unit: 75.0 }
	}
}

impl PixelScale {
	/// Create a new PixelScale with custom values
	pub fn new(pixels_per_x_unit: f32, pixels_per_y_unit: f32) -> Self {
		Self { pixels_per_x_unit, pixels_per_y_unit }
	}

	/// Scale a value for the X-axis
	pub fn scale_x(&self, value: f32) -> f32 {
		value * self.pixels_per_x_unit
	}

	/// Scale a value for the Y-axis
	pub fn scale_y(&self, value: f32) -> f32 {
		value * self.pixels_per_y_unit
	}

	/// Scale a Vec2 with X and Y scaling
	pub fn scale_vec2(&self, vec: Vec2) -> Vec2 {
		Vec2::new(self.scale_x(vec.x), self.scale_y(vec.y))
	}

	/// Scale a Vec3 with X and Y scaling (Z remains unchanged)
	pub fn scale_vec3(&self, vec: Vec3) -> Vec3 {
		Vec3::new(self.scale_x(vec.x), self.scale_y(vec.y), vec.z)
	}
}
