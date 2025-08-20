pub mod summary;
pub mod id;
pub mod subtask;
pub mod title;
pub mod embedded_subtask;

pub use id::Id;
pub use title::Title;
pub use subtask::Subtask;
pub use summary::Summary;
pub use embedded_subtask::EmbeddedSubtask;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::long_id::LongIdError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Task {
    /// The id of the task is the unique identifier of the task.
    pub id: Id,
    /// The title of the task is the main title of the task.
    pub title: Title,
    /// The subtasks of the task are a small finite set of subtasks and is non-recursive.
    /// 
    /// The should be embedded within the task structure.
    /// There is no need to have a separate relational structure for subtasks.
    /// 
    /// This is [BTreeSet] because subtasks are placed in a user-defined order which is computed based on the position of the subtask.
    pub subtasks: BTreeSet<EmbeddedSubtask>,
    /// The summary of the task is a short summary of the task and its subtasks.
    pub summary: Summary,
}

impl Task {
    pub fn new(id: Id, title: Title, subtasks: BTreeSet<EmbeddedSubtask>, summary: Summary) -> Self {
        Self { id, title, subtasks, summary }
    }

    /// Creates a new test task.
    #[cfg(test)]
    pub fn new_test() -> Self {
        Self { id: Id::new_test(), title: Title::new_test(), subtasks: BTreeSet::new(), summary: Summary::new_test() }
    }

    /// Construcs with a specified id.
    pub fn with_id(self, id: Id) -> Self {
        Self { id, ..self }
    }

    pub fn test_from_id_string(id: &str) -> Result<Self, LongIdError> {
        Ok(Self { id: Id::from_string(id)?, title: Title::new_test(), subtasks: BTreeSet::new(), summary: Summary::new_test() })
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

    pub fn title(&self) -> &Title {
        &self.title
    }
}