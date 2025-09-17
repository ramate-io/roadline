use crate::components::Dependency;
use crate::resources::{RenderUpdateEvent, Roadline};
use crate::RoadlineRenderConfig;
use bevy::prelude::*;

/// Configuration for dependency systems
pub struct DependencySystemConfig;

impl DependencySystemConfig {
	/// Builds an owned closure for updating dependency renderers
	pub fn build() -> impl FnMut(
		Commands,
		EventReader<RenderUpdateEvent>,
		Option<Res<Roadline>>,
		Res<RoadlineRenderConfig>,
		Query<Entity, With<Dependency>>,
	) {
		move |mut _commands: Commands,
		      _render_events: EventReader<RenderUpdateEvent>,
		      _reified_opt: Option<Res<Roadline>>,
		      _config: Res<RoadlineRenderConfig>,
		      _existing_dependencies: Query<Entity, With<Dependency>>| {
			// TODO: Implement dependency rendering logic
		}
	}
}
