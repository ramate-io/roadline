use super::date::Date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Start(Date);

impl Start {
    pub fn new(date: Date) -> Self {
        Self(date)
    }
}