use crate::long_id::{LongId, LongIdError};
use serde::{Serialize, Deserialize};

/// The id of a task.
///
/// This is the id of the task.
/// It is used to identify the task and to display it in the UI.
/// It is also used to search for the task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Id(LongId);

impl Id {
    /// Creates a new Id from a byte array.
    pub fn new(bytes: [u8; 512]) -> Self {
        Self(LongId::new(bytes))
    }

    /// Creates a new Id from a string, padding with zeros if necessary.
    /// Returns an error if the string is longer than 512 bytes.
    pub fn from_string(s: &str) -> Result<Self, LongIdError> {
        Ok(Self(LongId::from_string(s)?))
    }

    /// Creates a new Id from a string, truncating if necessary.
    /// This method never fails but may lose data for strings longer than 512 bytes.
    pub fn from_string_truncate(s: &str) -> Self {
        Self(LongId::from_string_truncate(s))
    }

    /// Returns the raw byte array.
    pub fn as_bytes(&self) -> &[u8; 512] {
        self.0.as_bytes()
    }

    /// Converts to a string, trimming null bytes.
    pub fn to_string_lossy(&self) -> String {
        self.0.to_string_lossy()
    }
}