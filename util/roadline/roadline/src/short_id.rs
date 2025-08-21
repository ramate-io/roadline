use serde::{Deserialize, Serialize};

/// Errors thrown by the [ShortId].
#[derive(Debug, thiserror::Error)]
pub enum ShortIdError {
	#[error("ShortId internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("ShortId invalid UTF-8: {0}")]
	InvalidUtf8(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Too long: {0}")]
    TooLong(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A long id is a 512 byte id.
///
/// This is used to identify a task or dependency.
/// It is also used to search for the task or dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ShortId(u8);

impl ShortId {
    /// Creates a new ShortId from a byte array.
    pub fn new(byte: u8) -> Self {
        Self(byte)
    }
    /// Creates a new test ShortId.
    pub fn new_test() -> Self {
        Self(0)
    }
}

impl From<u8> for ShortId {
    fn from(byte: u8) -> Self {
        Self::new(byte)
    }
}

impl From<ShortId> for u8 {
    fn from(id: ShortId) -> Self {
        id.0
    }
}