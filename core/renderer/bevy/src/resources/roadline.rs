use bevy::prelude::*;
use roadline_representation_core::roadline::Roadline as CoreRoadline;

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
