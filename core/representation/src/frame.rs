use roadline_util::task::Task;
use roadline_util::dependency::Dependency;
use thiserror::Error;

/// Error types for FrameGraph operations
#[derive(Error, Debug)]
pub enum FrameError {
    #[error("Internal frame error: {0:?}")]
    Internal(Box<dyn std::error::Error>),
}

/// A frame is a collection of tasks and dependencies.
/// 
/// Other representation APIs are borrow-based and will essentially be a view of a frame.
pub struct Frame {
    /// The tasks in the frame.
    pub tasks: Vec<Task>,
    /// The dependencies in the frame.
    pub dependencies: Vec<Dependency>,
}


impl Frame {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    /// Adds a task to the frame.
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Adds a dependency to the frame.
    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }

    /// Iterates over the tasks in the frame.
    pub fn tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }

    /// Iterates over the dependencies in the frame.
    pub fn dependencies(&self) -> impl Iterator<Item = &Dependency> {
        self.dependencies.iter()
    }
}