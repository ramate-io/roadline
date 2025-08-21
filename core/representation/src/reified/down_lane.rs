use serde::{Deserialize, Serialize};
use crate::grid_algebra::lane::LaneId;
use super::reified_unit::ReifiedUnit;

/// The padding of the down lane. 
/// 
/// This is u16 because the lane is u8, so we need extra space to store the padding in down units. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DownLanePadding(ReifiedUnit);

impl DownLanePadding {
    pub fn new(padding: ReifiedUnit) -> Self {
        Self(padding)
    }

    pub fn value(&self) -> ReifiedUnit {
        self.0
    }
}

/// The range of the down lane. 
/// 
/// This is u16 because the lane is u8, so we need extra space to store the range in down units. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DownLaneRange {
    start: ReifiedUnit,
    end: ReifiedUnit,
}

/// The down lane. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DownLane {
    /// The original lane. 
    lane_id: LaneId,
    /// The padding of the down lane. 
    padding: DownLanePadding,
    /// The range of the down lane. 
    range: DownLaneRange,
}

impl DownLane {
    pub fn new(lane_id: LaneId, padding: DownLanePadding, range: DownLaneRange) -> Self {
        Self { lane_id, padding, range }
    }

    pub fn canonical_from_lane(lane: LaneId, padding: DownLanePadding) -> Self {
    
        // The lane new start range should be (2 + paddding) * lane_id
        let new_start = ReifiedUnit::new((2 + padding.value().value()) * lane.value() as u16);

        // The lane new end range should be (2 + paddding) * lane_id + 2
        let new_end = ReifiedUnit::new((2 + padding.value().value()) * (lane.value() as u16 + 1));

        let range = DownLaneRange {
            start: new_start,
            end: new_end,
        };

        Self::new(lane, padding, range)
    }

    pub fn lane_id(&self) -> LaneId {
        self.lane_id
    }

    pub fn padding(&self) -> &DownLanePadding {
        &self.padding
    }

    pub fn range(&self) -> &DownLaneRange {
        &self.range
    }

    /// Get the midpoint of this lane (for connection points)
    pub fn midpoint(&self) -> ReifiedUnit {
        let start_val = self.range.start.value();
        let end_val = self.range.end.value();
        ReifiedUnit::new((start_val + end_val) / 2)
    }
}

impl DownLaneRange {
    pub fn new(start: ReifiedUnit, end: ReifiedUnit) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> ReifiedUnit {
        self.start
    }

    pub fn end(&self) -> ReifiedUnit {
        self.end
    }
}