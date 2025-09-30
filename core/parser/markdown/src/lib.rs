//! Markdown parser for roadmap documents.
//!
//! This module provides a comprehensive parser for roadmap documents in the format
//! used by OAC (Ordered Atomic Collaboration) roadmaps. It parses tasks, subtasks,
//! dependencies, and dates into the roadline representation system.

pub mod dependency;
pub mod error;
pub mod range;
pub mod subtask;
pub mod task;

pub use dependency::DependencyParser;
pub use error::MarkdownParseError;
pub use range::{EndDate, StartDate};
pub use subtask::SubtaskParser;
pub use task::TaskParser;

use crate::task::TaskSection;
use roadline_representation_core::roadline::RoadlineBuilder;
use roadline_util::task::{Id as TaskId, Task};
use std::collections::HashMap;

/// Main parser for roadmap markdown documents.
///
/// This parser can parse complete roadmap documents and convert them into
/// roadline representations using the RoadlineBuilder.
///
/// # Example
///
/// ```no_run
/// use roadline_parser_markdown::RoadmapParser;
/// use roadline_representation_core::roadline::RoadlineBuilder;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let markdown_content = r#"
/// ### T1: Push Towards Validation
/// - **Starts:** T0 + 0 months
/// - **Depends-on:** $\\emptyset$
/// - **Ends:** T1 + 1 month
/// - **Contents:**
///     - **T1.1**: Complete draft of OART-1: BFA
/// "#;
///
/// let parser = RoadmapParser::new();
/// let tasks = parser.parse_tasks(markdown_content)?;
/// let roadline = RoadlineBuilder::new()
///     .tasks(tasks)?
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RoadmapParser {
	pub task_parser: TaskParser,
	pub dependency_parser: DependencyParser,
}

impl Default for RoadmapParser {
	fn default() -> Self {
		Self::new()
	}
}

impl RoadmapParser {
	/// Create a new roadmap parser with default configuration.
	pub fn new() -> Self {
		Self { task_parser: TaskParser::new(), dependency_parser: DependencyParser::new() }
	}

	/// Parse a complete markdown document and return a vector of tasks.
	///
	/// This method parses the entire document, extracting all tasks and their
	/// subtasks, dependencies, and temporal information.
	pub fn parse_tasks(&self, content: &str) -> Result<Vec<Task>, MarkdownParseError> {
		let mut tasks = Vec::new();
		let mut task_sections = self.extract_task_sections(content)?;

		// First pass: parse all tasks without dependencies
		for section in &mut task_sections {
			let task = self.task_parser.parse_task_section(section)?;
			tasks.push(task);
		}

		// Second pass: resolve dependencies
		let task_map: HashMap<u8, TaskId> =
			tasks.iter().map(|task| (u8::from(*task.id()), *task.id())).collect();

		for (i, section) in task_sections.iter().enumerate() {
			if let Some(dependencies) = self.dependency_parser.parse_dependencies(section)? {
				for dep_id in dependencies {
					if let Some(dep_task_id) = task_map.get(&dep_id) {
						tasks[i].depends_on_mut().insert(*dep_task_id);
					}
				}
			}
		}

		Ok(tasks)
	}

	/// Parse a markdown document and build a roadline representation.
	///
	/// This is a convenience method that combines parsing and roadline building.
	pub fn parse_and_build(
		&self,
		content: &str,
	) -> Result<roadline_representation_core::roadline::Roadline, MarkdownParseError> {
		let tasks = self.parse_tasks(content)?;
		let mut builder = RoadlineBuilder::new();

		for task in tasks {
			builder.add_task(task)?;
		}

		builder.build().map_err(|e| MarkdownParseError::RoadlineBuilder { source: e })
	}

	/// Extract task sections from markdown content.
	///
	/// This method identifies and extracts individual task sections from the
	/// markdown document.
	fn extract_task_sections(&self, content: &str) -> Result<Vec<TaskSection>, MarkdownParseError> {
		let mut sections = Vec::new();
		let lines: Vec<&str> = content.lines().collect();
		let mut current_section: Option<TaskSection> = None;

		for (line_num, line) in lines.iter().enumerate() {
			let line = line.trim();

			// Check if this is a task header (starts with ### T)
			if line.starts_with("### T") && line.contains(':') {
				// Save previous section if it exists
				if let Some(section) = current_section.take() {
					sections.push(section);
				}

				// Start new section
				current_section = Some(TaskSection {
					header: line.to_string(),
					content: Vec::new(),
					line_number: line_num + 1,
				});
			} else if let Some(ref mut section) = current_section {
				section.content.push(line.to_string());
			}
		}

		// Don't forget the last section
		if let Some(section) = current_section {
			sections.push(section);
		}

		if sections.is_empty() {
			return Err(MarkdownParseError::NoTasksFound);
		}

		Ok(sections)
	}
}
