pub mod cursor_interaction;
pub mod spawning;

use bevy::prelude::*;

pub use cursor_interaction::hovers::DependencyHoverSystem;
pub use spawning::DependencySpawningSystem;

#[derive(Component)]
pub struct DependencyHoverable;

#[derive(Component)]
pub struct DependencyCurveData {
	pub start: Vec3,
	pub end: Vec3,
	pub control1: Vec3,
	pub control2: Vec3,
}
