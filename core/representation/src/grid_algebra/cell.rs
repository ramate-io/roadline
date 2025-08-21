use serde::{Deserialize, Serialize};
use crate::grid_algebra::{lane::LaneId, stretch::Stretch};

/// A cell in the grid. 
/// 
/// The cell is the information defining placement in the grid.
/// Note that this is effectively a sparse matrix approach, 
/// otherwise, we would actually store a full matrix of cells representing units. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Cell {
    stretch: Stretch,
    lane: LaneId,
}

impl Cell {
    pub fn new(stretch: Stretch, lane: LaneId) -> Self {
        Self { stretch, lane }
    }

    pub fn stretch(&self) -> &Stretch {
        &self.stretch
    }

    pub fn lane(&self) -> &LaneId {
        &self.lane
    }

    pub fn lane_id(&self) -> u8 {
        self.lane.into()
    }
}
