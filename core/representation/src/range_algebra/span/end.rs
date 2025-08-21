use super::date::Date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct End(Date);

impl End {
    pub fn new(date: Date) -> Self {
        Self(date)
    }
    
    /// Returns the inner Date value.
    pub fn inner(&self) -> Date {
        self.0
    }
}