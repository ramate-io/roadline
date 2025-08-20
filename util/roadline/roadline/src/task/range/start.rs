use super::TargetDate;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Start(TargetDate);

impl From<TargetDate> for Start {
    fn from(target_date: TargetDate) -> Self {
        Self(target_date)
    }
}

impl From<Start> for TargetDate {
    fn from(start: Start) -> Self {
        start.0
    }
}

impl Start {
    pub fn new_test() -> Self {
        Self(TargetDate::new_test())
    }
}