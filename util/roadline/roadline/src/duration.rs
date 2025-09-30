use serde::{Deserialize, Serialize};
use std::time::Duration as StdDuration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Duration(pub StdDuration);

impl Duration {
	pub fn duration(&self) -> StdDuration {
		self.0
	}
}

impl From<StdDuration> for Duration {
	fn from(duration: StdDuration) -> Self {
		Self(duration)
	}
}

impl From<Duration> for StdDuration {
	fn from(duration: Duration) -> Self {
		duration.0
	}
}

impl Duration {
	pub fn new_test() -> Self {
		Self(StdDuration::from_secs(0))
	}
}
