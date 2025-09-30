//! Task parsing functionality for markdown roadmap documents.

use super::error::MarkdownParseError;
use super::subtask::SubtaskParser;
use super::date::DateParser;
use roadline_util::task::{
    Task, Id as TaskId, Title, Summary, Range, Start, End, EmbeddedSubtask
};
use roadline_util::task::range::{TargetDate, PointOfReference};
use roadline_util::duration::Duration;
use std::collections::BTreeSet;
use std::time::Duration as StdDuration;

/// Parser for individual tasks in markdown documents.
///
/// This parser handles the parsing of task headers, metadata fields,
/// and subtasks from markdown sections.
#[derive(Debug, Clone)]
pub struct TaskParser {
    subtask_parser: SubtaskParser,
    pub date_parser: DateParser,
}

impl Default for TaskParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskParser {
    /// Create a new task parser.
    pub fn new() -> Self {
        Self {
            subtask_parser: SubtaskParser::new(),
            date_parser: DateParser::new(),
        }
    }

    /// Parse a complete task section from markdown.
    ///
    /// This method parses the task header, extracts metadata fields,
    /// and processes all subtasks.
    pub fn parse_task_section(&self, section: &TaskSection) -> Result<Task, MarkdownParseError> {
        // Parse the task header to get ID and title
        let (task_id, title) = self.parse_task_header(&section.header)?;
        
        // Parse metadata fields
        let metadata = self.parse_metadata(&section.content)?;
        
        // Parse subtasks from the Contents section
        let subtasks = self.parse_subtasks(&section.content)?;
        
        // Create the task range
        let range = self.create_task_range(&metadata, &task_id)?;
        
        // Create summary from title and subtasks
        let summary = self.create_summary(&title, &subtasks);
        
        Ok(Task::new(
            task_id,
            title,
            BTreeSet::new(), // Dependencies will be resolved later
            subtasks,
            summary,
            range,
        ))
    }

    /// Parse the task header to extract ID and title.
    ///
    /// Expected format: "### T1: Task Title"
    fn parse_task_header(&self, header: &str) -> Result<(TaskId, Title), MarkdownParseError> {
        let header = header.trim();
        
        if !header.starts_with("### T") {
            return Err(MarkdownParseError::InvalidTaskId {
                header: header.to_string(),
            });
        }

        // Remove the "### " prefix
        let content = &header[4..];
        
        // Find the colon separator
        let colon_pos = content.find(':')
            .ok_or_else(|| MarkdownParseError::InvalidTaskTitle {
                header: header.to_string(),
            })?;

        // Extract task ID (e.g., "T1")
        let task_id_str = content[..colon_pos].trim();
        let task_id = self.parse_task_id(task_id_str)?;
        
        // Extract title (everything after the colon)
        let title_str = content[colon_pos + 1..].trim();
        let title = Title { text: title_str.to_string() };

        Ok((task_id, title))
    }

    /// Parse a task ID string into a TaskId.
    ///
    /// Expected format: "T1", "T2", etc.
    fn parse_task_id(&self, id_str: &str) -> Result<TaskId, MarkdownParseError> {
        if !id_str.starts_with('T') {
            return Err(MarkdownParseError::InvalidTaskId {
                header: id_str.to_string(),
            });
        }

        let number_str = &id_str[1..];
        let number: u8 = number_str.parse()
            .map_err(|_| MarkdownParseError::InvalidTaskId {
                header: id_str.to_string(),
            })?;

        Ok(TaskId::new(number))
    }

    /// Parse metadata fields from the task content.
    ///
    /// This includes fields like "Starts:", "Depends-on:", "Ends:", etc.
    fn parse_metadata(&self, content: &[String]) -> Result<TaskMetadata, MarkdownParseError> {
        let mut metadata = TaskMetadata::default();

        for line in content {
            let line = line.trim();
            
            if let Some((field, value)) = self.parse_field_line(line) {
                match field.as_str() {
                    "Starts" => metadata.starts = Some(value),
                    "Depends-on" => metadata.depends_on = Some(value),
                    "Ends" => metadata.ends = Some(value),
                    _ => {} // Ignore unknown fields
                }
            }
        }

        Ok(metadata)
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

    /// Parse subtasks from the Contents section.
    fn parse_subtasks(&self, content: &[String]) -> Result<BTreeSet<EmbeddedSubtask>, MarkdownParseError> {
        let mut subtasks = BTreeSet::new();
        let mut in_contents = false;

        for line in content {
            let line = line.trim();
            
            if line == "- **Contents:**" {
                in_contents = true;
                continue;
            }
            
            if in_contents && line.starts_with("- **") && line.contains(":**") {
                if let Some(subtask) = self.subtask_parser.parse_subtask_line(line)? {
                    subtasks.insert(EmbeddedSubtask::new(subtask));
                }
            } else if in_contents && !line.starts_with("    -") {
                // End of contents section
                break;
            }
        }

        Ok(subtasks)
    }

    /// Create a task range from metadata.
    fn create_task_range(&self, metadata: &TaskMetadata, _task_id: &TaskId) -> Result<Range, MarkdownParseError> {
        // Parse the start date
        let start = if let Some(starts) = &metadata.starts {
            if starts.starts_with('T') {
                // Relative start: "T0 + 1 month"
                let parts: Vec<&str> = starts.split(" + ").collect();
                if parts.len() == 2 {
                    let ref_task_str = parts[0].trim();
                    let duration_str = parts[1].trim();
                    
                    // Parse the reference task ID
                    let ref_task_id = if ref_task_str == "T0" {
                        TaskId::new(0)
                    } else if ref_task_str.starts_with('T') {
                        let number_str = &ref_task_str[1..];
                        let number: u8 = number_str.parse()
                            .map_err(|_| MarkdownParseError::InvalidDateExpression {
                                expression: starts.clone(),
                            })?;
                        TaskId::new(number)
                    } else {
                        return Err(MarkdownParseError::InvalidDateExpression {
                            expression: starts.clone(),
                        });
                    };
                    
                    let duration = self.date_parser.parse_duration(duration_str)?;
                    Start::from(TargetDate {
                        point_of_reference: PointOfReference::from(ref_task_id),
                        duration: Duration::from(duration),
                    })
                } else {
                    return Err(MarkdownParseError::InvalidDateExpression {
                        expression: starts.clone(),
                    });
                }
            } else {
                // Absolute start date
                let _start_date = self.date_parser.parse_start_date(starts)?;
                Start::from(TargetDate {
                    point_of_reference: PointOfReference::from(TaskId::new(0)), // Use task 0 as epoch
                    duration: Duration::from(StdDuration::from_secs(0)),
                })
            }
        } else {
            // Default start
            Start::from(TargetDate {
                point_of_reference: PointOfReference::from(TaskId::new(0)),
                duration: Duration::from(StdDuration::from_secs(0)),
            })
        };
        
        // Parse the end date
        let end = if let Some(ends) = &metadata.ends {
            let end_format = self.date_parser.parse_end_date_format(ends)?;
            match end_format {
                super::date::EndDateFormat::Duration(duration) => {
                    End::from(Duration::from(duration))
                }
                super::date::EndDateFormat::TaskReference(_task_ref, duration) => {
                    End::from(Duration::from(duration))
                }
                super::date::EndDateFormat::Absolute(_date) => {
                    // For absolute dates, we'll use a default duration for now
                    End::from(Duration::from(StdDuration::from_secs(86400 * 30)))
                }
            }
        } else {
            // Default end duration
            End::from(Duration::from(StdDuration::from_secs(86400 * 30)))
        };
        
        Ok(Range::new(start, end))
    }

    /// Create a summary from the task title and subtasks.
    fn create_summary(&self, title: &Title, subtasks: &BTreeSet<EmbeddedSubtask>) -> Summary {
        let mut summary_text = title.text.clone();
        
        if !subtasks.is_empty() {
            summary_text.push_str(&format!(" ({} subtasks)", subtasks.len()));
        }
        
        Summary { text: summary_text }
    }
}

/// Metadata extracted from a task section.
#[derive(Debug, Clone, Default)]
struct TaskMetadata {
    starts: Option<String>,
    depends_on: Option<String>,
    ends: Option<String>,
}

/// Represents a task section in the markdown document.
#[derive(Debug, Clone)]
pub struct TaskSection {
    pub header: String,
    pub content: Vec<String>,
    pub line_number: usize,
}

impl TaskSection {
    /// Get the full content of the section as a single string.
    pub fn full_content(&self) -> String {
        let mut result = vec![self.header.clone()];
        result.extend(self.content.clone());
        result.join("\n")
    }
}
