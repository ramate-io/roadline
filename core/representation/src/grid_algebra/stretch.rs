use serde::{Deserialize, Serialize};

/// The unitless range of a stretch. 
/// 
/// This is used to represent the range of a stretch in a grid. 
/// 
/// The start and end are the indices of the lanes that the stretch spans. 
/// 
/// The start is inclusive and the end is exclusive. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StretchRange {
    start: u8,
    end: u8,
}

/// The stretch unit is the unit of time that the stretch is measured in. 
/// 
/// The u64 value within a stretch variant is the number of seconds in the stretch. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StretchUnit {
    Days = 86400,
    Weeks = 604800,
    BiWeeks = 1209600,
    Months = 2629746,
    BiMonths = 5259492,
    Quarters = 7889238,
    BiQuarters = 15778476,
    Years = 31556952,
    BiYears = 63113904,
}

impl StretchUnit {
    pub fn seconds(&self) -> u64 {
        *self as u64
    }

    /// Matches the closest stretch unit less than the average number of seconds. 
    pub fn from_average_seconds(average_seconds: u64) -> Self {
        let mut best_unit = Self::Days;
        let mut best_diff = average_seconds - Self::Days.seconds();

        for unit in [Self::Weeks, Self::BiWeeks, Self::Months, Self::BiMonths, Self::Quarters, Self::BiQuarters, Self::Years, Self::BiYears] {
            let diff = average_seconds - unit.seconds();
            if diff.abs() < best_diff.abs() {
                best_unit = unit;
                best_diff = diff;
            }
        }

        best_unit
    }
}


/// The stretch with a range and unit. 
/// 
/// Generally, the unit should only be stored in one place, i.e., on the grid_algebra::Grid. 
/// Then, we a stretch is read, it is added to StretchRange with the unit. 
/// 
/// This should be fine to copy because the StretchRange should fit within a machine word. 
/// No benefit to borrows. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Stretch {
    range: StretchRange,
    unit: StretchUnit,
}

impl Stretch {
    pub fn new(range: StretchRange, unit: StretchUnit) -> Self {
        Self { range, unit }
    }
}