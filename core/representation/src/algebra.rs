pub mod span;

pub use span::Span;

use crate::graph::Graph;
use roadline_util::task::id::Id as TaskId;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// A structure used to compute and store  the span algebra of a graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Algebra {
    /// The graph of the algebra is the graph of the tasks and dependencies.
    pub graph: Graph,
    /// The spans of the algebra are the spans of time for the tasks in the graph.
    pub spans: HashMap<TaskId, Span>,
}