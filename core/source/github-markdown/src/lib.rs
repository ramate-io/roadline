//! GitHub markdown source for roadline.
//!
//! This module provides functionality to fetch markdown content from GitHub
//! and convert it into a RoadlineBuilder. It supports multiple GitHub URL formats:
//!
//! 1. Raw content URLs (e.g., `https://raw.githubusercontent.com/...`)
//! 2. Repository paths (org, repo, path)
//! 3. Tree links (e.g., `https://github.com/org/repo/tree/branch/path`)
//!
//! # Example
//!
//! ```no_run
//! use roadline_source_github_markdown::GitHubSource;
//! use roadline_representation_core::roadline::RoadlineBuilder;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let source = GitHubSource::new();
//! let roadline = source
//!     .from_raw_url("https://raw.githubusercontent.com/ramate-io/oac/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md")
//!     .await?;
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod parser;
pub mod url;

#[cfg(test)]
mod tests;

pub use client::GitHubClient;
pub use error::GitHubSourceError;
pub use parser::GitHubSource;
pub use url::{GitHubUrl, GitHubUrlType};
