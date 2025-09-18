use bevy::prelude::*;
use bevy::ui::{Node, Val};

pub mod completed;
pub mod in_progress;
pub mod missed;
pub mod not_started;

pub use completed::{CompletedStatus, CompletedStatusBundle, CompletedStatusPreBundle};
pub use in_progress::{InProgressStatus, InProgressStatusBundle, InProgressStatusPreBundle};
pub use missed::{MissedStatus, MissedStatusBundle, MissedStatusPreBundle};
pub use not_started::{NotStartedStatus, NotStartedStatusBundle, NotStartedStatusPreBundle};

/// Use options for different status types
pub type StatusBundle = (
	Option<CompletedStatusBundle>,
	Option<InProgressStatusBundle>,
	Option<MissedStatusBundle>,
	Option<NotStartedStatusBundle>,
);

pub enum StatusPreBundle {
	Completed(CompletedStatusPreBundle),
	InProgress(InProgressStatusPreBundle),
	Missed(MissedStatusPreBundle),
	NotStarted(NotStartedStatusPreBundle),
}

impl StatusPreBundle {
	pub fn bundle(self) -> StatusBundle {
		match self {
			StatusPreBundle::Completed(completed) => (Some(completed.bundle()), None, None, None),
			StatusPreBundle::InProgress(in_progress) => {
				(None, Some(in_progress.bundle()), None, None)
			}
			StatusPreBundle::Missed(missed) => (None, None, Some(missed.bundle()), None),
			StatusPreBundle::NotStarted(not_started) => {
				(None, None, None, Some(not_started.bundle()))
			}
		}
	}
}

/// This should be an enum of bundlers
pub enum StatusBundler {
	NotStarted(NotStartedStatus),
	InProgress(InProgressStatus),
	Completed(CompletedStatus),
	Missed(MissedStatus),
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
