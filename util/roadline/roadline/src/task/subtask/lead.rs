use serde::{Deserialize, Serialize};

/// The lead of a subtask.
///
/// This is the lead of the subtask.
/// It is used to display the subtask in the UI.
/// It is also used to search for the subtask.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lead {
    pub name: String,
    pub email: String,
}

impl Lead {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}