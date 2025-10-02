pub mod embedded_subtask;
pub mod id;
pub mod range;
pub mod subtask;
pub mod summary;
pub mod title;

pub use embedded_subtask::EmbeddedSubtask;
pub use id::Id;
pub use range::{End, Range, Start, TargetDate};
use std::time::Duration as StdDuration;
pub use subtask::Subtask;
pub use summary::Summary;
pub use title::Title;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::short_id::ShortIdError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Task {
	/// The id of the task is the unique identifier of the task.
	pub id: Id,
	/// The title of the task is the main title of the task.
	pub title: Title,
	/// Which tasks the task depends on.
	pub depends_on: BTreeSet<Id>,
	/// The subtasks of the task are a small finite set of subtasks and is non-recursive.
	///
	/// The should be embedded within the task structure.
	/// There is no need to have a separate relational structure for subtasks.
	///
	/// This is [BTreeSet] because subtasks are placed in a user-defined order which is computed based on the position of the subtask.
	pub subtasks: BTreeSet<EmbeddedSubtask>,
	/// The summary of the task is a short summary of the task and its subtasks.
	pub summary: Summary,
	/// The range of the task is the start and end date of the task.
	pub range: Range,
}

impl Task {
	pub fn new(
		id: Id,
		title: Title,
		depends_on: BTreeSet<Id>,
		subtasks: BTreeSet<EmbeddedSubtask>,
		summary: Summary,
		range: Range,
	) -> Self {
		Self { id, title, depends_on, subtasks, summary, range }
	}

	/// Creates a new test task.
	pub fn new_test() -> Self {
		Self {
			id: Id::new_test(),
			title: Title::new_test(),
			depends_on: BTreeSet::new(),
			subtasks: BTreeSet::new(),
			summary: Summary::new_test(),
			range: Range::new_test(),
		}
	}

	/// Constructs with a specified id.
	pub fn with_id(self, id: Id) -> Self {
		Self { id, ..self }
	}

	/// Constructs with a specified set of dependencies
	///
	/// Takes an arbitrary iterator of ids.
	pub fn with_dependencies(mut self, deps: impl IntoIterator<Item = impl Into<Id>>) -> Self {
		self.dependencies_mut()
			.extend(deps.into_iter().map(|dep| dep.into()).collect::<Vec<Id>>());
		self
	}

	/// Constructs a task to start after a certain dependency ends
	///
	/// The start date of the task will be based on the end duration of the other task.
	pub fn after(self, other: &Self) -> Self {
		let start = Start::from(TargetDate {
			point_of_reference: other.id.into(),
			duration: other.range.end.duration().clone(),
		});

		let range = Range::new(start, self.range.end);

		Self { range, ..self }
	}

	/// Adds to the existing offset of the start date of the task
	///
	/// The start date of the task will be based on the existing offset of the other task.
	pub fn offset_start_date(self, offset: StdDuration) -> Self {
		let start = Start::from(TargetDate {
			point_of_reference: self.range.start.point_of_reference().clone(),
			duration: std::time::Duration::from(std::time::Duration::from_secs(
				self.range.start.duration().0.as_secs() + offset.as_secs(),
			))
			.into(),
		});

		let range = Range::new(start, self.range.end);

		Self { range, ..self }
	}

	/// Constructs a task to start after the dates of all dependencies
	pub fn for_standard_duration(self, duration: StdDuration) -> Self {
		let range = Range::new(self.range.start, duration.into());
		Self { range, ..self }
	}

	/// Creates a new test task from a string id.
	pub fn test_from_id(id: u8) -> Result<Self, ShortIdError> {
		Ok(Self::new_test().with_id(Id::new(id)))
	}

	/// Borrow the [EmbeddedSubtask]s set as a vector of [&Subtask]s.
	pub fn subtasks(&self) -> Vec<&Subtask> {
		self.subtasks.iter().map(|subtask| subtask.subtask()).collect()
	}

	pub fn summary(&self) -> &Summary {
		&self.summary
	}

	pub fn id(&self) -> &Id {
		&self.id
	}

	pub fn id_mut(&mut self) -> &mut Id {
		&mut self.id
	}

	pub fn title(&self) -> &Title {
		&self.title
	}

	pub fn title_mut(&mut self) -> &mut Title {
		&mut self.title
	}

	pub fn depends_on(&self) -> &BTreeSet<Id> {
		&self.depends_on
	}

	pub fn depends_on_mut(&mut self) -> &mut BTreeSet<Id> {
		&mut self.depends_on
	}

	pub fn dependencies(&self) -> &BTreeSet<Id> {
		&self.depends_on
	}

	pub fn dependencies_mut(&mut self) -> &mut BTreeSet<Id> {
		&mut self.depends_on
	}

	pub fn subtasks_mut(&mut self) -> &mut BTreeSet<EmbeddedSubtask> {
		&mut self.subtasks
	}

	pub fn summary_mut(&mut self) -> &mut Summary {
		&mut self.summary
	}

	pub fn range(&self) -> &Range {
		&self.range
	}

	pub fn range_mut(&mut self) -> &mut Range {
		&mut self.range
	}

	pub fn is_root(&self) -> bool {
		self.depends_on.is_empty()
	}
}
