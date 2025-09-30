//! Main GitHub source parser that converts GitHub markdown to RoadlineBuilder.

use crate::client::GitHubClient;
use crate::error::GitHubSourceError;
use crate::metadata::MutableGitHubMetadataCollector;
use crate::url::GitHubUrl;
use roadline_parser_markdown::{Instrumentation, MarkdownParseError, RoadlineParser, TaskParsedEvent};
use roadline_representation_core::roadline::RoadlineBuilder;
use std::cell::RefCell;
use std::rc::Rc;

/// GitHub source that can fetch markdown content and convert it to a RoadlineBuilder.
#[derive(Debug, Clone)]
pub struct GitHubSource {
	client: GitHubClient,
	parser: RoadlineParser,
}

impl GitHubSource {
	/// Create a new GitHub source with default configuration.
	pub fn new() -> Self {
		Self { client: GitHubClient::new(), parser: RoadlineParser::new() }
	}

	/// Create a new GitHub source with a personal access token.
	pub fn with_token(token: String) -> Self {
		Self { client: GitHubClient::with_token(token), parser: RoadlineParser::new() }
	}

	/// Fetch content from a raw GitHub URL and convert to RoadlineBuilder.
	///
	/// # Example
	///
	/// ```no_run
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// use roadline_source_github_markdown::GitHubSource;
	///
	/// let source = GitHubSource::new();
	/// let roadline = source
	///     .from_raw_url("https://raw.githubusercontent.com/ramate-io/oac/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md")
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn from_raw_url(
		&self,
		url: &str,
	) -> std::result::Result<RoadlineBuilder, GitHubSourceError> {
		let (github_url, _) = GitHubUrl::parse(url)?;
		self.from_github_url(&github_url).await
	}

	/// Fetch content from a GitHub tree/blob URL and convert to RoadlineBuilder.
	///
	/// # Example
	///
	/// ```no_run
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// use roadline_source_github_markdown::GitHubSource;
	///
	/// let source = GitHubSource::new();
	/// let roadline = source
	///     .from_tree_url("https://github.com/ramate-io/oac/tree/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md")
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn from_tree_url(
		&self,
		url: &str,
	) -> std::result::Result<RoadlineBuilder, GitHubSourceError> {
		let (github_url, _) = GitHubUrl::parse(url)?;
		self.from_github_url(&github_url).await
	}

	/// Fetch content from a GitHub blob URL and convert to RoadlineBuilder.
	///
	/// # Example
	///
	/// ```no_run
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// use roadline_source_github_markdown::GitHubSource;
	///
	/// let source = GitHubSource::new();
	/// let roadline = source
	///     .from_blob_url("https://github.com/ramate-io/oac/blob/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md")
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn from_blob_url(
		&self,
		url: &str,
	) -> std::result::Result<RoadlineBuilder, GitHubSourceError> {
		let (github_url, _) = GitHubUrl::parse(url)?;
		self.from_github_url(&github_url).await
	}

	/// Fetch content from a repository path and convert to RoadlineBuilder.
	///
	/// # Example
	///
	/// ```no_run
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// use roadline_source_github_markdown::GitHubSource;
	///
	/// let source = GitHubSource::new();
	/// let roadline = source
	///     .from_repository_path("ramate-io/oac/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md")
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn from_repository_path(
		&self,
		path: &str,
	) -> std::result::Result<RoadlineBuilder, GitHubSourceError> {
		let (github_url, _) = GitHubUrl::parse(path)?;
		self.from_github_url(&github_url).await
	}

	/// Fetch content from a parsed GitHub URL and convert to RoadlineBuilder.
	pub async fn from_github_url(
		&self,
		github_url: &GitHubUrl,
	) -> std::result::Result<RoadlineBuilder, GitHubSourceError> {
		let content = self.client.fetch_content(github_url).await?;
		self.parse_content(&content)
	}

	/// Parse markdown content and convert to RoadlineBuilder.
	pub fn parse_content(
		&self,
		content: &str,
	) -> std::result::Result<RoadlineBuilder, GitHubSourceError> {
		let tasks = self.parser.parse_tasks(content)?;
		let mut builder = RoadlineBuilder::new();

		for task in tasks {
			builder.add_task(task)?;
		}

		Ok(builder)
	}

	/// Get the underlying GitHub client for advanced usage.
	pub fn client(&self) -> &GitHubClient {
		&self.client
	}

	/// Get the underlying markdown parser for advanced usage.
	pub fn parser(&self) -> &RoadlineParser {
		&self.parser
	}

	/// Parse markdown content with metadata collection and convert to RoadlineBuilder.
	///
	/// This method returns both the RoadlineBuilder and a metadata collector that maps
	/// task IDs to their GitHub header fragments.
	pub fn parse_content_with_metadata(
		&self,
		content: &str,
	) -> std::result::Result<(RoadlineBuilder, MutableGitHubMetadataCollector), GitHubSourceError> {
		// Create a metadata collector with interior mutability
		let collector = Rc::new(RefCell::new(MutableGitHubMetadataCollector::new()));
		let collector_clone = Rc::clone(&collector);
		
		// Create an instrumentation wrapper
		let instrumentation = GitHubInstrumentation { collector: collector_clone };
		
		// Parse with instrumentation
		let tasks = self.parser.parse_tasks_with_instrumentation(content, &instrumentation)?;
		let mut builder = RoadlineBuilder::new();

		for task in tasks {
			builder.add_task(task)?;
		}

		// Extract the collector from the Rc<RefCell<>>
		let metadata = Rc::try_unwrap(collector)
			.map_err(|_| GitHubSourceError::FetchError {
				message: "Failed to unwrap metadata collector".to_string(),
			})?
			.into_inner();

		Ok((builder, metadata))
	}

	/// Fetch content from a GitHub URL with metadata collection.
	///
	/// This method returns both the RoadlineBuilder and a metadata collector that maps
	/// task IDs to their GitHub header fragments for creating deep links.
	pub async fn from_github_url_with_metadata(
		&self,
		github_url: &GitHubUrl,
	) -> std::result::Result<(RoadlineBuilder, MutableGitHubMetadataCollector), GitHubSourceError> {
		let content = self.client.fetch_content(github_url).await?;
		self.parse_content_with_metadata(&content)
	}
}

/// Instrumentation wrapper that collects metadata for GitHub fragment generation.
struct GitHubInstrumentation {
	collector: Rc<RefCell<MutableGitHubMetadataCollector>>,
}

impl Instrumentation for GitHubInstrumentation {
	fn on_task_parsed(&self, event: TaskParsedEvent) -> Result<(), MarkdownParseError> {
		self.collector.borrow_mut().record_task(event.task, &event.title_line_string);
		Ok(())
	}
}

impl Default for GitHubSource {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_from_raw_url() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();
		let roadline = source
            .from_raw_url("https://raw.githubusercontent.com/ramate-io/oac/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md")
            .await?;
		let roadline = roadline.build()?;

		// Should have parsed tasks from the OROAD-0 document
		assert!(roadline.graph().arena.tasks().len() > 0);
		Ok(())
	}

	#[tokio::test]
	async fn test_from_tree_url() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();
		let roadline = source
            .from_tree_url("https://github.com/ramate-io/oac/tree/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md")
            .await?;
		let roadline = roadline.build()?;

		// Should have parsed tasks from the OROAD-0 document
		assert!(roadline.graph().arena.tasks().len() > 0);
		Ok(())
	}

	#[tokio::test]
	async fn test_from_repository_path() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();
		let roadline = source
			.from_repository_path(
				"ramate-io/oac/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md",
			)
			.await?;
		let roadline = roadline.build()?;

		// Should have parsed tasks from the OROAD-0 document
		assert!(roadline.graph().arena.tasks().len() > 0);
		Ok(())
	}
}
