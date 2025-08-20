use crate::duration::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct End(Duration);

impl From<Duration> for End {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl From<End> for Duration {
    fn from(end: End) -> Self {
        end.0
    }
}

impl End {
    pub fn new_test() -> Self {
        Self(Duration::new_test())
    }
}