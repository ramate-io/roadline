use bevy::prelude::*;

/// Component for tracking render state
#[derive(Component, Debug)]
pub struct RenderState {
	pub needs_update: bool,
}

impl RenderState {
	pub fn new() -> Self {
		Self { needs_update: true }
	}

	pub fn mark_dirty(&mut self) {
		self.needs_update = true;
	}

	pub fn mark_clean(&mut self) {
		self.needs_update = false;
	}
}

impl Default for RenderState {
	fn default() -> Self {
		Self::new()
	}
}
