use roadline_util::dependency::Id;
use serde::{Deserialize, Serialize};
use super::reified_unit::ReifiedUnit;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct JointUnit(ReifiedUnit);

impl JointUnit {
    pub fn new(unit: ReifiedUnit) -> Self {
        Self(unit)
    }

    pub fn value(&self) -> ReifiedUnit {
        self.0
    }
}


/// The joint. 
/// 
/// This is the joint of the down cell and the joint cell. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Joint {
    dependency: Id,
    joint_shape: Vec<JointUnit>,
}