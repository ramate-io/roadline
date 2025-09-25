use crate::components::{Dependency, SelectionState};
use crate::resources::SelectionResource;
use bevy::prelude::*;
use bevy::sprite::ColorMaterial;

use crate::systems::dependency::{DependencyCurveData, DependencyHoverable};

/// Helper struct for edge selection logic
#[derive(Debug, Clone, Copy)]
struct EdgeSelection {
	from: SelectionState,
	to: SelectionState,
}

impl EdgeSelection {
	/// Create an EdgeSelection from two SelectionStates
	fn new(from: SelectionState, to: SelectionState) -> Self {
		Self { from, to }
	}

	/// Check if this edge is connected to any selection (selected, descendant, or parent)
	fn is_connected_to_selection(&self) -> bool {
		self.from == SelectionState::Selected
			|| self.from == SelectionState::Descendant
			|| (self.from == SelectionState::Parent && self.to == SelectionState::Selected)
			|| (self.from == SelectionState::Parent && self.to == SelectionState::Descendant)
			|| (self.from == SelectionState::Parent && self.to == SelectionState::Parent)
	}

	/// Check if this edge should be colored red (parent-related)
	fn should_be_red(&self) -> bool {
		(self.from == SelectionState::Parent && self.to == SelectionState::Selected)
			|| (self.from == SelectionState::Parent && self.to == SelectionState::Descendant)
			|| (self.from == SelectionState::Parent && self.to == SelectionState::Parent)
	}
}

/// Configuration for dependency hover systems
#[derive(Debug, Clone, Resource)]
pub struct DependencyHoverSystem {
	pub hover_distance_threshold: f32,
	pub default_color: Color,
	pub hover_color: Color,
	pub selected_color: Color,
	pub parent_color: Color,
}

impl Default for DependencyHoverSystem {
	fn default() -> Self {
		Self {
			hover_distance_threshold: 30.0,
			default_color: Color::BLACK,
			hover_color: Color::oklch(0.5, 0.137, 235.06), // Dark blue
			selected_color: Color::oklch(0.5, 0.137, 235.06), // Dark blue
			parent_color: Color::oklch(0.5, 0.137, 0.0), // Red
		}
	}
}

impl DependencyHoverSystem {
	/// Builds a system function for handling dependency hover effects
	pub fn build(
		self,
	) -> impl FnMut(
		ResMut<Assets<ColorMaterial>>,
		Query<
			(&Camera, &GlobalTransform),
			(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
		>,
		Query<&Window>,
		Query<
			(Entity, &Transform, &MeshMaterial2d<ColorMaterial>, &DependencyCurveData, &Dependency),
			With<DependencyHoverable>,
		>,
		Res<SelectionResource>,
	) {
		move |mut materials: ResMut<Assets<ColorMaterial>>,
		      camera_query: Query<
				(&Camera, &GlobalTransform),
				(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
			>,
		      windows: Query<&Window>,
		      dependency_query: Query<
				(Entity, &Transform, &MeshMaterial2d<ColorMaterial>, &DependencyCurveData, &Dependency),
				With<DependencyHoverable>,
			>,
		      selection_resource: Res<SelectionResource>| {
			self.handle_dependency_hovers(
				&mut materials,
				&camera_query,
				&windows,
				&dependency_query,
				&selection_resource,
			);
		}
	}

	/// Handle dependency hover effects and selection coloring
	pub fn handle_dependency_hovers(
		&self,
		materials: &mut ResMut<Assets<ColorMaterial>>,
		camera_query: &Query<
			(&Camera, &GlobalTransform),
			(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
		>,
		windows: &Query<&Window>,
		dependency_query: &Query<
			(Entity, &Transform, &MeshMaterial2d<ColorMaterial>, &DependencyCurveData, &Dependency),
			With<DependencyHoverable>,
		>,
		selection_resource: &Res<SelectionResource>,
	) {
		// Get camera and window info
		let Ok((camera, camera_transform)) = camera_query.single() else {
			return;
		};
		let Ok(window) = windows.single() else {
			return;
		};

		// Get mouse position
		if let Some(cursor_position) = window.cursor_position() {
			// Convert screen coordinates to world coordinates
			let world_pos_result = camera.viewport_to_world_2d(camera_transform, cursor_position);
			if let Ok(world_pos) = world_pos_result {
				// Check each dependency curve for hover and selection
				for (_entity, _transform, mesh_material, curve_data, dependency) in
					dependency_query.iter()
				{
					// Create edge selection helper
					let from_task_state =
						selection_resource.get_task_state(&dependency.dependency_id.from());
					let to_task_state =
						selection_resource.get_task_state(&dependency.dependency_id.to());
					let edge_selection = EdgeSelection::new(from_task_state, to_task_state);

					// Calculate distance to the bezier curve for hover detection
					let mouse_pos_3d = Vec3::new(world_pos.x, world_pos.y, 0.0);
					let distance_to_curve = distance_to_bezier_curve(
						mouse_pos_3d,
						curve_data.start,
						curve_data.control1,
						curve_data.control2,
						curve_data.end,
					);

					// Determine the color based on selection state and hover
					let new_color = if edge_selection.should_be_red() {
						// If connected to parent task, show red
						self.parent_color
					} else if edge_selection.is_connected_to_selection() {
						// If connected to selection/descendant, show dark blue
						self.selected_color
					} else if distance_to_curve < self.hover_distance_threshold {
						// If hovering and not connected to selection, show dark blue
						self.hover_color
					} else {
						// Default black
						self.default_color
					};

					// Apply the color
					if let Some(material) = materials.get_mut(&mesh_material.0) {
						material.color = new_color;
					}
				}
			}
		}
	}
}

/// Calculate the distance from a point to a cubic bezier curve
fn distance_to_bezier_curve(point: Vec3, p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> f32 {
	// Sample points along the curve and find the minimum distance
	let mut min_distance = f32::INFINITY;
	let steps = 32; // Number of samples along the curve

	for i in 0..=steps {
		let t = i as f32 / steps as f32;
		let curve_point = cubic_bezier(p0, p1, p2, p3, t);
		let distance = point.distance(curve_point);
		min_distance = min_distance.min(distance);
	}

	min_distance
}

// === cubic bezier helper ===
fn cubic_bezier(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
	let u = 1.0 - t;
	u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}
