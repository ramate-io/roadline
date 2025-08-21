use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use super::super::RangeAlgebraError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Date(DateTime<Utc>);

impl Date {
    pub fn new(date: DateTime<Utc>) -> Self {
        Self(date)
    }
    
    /// Returns the inner DateTime<Utc> value.
    pub fn inner(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn start_of_epoch() -> Result<Self, RangeAlgebraError> {
        let date = NaiveDate::from_ymd_opt(1970, 1, 1).ok_or(RangeAlgebraError::InvalidDate { date: "1970-01-01".to_string() })?;
        let time = NaiveTime::from_hms_opt(0, 0, 0).ok_or(RangeAlgebraError::InvalidDate { date: "1970-01-01".to_string() })?;
        let date_time = NaiveDateTime::new(date, time);
        Ok(Self::new(DateTime::from_naive_utc_and_offset(date_time, Utc)))
    }
}