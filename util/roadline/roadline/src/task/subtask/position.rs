use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(u8);

impl Position {
    pub fn new(index: u8) -> Self {
        Self(index)
    }

    pub fn index(&self) -> u8 {
        self.0
    }
}