use crate::short_id::ShortId;
use serde::{Serialize, Deserialize};

/// The id of a task.
///
/// This is the id of the task.
/// It is used to identify the task and to display it in the UI.
/// It is also used to search for the task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Id(ShortId);

impl Id {
    /// Creates a new Id from a byte array.
    pub fn new(byte: u8) -> Self {
        Self(ShortId::new(byte))
    }

    pub fn new_test() -> Self {
        Self(ShortId::new_test())
    }
}