//! End date parsing functionality for markdown roadmap documents.

use super::super::error::MarkdownParseError;
use roadline_util::duration::Duration;
use roadline_util::task::range::End;
use std::time::Duration as StdDuration;

/// Represents different end date formats for backward compatibility.
#[derive(Debug, Clone, PartialEq)]
pub enum EndDateFormat {
	/// New format: just a duration (e.g., "1 month")
	Duration(StdDuration),
	/// Old format: task reference + duration (e.g., "T1 + 1 month")
	TaskReference(String, StdDuration),
}

/// Parser for end date expressions in markdown documents.
#[derive(Debug, Clone)]
pub struct EndDate {
	// Configuration for end date parsing
}

impl Default for EndDate {
	fn default() -> Self {
		Self::new()
	}
}

impl EndDate {
	/// Create a new end date parser.
	pub fn new() -> Self {
		Self {}
	}

	/// Parse an end date expression from a string.
	///
	/// Expected formats:
	/// - "T1 + 1 month" (old format: relative to task start)
	/// - "1 month" (new format: duration from start)
	pub fn parse(&self, expression: &str) -> Result<End, MarkdownParseError> {
		let format = self.parse_format(expression)?;

		match format {
			EndDateFormat::Duration(duration) => Ok(End::from(Duration::from(duration))),
			EndDateFormat::TaskReference(_task_ref, duration) => {
				// For old format, just extract the duration
				Ok(End::from(Duration::from(duration)))
			}
		}
	}

	/// Parse an end date expression into the appropriate format.
	fn parse_format(&self, expression: &str) -> Result<EndDateFormat, MarkdownParseError> {
		let expression = expression.trim();

		if expression.starts_with('T') && expression.contains(" + ") {
			// Old format: "T1 + 1 month"
			let parts: Vec<&str> = expression.split(" + ").collect();
			if parts.len() == 2 {
				let task_ref = parts[0].trim().to_string();
				let duration_str = parts[1].trim();
				let duration = self.parse_duration(duration_str)?;
				return Ok(EndDateFormat::TaskReference(task_ref, duration));
			}
		} else if expression.chars().any(|c| c.is_alphabetic()) && !expression.contains('-') {
			// New format: "1 month", "2 weeks", etc. (contains letters but no dashes)
			let duration = self.parse_duration(expression)?;
			return Ok(EndDateFormat::Duration(duration));
		}

		Err(MarkdownParseError::InvalidDateExpression { expression: expression.to_string() })
	}

	/// Parse a duration expression into a standard duration.
	fn parse_duration(&self, expression: &str) -> Result<StdDuration, MarkdownParseError> {
		let expression = expression.trim().to_lowercase();

		if expression.ends_with("month") || expression.ends_with("months") {
			let number = self.extract_number(&expression)?;
			Ok(StdDuration::from_secs(86400 * 30 * number as u64))
		} else if expression.ends_with("week") || expression.ends_with("weeks") {
			let number = self.extract_number(&expression)?;
			Ok(StdDuration::from_secs(86400 * 7 * number as u64))
		} else if expression.ends_with("day") || expression.ends_with("days") {
			let number = self.extract_number(&expression)?;
			Ok(StdDuration::from_secs(86400 * number as u64))
		} else {
			Err(MarkdownParseError::InvalidDurationExpression {
				expression: expression.to_string(),
			})
		}
	}

	/// Extract a number from the beginning of a string.
	fn extract_number(&self, s: &str) -> Result<u32, MarkdownParseError> {
		let mut number_str = String::new();

		for ch in s.chars() {
			if ch.is_ascii_digit() {
				number_str.push(ch);
			} else {
				break;
			}
		}

		if number_str.is_empty() {
			return Err(MarkdownParseError::InvalidDurationExpression {
				expression: s.to_string(),
			});
		}

		number_str.parse().map_err(|_| MarkdownParseError::InvalidDurationExpression {
			expression: s.to_string(),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_end_date_parsing() {
		let parser = EndDate::new();

		// Test new format
		let result = parser.parse("1 month").unwrap();
		assert_eq!(result.duration().0.as_secs(), 86400 * 30);

		// Test old format
		let result = parser.parse("T1 + 1 month").unwrap();
		assert_eq!(result.duration().0.as_secs(), 86400 * 30);
	}

	#[test]
	fn test_duration_parsing() {
		let parser = EndDate::new();

		// Test various duration formats
		let result = parser.parse_duration("1 month").unwrap();
		assert_eq!(result.as_secs(), 86400 * 30);

		let result = parser.parse_duration("2 weeks").unwrap();
		assert_eq!(result.as_secs(), 86400 * 14);

		let result = parser.parse_duration("30 days").unwrap();
		assert_eq!(result.as_secs(), 86400 * 30);
	}
}
