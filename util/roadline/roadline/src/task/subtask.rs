pub mod title;
pub mod content;
pub mod status;
pub mod lead;
pub mod position;
pub mod id;

pub use id::Id;
pub use position::Position;
pub use title::Title;
pub use content::Content;
pub use status::Status;
pub use lead::Lead;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subtask {
    /// The id of the subtask.
    pub id: Id,
    /// The position of the subtask. This also serves as the id of the subtask.
    pub position: Position,
    /// The title of the subtask.
    pub title: Title,
    /// The content of the subtask.
    pub content: Content,
    /// The status of the subtask.
    pub status: Status,
    /// The lead of the subtask.
    pub lead: Lead,
}

impl Subtask {
    pub fn new(id: Id, position: Position, title: Title, content: Content, status: Status, lead: Lead) -> Self {
        Self { id, position, title, content, status, lead }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
    
    pub fn title(&self) -> &Title {
        &self.title
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
    
    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn lead(&self) -> &Lead {
        &self.lead
    }
    
    
}