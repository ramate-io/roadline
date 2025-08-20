pub mod id;
pub use id::Id;
use crate::short_id::ShortIdError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Dependency {
    pub id: Id,
}

impl Dependency {
    
    pub fn test_from_id(id: u8) -> Result<Self, ShortIdError> {
        Ok(Self { id: Id::new(id) })
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}