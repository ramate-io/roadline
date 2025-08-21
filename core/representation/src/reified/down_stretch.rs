use crate::grid_algebra::stretch::Stretch;
use super::reified_unit::ReifiedUnit;
use serde::{Deserialize, Serialize};


/// The range of the down stretch. 
/// 
/// This is u16 because the stretch is u8, so we need extra space to store the range in down units. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DownStretchRange {
    start: ReifiedUnit,
    end: ReifiedUnit,
}

impl DownStretchRange {
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

/// The trim is the number of down units to trim from the stretch. 
/// 
/// This is u16 because the stretch is u8, so we need extra space to store the trim. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Trim(ReifiedUnit);

impl Trim {
    pub fn new(trim: ReifiedUnit) -> Self {
        Self(trim)
    }

    pub fn value(&self) -> ReifiedUnit {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DownStretch {
    /// The original stretch. 
    stretch: Stretch,
    /// The trim used to compute the down stretch. 
    trim: Trim,
    /// The down stretch. 
    down_stretch: DownStretchRange,
}

impl DownStretch {
    pub fn new(stretch: Stretch, trim: Trim, down_stretch: DownStretchRange) -> Self {
        Self { stretch, trim, down_stretch }
    }

    pub fn canonical_from_stretch(stretch: Stretch, trim: Trim) -> Self {
        let unit = stretch.unit().down(1);
        let (start, end) = stretch.scale(unit);

        // subtract the trim from the end
        let end = end as u16 - trim.value().value();

        let down_stretch = DownStretchRange::new(
            ReifiedUnit::new(start as u16),
            ReifiedUnit::new(end as u16),
        );

        Self::new(stretch, trim, down_stretch)
    }

    pub fn stretch(&self) -> &Stretch {
        &self.stretch
    }

    pub fn trim(&self) -> &Trim {
        &self.trim
    }

    pub fn down_stretch(&self) -> &DownStretchRange {
        &self.down_stretch
    }

    /// Get the connection point at the end of this stretch (right edge, for outgoing connections)
    pub fn outgoing_connection_point(&self) -> ReifiedUnit {
        self.down_stretch.end()
    }

    /// Get the connection point at the start of this stretch (left edge, for incoming connections)
    pub fn incoming_connection_point(&self) -> ReifiedUnit {
        self.down_stretch.start()
    }
}