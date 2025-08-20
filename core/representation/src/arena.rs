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
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}