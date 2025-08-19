use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusDate {
    pub date: DateTime<Utc>,
}

/// The status of a subtask.
///
/// This is the status of the subtask.
/// It is used to display the subtask in the UI.
/// It is also used to search for the subtask.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Incomplete,
    InProgress, 
    Complete(StatusDate),
}