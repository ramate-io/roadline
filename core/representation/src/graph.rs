use roadline_util::task::Id as TaskId;
use roadline_util::dependency::id::Id as DependencyId;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A predicate is the right side of a relationship between two task, i.e., the right side of a fact triple. 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Predicate {
    pub dependency_id: DependencyId,
    pub task_id: TaskId,
}

/// A graph is a collection of facts, wherein the the left subject of the fact is a task which is mapped to a list of predicates.
/// 
/// We use separate backing tables for full tasks and dependencies
/// because we want this to be able to perform efficiently in contexts such as a rendered.
/// 
/// We also generally want a common memory frame from which the full semantic graph can be reconstructed.
/// While this graph merely represents the structure of relationships between tasks. 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Graph {
    pub facts: HashMap<TaskId, Vec<Predicate>>,
}