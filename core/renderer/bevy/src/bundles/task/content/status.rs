pub mod completed;
pub mod in_progress;
pub mod missed;
pub mod not_started;

pub use completed::{CompletedStatusBundle, CompletedStatusPreBundle, CompletedStatusSpawner};
pub use in_progress::{InProgressStatusBundle, InProgressStatusPreBundle, InProgressStatusSpawner};
pub use missed::{MissedStatusBundle, MissedStatusPreBundle, MissedStatusSpawner};
pub use not_started::{NotStartedStatusBundle, NotStartedStatusPreBundle, NotStartedStatusSpawner};

use bevy::ecs::bundle::BundleFromComponents;
use bevy::prelude::*;
use std::marker::PhantomData;

/// A maker trait for status bundle variants.
pub trait StatusBundlable: Bundle + BundleFromComponents {
	fn new_status_bundle(completed: u32, total: u32) -> Self;
}

#[derive(Component)]
pub struct StatusMarker;

#[derive(Bundle)]
pub struct StatusBundle<T: StatusBundlable> {
	pub marker: StatusMarker,
	pub bundle: T,
}

pub struct StatusPreBundle<T: StatusBundlable> {
	pub bundle: StatusBundle<T>,
}

impl<T> StatusPreBundle<T>
where
	T: StatusBundlable,
{
	pub fn bundle(self) -> StatusBundle<T> {
		self.bundle
	}
}

/// This should be an enum of bundlers
pub struct StatusSpawner<T: StatusBundlable> {
	pub completed: u32,
	pub total: u32,
	pub phantom: PhantomData<T>,
}

impl<T> StatusSpawner<T>
where
	T: StatusBundlable,
{
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total, phantom: PhantomData }
	}

	pub fn pre_bundle(self) -> StatusPreBundle<T> {
		StatusPreBundle {
			bundle: StatusBundle {
				marker: StatusMarker,
				bundle: T::new_status_bundle(self.completed, self.total),
			},
		}
	}
}
