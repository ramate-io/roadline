pub mod completed;
pub mod in_progress;
pub mod missed;
pub mod not_started;

pub use completed::{CompletedStatusBundle, CompletedStatusBundler, CompletedStatusPreBundle};
pub use in_progress::{InProgressStatusBundle, InProgressStatusBundler, InProgressStatusPreBundle};
pub use missed::{MissedStatusBundle, MissedStatusBundler, MissedStatusPreBundle};
pub use not_started::{NotStartedStatusBundle, NotStartedStatusBundler, NotStartedStatusPreBundle};

use bevy::prelude::Component;

/// Use options for different status types
#[derive(Component)]
pub enum StatusBundle {
	Completed(CompletedStatusBundle),
	InProgress(InProgressStatusBundle),
	Missed(MissedStatusBundle),
	NotStarted(NotStartedStatusBundle),
}

pub enum StatusPreBundle {
	Completed(CompletedStatusPreBundle),
	InProgress(InProgressStatusPreBundle),
	Missed(MissedStatusPreBundle),
	NotStarted(NotStartedStatusPreBundle),
}

impl StatusPreBundle {
	pub fn bundle(self) -> StatusBundle {
		match self {
			StatusPreBundle::Completed(completed) => StatusBundle::Completed(completed.bundle()),
			StatusPreBundle::InProgress(in_progress) => {
				StatusBundle::InProgress(in_progress.bundle())
			}
			StatusPreBundle::Missed(missed) => StatusBundle::Missed(missed.bundle()),
			StatusPreBundle::NotStarted(not_started) => {
				StatusBundle::NotStarted(not_started.bundle())
			}
		}
	}
}

/// This should be an enum of bundlers
pub enum StatusBundler {
	NotStarted(NotStartedStatusBundler),
	InProgress(InProgressStatusBundler),
	Completed(CompletedStatusBundler),
	Missed(MissedStatusBundler),
}

impl StatusBundler {
	pub fn new(completed: u32, total: u32) -> Self {
		// Determine status type based on completion
		if completed == 0 {
			// Not started - blue
			Self::NotStarted(NotStartedStatusBundler::new(total))
		} else if completed == total {
			// Completed - green
			Self::Completed(CompletedStatusBundler::new(completed, total))
		} else {
			// In progress - yellow
			Self::InProgress(InProgressStatusBundler::new(completed, total))
		}
	}

	pub fn pre_bundle(self) -> StatusPreBundle {
		match self {
			StatusBundler::NotStarted(bundler) => StatusPreBundle::NotStarted(bundler.pre_bundle()),
			StatusBundler::InProgress(bundler) => StatusPreBundle::InProgress(bundler.pre_bundle()),
			StatusBundler::Completed(bundler) => StatusPreBundle::Completed(bundler.pre_bundle()),
			StatusBundler::Missed(bundler) => StatusPreBundle::Missed(bundler.pre_bundle()),
		}
	}
}
