use crate::components::{Dependency, SelectionState};
use crate::resources::{RenderUpdateEvent, Roadline, SelectionResource};
use crate::RoadlineRenderConfig;
use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;
use bevy::sprite::{ColorMaterial, MeshMaterial2d};

#[derive(Component)]
pub struct DependencyHoverable;

#[derive(Component)]
pub struct DependencyCurveData {
	pub start: Vec3,
	pub end: Vec3,
	pub control1: Vec3,
	pub control2: Vec3,
}

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
		ResMut<Assets<Mesh>>,
		ResMut<Assets<ColorMaterial>>,
	) {
		move |mut commands: Commands,
		      render_events: EventReader<RenderUpdateEvent>,
		      reified_opt: Option<Res<Roadline>>,
		      _config: Res<RoadlineRenderConfig>,
		      existing_dependencies: Query<Entity, With<Dependency>>,
		      mut meshes: ResMut<Assets<Mesh>>,
		      mut materials: ResMut<Assets<ColorMaterial>>| {
			// Only update if we received a render event and have reified data
			if render_events.is_empty() || reified_opt.is_none() {
				return;
			}

			let reified = reified_opt.unwrap();

			// Clear existing dependency entities
			for entity in existing_dependencies.iter() {
				commands.entity(entity).despawn();
			}

			// Get the visual bounds to scale everything properly
			let (max_width, max_height) = reified.visual_bounds();
			let max_width_f32 = max_width.value() as f32;
			let max_height_f32 = max_height.value() as f32;

			// Scale factor: same as tasks
			let pixels_per_unit = 50.0;

			// Calculate offsets to center the content around (0,0)
			let content_width_pixels = max_width_f32 * pixels_per_unit;
			let content_height_pixels = max_height_f32 * pixels_per_unit;
			let offset_x = -content_width_pixels / 2.0;
			let offset_y = -content_height_pixels / 2.0;

			// Create dependency curves for each bezier curve
			for (dependency_id, start_point, end_point, control1, control2) in
				reified.bezier_curves()
			{
				println!(
					"dependency_id: {:?}, start: {:?}, end: {:?}, control1: {:?}, control2: {:?}",
					dependency_id, start_point, end_point, control1, control2
				);

				// Convert reified units to pixel coordinates
				let start_pos = Vec3::new(
					start_point.x.value() as f32 * pixels_per_unit + offset_x,
					start_point.y.value() as f32 * pixels_per_unit + offset_y,
					0.0,
				);
				let end_pos = Vec3::new(
					end_point.x.value() as f32 * pixels_per_unit + offset_x,
					end_point.y.value() as f32 * pixels_per_unit + offset_y,
					0.0,
				);
				let control1_pos = Vec3::new(
					control1.x.value() as f32 * pixels_per_unit + offset_x,
					control1.y.value() as f32 * pixels_per_unit + offset_y,
					0.0,
				);
				let control2_pos = Vec3::new(
					control2.x.value() as f32 * pixels_per_unit + offset_x,
					control2.y.value() as f32 * pixels_per_unit + offset_y,
					0.0,
				);

				println!(
					"start_pos: {:?}, end_pos: {:?}, control1_pos: {:?}, control2_pos: {:?}",
					start_pos, end_pos, control1_pos, control2_pos
				);

				// Generate noisy control points for elbow joint behavior
				let (noisy_control1, noisy_control2) = control_points(start_pos, end_pos);

				// Create ribbon mesh for this dependency
				let ribbon_mesh =
					ribbon_between(start_pos, end_pos, noisy_control1, noisy_control2, 2.0, 64);
				let mesh_handle = meshes.add(ribbon_mesh);
				let material_handle = materials.add(ColorMaterial::from(Color::BLACK)); // Dark gray

				// Spawn the dependency curve
				commands.spawn((
					Dependency::new(*dependency_id),
					DependencyHoverable,
					DependencyCurveData {
						start: start_pos,
						end: end_pos,
						control1: noisy_control1,
						control2: noisy_control2,
					},
					Mesh2d(mesh_handle),
					MeshMaterial2d(material_handle),
					Transform::default(),
					Visibility::Visible,
					RenderLayers::layer(2),
				));
			}
		}
	}
}

// === cubic bezier helper ===
fn cubic_bezier(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
	let u = 1.0 - t;
	u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}

// === tiny hash function for deterministic "noise" ===
fn hash_f32(x: f32) -> f32 {
	(x.sin() * 43758.5453).fract()
}

/// Generate noisy control points for elbow joint behavior
fn control_points(a: Vec3, b: Vec3) -> (Vec3, Vec3) {
	let midpoint = (a + b) * 0.5;
	let distance = (a - b).length();

	// Deterministic "random" offset based on anchor positions
	let seed = a.x + a.y + b.x + b.y;
	let noise1 = hash_f32(seed) - 0.5;
	let noise2 = hash_f32(seed * 1.37) - 0.5;

	// Create elbow joints: control_1 = (midpoint_x, start_y), control_2 = (midpoint_x, end_y)
	// Add more variation around midpoint_x, less variation in y
	let midpoint_noise_strength = distance * 0.05; // More variation in midpoint x position
	let y_noise_strength = distance * 0.005; // Much less variation in y positions

	let control1 = Vec3::new(
		midpoint.x + 0.25 * distance + noise1 * midpoint_noise_strength,
		a.y + noise1 * y_noise_strength,
		0.0,
	);

	let control2 = Vec3::new(
		midpoint.x - 0.25 * distance + noise2 * midpoint_noise_strength,
		b.y + noise2 * y_noise_strength,
		0.0,
	);

	(control1, control2)
}

/// Build a ribbon mesh along a cubic bezier curve using provided control points
fn ribbon_between(
	start: Vec3,
	end: Vec3,
	control1: Vec3,
	control2: Vec3,
	width: f32,
	steps: usize,
) -> Mesh {
	// sample points along bezier
	let mut points = Vec::new();
	for i in 0..=steps {
		let t = i as f32 / steps as f32;
		points.push(cubic_bezier(start, control1, control2, end, t));
	}

	// build mesh data
	let mut positions: Vec<[f32; 3]> = Vec::new();
	let mut uvs: Vec<[f32; 2]> = Vec::new();
	let mut indices: Vec<u32> = Vec::new();

	for i in 0..points.len() {
		if i == points.len() - 1 {
			break;
		}
		let p = points[i];
		let tangent = (points[i + 1] - p).normalize();
		let normal = Vec3::new(-tangent.y, tangent.x, 0.0).normalize(); // 2D sideways

		// Add ribboning variation - width changes along the curve with phase offset
		let t = i as f32 / (points.len() - 1) as f32;
		let phase_offset = (start.x + start.y + end.x + end.y) * 0.1; // Phase offset based on curve position
		let ribbon_variation = 1.0 + 0.3 * ((t * 6.28) + phase_offset).sin(); // Sine wave with offset
		let current_width = width * ribbon_variation;

		let v1 = p + normal * current_width * 0.5;
		let v2 = p - normal * current_width * 0.5;

		positions.push(v1.into());
		positions.push(v2.into());

		// simple UVs along length
		let u = i as f32 / (points.len() - 1) as f32;
		uvs.push([u, 0.0]);
		uvs.push([u, 1.0]);

		if i < points.len() - 2 {
			let base = (i * 2) as u32;
			indices.extend_from_slice(&[base, base + 1, base + 2, base + 1, base + 3, base + 2]);
		}
	}

	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
	mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
	mesh.insert_indices(Indices::U32(indices));
	mesh
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

/// System to handle hover effects and selection coloring on dependency curves
pub fn dependency_hover_system(
	mut materials: ResMut<Assets<ColorMaterial>>,
	camera_query: Query<
		(&Camera, &GlobalTransform),
		(With<Camera2d>, Without<bevy::ui::IsDefaultUiCamera>),
	>,
	windows: Query<&Window>,
	dependency_query: Query<
		(Entity, &Transform, &MeshMaterial2d<ColorMaterial>, &DependencyCurveData, &Dependency),
		With<DependencyHoverable>,
	>,
	selection_resource: Res<SelectionResource>,
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
				// Check if this dependency starts from a selected/descendant/parent task
				let from_task_state =
					selection_resource.get_task_state(&dependency.dependency_id.from());
				let to_task_state =
					selection_resource.get_task_state(&dependency.dependency_id.to());
				let is_connected_to_selection = from_task_state == SelectionState::Selected
					|| from_task_state == SelectionState::Descendant
					|| (from_task_state == SelectionState::Parent
						&& to_task_state == SelectionState::Selected)
					|| (from_task_state == SelectionState::Parent
						&& to_task_state == SelectionState::Selected)
					|| (from_task_state == SelectionState::Parent
						&& to_task_state == SelectionState::Parent);

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
				let new_color = if (from_task_state == SelectionState::Parent
					&& to_task_state == SelectionState::Selected)
					|| (from_task_state == SelectionState::Parent
						&& to_task_state == SelectionState::Selected)
					|| (from_task_state == SelectionState::Parent
						&& to_task_state == SelectionState::Parent)
				{
					// If connected to parent task, show red
					Color::oklch(0.5, 0.137, 0.0) // Red
				} else if is_connected_to_selection {
					// If connected to selection/descendant, show dark blue
					Color::oklch(0.5, 0.137, 235.06) // Dark blue
				} else if distance_to_curve < 30.0 {
					// If hovering and not connected to selection, show dark blue
					Color::oklch(0.5, 0.137, 235.06) // Dark blue
				} else {
					// Default black
					Color::BLACK
				};

				// Apply the color
				if let Some(material) = materials.get_mut(&mesh_material.0) {
					material.color = new_color;
				}
			}
		}
	}
}
