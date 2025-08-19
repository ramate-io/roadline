//! Graph implementation modules.
//! 
//! This module contains the implementation of graph operations split into logical components:
//! - `mutation`: Adding and removing tasks and dependencies
//! - `query`: Lookups, inspections, and basic graph queries  
//! - `traversal`: DFS, BFS, and path-finding algorithms
//! - `analysis`: Cycle detection, topological sorting, and structural analysis

use roadline_util::task::Id as TaskId;
use roadline_util::dependency::id::Id as DependencyId;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Errors thrown by the [Graph].
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
	#[error("Graph internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

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

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Self {
            facts: HashMap::new(),
        }
    }

    /// Creates a new graph with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            facts: HashMap::with_capacity(capacity),
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

pub mod mutation;
pub mod query;
pub mod traversal;
pub mod analysis;

#[cfg(test)]
pub mod tests;

#[cfg(test)]
pub mod test_utils;

// Re-export key types for convenience
// currently this is unnecessary because there aren't any new types
// pub use mutation::*;
// pub use query::*;
// pub use traversal::*;
// pub use analysis::*;