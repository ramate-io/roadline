use roadline_util::task::Task;
use roadline_util::dependency::Dependency;
use std::collections::HashMap;

/// A predicate is the right side of a relationship between two task, i.e., the right side of a fact triple. 
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Predicate<'a> {
    pub dependency: &'a Dependency,
    pub task: &'a Task,
}

/// A frame graph is a collection of facts, wherein the the left subject of the fact is a task which is mapped to a list of predicates.
/// 
/// A frame graph uses borrowed versions of the Tasks and Dependencies thus requiring 
/// that a live "frame" is available. 
/// 
/// The frame graph is useful for enforcing zero-copy semantics. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameGraph<'a> {
    pub facts: HashMap<&'a Task, Vec<Predicate<'a>>>,
}