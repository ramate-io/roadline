//! Markdown parser for roadmap documents.
//!
//! This module provides a comprehensive parser for roadmap documents in the format
//! used by OAC (Ordered Atomic Collaboration) roadmaps. It parses tasks, subtasks,
//! dependencies, and dates into the roadline representation system.

pub mod dependency;
pub mod error;
pub mod instrument;
pub mod range;
pub mod subtask;
pub mod summary;
pub mod task;
pub mod tests;

pub use dependency::DependencyParser;
pub use error::MarkdownParseError;
pub use instrument::{Instrumentation, TaskParsedEvent};
pub use range::{EndDate, StartDate};
pub use subtask::SubtaskParser;
pub use summary::SummaryParser;
pub use task::TaskParser;

use crate::task::TaskSection;
use roadline_representation_core::roadline::RoadlineBuilder;
use roadline_util::task::Task;

/// Main parser for roadmap markdown documents.
///
/// This parser can parse complete roadmap documents and convert them into
/// roadline representations using the RoadlineBuilder.
///
/// # Example
///
/// ```no_run
/// use roadline_parser_markdown::RoadlineParser;
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
/// let parser = RoadlineParser::new();
/// let tasks = parser.parse_tasks(markdown_content)?;
/// let roadline = RoadlineBuilder::new()
///     .tasks(tasks)?
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RoadlineParser {
	pub task_parser: TaskParser,
	pub dependency_parser: DependencyParser,
}

impl Default for RoadlineParser {
	fn default() -> Self {
		Self::new()
	}
}

impl RoadlineParser {
	/// Create a new roadmap parser with default configuration.
	pub fn new() -> Self {
		Self { task_parser: TaskParser::new(), dependency_parser: DependencyParser::new() }
	}

	/// Parse a complete markdown document and return a vector of tasks.
	///
	/// This method parses the entire document, extracting all tasks and their
	/// subtasks, dependencies, and temporal information.
	pub fn parse_tasks(&self, content: &str) -> Result<Vec<Task>, MarkdownParseError> {
		self.parse_tasks_with_instrumentation(content, &NoOpInstrumentation)
	}

	/// Parse a complete markdown document with instrumentation and return a vector of tasks.
	///
	/// This method parses the entire document, extracting all tasks and their
	/// subtasks, dependencies, and temporal information, while emitting instrumentation events.
	/// 
	/// If you don't need instrumentation, use `parse_tasks` instead.
	pub fn parse_tasks_with_instrumentation<I: Instrumentation>(
		&self,
		content: &str,
		instrumentation: &I,
	) -> Result<Vec<Task>, MarkdownParseError> {
		let mut tasks = Vec::new();
		let mut task_sections = self.extract_task_sections(content)?;

		// First pass: parse all tasks without dependencies
		for section in &mut task_sections {
			let task = self.task_parser.parse_task_section(section)?;
			
			// Emit instrumentation event
			let event = TaskParsedEvent {
				task: *task.id(),
				title_line_string: section.header.clone(),
			};
			instrumentation.on_task_parsed(event)?;
			
			tasks.push(task);
		}

		// Second pass: resolve dependencies
		for (i, section) in task_sections.iter().enumerate() {
			if let Some(dependencies) = self.dependency_parser.parse_dependencies(section)? {
				for dep_id in dependencies {
					tasks[i].depends_on_mut().insert(dep_id);
				}
			}
		}

		Ok(tasks)
	}
}

/// No-op instrumentation implementation for when instrumentation is not needed.
///
/// This is the default instrumentation used by the parser when no specific
/// instrumentation is provided. It performs no operations and always succeeds.
#[derive(Debug, Clone, Default)]
pub struct NoOpInstrumentation;

impl Instrumentation for NoOpInstrumentation {
	fn on_task_parsed(&self, _event: TaskParsedEvent) -> Result<(), MarkdownParseError> {
		Ok(())
	}
}

impl RoadlineParser {
	/// Parse a markdown document and build a roadline representation.
	///
	/// This is a convenience method that combines parsing and roadline building.
	pub fn parse_and_build(
		&self,
		content: &str,
	) -> Result<roadline_representation_core::roadline::Roadline, MarkdownParseError> {
		self.parse_and_build_with_instrumentation(content, &NoOpInstrumentation)
	}

	/// Parse a markdown document with instrumentation and build a roadline representation.
	///
	/// This is a convenience method that combines parsing and roadline building with instrumentation.
	/// 
	/// If you don't need instrumentation, use `parse_and_build` instead.
	pub fn parse_and_build_with_instrumentation<I: Instrumentation>(
		&self,
		content: &str,
		instrumentation: &I,
	) -> Result<roadline_representation_core::roadline::Roadline, MarkdownParseError> {
		let tasks = self.parse_tasks_with_instrumentation(content, instrumentation)?;
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
