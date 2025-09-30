//! Comprehensive tests for GitHub source functionality.

use crate::{GitHubSource, GitHubUrl, GitHubUrlType};

/// Test URL for OROAD-0 document
const OROAD_0_RAW_URL: &str = "https://raw.githubusercontent.com/ramate-io/oac/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";
const OROAD_0_TREE_URL: &str = "https://github.com/ramate-io/oac/tree/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";
const OROAD_0_BLOB_URL: &str = "https://github.com/ramate-io/oac/blob/refs/heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";
const OROAD_0_REPO_PATH: &str =
	"ramate-io/oac/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md";

#[cfg(test)]
mod integration_tests {
	use super::*;

	#[tokio::test]
	async fn test_oroad_0_from_raw_url() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();
		let roadline = source.from_raw_url(OROAD_0_RAW_URL).await?;
		let roadline = roadline.build()?;

		// OROAD-0 should have multiple tasks (T1-T9)
		let tasks = roadline.graph().arena.tasks();
		assert!(tasks.len() >= 9, "Expected at least 9 tasks, got {}", tasks.len());

		Ok(())
	}

	#[tokio::test]
	async fn test_oroad_0_from_tree_url() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();
		let roadline = source.from_tree_url(OROAD_0_TREE_URL).await?;
		let roadline = roadline.build()?;

		// Should have the same content as raw URL
		let tasks = roadline.graph().arena.tasks();
		assert!(tasks.len() >= 9, "Expected at least 9 tasks, got {}", tasks.len());
		Ok(())
	}

	#[tokio::test]
	async fn test_oroad_0_from_blob_url() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();
		let roadline = source.from_blob_url(OROAD_0_BLOB_URL).await?;
		let roadline = roadline.build()?;

		// Should have the same content as other URLs
		let tasks = roadline.graph().arena.tasks();
		assert!(tasks.len() >= 9, "Expected at least 9 tasks, got {}", tasks.len());
		Ok(())
	}

	#[tokio::test]
	async fn test_oroad_0_from_repository_path() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();
		let roadline = source.from_repository_path(OROAD_0_REPO_PATH).await?;
		let roadline = roadline.build()?;

		// Should have the same content as other URLs
		let tasks = roadline.graph().arena.tasks();
		assert!(tasks.len() >= 9, "Expected at least 9 tasks, got {}", tasks.len());
		Ok(())
	}

	#[tokio::test]
	async fn test_all_url_formats_produce_same_result() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();

		let raw_result = source.from_raw_url(OROAD_0_RAW_URL).await?;
		let tree_result = source.from_tree_url(OROAD_0_TREE_URL).await?;
		let blob_result = source.from_blob_url(OROAD_0_BLOB_URL).await?;
		let repo_result = source.from_repository_path(OROAD_0_REPO_PATH).await?;

		// Build all roadlines
		let raw_roadline = raw_result.build()?;
		let tree_roadline = tree_result.build()?;
		let blob_roadline = blob_result.build()?;
		let repo_roadline = repo_result.build()?;

		// All should have the same number of tasks
		assert_eq!(
			raw_roadline.graph().arena.tasks().len(),
			tree_roadline.graph().arena.tasks().len()
		);
		assert_eq!(
			tree_roadline.graph().arena.tasks().len(),
			blob_roadline.graph().arena.tasks().len()
		);
		assert_eq!(
			blob_roadline.graph().arena.tasks().len(),
			repo_roadline.graph().arena.tasks().len()
		);

		// All should have the same task titles (sort for deterministic comparison)
		let mut raw_titles: Vec<String> = raw_roadline
			.graph()
			.arena
			.tasks()
			.iter()
			.map(|(_, task)| task.title().text.clone())
			.collect();
		raw_titles.sort();

		let mut tree_titles: Vec<String> = tree_roadline
			.graph()
			.arena
			.tasks()
			.iter()
			.map(|(_, task)| task.title().text.clone())
			.collect();
		tree_titles.sort();

		let mut blob_titles: Vec<String> = blob_roadline
			.graph()
			.arena
			.tasks()
			.iter()
			.map(|(_, task)| task.title().text.clone())
			.collect();
		blob_titles.sort();

		let mut repo_titles: Vec<String> = repo_roadline
			.graph()
			.arena
			.tasks()
			.iter()
			.map(|(_, task)| task.title().text.clone())
			.collect();
		repo_titles.sort();

		assert_eq!(raw_titles, tree_titles);
		assert_eq!(tree_titles, blob_titles);
		assert_eq!(blob_titles, repo_titles);
		Ok(())
	}
}

#[cfg(test)]
mod url_parsing_tests {
	use super::*;

	#[test]
	fn test_parse_oroad_0_raw_url() {
		let (parsed, url_type) = GitHubUrl::parse(OROAD_0_RAW_URL).unwrap();

		assert_eq!(url_type, GitHubUrlType::Raw);
		assert_eq!(parsed.owner, "ramate-io");
		assert_eq!(parsed.repo, "oac");
		assert_eq!(parsed.reference, "refs");
		assert_eq!(
			parsed.path,
			"heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md"
		);
	}

	#[test]
	fn test_parse_oroad_0_tree_url() {
		let (parsed, url_type) = GitHubUrl::parse(OROAD_0_TREE_URL).unwrap();

		assert_eq!(url_type, GitHubUrlType::Tree);
		assert_eq!(parsed.owner, "ramate-io");
		assert_eq!(parsed.repo, "oac");
		assert_eq!(parsed.reference, "refs");
		assert_eq!(
			parsed.path,
			"heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md"
		);
	}

	#[test]
	fn test_parse_oroad_0_blob_url() {
		let (parsed, url_type) = GitHubUrl::parse(OROAD_0_BLOB_URL).unwrap();

		assert_eq!(url_type, GitHubUrlType::Blob);
		assert_eq!(parsed.owner, "ramate-io");
		assert_eq!(parsed.repo, "oac");
		assert_eq!(parsed.reference, "refs");
		assert_eq!(
			parsed.path,
			"heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md"
		);
	}

	#[test]
	fn test_parse_oroad_0_repository_path() {
		let (parsed, url_type) = GitHubUrl::parse(OROAD_0_REPO_PATH).unwrap();

		assert_eq!(url_type, GitHubUrlType::Repository);
		assert_eq!(parsed.owner, "ramate-io");
		assert_eq!(parsed.repo, "oac");
		assert_eq!(parsed.reference, "main"); // Default to main
		assert_eq!(parsed.path, "oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md");
	}

	#[test]
	fn test_url_conversion() {
		let (parsed, _) = GitHubUrl::parse(OROAD_0_RAW_URL).unwrap();

		// Test conversion to raw URL
		let raw_url = parsed.to_raw_url();
		assert_eq!(raw_url, OROAD_0_RAW_URL);

		// Test conversion to API URL
		let api_url = parsed.to_api_url();
		assert!(api_url.contains("api.github.com"));
		assert!(api_url.contains("ramate-io/oac"));
		assert!(
			api_url.contains("heads/main/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md")
		);
		assert!(api_url.contains("ref=refs"));
	}
}

#[cfg(test)]
mod error_handling_tests {
	use super::*;

	#[test]
	fn test_invalid_url_formats() {
		let invalid_urls = vec![
			"https://example.com/file.md",
			"not-a-url",
			"https://github.com/owner/repo",
			"https://raw.githubusercontent.com/owner",
		];

		for url in invalid_urls {
			let result = GitHubUrl::parse(url);
			assert!(result.is_err(), "Should fail to parse invalid URL: {}", url);
		}
	}

	#[tokio::test]
	async fn test_nonexistent_repository() {
		let source = GitHubSource::new();
		let result = source
			.from_raw_url("https://raw.githubusercontent.com/nonexistent/repo/main/README.md")
			.await;

		assert!(result.is_err(), "Should fail to fetch from nonexistent repository");
	}

	#[tokio::test]
	async fn test_nonexistent_file() {
		let source = GitHubSource::new();
		let result = source
			.from_raw_url("https://raw.githubusercontent.com/ramate-io/oac/main/nonexistent.md")
			.await;

		assert!(result.is_err(), "Should fail to fetch nonexistent file");
	}

	#[tokio::test]
	async fn test_metadata_collection() -> Result<(), Box<dyn std::error::Error>> {
		let source = GitHubSource::new();
		let (github_url, _) = GitHubUrl::parse(OROAD_0_RAW_URL)?;

		// Parse with metadata collection
		let (builder, metadata) = source.from_github_url_with_metadata(&github_url).await?;
		let roadline = builder.build()?;

		// Verify we have tasks
		assert!(!roadline.graph().arena.tasks().is_empty());

		// Verify we have metadata for each task
		assert_eq!(metadata.fragments().len(), roadline.graph().arena.tasks().len());

		// Check that fragments are properly sanitized
		for (_task_id, fragment) in metadata.fragments() {
			// Fragment should be lowercase and use hyphens
			assert!(fragment
				.chars()
				.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'));
			// Fragment should not start or end with hyphens
			assert!(!fragment.starts_with('-'));
			assert!(!fragment.ends_with('-'));
			// Fragment should not have consecutive hyphens
			assert!(!fragment.contains("--"));

			// Verify we can create a GitHub URL with this fragment
			let fragment_url = github_url.to_blob_url_with_fragment(fragment);
			assert!(fragment_url.contains('#'));
			assert!(fragment_url.ends_with(fragment));
		}

		Ok(())
	}
}
