pub mod id;
pub use id::Id;
use crate::long_id::LongIdError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Dependency {
    pub id: Id,
}

impl Dependency {
    
    pub fn test_from_id_string(id: &str) -> Result<Self, LongIdError> {
        Ok(Self { id: Id::from_string(id)? })
    }
}