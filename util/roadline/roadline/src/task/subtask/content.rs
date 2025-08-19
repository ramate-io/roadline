use serde::{Deserialize, Serialize};

/// The content of a subtask.
///
/// This is the content of the subtask.
/// It is used to display the subtask in the UI.
/// It is also used to search for the task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Content {
    pub text: String,
}

impl AsRef<str> for Content {
    fn as_ref(&self) -> &str {
        &self.text
    }
}