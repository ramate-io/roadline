//! Metadata collection for GitHub source instrumentation.
pub mod fragment;

use fragment::extract_markdown_fragments;
use roadline_parser_markdown::error::MarkdownParseError;
use roadline_parser_markdown::{Instrumentation, TaskParsedEvent};
use roadline_util::task::Id as TaskId;
use std::collections::HashMap;

/// Instrumentation that collects task metadata for GitHub source.
#[derive(Debug)]
pub struct GitHubMetadataCollector {
	/// Mapping from task ID to sanitized header fragment
	task_fragments: HashMap<TaskId, String>,
}

impl GitHubMetadataCollector {
	/// Create a new metadata collector.
	pub fn new() -> Self {
		Self { task_fragments: HashMap::new() }
	}

	/// Record a task with its header line.
	pub fn record_task(
		&mut self,
		task_id: TaskId,
		header_line: &str,
	) -> Result<(), MarkdownParseError> {
		let fragment = Self::sanitize_header_to_fragment(header_line)?;
		self.task_fragments.insert(task_id, fragment);
		Ok(())
	}

	/// Get the fragment for a task ID.
	pub fn get_fragment(&self, task_id: &TaskId) -> Option<&String> {
		self.task_fragments.get(task_id)
	}

	/// Get all collected fragments.
	pub fn fragments(&self) -> &HashMap<TaskId, String> {
		&self.task_fragments
	}

	/// Sanitize a header line to create a GitHub fragment.
	///
	/// This converts a markdown header like "### T1: Push Towards Validation"
	/// into a GitHub-compatible fragment like "t1-push-towards-validation"
	///
	/// GitHub's algorithm:
	/// 1. Remove leading '#' characters and whitespace
	/// 2. Convert to lowercase
	/// 3. Keep alphanumeric characters, spaces, and hyphens
	/// 4. Replace spaces with hyphens
	/// 5. Remove consecutive hyphens
	/// 6. Strip leading/trailing hyphens
	///
	/// Based on: https://github.com/lycheeverse/lychee/blob/d6c2bbe6f1e7b9e83889fc1e7fc675a38a7dd75f/lychee-lib/src/extract/markdown.rs#L177
	fn sanitize_header_to_fragment(header: &str) -> Result<String, MarkdownParseError> {
		extract_markdown_fragments(header)
			.iter()
			.next()
			.cloned()
			.ok_or(MarkdownParseError::InvalidTaskId { header: header.to_string() })
	}
}

impl Default for GitHubMetadataCollector {
	fn default() -> Self {
		Self::new()
	}
}

impl Instrumentation for GitHubMetadataCollector {
	fn on_task_parsed(&mut self, event: TaskParsedEvent) -> Result<(), MarkdownParseError> {
		self.record_task(event.task, &event.title_line_string)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sanitize_header_to_fragment() -> Result<(), MarkdownParseError> {
		assert_eq!(
			GitHubMetadataCollector::sanitize_header_to_fragment(
				"### T1: Push Towards Validation"
			)?,
			"t1-push-towards-validation"
		);

		assert_eq!(
			GitHubMetadataCollector::sanitize_header_to_fragment(
				"## T2: Validation and Accepting Contributions"
			)?,
			"t2-validation-and-accepting-contributions"
		);

		assert_eq!(
			GitHubMetadataCollector::sanitize_header_to_fragment("# T9: An Interlude")?,
			"t9-an-interlude"
		);

		assert_eq!(
			GitHubMetadataCollector::sanitize_header_to_fragment(
				"### T1.1: Complete draft of OART-1: BFA"
			)?,
			"t11-complete-draft-of-oart-1-bfa"
		);

		assert_eq!(
			GitHubMetadataCollector::sanitize_header_to_fragment(
				"### T3: Continued Validation and [`fuste`](https://github.com/ramate-io/fuste) MVP"
			)?,
			"t3-continued-validation-and-fuste-mvp"
		);

		Ok(())
	}

	#[test]
	fn test_metadata_collector() -> Result<(), MarkdownParseError> {
		let mut collector = GitHubMetadataCollector::new();
		let task_id = TaskId::new(1);

		collector.record_task(task_id, "### T1: Push Towards Validation")?;

		let fragment = collector.get_fragment(&task_id);
		assert_eq!(fragment, Some(&"t1-push-towards-validation".to_string()));

		Ok(())
	}
}
