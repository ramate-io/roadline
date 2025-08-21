use serde::{Deserialize, Serialize};

use super::{down_lane::DownLane, down_stretch::DownStretch};
use crate::grid_algebra::cell::Cell;

/// The down cell. 
/// 
/// This is the cell in the down lane. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DownCell {
    /// The original cell. 
    cell: Cell,
    /// The down lane. 
    down_lane: DownLane,
    /// The down stretch. 
    down_stretch: DownStretch,
}

impl DownCell {
    pub fn new(cell: Cell, down_lane: DownLane, down_stretch: DownStretch) -> Self {
        Self { cell, down_lane, down_stretch }
    }
}