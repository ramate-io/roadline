use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ReifiedUnit(u16);

impl ReifiedUnit {
    pub fn new(position: u16) -> Self {
        Self(position)
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}