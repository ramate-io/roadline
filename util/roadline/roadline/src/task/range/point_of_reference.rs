use crate::task::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointOfReference(pub Id);

impl From<Id> for PointOfReference {
	fn from(id: Id) -> Self {
		Self(id)
	}
}

impl From<PointOfReference> for Id {
	fn from(point_of_reference: PointOfReference) -> Self {
		point_of_reference.0
	}
}

impl PointOfReference {
	pub fn new_test() -> Self {
		Self(Id::new_test())
	}
}
