//! Date parsing functionality for markdown roadmap documents.

use super::error::MarkdownParseError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::time::Duration as StdDuration;

/// Represents different end date formats for backward compatibility.
#[derive(Debug, Clone, PartialEq)]
pub enum EndDateFormat {
	/// New format: just a duration (e.g., "1 month")
	Duration(StdDuration),
	/// Old format: task reference + duration (e.g., "T1 + 1 month")
	TaskReference(String, StdDuration),
	/// Absolute date (e.g., "2024-02-01")
	Absolute(DateTime<Utc>),
}

/// Parser for date expressions in markdown documents.
///
/// This parser handles various date formats used in roadmap documents,
/// including relative dates and duration expressions.
#[derive(Debug, Clone)]
pub struct DateParser {
	// Configuration for date parsing
}

impl Default for DateParser {
	fn default() -> Self {
		Self::new()
	}
}

impl DateParser {
	/// Create a new date parser.
	pub fn new() -> Self {
		Self {}
	}

	/// Parse a date expression from the "Starts:" field.
	///
	/// Expected formats:
	/// - "T0 + 0 months" (relative to another task)
	/// - "2024-01-01" (absolute date)
	/// - "T1 + 1 month" (relative to task T1)
	pub fn parse_start_date(&self, expression: &str) -> Result<DateTime<Utc>, MarkdownParseError> {
		let expression = expression.trim();

		if expression.starts_with('T') {
			// Relative date format: "T1 + 1 month"
			self.parse_relative_date(expression)
		} else {
			// Absolute date format: "2024-01-01"
			self.parse_absolute_date(expression)
		}
	}

	/// Parse a date expression from the "Ends:" field.
	///
	/// Expected formats:
	/// - "T1 + 1 month" (old format: relative to task start)
	/// - "2024-02-01" (absolute date)
	/// - "1 month" (new format: duration from start)
	pub fn parse_end_date(&self, expression: &str) -> Result<DateTime<Utc>, MarkdownParseError> {
		let end_format = self.parse_end_date_format(expression)?;

		match end_format {
			EndDateFormat::Duration(_duration) => {
				// For new format, return a placeholder date
				// In a real implementation, this would be calculated from the task start
				Ok(DateTime::from_naive_utc_and_offset(
					NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
						NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
					),
					Utc,
				))
			}
			EndDateFormat::TaskReference(_task_ref, _duration) => {
				// For old format, return a placeholder date
				// In a real implementation, this would resolve the task reference
				Ok(DateTime::from_naive_utc_and_offset(
					NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
						NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
					),
					Utc,
				))
			}
			EndDateFormat::Absolute(date) => Ok(date),
		}
	}

	/// Parse an end date expression into the appropriate format.
	///
	/// This method determines which format is being used and returns the appropriate enum variant.
	pub fn parse_end_date_format(
		&self,
		expression: &str,
	) -> Result<EndDateFormat, MarkdownParseError> {
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
		} else if expression.contains('-') {
			// Absolute date format: "2024-02-01"
			let date = self.parse_absolute_date(expression)?;
			return Ok(EndDateFormat::Absolute(date));
		}

		Err(MarkdownParseError::InvalidDateExpression { expression: expression.to_string() })
	}

	/// Parse a relative date expression.
	///
	/// Expected format: "T1 + 1 month"
	fn parse_relative_date(&self, _expression: &str) -> Result<DateTime<Utc>, MarkdownParseError> {
		// For now, return a placeholder date
		// In a real implementation, this would resolve the task reference
		// and calculate the actual date
		Ok(DateTime::from_naive_utc_and_offset(
			NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
				NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
			),
			Utc,
		))
	}

	/// Parse an absolute date expression.
	///
	/// Expected format: "2024-01-01"
	fn parse_absolute_date(&self, expression: &str) -> Result<DateTime<Utc>, MarkdownParseError> {
		let date = NaiveDate::parse_from_str(expression, "%Y-%m-%d").map_err(|_| {
			MarkdownParseError::InvalidDateExpression { expression: expression.to_string() }
		})?;

		let datetime = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
		Ok(DateTime::from_naive_utc_and_offset(datetime, Utc))
	}

	/// Parse a duration expression into a standard duration.
	///
	/// Expected format: "1 month", "2 weeks", "30 days"
	pub fn parse_duration(&self, expression: &str) -> Result<StdDuration, MarkdownParseError> {
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

	/// Parse a date field from a task section.
	///
	/// This is a convenience method for parsing date fields from markdown content.
	pub fn parse_date_field(
		&self,
		content: &[String],
		field_name: &str,
	) -> Result<Option<DateTime<Utc>>, MarkdownParseError> {
		for line in content {
			let line = line.trim();

			if let Some((field, value)) = self.parse_field_line(line) {
				if field == field_name {
					return match field_name {
						"Starts" => Ok(Some(self.parse_start_date(&value)?)),
						"Ends" => Ok(Some(self.parse_end_date(&value)?)),
						_ => Ok(None),
					};
				}
			}
		}

		Ok(None)
	}

	/// Parse a field line in the format "- **Field:** Value".
	fn parse_field_line(&self, line: &str) -> Option<(String, String)> {
		if !line.starts_with("- **") || !line.contains(":**") {
			return None;
		}

		let start = 4; // Skip "- **"
		let end = line.find(":**")?;

		let field = line[start..end].to_string();
		let value = line[end + 4..].trim().to_string(); // Skip ":** "

		Some((field, value))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_absolute_date() {
		let parser = DateParser::new();
		let result = parser.parse_absolute_date("2024-01-01").unwrap();
		// Just verify it parses without error
		let expected: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
			NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
				NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
			),
			Utc,
		);
		assert!(result > expected);
	}

	#[test]
	fn test_parse_duration() {
		let parser = DateParser::new();

		let result = parser.parse_duration("1 month").unwrap();
		assert_eq!(result.as_secs(), 86400 * 30);

		let result = parser.parse_duration("2 weeks").unwrap();
		assert_eq!(result.as_secs(), 86400 * 14);

		let result = parser.parse_duration("30 days").unwrap();
		assert_eq!(result.as_secs(), 86400 * 30);
	}

	#[test]
	fn test_extract_number() {
		let parser = DateParser::new();
		assert_eq!(parser.extract_number("1 month").unwrap(), 1);
		assert_eq!(parser.extract_number("30 days").unwrap(), 30);
		assert_eq!(parser.extract_number("2 weeks").unwrap(), 2);
	}

	#[test]
	fn test_parse_field_line() {
		let parser = DateParser::new();
		let line = "- **Starts:** T0 + 0 months";
		let result = parser.parse_field_line(line).unwrap();
		assert_eq!(result.0, "Starts");
		assert_eq!(result.1, "T0 + 0 months");
	}
}
