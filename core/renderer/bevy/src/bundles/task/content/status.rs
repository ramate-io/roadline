pub mod completed;
pub mod in_progress;
pub mod missed;
pub mod not_started;

pub use completed::CompletedStatusSpawner;
pub use in_progress::InProgressStatusSpawner;
pub use missed::MissedStatusSpawner;
pub use not_started::NotStartedStatusSpawner;

use bevy::prelude::*;

#[derive(Component)]
pub struct StatusMarker;

pub enum StatusSpawner {
	NotStarted(NotStartedStatusSpawner),
	InProgress(InProgressStatusSpawner),
	Completed(CompletedStatusSpawner),
	Missed(MissedStatusSpawner),
}

impl StatusSpawner {
	pub fn new(completed: u32, total: u32) -> Self {
		// Determine status type based on completion
		if completed == 0 {
			// Not started
			Self::NotStarted(NotStartedStatusSpawner::new(total))
		} else if completed == total {
			// Completed
			Self::Completed(CompletedStatusSpawner::new(completed, total))
		} else {
			// In progress
			Self::InProgress(InProgressStatusSpawner::new(completed, total))
		}
	}

	pub fn spawn(self, commands: &mut Commands, parent: Entity) {
		match self {
			StatusSpawner::NotStarted(spawner) => spawner.spawn(commands, parent),
			StatusSpawner::InProgress(spawner) => spawner.spawn(commands, parent),
			StatusSpawner::Completed(spawner) => spawner.spawn(commands, parent),
			StatusSpawner::Missed(spawner) => spawner.spawn(commands, parent),
		}
	}
}
