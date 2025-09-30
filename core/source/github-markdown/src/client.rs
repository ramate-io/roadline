//! GitHub API client for fetching content.

use crate::error::GitHubSourceError;
use crate::url::GitHubUrl;
use reqwest::Client;
use serde::Deserialize;

/// GitHub API client for fetching repository content.
#[derive(Debug, Clone)]
pub struct GitHubClient {
	client: Client,
	token: Option<String>,
}

/// GitHub API response for content endpoint.
#[derive(Debug, Deserialize)]
struct GitHubContentResponse {
	content: String,
	encoding: String,
}

impl GitHubClient {
	/// Create a new GitHub client without authentication.
	pub fn new() -> Self {
		Self { client: Client::new(), token: None }
	}

	/// Create a new GitHub client with a personal access token.
	pub fn with_token(token: String) -> Self {
		Self { client: Client::new(), token: Some(token) }
	}

	/// Fetch content from a GitHub URL.
	///
	/// This method automatically determines the best way to fetch the content
	/// based on the URL type and uses the most efficient method.
    pub async fn fetch_content(&self, github_url: &GitHubUrl) -> std::result::Result<String, GitHubSourceError> {
		// For raw URLs, fetch directly from raw.githubusercontent.com
		let raw_url = github_url.to_raw_url();
		self.fetch_raw_content(&raw_url).await
	}

	/// Fetch content directly from a raw GitHub URL.
    pub async fn fetch_raw_content(&self, raw_url: &str) -> std::result::Result<String, GitHubSourceError> {
		let mut request = self.client.get(raw_url);

		// Add authentication if available
		if let Some(ref token) = self.token {
			request = request.header("Authorization", format!("token {}", token));
		}

		let response = request.send().await?;

		if !response.status().is_success() {
			return Err(GitHubSourceError::FetchError {
				message: format!("Failed to fetch content from {}: {}", raw_url, response.status()),
			});
		}

		let content = response.text().await?;
		Ok(content)
	}

	/// Fetch content using the GitHub API.
	///
	/// This method is more reliable for private repositories but requires
	/// authentication and has rate limits.
    pub async fn fetch_content_via_api(&self, github_url: &GitHubUrl) -> std::result::Result<String, GitHubSourceError> {
		let api_url = github_url.to_api_url();
		let mut request = self.client.get(&api_url);

		// Add authentication if available
		if let Some(ref token) = self.token {
			request = request.header("Authorization", format!("token {}", token));
		}

		// Add user agent header (required by GitHub API)
		request = request.header("User-Agent", "roadline-source-github-markdown");

		let response = request.send().await?;

		if !response.status().is_success() {
			return Err(GitHubSourceError::FetchError {
				message: format!(
					"Failed to fetch content from GitHub API {}: {}",
					api_url,
					response.status()
				),
			});
		}

		let content_response: GitHubContentResponse = response.json().await?;

		// Decode base64 content
		if content_response.encoding == "base64" {
			let decoded = base64::Engine::decode(
				&base64::engine::general_purpose::STANDARD,
				&content_response.content,
			)
			.map_err(|e| GitHubSourceError::FetchError {
				message: format!("Failed to decode base64 content: {}", e),
			})?;

			String::from_utf8(decoded).map_err(|e| GitHubSourceError::FetchError {
				message: format!("Failed to convert decoded content to UTF-8: {}", e),
			})
		} else {
			Err(GitHubSourceError::FetchError {
				message: format!("Unsupported encoding: {}", content_response.encoding),
			})
		}
	}
}

impl Default for GitHubClient {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_fetch_content_from_oroad() -> Result<(), Box<dyn std::error::Error>> {
		let client = GitHubClient::new();
		let github_url = GitHubUrl {
			owner: "ramate-io".to_string(),
			repo: "oac".to_string(),
			path: "oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md".to_string(),
			reference: "refs/heads/main".to_string(),
		};

		let content = client.fetch_content(&github_url).await?;
		assert!(content.contains("OROAD-0"));
		assert!(content.contains("The Attempt"));
		Ok(())
	}

	#[tokio::test]
	async fn test_fetch_raw_content() -> Result<(), Box<dyn std::error::Error>> {
		let client = GitHubClient::new();
		let raw_url = "https://raw.githubusercontent.com/ramate-io/oac/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";

		let content = client.fetch_raw_content(raw_url).await?;
		assert!(content.contains("OROAD-0"));
		Ok(())
	}
}
