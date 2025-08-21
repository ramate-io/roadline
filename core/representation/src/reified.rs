pub mod down_stretch;
pub mod down_lane;
pub mod down_cell;
pub mod joint;
pub mod reified_unit;

pub use down_cell::DownCell;
pub use joint::Joint;

use crate::grid_algebra::GridAlgebra;
use roadline_util::dependency::Id as DependencyId;
use roadline_util::task::Id as TaskId;
use std::collections::HashMap;


pub struct Reified {
    grid: GridAlgebra,
    down_cells: HashMap<TaskId, DownCell>,
    joints: HashMap<DependencyId, Joint>,
}