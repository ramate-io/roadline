//! Error types for markdown parsing.

use thiserror::Error;
use roadline_representation_core::roadline::RoadlineBuilderError;

/// Comprehensive error type for markdown parsing operations.
#[derive(Debug, Error)]
pub enum MarkdownParseError {
    /// No tasks were found in the markdown content.
    #[error("No tasks found in markdown content")]
    NoTasksFound,

    /// Failed to parse a task ID from a header.
    #[error("Failed to parse task ID from header: {header}")]
    InvalidTaskId { header: String },

    /// Failed to parse a task title from a header.
    #[error("Failed to parse task title from header: {header}")]
    InvalidTaskTitle { header: String },

    /// Failed to parse a subtask ID.
    #[error("Failed to parse subtask ID: {id}")]
    InvalidSubtaskId { id: String },

    /// Failed to parse a subtask title.
    #[error("Failed to parse subtask title: {title}")]
    InvalidSubtaskTitle { title: String },

    /// Failed to parse a date expression.
    #[error("Failed to parse date expression: {expression}")]
    InvalidDateExpression { expression: String },

    /// Failed to parse a dependency reference.
    #[error("Failed to parse dependency reference: {reference}")]
    InvalidDependencyReference { reference: String },

    /// Failed to parse a duration expression.
    #[error("Failed to parse duration expression: {expression}")]
    InvalidDurationExpression { expression: String },

    /// Missing required field in task section.
    #[error("Missing required field '{field}' in task section")]
    MissingRequiredField { field: String },

    /// Invalid field format in task section.
    #[error("Invalid format for field '{field}': {value}")]
    InvalidFieldFormat { field: String, value: String },

    /// Error from the roadline builder.
    #[error("Roadline builder error: {source}")]
    RoadlineBuilder {
        #[from]
        source: RoadlineBuilderError,
    },

    /// General parsing error with context.
    #[error("Parsing error at line {line}: {message}")]
    ParseError { line: usize, message: String },

    /// IO error when reading files.
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    /// Chrono date parsing error.
    #[error("Date parsing error: {source}")]
    Chrono {
        #[from]
        source: chrono::ParseError,
    },
}

impl MarkdownParseError {
    /// Create a parse error with line context.
    pub fn with_line(line: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            line,
            message: message.into(),
        }
    }

    /// Create an error for missing required field.
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingRequiredField {
            field: field.into(),
        }
    }

    /// Create an error for invalid field format.
    pub fn invalid_field_format(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::InvalidFieldFormat {
            field: field.into(),
            value: value.into(),
        }
    }
}
