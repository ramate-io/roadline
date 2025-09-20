use crate::components::Dependency;
use crate::resources::{RenderUpdateEvent, Roadline};
use crate::RoadlineRenderConfig;
use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;
use bevy::sprite::{ColorMaterial, MeshMaterial2d};

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
					ribbon_between(start_pos, end_pos, noisy_control1, noisy_control2, 1.0, 64);
				let mesh_handle = meshes.add(ribbon_mesh);
				let material_handle = materials.add(ColorMaterial::from(Color::BLACK)); // Dark gray

				// Spawn the dependency curve
				commands.spawn((
					Dependency::new(*dependency_id),
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
	let _mid = (a + b) * 0.5;
	let dir = (b - a).normalize();
	let distance = (a - b).length();

	// Deterministic "random" offset based on anchor positions
	let seed = a.x + a.y + b.x + b.y;
	let noise1 = hash_f32(seed) - 0.5;
	let noise2 = hash_f32(seed * 1.37) - 0.5;

	// For elbow joints, we want predictable bumps
	// Determine if this is more horizontal or vertical
	let is_more_horizontal = dir.x.abs() > dir.y.abs();

	let strength = distance * 0.3; // 30% of distance for more pronounced elbows

	let (offset1, offset2) = if is_more_horizontal {
		// Horizontal line - bump vertically
		let vertical_offset = Vec3::new(0.0, noise1 * strength, 0.0);
		let vertical_offset2 = Vec3::new(0.0, noise2 * strength, 0.0);
		(vertical_offset, vertical_offset2)
	} else {
		// Vertical line - bump horizontally
		let horizontal_offset = Vec3::new(noise1 * strength, 0.0, 0.0);
		let horizontal_offset2 = Vec3::new(noise2 * strength, 0.0, 0.0);
		(horizontal_offset, horizontal_offset2)
	};

	// Position control points at 1/3 and 2/3 along the line for elbow effect
	let control1_pos = a + (b - a) * 0.33 + offset1;
	let control2_pos = a + (b - a) * 0.67 + offset2;

	(control1_pos, control2_pos)
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

		let v1 = p + normal * width * 0.5;
		let v2 = p - normal * width * 0.5;

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
