pub mod id;
pub use id::Id;
use crate::short_id::ShortIdError;
use crate::task::id::Id as TaskId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Dependency {
    pub id: Id,
}

impl Dependency {

    pub fn new(from: TaskId, to: TaskId) -> Self {
        Self { id: Id::new(from, to) }
    }
    
    pub fn test_from_id(from: u8, to: u8) -> Result<Self, ShortIdError> {
        Ok(Self { id: Id::from_u8(from, to) })
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}