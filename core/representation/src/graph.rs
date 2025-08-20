//! # Graph: Borrow-Based Graph Representation
//!
//! This module implements a graph representation that operates directly on borrowed 
//! Task and Dependency objects, rather than working with IDs. This design choice
//! reflects the philosophy that the graph structure is an **overlay** on existing
//! task and dependency data, not a separate data structure that owns the entities.
//!
//! ## Design Philosophy
//!
//! ### 1. Zero-Copy In-Memory Operations
//! The Graph is designed for efficient in-memory operations where we have a 
//! "frame" of live Task and Dependency objects. Instead of copying or cloning data,
//! we work directly with references, enabling zero-copy semantics for all graph
//! operations.
//!
//! ### 2. Graph as Structural Overlay
//! Rather than treating the graph as a container that owns tasks and dependencies,
//! we treat it as a **structural overlay** that describes relationships between
//! existing entities. This approach:
//! - Separates concerns: data storage vs. relationship modeling
//! - Enables multiple graph views over the same data
//! - Reduces memory overhead and eliminates data duplication
//! - Makes the graph structure ephemeral and reconstructable
//!
//! ### 3. Frame-Based Lifetime Management
//! The lifetime parameter `'a` ensures that the graph cannot outlive the frame
//! of data it references. This provides compile-time guarantees that all graph
//! operations will have valid data to work with, eliminating entire classes of
//! runtime errors.
//!
//! This module contains the implementation of graph operations split into logical components:
//! - `mutation`: Adding and removing tasks and dependencies
//! - `query`: Lookups, inspections, and basic graph queries  
//! - `traversal`: DFS, BFS, and path-finding algorithms
//! - `analysis`: Cycle detection, topological sorting, and structural analysis

use crate::frame::Frame;
use roadline_util::task::Task;
use roadline_util::dependency::Dependency;
use std::collections::HashMap;
use thiserror::Error;

/// Error types for Graph operations
#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Task not found in graph: {0:?}")]
    TaskNotFound(String),
    #[error("Dependency not found: {0:?}")]
    DependencyNotFound(String),
    #[error("Cycle detected in graph")]
    CycleDetected,
    #[error("Invalid graph state: {0}")]
    InvalidState(String),
    #[error("Graph operation failed: {0}")]
    OperationFailed(#[from] anyhow::Error),
}

/// A predicate represents the right side of a dependency relationship.
/// 
/// In the triple (subject_task, dependency, predicate_task), this struct
/// captures the dependency and target task. This design allows us to model
/// rich relationships where the dependency type provides semantic meaning
/// to the connection between tasks.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Predicate<'a> {
    /// The dependency that defines the relationship type/semantics
    pub dependency: &'a Dependency,
    /// The target task in this dependency relationship
    pub task: &'a Task,
}

/// A Graph represents task dependency relationships using borrowed references.
/// 
/// ## Core Design Principles
/// 
/// ### Overlay Architecture
/// The Graph acts as a structural overlay on existing Task and Dependency
/// objects. It doesn't own the data but describes how pieces relate to each other.
/// This separation allows:
/// - Independent lifecycle management of data vs. structure
/// - Multiple concurrent graph views over the same data
/// - Efficient reconstruction when data changes
/// 
/// ### Zero-Copy Operations
/// All graph operations work directly with borrowed data, eliminating:
/// - Memory allocations for temporary graph state
/// - Data copying during traversals and analysis
/// - Cache misses from scattered data layout
/// 
/// ### Frame-Bound Lifetime Safety
/// The lifetime parameter `'a` ensures the graph cannot outlive its data frame,
/// providing compile-time guarantees of memory safety without runtime overhead.
/// 
/// ## Implementation Notes
/// 
/// The facts HashMap maps from &Task (subject) to Vec<Predicate> (relationships).
/// This structure naturally supports:
/// - O(1) lookup of outgoing dependencies for any task
/// - Efficient iteration over all relationships
/// - Memory layout that respects reference locality
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph<'a> {
    /// Maps each task to its outgoing dependency relationships.
    /// 
    /// Structure: subject_task -> [(dependency, target_task), ...]
    /// This allows efficient lookup of "what does this task depend on?"
    pub facts: HashMap<&'a Task, Vec<Predicate<'a>>>,
}

impl<'a> Graph<'a> {
    /// Creates a new empty graph.
    /// 
    /// This creates an empty structural overlay ready to describe relationships
    /// between tasks and dependencies that will be provided later. The graph
    /// starts with no relationships and can be built incrementally.
    pub fn new() -> Self {
        Self {
            facts: HashMap::new(),
        }
    }

    /// Creates a new graph with pre-allocated capacity.
    /// 
    /// When you know approximately how many tasks will participate in the graph,
    /// this constructor can improve performance by reducing HashMap reallocations
    /// during graph construction.
    /// 
    /// # Arguments
    /// * `capacity` - Expected number of tasks that will have outgoing dependencies
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            facts: HashMap::with_capacity(capacity),
        }
    }

    /// Reads a frame into the graph.
    /// 
    /// This function reads a frame of tasks and dependencies and constructs a graph
    /// that describes the relationships between them.
    /// 
    /// # Arguments
    /// * `frame` - The frame to read from
    pub fn from_frame(frame: &'a Frame) -> Self {
        let mut graph = Self::new();

        // add all tasks to the graph
        for task in frame.tasks() {
            graph.add_task(task);
        }

        graph
    }
}

impl<'a> Default for Graph<'a> {
    /// Default constructor creates an empty graph.
    /// 
    /// Equivalent to Graph::new(), provided for consistency with Rust conventions.
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