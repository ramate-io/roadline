use crate::task::id::Id as TaskId;
use serde::{Deserialize, Serialize};

/// The id of a dependency.
///
/// This is the id of the dependency.
/// It is used to identify the dependency and to display it in the UI.
/// It is also used to search for the task.
///
/// A depdenency is identified by the id of the from task and the id of the to task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Id {
	from: TaskId,
	to: TaskId,
}

impl Id {
	/// Creates a new Id from a byte array.
	pub fn new(from: TaskId, to: TaskId) -> Self {
		Self { from, to }
	}

	/// Creates a new id from a u8
	pub fn from_u8(from: u8, to: u8) -> Self {
		Self { from: TaskId::new(from), to: TaskId::new(to) }
	}

	/// Creates a new test Id.
	pub fn new_test() -> Self {
		Self { from: TaskId::new_test(), to: TaskId::new_test() }
	}

	/// Gets the "from" task ID
	pub fn from(&self) -> TaskId {
		self.from
	}

	/// Gets the "to" task ID
	pub fn to(&self) -> TaskId {
		self.to
	}
}
