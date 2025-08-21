use roadline_util::dependency::id::Id as DependencyId;
use roadline_util::task::Id as TaskId;

use serde::{Deserialize, Serialize};

/// A predicate is the right side of a relationship between two task, i.e., the right side of a fact triple. 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Predicate {
    pub dependency_id: DependencyId,
    pub task_id: TaskId,
}
