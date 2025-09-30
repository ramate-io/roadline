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
	/// * `starts` - Required start date expression (e.g., "T0 + 0 months")
	/// * `ends` - Required end date expression (e.g., "1 month")
	/// * `task_id` - The task ID for reference
	///
	/// # Returns
	/// A `Range` object with parsed start and end dates.
	///
	/// # Errors
	/// Returns an error if either start or end date is missing.
	pub fn parse(
		&self,
		starts: Option<&str>,
		ends: Option<&str>,
		_task_id: &TaskId,
	) -> Result<Range, MarkdownParseError> {
		// Parse the start date - required
		let start = match starts {
			Some(starts) => self.start_parser.parse(starts)?,
			None => {
				return Err(MarkdownParseError::InvalidDateExpression {
					expression: "Missing start date".to_string(),
				});
			}
		};

		// Parse the end date - required
		let end = match ends {
			Some(ends) => self.end_parser.parse(ends)?,
			None => {
				return Err(MarkdownParseError::InvalidDateExpression {
					expression: "Missing end date".to_string(),
				});
			}
		};

		Ok(Range::new(start, end))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_with_both_dates() -> Result<(), MarkdownParseError> {
		let parser = RangeParser::new();
		let result = parser.parse(Some("T0 + 0 months"), Some("1 month"), &TaskId::new(1))?;
		assert!(result.start().point_of_reference().0.value() == 0);
		Ok(())
	}

	#[test]
	fn test_parse_missing_start_date() {
		let parser = RangeParser::new();
		let result = parser.parse(None, Some("1 month"), &TaskId::new(1));
		assert!(result.is_err());
		if let Err(MarkdownParseError::InvalidDateExpression { expression }) = result {
			assert_eq!(expression, "Missing start date");
		} else {
			panic!("Expected InvalidDateExpression error");
		}
	}

	#[test]
	fn test_parse_missing_end_date() {
		let parser = RangeParser::new();
		let result = parser.parse(Some("T0 + 0 months"), None, &TaskId::new(1));
		assert!(result.is_err());
		if let Err(MarkdownParseError::InvalidDateExpression { expression }) = result {
			assert_eq!(expression, "Missing end date");
		} else {
			panic!("Expected InvalidDateExpression error");
		}
	}

	#[test]
	fn test_parse_missing_both_dates() {
		let parser = RangeParser::new();
		let result = parser.parse(None, None, &TaskId::new(1));
		assert!(result.is_err());
		if let Err(MarkdownParseError::InvalidDateExpression { expression }) = result {
			assert_eq!(expression, "Missing start date");
		} else {
			panic!("Expected InvalidDateExpression error");
		}
	}
}
