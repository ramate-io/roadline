use serde::{Deserialize, Serialize};

/// The unitless range of a stretch. 
/// 
/// This is used to represent the range of a stretch in a grid. 
/// 
/// The start and end are the indices of the time units that the stretch spans. 
/// 
/// The start is inclusive and the end is exclusive. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StretchRange {
    start: u8,
    end: u8,
}

impl StretchRange {
    pub fn new(start: u8, end: u8) -> Self {
        assert!(start <= end, "StretchRange start must be <= end");
        Self { start, end }
    }

    pub fn start(&self) -> u8 {
        self.start
    }

    pub fn end(&self) -> u8 {
        self.end
    }

    pub fn duration(&self) -> u8 {
        self.end - self.start
    }

    pub fn contains(&self, index: u8) -> bool {
        index >= self.start && index < self.end
    }

    pub fn overlaps(&self, other: &StretchRange) -> bool {
        self.start < other.end && other.start < self.end
    }
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

    /// Matches the closest stretch unit to the average number of seconds.
    /// Prefers smaller units when the difference is equal.
    pub fn from_average_seconds(average_seconds: u64) -> Self {
        let all_units = [
            Self::Days,
            Self::Weeks, 
            Self::BiWeeks, 
            Self::Months, 
            Self::BiMonths, 
            Self::Quarters, 
            Self::BiQuarters, 
            Self::Years, 
            Self::BiYears
        ];

        let mut best_unit = Self::Days;
        let mut best_diff = if average_seconds >= Self::Days.seconds() {
            average_seconds - Self::Days.seconds()
        } else {
            Self::Days.seconds() - average_seconds
        };

        for unit in all_units.iter().skip(1) {
            let diff = if average_seconds >= unit.seconds() {
                average_seconds - unit.seconds()
            } else {
                unit.seconds() - average_seconds
            };
            
            // Prefer this unit if it's closer, or if it's equal distance but smaller
            if diff < best_diff || (diff == best_diff && unit.seconds() < best_unit.seconds()) {
                best_unit = *unit;
                best_diff = diff;
            }
        }

        best_unit
    }

    /// Finds the closest unit that is at most the average duration, then moves to the next smallest.
    /// This ensures the grid has good granularity and readability.
    pub fn canonical_from_average_seconds(average_seconds: u64) -> Self {
        let all_units = [
            Self::Days,
            Self::Weeks, 
            Self::BiWeeks, 
            Self::Months, 
            Self::BiMonths, 
            Self::Quarters, 
            Self::BiQuarters, 
            Self::Years, 
            Self::BiYears
        ];

        // Find the largest unit that is still <= average_seconds
        let mut canonical_unit = Self::Days; // Default fallback
        
        for &unit in &all_units {
            if unit.seconds() <= average_seconds {
                canonical_unit = unit;
            } else {
                break; // Units are ordered, so we can stop here
            }
        }

        // Move to the next smallest unit from the canonical choice
        match canonical_unit {
            Self::BiYears => Self::Years,
            Self::Years => Self::BiQuarters,
            Self::BiQuarters => Self::Quarters,
            Self::Quarters => Self::BiMonths,
            Self::BiMonths => Self::Months,
            Self::Months => Self::BiWeeks,
            Self::BiWeeks => Self::Weeks,
            Self::Weeks => Self::Days,
            Self::Days => Self::Days, // Can't go smaller
        }
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

    pub fn range(&self) -> &StretchRange {
        &self.range
    }

    pub fn unit(&self) -> StretchUnit {
        self.unit
    }

    pub fn start(&self) -> u8 {
        self.range.start()
    }

    pub fn end(&self) -> u8 {
        self.range.end()
    }

    pub fn duration(&self) -> u8 {
        self.range.duration()
    }

    pub fn overlaps(&self, other: &Stretch) -> bool {
        self.unit == other.unit && self.range.overlaps(&other.range)
    }
}