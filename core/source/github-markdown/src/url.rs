//! GitHub URL parsing and handling.

use crate::error::GitHubSourceError;
use std::fmt;

/// Represents a parsed GitHub URL with its components.
#[derive(Debug, Clone, PartialEq)]
pub struct GitHubUrl {
	pub owner: String,
	pub repo: String,
	pub path: String,
	pub reference: String, // branch, tag, or commit
}

/// Types of GitHub URLs that can be parsed.
#[derive(Debug, Clone, PartialEq)]
pub enum GitHubUrlType {
	/// Raw content URL (raw.githubusercontent.com)
	Raw,
	/// Repository tree URL (github.com/.../tree/...)
	Tree,
	/// Repository blob URL (github.com/.../blob/...)
	Blob,
	/// Direct repository path (org/repo/path)
	Repository,
}

impl GitHubUrl {
	/// Parse a GitHub URL from various formats.
	///
	/// Supported formats:
	/// - Raw content URLs: `https://raw.githubusercontent.com/owner/repo/branch/path`
	/// - Tree URLs: `https://github.com/owner/repo/tree/branch/path`
	/// - Blob URLs: `https://github.com/owner/repo/blob/branch/path`
	/// - Repository paths: `owner/repo/path` (assumes main branch)
	pub fn parse(url: &str) -> std::result::Result<(Self, GitHubUrlType), GitHubSourceError> {
		// Handle raw content URLs
		if url.contains("raw.githubusercontent.com") {
			return Self::parse_raw_url(url);
		}

		// Handle GitHub tree/blob URLs
		if url.contains("github.com") {
			if url.contains("/tree/") {
				return Self::parse_tree_url(url);
			} else if url.contains("/blob/") {
				return Self::parse_blob_url(url);
			}
		}

		// Handle simple repository paths (owner/repo/path)
		if !url.starts_with("http") && url.matches('/').count() >= 2 {
			return Self::parse_repository_path(url);
		}

		Err(GitHubSourceError::UrlParsing {
			message: format!("Unsupported GitHub URL format: {}", url),
		})
	}

	/// Parse a raw content URL.
	fn parse_raw_url(url: &str) -> std::result::Result<(Self, GitHubUrlType), GitHubSourceError> {
		// Format: https://raw.githubusercontent.com/owner/repo/branch/path
		let parts: Vec<&str> = url.split('/').collect();

		if parts.len() < 7 || parts[2] != "raw.githubusercontent.com" {
			return Err(GitHubSourceError::UrlParsing {
				message: format!("Invalid raw content URL format: {}", url),
			});
		}

		let owner = parts[3].to_string();
		let repo = parts[4].to_string();
		let reference = parts[5].to_string();
		let path = parts[6..].join("/");

		Ok((Self { owner, repo, path, reference }, GitHubUrlType::Raw))
	}

	/// Parse a tree URL.
	fn parse_tree_url(url: &str) -> std::result::Result<(Self, GitHubUrlType), GitHubSourceError> {
		// Format: https://github.com/owner/repo/tree/branch/path
		let parts: Vec<&str> = url.split('/').collect();

		if parts.len() < 7 || parts[2] != "github.com" || parts[5] != "tree" {
			return Err(GitHubSourceError::UrlParsing {
				message: format!("Invalid tree URL format: {}", url),
			});
		}

		let owner = parts[3].to_string();
		let repo = parts[4].to_string();
		let reference = parts[6].to_string();
		let path = if parts.len() > 7 { parts[7..].join("/") } else { String::new() };

		Ok((Self { owner, repo, path, reference }, GitHubUrlType::Tree))
	}

	/// Parse a blob URL.
	fn parse_blob_url(url: &str) -> std::result::Result<(Self, GitHubUrlType), GitHubSourceError> {
		// Format: https://github.com/owner/repo/blob/branch/path
		let parts: Vec<&str> = url.split('/').collect();

		if parts.len() < 7 || parts[2] != "github.com" || parts[5] != "blob" {
			return Err(GitHubSourceError::UrlParsing {
				message: format!("Invalid blob URL format: {}", url),
			});
		}

		let owner = parts[3].to_string();
		let repo = parts[4].to_string();
		let reference = parts[6].to_string();
		let path = if parts.len() > 7 { parts[7..].join("/") } else { String::new() };

		Ok((Self { owner, repo, path, reference }, GitHubUrlType::Blob))
	}

	/// Parse a repository path (owner/repo/path).
	fn parse_repository_path(
		path: &str,
	) -> std::result::Result<(Self, GitHubUrlType), GitHubSourceError> {
		let parts: Vec<&str> = path.split('/').collect();

		if parts.len() < 3 {
			return Err(GitHubSourceError::UrlParsing {
				message: format!("Invalid repository path format: {}", path),
			});
		}

		let owner = parts[0].to_string();
		let repo = parts[1].to_string();
		let file_path = parts[2..].join("/");

		Ok((
			Self {
				owner,
				repo,
				path: file_path,
				reference: "main".to_string(), // Default to main branch
			},
			GitHubUrlType::Repository,
		))
	}

	/// Convert to a raw content URL.
	pub fn to_raw_url(&self) -> String {
		format!(
			"https://raw.githubusercontent.com/{}/{}/{}/{}",
			self.owner, self.repo, self.reference, self.path
		)
	}

	/// Convert to a GitHub API content URL.
	pub fn to_api_url(&self) -> String {
		format!(
			"https://api.github.com/repos/{}/{}/contents/{}?ref={}",
			self.owner, self.repo, self.path, self.reference
		)
	}
}

impl fmt::Display for GitHubUrl {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}/{}/{}@{}", self.owner, self.repo, self.path, self.reference)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_raw_url() -> Result<(), Box<dyn std::error::Error>> {
		let url = "https://raw.githubusercontent.com/ramate-io/oac/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";
		let (parsed, url_type) = GitHubUrl::parse(url)?;

		assert_eq!(url_type, GitHubUrlType::Raw);
		assert_eq!(parsed.owner, "ramate-io");
		assert_eq!(parsed.repo, "oac");
		assert_eq!(parsed.reference, "refs");
		assert_eq!(
			parsed.path,
			"heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md"
		);
		Ok(())
	}

	#[test]
	fn test_parse_tree_url() -> Result<(), Box<dyn std::error::Error>> {
		let url = "https://github.com/ramate-io/oac/tree/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";
		let (parsed, url_type) = GitHubUrl::parse(url)?;

		assert_eq!(url_type, GitHubUrlType::Tree);
		assert_eq!(parsed.owner, "ramate-io");
		assert_eq!(parsed.repo, "oac");
		assert_eq!(parsed.reference, "main");
		assert_eq!(parsed.path, "oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md");
		Ok(())
	}

	#[test]
	fn test_parse_blob_url() -> Result<(), Box<dyn std::error::Error>> {
		let url = "https://github.com/ramate-io/oac/blob/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";
		let (parsed, url_type) = GitHubUrl::parse(url)?;

		assert_eq!(url_type, GitHubUrlType::Blob);
		assert_eq!(parsed.owner, "ramate-io");
		assert_eq!(parsed.repo, "oac");
		assert_eq!(parsed.reference, "main");
		assert_eq!(parsed.path, "oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md");
		Ok(())
	}

	#[test]
	fn test_parse_repository_path() -> Result<(), Box<dyn std::error::Error>> {
		let path = "ramate-io/oac/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";
		let (parsed, url_type) = GitHubUrl::parse(path)?;

		assert_eq!(url_type, GitHubUrlType::Repository);
		assert_eq!(parsed.owner, "ramate-io");
		assert_eq!(parsed.repo, "oac");
		assert_eq!(parsed.reference, "main");
		assert_eq!(parsed.path, "oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md");
		Ok(())
	}

	#[test]
	fn test_to_raw_url() {
		let url = GitHubUrl {
			owner: "ramate-io".to_string(),
			repo: "oac".to_string(),
			path: "README.md".to_string(),
			reference: "main".to_string(),
		};

		assert_eq!(
			url.to_raw_url(),
			"https://raw.githubusercontent.com/ramate-io/oac/main/README.md"
		);
	}

	#[test]
	fn test_to_api_url() {
		let url = GitHubUrl {
			owner: "ramate-io".to_string(),
			repo: "oac".to_string(),
			path: "README.md".to_string(),
			reference: "main".to_string(),
		};

		assert_eq!(
			url.to_api_url(),
			"https://api.github.com/repos/ramate-io/oac/contents/README.md?ref=main"
		);
	}
}
