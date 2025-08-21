use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Date(DateTime<Utc>);

impl Date {
    pub fn new(date: DateTime<Utc>) -> Self {
        Self(date)
    }
    
    /// Returns the inner DateTime<Utc> value.
    pub fn inner(&self) -> DateTime<Utc> {
        self.0
    }
}