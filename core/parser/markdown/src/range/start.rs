//! Start date parsing functionality for markdown roadmap documents.

use super::super::error::MarkdownParseError;
use roadline_util::duration::Duration;
use roadline_util::task::{
	range::{PointOfReference, Start, TargetDate},
	Id as TaskId,
};
use std::time::Duration as StdDuration;

/// Represents different start date formats.
#[derive(Debug, Clone, PartialEq)]
pub enum StartDateFormat {
	/// Relative start: "T1 + 1 month"
	TaskReference(String, StdDuration),
}

/// Parser for start date expressions in markdown documents.
#[derive(Debug, Clone)]
pub struct StartDate {
	// Configuration for start date parsing
}

impl Default for StartDate {
	fn default() -> Self {
		Self::new()
	}
}

impl StartDate {
	/// Create a new start date parser.
	pub fn new() -> Self {
		Self {}
	}

	/// Parse a start date expression from a string.
	///
	/// Expected formats:
	/// - "T0 + 0 months" (relative to another task)
	/// - "T1 + 1 month" (relative to task T1)
	pub fn parse(&self, expression: &str) -> Result<Start, MarkdownParseError> {
		let format = self.parse_format(expression)?;

		match format {
			StartDateFormat::TaskReference(task_ref, duration) => {
				let ref_task_id = self.parse_task_id(&task_ref)?;
				Ok(Start::from(TargetDate {
					point_of_reference: PointOfReference::from(ref_task_id),
					duration: Duration::from(duration),
				}))
			}
		}
	}

	/// Parse a start date expression into the appropriate format.
	fn parse_format(&self, expression: &str) -> Result<StartDateFormat, MarkdownParseError> {
		let expression = expression.trim();

		if expression.starts_with('T') && expression.contains(" + ") {
			// Relative format: "T1 + 1 month"
			let parts: Vec<&str> = expression.split(" + ").collect();
			if parts.len() == 2 {
				let task_ref = parts[0].trim().to_string();
				let duration_str = parts[1].trim();
				let duration = self.parse_duration(duration_str)?;
				return Ok(StartDateFormat::TaskReference(task_ref, duration));
			}
		}

		Err(MarkdownParseError::InvalidDateExpression { expression: expression.to_string() })
	}

	/// Parse a task ID from a string.
	fn parse_task_id(&self, task_str: &str) -> Result<TaskId, MarkdownParseError> {
		if task_str == "T0" {
			Ok(TaskId::new(0))
		} else if task_str.starts_with('T') {
			let number_str = &task_str[1..];
			let number: u8 = number_str.parse().map_err(|_| {
				MarkdownParseError::InvalidDateExpression { expression: task_str.to_string() }
			})?;
			Ok(TaskId::new(number))
		} else {
			Err(MarkdownParseError::InvalidDateExpression { expression: task_str.to_string() })
		}
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
	fn test_start_date_parsing() {
		let parser = StartDate::new();

		// Test relative start
		let result = parser.parse("T1 + 1 month").unwrap();
		assert!(result.point_of_reference().0.value() == 1);

		// Test T0 start
		let result = parser.parse("T0 + 0 months").unwrap();
		assert!(result.point_of_reference().0.value() == 0);
	}

	#[test]
	fn test_duration_parsing() {
		let parser = StartDate::new();

		// Test various duration formats
		let result = parser.parse_duration("1 month").unwrap();
		assert_eq!(result.as_secs(), 86400 * 30);

		let result = parser.parse_duration("2 weeks").unwrap();
		assert_eq!(result.as_secs(), 86400 * 14);

		let result = parser.parse_duration("30 days").unwrap();
		assert_eq!(result.as_secs(), 86400 * 30);
	}
}
