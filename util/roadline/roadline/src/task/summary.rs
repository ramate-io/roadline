use serde::{Deserialize, Serialize};

/// The summary of a task.
///
/// This is a short summary of the task.
/// It is used to display the task in the UI.
/// It is also used to search for the task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Summary {
    pub text: String,
}

impl AsRef<str> for Summary {
    fn as_ref(&self) -> &str {
        &self.text
    }
}