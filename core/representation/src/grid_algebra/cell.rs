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

