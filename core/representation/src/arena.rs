use roadline_util::task::{Task, Id as TaskId};
use roadline_util::dependency::{Dependency, Id as DependencyId};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arena {
    pub tasks: HashMap<TaskId, Task>,
    pub dependencies: HashMap<DependencyId, Dependency>,
}

impl Arena {
    pub fn new() -> Self {
        Self { tasks: HashMap::new(), dependencies: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { tasks: HashMap::with_capacity(capacity), dependencies: HashMap::with_capacity(capacity) }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(*task.id(), task);
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.insert(*dependency.id(), dependency);
    }

    pub fn tasks(&self) -> &HashMap<TaskId, Task> {
        &self.tasks
    }

    pub fn dependencies(&self) -> &HashMap<DependencyId, Dependency> {
        &self.dependencies
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}