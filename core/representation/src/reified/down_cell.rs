use serde::{Deserialize, Serialize};

use super::{down_lane::DownLane, down_stretch::DownStretch, reified_unit::ReifiedUnit};
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

	pub fn cell(&self) -> &Cell {
		&self.cell
	}

	pub fn down_lane(&self) -> &DownLane {
		&self.down_lane
	}

	pub fn down_stretch(&self) -> &DownStretch {
		&self.down_stretch
	}

	/// Get the upper left bound as a reified unit
	pub fn upper_left(&self) -> (ReifiedUnit, ReifiedUnit) {
		(self.down_lane().range().start(), self.down_stretch().range().start())
	}

	/// Get the height of the downcell
	pub fn height(&self) -> ReifiedUnit {
		self.down_lane().height()
	}

	/// Get the width of the downcell
	pub fn width(&self) -> ReifiedUnit {
		self.down_stretch().width()
	}

	/// Get the outgoing connection point (right edge center)
	pub fn outgoing_connection_point(&self) -> (ReifiedUnit, ReifiedUnit) {
		(self.down_stretch.outgoing_connection_point(), self.down_lane.midpoint())
	}

	/// Get the incoming connection point (left edge center)
	pub fn incoming_connection_point(&self) -> (ReifiedUnit, ReifiedUnit) {
		(self.down_stretch.incoming_connection_point(), self.down_lane.midpoint())
	}
}
