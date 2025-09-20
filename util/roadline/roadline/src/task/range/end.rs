use crate::duration::Duration;
use std::time::Duration as StdDuration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct End(Duration);

impl End {
	pub fn new(duration: Duration) -> Self {
		Self(duration)
	}

	pub fn duration(&self) -> &Duration {
		&self.0
	}
}

impl From<Duration> for End {
	fn from(duration: Duration) -> Self {
		Self(duration)
	}
}

impl From<End> for Duration {
	fn from(end: End) -> Self {
		end.0
	}
}

impl End {
	pub fn new_test() -> Self {
		Self(Duration::new_test())
	}
}

impl From<StdDuration> for End {
	fn from(duration: StdDuration) -> Self {
		Self(Duration::from(duration))
	}
}
