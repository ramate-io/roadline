use crate::error::MarkdownParseError;
use roadline_util::task::Id as TaskId;

#[derive(Debug, Clone)]
pub struct TaskParsedEvent {
	pub task: TaskId,
	pub line_string: String,
}

pub trait Instrumentation {
	fn on_task_parsed(&self, task: TaskParsedEvent) -> Result<(), MarkdownParseError>;
}
