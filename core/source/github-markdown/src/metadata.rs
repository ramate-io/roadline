//! Metadata collection for GitHub source instrumentation.

use std::collections::HashMap;
use roadline_parser_markdown::{Instrumentation, TaskParsedEvent};
use roadline_parser_markdown::error::MarkdownParseError;
use roadline_util::task::Id as TaskId;

/// Instrumentation that collects task metadata for GitHub source.
#[derive(Debug, Clone)]
pub struct GitHubMetadataCollector {
    /// Mapping from task ID to sanitized header fragment
    task_fragments: HashMap<TaskId, String>,
}

impl GitHubMetadataCollector {
    /// Create a new metadata collector.
    pub fn new() -> Self {
        Self {
            task_fragments: HashMap::new(),
        }
    }

    /// Get the fragment for a task ID.
    pub fn get_fragment(&self, task_id: &TaskId) -> Option<&String> {
        self.task_fragments.get(task_id)
    }

    /// Get all collected fragments.
    pub fn fragments(&self) -> &HashMap<TaskId, String> {
        &self.task_fragments
    }

    /// Sanitize a header line to create a GitHub fragment.
    ///
    /// This converts a markdown header like "### T1: Push Towards Validation"
    /// into a GitHub-compatible fragment like "t1-push-towards-validation"
    ///
    /// GitHub's algorithm:
    /// 1. Remove leading '#' characters and whitespace
    /// 2. Convert to lowercase
    /// 3. Keep alphanumeric characters, spaces, and hyphens
    /// 4. Replace spaces with hyphens
    /// 5. Remove consecutive hyphens
    /// 6. Strip leading/trailing hyphens
    ///
    /// Based on: https://github.com/lycheeverse/lychee/blob/d6c2bbe6f1e7b9e83889fc1e7fc675a38a7dd75f/lychee-lib/src/extract/markdown.rs#L177
    fn sanitize_header_to_fragment(header: &str) -> String {
        // Remove markdown header markers (###, ##, #)
        let cleaned = header
            .trim_start_matches('#')
            .trim();

        // GitHub's sanitization:
        // - Convert to lowercase
        // - Keep only alphanumeric, spaces, and hyphens
        // - Replace spaces with hyphens
        // - Remove punctuation except hyphens
        let mut result = String::new();
        let mut prev_was_hyphen = false;

        for c in cleaned.chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    result.push(c.to_ascii_lowercase());
                    prev_was_hyphen = false;
                }
                ' ' | '-' | '_' => {
                    if !prev_was_hyphen && !result.is_empty() {
                        result.push('-');
                        prev_was_hyphen = true;
                    }
                }
                _ => {
                    // Punctuation and special characters are dropped
                    // This includes colons, periods, commas, etc.
                }
            }
        }

        // Remove trailing hyphen if present
        result.trim_end_matches('-').to_string()
    }
}

impl Default for GitHubMetadataCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl Instrumentation for GitHubMetadataCollector {
    fn on_task_parsed(&self, _event: TaskParsedEvent) -> Result<(), MarkdownParseError> {
        // Note: We need to mutate self, but the trait doesn't allow that.
        // This is a limitation of the current design. In a real implementation,
        // we might need to use interior mutability (RefCell/Mutex) or change
        // the trait design.
        
        // For now, we'll implement this as a no-op and handle collection
        // differently in the GitHub source implementation.
        Ok(())
    }
}

/// A mutable version of the metadata collector that can actually collect data.
#[derive(Debug)]
pub struct MutableGitHubMetadataCollector {
    /// Mapping from task ID to sanitized header fragment
    task_fragments: HashMap<TaskId, String>,
}

impl MutableGitHubMetadataCollector {
    /// Create a new mutable metadata collector.
    pub fn new() -> Self {
        Self {
            task_fragments: HashMap::new(),
        }
    }

    /// Record a task with its header line.
    pub fn record_task(&mut self, task_id: TaskId, header_line: &str) {
        let fragment = Self::sanitize_header_to_fragment(header_line);
        self.task_fragments.insert(task_id, fragment);
    }

    /// Get the fragment for a task ID.
    pub fn get_fragment(&self, task_id: &TaskId) -> Option<&String> {
        self.task_fragments.get(task_id)
    }

    /// Get all collected fragments.
    pub fn fragments(&self) -> &HashMap<TaskId, String> {
        &self.task_fragments
    }

    /// Convert to immutable collector.
    pub fn into_immutable(self) -> GitHubMetadataCollector {
        GitHubMetadataCollector {
            task_fragments: self.task_fragments,
        }
    }

    /// Sanitize a header line to create a GitHub fragment.
    ///
    /// This converts a markdown header like "### T1: Push Towards Validation"
    /// into a GitHub-compatible fragment like "t1-push-towards-validation"
    ///
    /// GitHub's algorithm:
    /// 1. Remove leading '#' characters and whitespace
    /// 2. Convert to lowercase
    /// 3. Keep alphanumeric characters, spaces, and hyphens
    /// 4. Replace spaces with hyphens
    /// 5. Remove consecutive hyphens
    /// 6. Strip leading/trailing hyphens
    ///
    /// Based on: https://github.com/lycheeverse/lychee/blob/d6c2bbe6f1e7b9e83889fc1e7fc675a38a7dd75f/lychee-lib/src/extract/markdown.rs#L177
    fn sanitize_header_to_fragment(header: &str) -> String {
        // Remove markdown header markers (###, ##, #)
        let cleaned = header
            .trim_start_matches('#')
            .trim();

        // GitHub's sanitization:
        // - Convert to lowercase
        // - Keep only alphanumeric, spaces, and hyphens
        // - Replace spaces with hyphens
        // - Remove punctuation except hyphens
        let mut result = String::new();
        let mut prev_was_hyphen = false;

        for c in cleaned.chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    result.push(c.to_ascii_lowercase());
                    prev_was_hyphen = false;
                }
                ' ' | '-' | '_' => {
                    if !prev_was_hyphen && !result.is_empty() {
                        result.push('-');
                        prev_was_hyphen = true;
                    }
                }
                _ => {
                    // Punctuation and special characters are dropped
                    // This includes colons, periods, commas, etc.
                }
            }
        }

        // Remove trailing hyphen if present
        result.trim_end_matches('-').to_string()
    }
}

impl Default for MutableGitHubMetadataCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_header_to_fragment() {
        assert_eq!(
            MutableGitHubMetadataCollector::sanitize_header_to_fragment("### T1: Push Towards Validation"),
            "t1-push-towards-validation"
        );
        
        assert_eq!(
            MutableGitHubMetadataCollector::sanitize_header_to_fragment("## T2: Validation and Accepting Contributions"),
            "t2-validation-and-accepting-contributions"
        );
        
        assert_eq!(
            MutableGitHubMetadataCollector::sanitize_header_to_fragment("# T9: An Interlude"),
            "t9-an-interlude"
        );
        
        assert_eq!(
            MutableGitHubMetadataCollector::sanitize_header_to_fragment("### T1.1: Complete draft of OART-1: BFA"),
            "t11-complete-draft-of-oart-1-bfa"
        );
    }

    #[test]
    fn test_metadata_collector() {
        let mut collector = MutableGitHubMetadataCollector::new();
        let task_id = TaskId::new(1);
        
        collector.record_task(task_id, "### T1: Push Towards Validation");
        
        let fragment = collector.get_fragment(&task_id);
        assert_eq!(fragment, Some(&"t1-push-towards-validation".to_string()));
    }
}