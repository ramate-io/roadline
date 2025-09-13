//! Graph implementation modules.
//!
//! This module contains the implementation of graph operations split into logical components:
//! - `mutation`: Adding and removing tasks and dependencies
//! - `query`: Lookups, inspections, and basic graph queries  
//! - `traversal`: DFS, BFS, and path-finding algorithms
//! - `analysis`: Cycle detection, topological sorting, and structural analysis

pub mod operations;
pub mod predicate;

pub use predicate::Predicate;

use crate::arena::Arena;
use roadline_util::dependency::{Dependency, Id as DependencyId};
use roadline_util::task::{Id as TaskId, Task};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Errors thrown by the [Graph].
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
	#[error("Graph internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A graph is a collection of facts, wherein the the left subject of the fact is a task which is mapped to a list of predicates.
///
/// We use separate backing tables for full tasks and dependencies
/// because we want this to be able to perform efficiently in contexts such as a rendered.
///
/// We also generally want a common memory frame from which the full semantic graph can be reconstructed.
/// While this graph merely represents the structure of relationships between tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
	/// The arena of the graph is the arena of the tasks and dependencies.
	pub arena: Arena,
	/// The facts of the graph are the predicates that relate tasks to each other.
	pub facts: HashMap<TaskId, Vec<Predicate>>,
}

impl Graph {
	/// Creates a new empty graph.
	pub fn new() -> Self {
		Self { arena: Arena::new(), facts: HashMap::new() }
	}

	/// Creates a new graph with the specified capacity.
	pub fn with_capacity(capacity: usize) -> Self {
		Self { arena: Arena::with_capacity(capacity), facts: HashMap::with_capacity(capacity) }
	}

	/// Borrows the arena of the graph.
	pub fn arena(&self) -> &Arena {
		&self.arena
	}

	/// Gets the task for a given task id.
	pub fn task(&self, task_id: &TaskId) -> Option<&Task> {
		self.arena.tasks().get(task_id)
	}

	/// Gets the dependency for a given dependency id.
	pub fn dependency(&self, dependency_id: &DependencyId) -> Option<&Dependency> {
		self.arena.dependencies().get(dependency_id)
	}

	/// Consumes the graph and returns the arena and facts.
	pub fn into_parts(self) -> (Arena, HashMap<TaskId, Vec<Predicate>>) {
		(self.arena, self.facts)
	}
}

impl Default for Graph {
	fn default() -> Self {
		Self::new()
	}
}
