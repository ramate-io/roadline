//! Dependency parsing functionality for markdown roadmap documents.

use super::error::MarkdownParseError;
use super::task::TaskSection;
use roadline_util::task::Id as TaskId;

/// Parser for task dependencies in markdown documents.
///
/// This parser handles the parsing of dependency references from the
/// "Depends-on:" field in task sections.
#[derive(Debug, Clone)]
pub struct DependencyParser {
	// Configuration for dependency parsing
}

impl Default for DependencyParser {
	fn default() -> Self {
		Self::new()
	}
}

impl DependencyParser {
	/// Create a new dependency parser.
	pub fn new() -> Self {
		Self {}
	}

	/// Parse dependencies from a task section.
	///
	/// This method looks for the "Depends-on:" field and extracts
	/// all dependency references.
	pub fn parse_dependencies(
		&self,
		section: &TaskSection,
	) -> Result<Option<Vec<TaskId>>, MarkdownParseError> {
		for line in &section.content {
			let line = line.trim();

			if let Some((field, value)) = self.parse_field_line(line) {
				if field == "Depends-on" {
					return self.parse_dependency_value(&value);
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
		let value_start = end + 4; // Skip ":** "
		let value = if value_start < line.len() {
			line[value_start..].trim().to_string()
		} else {
			String::new()
		};

		Some((field, value))
	}

	/// Parse the dependency value string.
	///
	/// This handles various formats:
	/// - Single dependency: "[T1](#t1-title)"
	/// - Multiple dependencies: "[T1](#t1-title), [T2](#t2-title)"
	/// - Empty set: "$\\emptyset$"
	fn parse_dependency_value(
		&self,
		value: &str,
	) -> Result<Option<Vec<TaskId>>, MarkdownParseError> {
		let value = value.trim();

		// Handle empty set
		if value == "$\\emptyset$" || value.is_empty() {
			return Ok(Some(Vec::new()));
		}

		// Split by commas and parse each dependency
		let dependencies: Result<Vec<TaskId>, _> =
			value.split(',').map(|dep| self.parse_single_dependency(dep.trim())).collect();

		Ok(Some(dependencies?))
	}

	/// Parse a single dependency reference.
	///
	/// Expected format: "[T1](#t1-title)"
	fn parse_single_dependency(&self, dep_str: &str) -> Result<TaskId, MarkdownParseError> {
		if !dep_str.starts_with('[') || !dep_str.contains(']') {
			return Err(MarkdownParseError::InvalidDependencyReference {
				reference: dep_str.to_string(),
			});
		}

		// Find the end of the link text
		let end_bracket =
			dep_str
				.find(']')
				.ok_or_else(|| MarkdownParseError::InvalidDependencyReference {
					reference: dep_str.to_string(),
				})?;

		// Extract the task ID (e.g., "T1")
		let task_id = &dep_str[1..end_bracket];

		// Validate that it's a valid task ID format
		if !task_id.starts_with('T') {
			return Err(MarkdownParseError::InvalidDependencyReference {
				reference: dep_str.to_string(),
			});
		}

		// Parse the number part
		let number_str = &task_id[1..];
		let number: u8 = number_str.parse().map_err(|_| {
			MarkdownParseError::InvalidDependencyReference { reference: dep_str.to_string() }
		})?;

		Ok(TaskId::new(number))
	}

	/// Parse dependencies from a raw markdown line.
	///
	/// This is a convenience method for parsing a single line.
	pub fn parse_dependency_line(
		&self,
		line: &str,
	) -> Result<Option<Vec<TaskId>>, MarkdownParseError> {
		let line = line.trim();

		if let Some((field, value)) = self.parse_field_line(line) {
			if field == "Depends-on" {
				return self.parse_dependency_value(&value);
			}
		}

		Ok(None)
	}

	/// Check if a line contains a dependency field.
	pub fn is_dependency_line(&self, line: &str) -> bool {
		let line = line.trim();
		line.starts_with("- **Depends-on:**")
	}

	/// Extract task ID from a dependency reference.
	///
	/// This is a utility method for extracting just the task ID
	/// from a full dependency reference.
	pub fn extract_task_id(&self, dep_ref: &str) -> Result<TaskId, MarkdownParseError> {
		self.parse_single_dependency(dep_ref)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_empty_dependencies() -> Result<(), MarkdownParseError> {
		let parser = DependencyParser::new();
		let result = parser.parse_dependency_value("$\\emptyset$")?;
		assert_eq!(result, Some(Vec::new()));
		Ok(())
	}

	#[test]
	fn test_parse_single_dependency() -> Result<(), MarkdownParseError> {
		let parser = DependencyParser::new();
		let result = parser.parse_dependency_value("[T1](#t1-title)")?;
		assert_eq!(result, Some(vec![TaskId::new(1)]));
		Ok(())
	}

	#[test]
	fn test_parse_multiple_dependencies() -> Result<(), MarkdownParseError> {
		let parser = DependencyParser::new();
		let result = parser.parse_dependency_value("[T1](#t1-title), [T2](#t2-title)")?;
		assert_eq!(result, Some(vec![TaskId::new(1), TaskId::new(2)]));
		Ok(())
	}

	#[test]
	fn test_parse_dependency_line() -> Result<(), MarkdownParseError> {
		let parser = DependencyParser::new();
		let line = "- **Depends-on:** [T1](#t1-title), [T2](#t2-title)";
		let result = parser.parse_dependency_line(line)?;
		assert_eq!(result, Some(vec![TaskId::new(1), TaskId::new(2)]));
		Ok(())
	}

	#[test]
	fn test_is_dependency_line() {
		let parser = DependencyParser::new();
		assert!(parser.is_dependency_line("- **Depends-on:** [T1](#t1-title)"));
		assert!(!parser.is_dependency_line("- **Starts:** T0 + 0 months"));
	}
}
