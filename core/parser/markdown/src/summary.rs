//! Summary parsing functionality for markdown roadmap documents.

use roadline_util::task::Summary;

/// Parser for task summaries in markdown documents.
///
/// This parser handles the extraction of summary text from task content
/// before any subsections or metadata fields.
#[derive(Debug, Clone)]
pub struct SummaryParser {
    // Configuration for summary parsing
}

impl Default for SummaryParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SummaryParser {
    /// Create a new summary parser.
    pub fn new() -> Self {
        Self {}
    }

    /// Create a summary from the task content before any subsections.
    ///
    /// This method extracts descriptive text from the task content,
    /// stopping at the Contents section and skipping metadata fields.
    ///
    /// # Arguments
    /// * `content` - The content lines of the task section
    ///
    /// # Returns
    /// A `Summary` object containing the extracted text.
    pub fn parse(&self, content: &[String]) -> Summary {
        let mut summary_lines = Vec::new();

        for line in content {
            let line = line.trim();

            // Stop at the Contents section
            if line == "- **Contents:**" {
                break;
            }

            // Skip metadata fields (lines starting with "- **")
            if line.starts_with("- **") && line.contains(":**") {
                continue;
            }

            // Skip empty lines at the beginning
            if summary_lines.is_empty() && line.is_empty() {
                continue;
            }

            // Add content lines to summary
            if !line.is_empty() {
                summary_lines.push(line.to_string());
            }
        }

        let summary_text = summary_lines.join(" ").trim().to_string();
        Summary { text: summary_text }
    }
}
