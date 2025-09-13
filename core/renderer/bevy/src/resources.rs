use bevy::prelude::*;
use roadline_representation_core::roadline::Roadline as CoreRoadline;

/// Event to trigger render updates
#[derive(Event, Debug)]
pub struct RenderUpdateEvent;

/// Resource to track the current viewport/camera bounds
#[derive(Resource, Debug)]
pub struct ViewportBounds {
	pub min_x: f32,
	pub max_x: f32,
	pub min_y: f32,
	pub max_y: f32,
}

impl ViewportBounds {
	pub fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
		Self { min_x, max_x, min_y, max_y }
	}

	pub fn width(&self) -> f32 {
		self.max_x - self.min_x
	}

	pub fn height(&self) -> f32 {
		self.max_y - self.min_y
	}

	pub fn center(&self) -> (f32, f32) {
		((self.min_x + self.max_x) / 2.0, (self.min_y + self.max_y) / 2.0)
	}
}

impl Default for ViewportBounds {
	fn default() -> Self {
		Self::new(-500.0, 500.0, -300.0, 300.0)
	}
}

/// Bevy Resource wrapper for the core Reified representation
#[derive(Resource, Debug, Clone)]
pub struct Roadline {
	inner: CoreRoadline,
}

impl Roadline {
	pub fn new(reified: CoreRoadline) -> Self {
		Self { inner: reified }
	}

	/// Get a reference to the inner reified data
	pub fn inner(&self) -> &CoreRoadline {
		&self.inner
	}

	/// Get a mutable reference to the inner reified data
	pub fn inner_mut(&mut self) -> &mut CoreRoadline {
		&mut self.inner
	}

	/// Consume the wrapper and return the inner reified data
	pub fn into_inner(self) -> CoreRoadline {
		self.inner
	}
}

impl From<CoreRoadline> for Roadline {
	fn from(reified: CoreRoadline) -> Self {
		Self::new(reified)
	}
}

impl AsRef<CoreRoadline> for Roadline {
	fn as_ref(&self) -> &CoreRoadline {
		&self.inner
	}
}

impl AsMut<CoreRoadline> for Roadline {
	fn as_mut(&mut self) -> &mut CoreRoadline {
		&mut self.inner
	}
}

// Implement Deref and DerefMut for convenient access to inner methods
impl std::ops::Deref for Roadline {
	type Target = CoreRoadline;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl std::ops::DerefMut for Roadline {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}
