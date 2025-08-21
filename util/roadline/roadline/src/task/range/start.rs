use super::{TargetDate, PointOfReference};
use crate::duration::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Start(TargetDate);

impl Start {
    pub fn point_of_reference(&self) -> &PointOfReference {
        self.0.point_of_reference()
    }

    pub fn duration(&self) -> &Duration {
        self.0.duration()
    }
}

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