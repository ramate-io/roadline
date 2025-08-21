pub mod lane;
pub mod stretch;
pub mod cell;

pub use lane::LaneId;
pub use stretch::{Stretch, StretchRange, StretchUnit};
pub use cell::Cell;

use crate::range_algebra::RangeAlgebra;
use roadline_util::task::id::Id as TaskId;
use std::collections::HashMap;

pub struct Grid {
    range_algebra: RangeAlgebra,
    tasks: HashMap<TaskId, Cell>,
}

impl Grid {
    pub fn new(range_algebra: RangeAlgebra) -> Self {
        Self { range_algebra, tasks: HashMap::new() }
    }
}