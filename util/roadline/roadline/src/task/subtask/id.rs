use crate::short_id::ShortId;
use serde::{Serialize, Deserialize};

/// The id of a subtask.
///
/// This is the id of the subtask.
/// It is used to identify the subtask and to display it in the UI.
/// It is also used to search for the subtask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Id(ShortId);

impl Id {
    /// Creates a new Id from a byte array.
    pub fn new(byte: u8) -> Self {
        Self(ShortId::new(byte))
    }   

}