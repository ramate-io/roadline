pub mod date;
pub mod start;
pub mod end;

pub use date::Date;
pub use start::Start;
pub use end::End;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: Start,
    pub end: End,
}

impl Span {
    pub fn new(start: Start, end: End) -> Self {
        Self { start, end }
    }
}
