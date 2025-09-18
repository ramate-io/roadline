pub mod completed;
pub mod in_progress;
pub mod missed;
pub mod not_started;

pub use completed::{CompletedStatusBundle, CompletedStatusBundler, CompletedStatusPreBundle};
pub use in_progress::{InProgressStatusBundle, InProgressStatusBundler, InProgressStatusPreBundle};
pub use missed::{MissedStatusBundle, MissedStatusBundler, MissedStatusPreBundle};
pub use not_started::{NotStartedStatusBundle, NotStartedStatusBundler, NotStartedStatusPreBundle};

use bevy::prelude::*;

/// relationship that defines which uinodes are anchored to this entity
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[relationship_target(relationship = StatusUiNode, linked_spawn)]
pub struct StatusNodes(Vec<Entity>);

/// Component that will continuosly update the UI location on screen, to match an in world location either chosen as a fixed
/// position, or chosen as another entities ['GlobalTransformation']
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[relationship(relationship_target = StatusNodes)]
#[require(Node)]
pub struct StatusUiNode {
	/// The Ui will be placed onto the screen, matching where this entity is located in the world
	#[relationship]
	pub target: Entity,
}

#[derive(Component)]
pub struct StatusMarker;

/// Use options for different status types
#[derive(Bundle)]
pub struct StatusBundle {
	pub marker: StatusMarker,
	pub node: Node,
	pub completed: Text,
}

pub enum StatusPreBundle {
	Completed(CompletedStatusPreBundle),
	InProgress(InProgressStatusPreBundle),
	Missed(MissedStatusPreBundle),
	NotStarted(NotStartedStatusPreBundle),
}

impl StatusPreBundle {
	pub fn bundle(self) -> StatusBundle {
		StatusBundle {
			marker: StatusMarker,
			node: Node::default(),
			completed: Text::new("Completed"),
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
