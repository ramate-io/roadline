use crate::duration::Duration;
use super::PointOfReference;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetDate {
    /// The point of reference (another task).
    pub point_of_reference: PointOfReference,
    /// The duration since the point of reference. 
    pub duration: Duration,
}

impl TargetDate {
    pub fn new_test() -> Self {
        Self { point_of_reference: PointOfReference::new_test(), duration: Duration::new_test() }
    }
}