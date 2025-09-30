//! Range parsing functionality for markdown roadmap documents.

pub mod end;
pub mod start;

pub use end::EndDate;
pub use start::StartDate;

use super::error::MarkdownParseError;
use roadline_util::task::{range::Range, Id as TaskId};

/// Parser for task ranges in markdown documents.
///
/// This parser handles the parsing of both start and end dates
/// to create a complete Range object.
#[derive(Debug, Clone)]
pub struct RangeParser {
	start_parser: StartDate,
	end_parser: EndDate,
}

impl Default for RangeParser {
	fn default() -> Self {
		Self::new()
	}
}

impl RangeParser {
	/// Create a new range parser.
	pub fn new() -> Self {
		Self { start_parser: StartDate::new(), end_parser: EndDate::new() }
	}

	/// Parse a complete range from start and end date expressions.
	///
	/// # Arguments
	/// * `starts` - Optional start date expression (e.g., "T0 + 0 months")
	/// * `ends` - Optional end date expression (e.g., "1 month")
	/// * `task_id` - The task ID for default start reference
	///
	/// # Returns
	/// A `Range` object with parsed start and end dates.
	pub fn parse(
		&self,
		starts: Option<&str>,
		ends: Option<&str>,
		_task_id: &TaskId,
	) -> Result<Range, MarkdownParseError> {
		// Parse the start date
		let start = if let Some(starts) = starts {
			self.start_parser.parse(starts)?
		} else {
			// Default start: T0 + 0 months
			self.start_parser.parse("T0 + 0 months")?
		};

		// Parse the end date
		let end = if let Some(ends) = ends {
			self.end_parser.parse(ends)?
		} else {
			// Default end: 1 month duration
			self.end_parser.parse("1 month")?
		};

		Ok(Range::new(start, end))
	}
}
