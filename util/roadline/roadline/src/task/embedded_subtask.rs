use serde::{Deserialize, Serialize};
use super::Subtask;

use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct EmbeddedSubtask(Subtask);

impl EmbeddedSubtask {
    pub fn new(subtask: Subtask) -> Self {
        Self(subtask)
    }

    pub fn subtask(&self) -> &Subtask {
        &self.0
    }
}

/// We now implement a comparison stack wherein position and id form the primary key.
/// We further order by poisition first and then by id.
impl PartialEq for EmbeddedSubtask {
    fn eq(&self, other: &Self) -> bool {
        self.0.position() == other.0.position() && self.0.id() == other.0.id()
    }
}

impl Eq for EmbeddedSubtask {}

impl PartialOrd for EmbeddedSubtask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.0.position(), self.0.id()).cmp(&(other.0.position(), other.0.id())))
    }
}

impl Ord for EmbeddedSubtask {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0.position(), self.0.id()).cmp(&(other.0.position(), other.0.id()))
    }
}