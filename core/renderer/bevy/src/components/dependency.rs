use bevy::prelude::*;
use roadline_util::dependency::Id as DependencyId;

/// Component that marks an entity as a dependency
#[derive(Component, Debug)]
pub struct Dependency {
	pub dependency_id: DependencyId,
}

impl Dependency {
	pub fn new(dependency_id: DependencyId) -> Self {
		Self { dependency_id }
	}
}
