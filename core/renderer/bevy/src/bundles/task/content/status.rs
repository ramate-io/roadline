use bevy::prelude::*;
use bevy::ui::{Node, Val};

pub mod completed;
pub mod in_progress;
pub mod missed;
pub mod not_started;

pub use completed::CompletedStatus;
pub use in_progress::InProgressStatus;
pub use missed::MissedStatus;
pub use not_started::NotStartedStatus;

pub type StatusBundle = (Node, BackgroundColor, Sprite);

pub struct StatusPreBundle(StatusBundle);

impl StatusPreBundle {
	pub fn bundle(self) -> StatusBundle {
		self.0
	}
}

pub struct StatusBundler {
	pub completed: u32,
	pub total: u32,
}

impl StatusBundler {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn pre_bundle(self) -> StatusPreBundle {
		// Determine status type based on completion
		if self.completed == 0 {
			// Not started - blue
			NotStartedStatus::new(self.total).pre_bundle()
		} else if self.completed == self.total {
			// Completed - green
			CompletedStatus::new(self.completed, self.total).pre_bundle()
		} else {
			// In progress - yellow
			InProgressStatus::new(self.completed, self.total).pre_bundle()
		}
	}
}
