use serde::{Deserialize, Serialize};

/// The title of a task.
///
/// This is the main title of the task.
/// It is used to identify the task and to display it in the UI.
/// It is also used to search for the task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Title {
    pub text: String,
}

/// Implement `AsRef<str>` for `Title` to allow for easy conversion to a string.
impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

impl Title {
    /// Creates a new test title.
    pub fn new_test() -> Self {
        Self { text: "Test Title".to_string() }
    }
}