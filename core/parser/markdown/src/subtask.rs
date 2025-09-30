//! Subtask parsing functionality for markdown roadmap documents.

use super::error::MarkdownParseError;
use roadline_util::task::subtask::{
    Subtask, Id as SubtaskId, Position, Title, Content, Status, Lead
};

/// Parser for individual subtasks in markdown documents.
///
/// This parser handles the parsing of subtask entries from the Contents
/// section of task definitions.
#[derive(Debug, Clone)]
pub struct SubtaskParser {
    // Configuration for subtask parsing
    default_status: Status,
    default_lead: Lead,
}

impl Default for SubtaskParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SubtaskParser {
    /// Create a new subtask parser.
    pub fn new() -> Self {
        Self {
            default_status: Status::Incomplete,
            default_lead: Lead::new("Unknown".to_string(), "unknown@example.com".to_string()),
        }
    }

    /// Parse a subtask line from the Contents section.
    ///
    /// Expected format: "- **[T1.1](#t11-title)**: Description"
    pub fn parse_subtask_line(&self, line: &str) -> Result<Option<Subtask>, MarkdownParseError> {
        let line = line.trim();
        
        if !line.starts_with("- **") || !line.contains(":**") {
            return Ok(None);
        }

        // Extract the subtask ID and title from the markdown link
        let (subtask_id, title) = self.parse_subtask_header(line)?;
        
        // Extract the description
        let description = self.extract_description(line)?;
        
        // Create the subtask
        let subtask = Subtask::new(
            subtask_id.clone(),
            Position::new(0), // Use 0 as default position for now
            title,
            Content { text: description },
            self.default_status.clone(),
            self.default_lead.clone(),
        );

        Ok(Some(subtask))
    }

    /// Parse the subtask header to extract ID and title.
    ///
    /// Expected format: "**[T1.1](#t11-title)**:"
    fn parse_subtask_header(&self, line: &str) -> Result<(SubtaskId, Title), MarkdownParseError> {
        // Find the start of the markdown link
        let link_start = line.find("[")
            .ok_or_else(|| MarkdownParseError::InvalidSubtaskId {
                id: line.to_string(),
            })?;
        
        let link_end = line.find("]")
            .ok_or_else(|| MarkdownParseError::InvalidSubtaskId {
                id: line.to_string(),
            })?;

        // Extract the link text (e.g., "T1.1")
        let link_text = &line[link_start + 1..link_end];
        
        // Parse the subtask ID
        let subtask_id = self.parse_subtask_id(link_text)?;
        
        // For now, use the ID as the title. In a real implementation,
        // we might extract the title from the anchor link
        let title = Title { text: link_text.to_string() };

        Ok((subtask_id, title))
    }

    /// Parse a subtask ID string into a SubtaskId.
    ///
    /// Expected format: "T1.1", "T1.2", etc.
    fn parse_subtask_id(&self, id_str: &str) -> Result<SubtaskId, MarkdownParseError> {
        // For now, create a simple ID based on the string
        // In a real implementation, this would parse the actual ID structure
        let hash = self.hash_string(id_str);
        Ok(SubtaskId::new(hash))
    }

    /// Extract the description from a subtask line.
    ///
    /// This is everything after the ":**" part.
    fn extract_description(&self, line: &str) -> Result<String, MarkdownParseError> {
        let colon_pos = line.find(":**")
            .ok_or_else(|| MarkdownParseError::InvalidSubtaskTitle {
                title: line.to_string(),
            })?;

        let description = line[colon_pos + 4..].trim().to_string();
        Ok(description)
    }

    /// Simple hash function for string to u8 conversion.
    fn hash_string(&self, s: &str) -> u8 {
        let mut hash = 0u8;
        for byte in s.bytes() {
            hash = hash.wrapping_add(byte);
        }
        hash
    }
}

