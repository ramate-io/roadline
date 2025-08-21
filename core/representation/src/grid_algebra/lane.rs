use roadline_util::short_id::ShortId;
use serde::{Deserialize, Serialize};

/// The identifier for a lane in the grid. 
/// 
/// We use a ShortId for this because it is the same size as the maximum number of tasks,
/// thus the same size as the maximum number of lanes. 
/// 
/// ShortId is also still positional, so it can be used as a lane index. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LaneId(ShortId);

impl LaneId {
    pub fn new(id: ShortId) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u8 {
        self.0.into()
    }
}

impl From<u8> for LaneId {
    fn from(id: u8) -> Self {
        Self(ShortId::new(id))
    }
}

impl From<LaneId> for u8 {
    fn from(id: LaneId) -> Self {
        id.0.into()
    }
}