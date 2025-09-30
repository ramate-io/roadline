//! Error types for GitHub source operations.

use thiserror::Error;

// Remove the type alias - we'll use explicit Result types throughout

/// Errors that can occur during GitHub source operations.
#[derive(Error, Debug)]
pub enum GitHubSourceError {
    /// Error parsing a GitHub URL.
    #[error("Failed to parse GitHub URL: {message}")]
    UrlParsing { message: String },

    /// Error fetching content from GitHub.
    #[error("Failed to fetch content from GitHub: {message}")]
    FetchError { message: String },

    /// Error parsing markdown content.
    #[error("Failed to parse markdown content: {source}")]
    MarkdownParse {
        #[from]
        source: roadline_parser_markdown::MarkdownParseError,
    },

    /// Error building roadline representation.
    #[error("Failed to build roadline representation: {source}")]
    RoadlineBuild {
        #[from]
        source: roadline_representation_core::roadline::RoadlineBuilderError,
    },

    /// HTTP request error.
    #[error("HTTP request failed: {source}")]
    Http {
        #[from]
        source: reqwest::Error,
    },

    /// IO error.
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
}
