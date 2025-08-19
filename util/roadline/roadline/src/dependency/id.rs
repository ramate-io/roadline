use serde::{Deserialize, Serialize};

/// The id of a dependency.
///
/// This is the id of the dependency.
/// It is used to identify the dependency and to display it in the UI.
/// It is also used to search for the dependency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Id(String);

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}