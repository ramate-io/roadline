use serde::{Deserialize, Serialize};

/// The id of a subtask.
///
/// This is the id of the subtask.
/// It is used to identify the subtask and to display it in the UI.
/// It is also used to search for the subtask.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Id(String);

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}