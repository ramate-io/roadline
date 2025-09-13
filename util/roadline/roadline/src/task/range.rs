pub mod end;
pub mod point_of_reference;
pub mod start;
pub mod target_date;

pub use end::End;
pub use point_of_reference::PointOfReference;
pub use start::Start;
pub use target_date::TargetDate;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range {
	pub start: Start,
	pub end: End,
}

impl Range {
	pub fn new(start: Start, end: End) -> Self {
		Self { start, end }
	}

	pub fn new_test() -> Self {
		Self { start: Start::new_test(), end: End::new_test() }
	}

	pub fn start(&self) -> &Start {
		&self.start
	}

	pub fn end(&self) -> &End {
		&self.end
	}
}
