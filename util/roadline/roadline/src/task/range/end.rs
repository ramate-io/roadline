use super::TargetDate;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct End(TargetDate);

impl From<TargetDate> for End {
    fn from(target_date: TargetDate) -> Self {
        Self(target_date)
    }
}

impl From<End> for TargetDate {
    fn from(end: End) -> Self {
        end.0
    }
}

impl End {
    pub fn new_test() -> Self {
        Self(TargetDate::new_test())
    }
}