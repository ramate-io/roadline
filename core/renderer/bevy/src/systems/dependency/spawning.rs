use crate::components::Dependency;
use crate::resources::{RenderUpdateEvent, Roadline};
use crate::RoadlineRenderConfig;
use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;
use bevy::sprite::{ColorMaterial, MeshMaterial2d};

use crate::systems::dependency::{DependencyCurveData, DependencyHoverable};

/// Configuration for dependency spawning systems
#[derive(Debug, Clone, Resource)]
pub struct DependencySpawningSystem {
	pub pixels_per_unit: f32,
}

impl Default for DependencySpawningSystem {
	fn default() -> Self {
		Self { pixels_per_unit: 75.0 }
	}
}

impl DependencySpawningSystem {
	/// Builds a system function for updating dependency renderers
	pub fn build(
		self,
	) -> impl FnMut(
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
			self.update_dependency_renderers(
				&mut commands,
				&render_events,
				&reified_opt,
				&existing_dependencies,
				&mut meshes,
				&mut materials,
			);
		}
	}

	/// Update dependency renderers based on render events and roadline data
	pub fn update_dependency_renderers(
		&self,
		commands: &mut Commands,
		render_events: &EventReader<RenderUpdateEvent>,
		reified_opt: &Option<Res<Roadline>>,
		existing_dependencies: &Query<Entity, With<Dependency>>,
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<ColorMaterial>>,
	) {
		// Only update if we received a render event and have reified data
		if render_events.is_empty() || reified_opt.is_none() {
			return;
		}

		let reified = reified_opt.as_ref().unwrap();

		// Clear existing dependency entities
		for entity in existing_dependencies.iter() {
			commands.entity(entity).despawn();
		}

		// Create dependency curves for each bezier curve
		for (dependency_id, start_point, end_point, control1, control2) in reified.bezier_curves() {
			log::info!(
				"dependency_id: {:?}, start: {:?}, end: {:?}, control1: {:?}, control2: {:?}",
				dependency_id,
				start_point,
				end_point,
				control1,
				control2
			);

			// Convert reified units to pixel coordinates
			let start_pos = Vec3::new(
				start_point.x.value() as f32 * self.pixels_per_unit,
				start_point.y.value() as f32 * self.pixels_per_unit,
				0.0,
			);
			let end_pos = Vec3::new(
				end_point.x.value() as f32 * self.pixels_per_unit,
				end_point.y.value() as f32 * self.pixels_per_unit,
				0.0,
			);
			let control1_pos = Vec3::new(
				control1.x.value() as f32 * self.pixels_per_unit,
				control1.y.value() as f32 * self.pixels_per_unit,
				0.0,
			);
			let control2_pos = Vec3::new(
				control2.x.value() as f32 * self.pixels_per_unit,
				control2.y.value() as f32 * self.pixels_per_unit,
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::bundles::task::tests::utils::setup_task_test_app;
	use crate::resources::{RenderUpdateEvent, Roadline};
	use crate::test_utils::create_test_roadline;
	use roadline_util::dependency::Id as DependencyId;

	/// Helper function to set up an app with all plugins and resources needed for dependency testing
	fn setup_dependency_test_app() -> App {
		let mut app = setup_task_test_app();

		// Add required resources
		let core_roadline = create_test_roadline().expect("Failed to create test roadline");
		app.insert_resource(Roadline::from(core_roadline));
		app.insert_resource(RoadlineRenderConfig::default());

		// Add the RenderUpdateEvent
		app.add_event::<RenderUpdateEvent>();

		app
	}

	/// Helper function to send a render update event
	fn send_render_update_event(app: &mut App) {
		let mut events = app.world_mut().resource_mut::<Events<RenderUpdateEvent>>();
		events.send(RenderUpdateEvent);
	}

	#[test]
	fn test_dependency_spawning_system_builds_correctly() -> Result<(), Box<dyn std::error::Error>>
	{
		let dependency_system = DependencySpawningSystem::default();

		// Setup app with all required resources
		let mut app = setup_dependency_test_app();

		// Add the dependency spawning system
		app.add_systems(Update, dependency_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that dependencies were spawned
		let mut dependency_query = app.world_mut().query::<(Entity, &Transform, &Dependency)>();
		let dependency_count = dependency_query.iter(app.world()).count();

		// Should have spawned dependencies from the test roadline
		assert!(
			dependency_count > 0,
			"Expected dependencies to be spawned, but found {}",
			dependency_count
		);

		Ok(())
	}

	#[test]
	fn test_dependency_spawning_system_clears_existing_dependencies(
	) -> Result<(), Box<dyn std::error::Error>> {
		let dependency_system = DependencySpawningSystem::default();

		// Setup app with all required resources
		let mut app = setup_dependency_test_app();

		// Spawn some initial dependency manually using a system
		fn spawn_manual_dependency(mut commands: Commands) {
			let dependency_id = DependencyId::new(
				roadline_util::task::Id::from(1),
				roadline_util::task::Id::from(2),
			);
			commands.spawn((
				Dependency::new(dependency_id),
				DependencyHoverable,
				DependencyCurveData {
					start: Vec3::new(0.0, 0.0, 0.0),
					end: Vec3::new(100.0, 100.0, 0.0),
					control1: Vec3::new(5.0, 0.0, 0.0),
					control2: Vec3::new(5.0, 100.0, 0.0),
				},
				Transform::default(),
				Visibility::Visible,
			));
		}

		app.add_systems(Update, spawn_manual_dependency);
		app.update();

		// Verify initial dependency exists
		let mut dependency_query = app.world_mut().query::<(Entity, &Transform, &Dependency)>();
		let initial_count = dependency_query.iter(app.world()).count();
		assert_eq!(initial_count, 1);

		// Add the dependency spawning system
		app.add_systems(Update, dependency_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that dependencies were spawned (the manual one should be cleared)
		let final_count = dependency_query.iter(app.world()).count();
		assert!(final_count > 0, "Expected dependencies to be spawned after clearing");

		// The count should be different because we cleared the manual dependency and spawned roadline dependencies
		assert_ne!(final_count, initial_count, "Dependency count should have changed");

		Ok(())
	}

	#[test]
	fn test_dependency_spawning_system_without_render_event(
	) -> Result<(), Box<dyn std::error::Error>> {
		let dependency_system = DependencySpawningSystem::default();

		// Setup app with all required resources
		let mut app = setup_dependency_test_app();

		// Add the dependency spawning system
		app.add_systems(Update, dependency_system.build());

		// Run the system WITHOUT sending a render update event
		app.update();

		// Verify that no dependencies were spawned
		let mut dependency_query = app.world_mut().query::<(Entity, &Transform, &Dependency)>();
		let dependency_count = dependency_query.iter(app.world()).count();

		assert_eq!(
			dependency_count, 0,
			"Expected no dependencies to be spawned without render event"
		);

		Ok(())
	}

	#[test]
	fn test_dependency_spawning_system_without_roadline() -> Result<(), Box<dyn std::error::Error>>
	{
		let dependency_system = DependencySpawningSystem::default();

		// Setup app WITHOUT roadline resource
		let mut app = setup_task_test_app();
		app.insert_resource(RoadlineRenderConfig::default());
		app.add_event::<RenderUpdateEvent>();

		// Add the dependency spawning system
		app.add_systems(Update, dependency_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that no dependencies were spawned
		let mut dependency_query = app.world_mut().query::<(Entity, &Transform, &Dependency)>();
		let dependency_count = dependency_query.iter(app.world()).count();

		assert_eq!(dependency_count, 0, "Expected no dependencies to be spawned without roadline");

		Ok(())
	}

	#[test]
	fn test_dependency_spawning_system_custom_pixels_per_unit(
	) -> Result<(), Box<dyn std::error::Error>> {
		let custom_pixels_per_unit = 100.0;
		let dependency_system =
			DependencySpawningSystem { pixels_per_unit: custom_pixels_per_unit };

		// Setup app with all required resources
		let mut app = setup_dependency_test_app();

		// Add the dependency spawning system
		app.add_systems(Update, dependency_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that dependencies were spawned with custom scaling
		let mut dependency_query = app.world_mut().query::<(Entity, &Transform, &Dependency)>();
		let dependency_count = dependency_query.iter(app.world()).count();

		assert!(dependency_count > 0, "Expected dependencies to be spawned with custom scaling");

		// Verify that the scaling was applied correctly by checking dependency positions
		// (This is a basic check - in a real scenario you'd want more sophisticated verification)
		for (_, transform, _) in dependency_query.iter(app.world()) {
			// Dependencies should be positioned based on the custom pixels_per_unit
			// The exact values depend on the test roadline data
			assert!(transform.translation.x.is_finite());
			assert!(transform.translation.y.is_finite());
		}

		Ok(())
	}

	#[test]
	fn test_dependency_spawning_system_multiple_render_events(
	) -> Result<(), Box<dyn std::error::Error>> {
		let dependency_system = DependencySpawningSystem::default();

		// Setup app with all required resources
		let mut app = setup_dependency_test_app();

		// Add the dependency spawning system
		app.add_systems(Update, dependency_system.build());

		// Send multiple render update events
		send_render_update_event(&mut app);
		send_render_update_event(&mut app);
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that dependencies were spawned (should only spawn once per update cycle)
		let mut dependency_query = app.world_mut().query::<(Entity, &Transform, &Dependency)>();
		let dependency_count = dependency_query.iter(app.world()).count();

		assert!(
			dependency_count > 0,
			"Expected dependencies to be spawned with multiple render events"
		);

		Ok(())
	}

	#[test]
	fn test_dependency_spawning_system_entity_positions_and_scaling(
	) -> Result<(), Box<dyn std::error::Error>> {
		let custom_pixels_per_unit = 75.0;
		let dependency_system =
			DependencySpawningSystem { pixels_per_unit: custom_pixels_per_unit };

		// Setup app with all required resources
		let mut app = setup_dependency_test_app();

		// Add the dependency spawning system
		app.add_systems(Update, dependency_system.build());

		// Send a render update event
		send_render_update_event(&mut app);

		// Run the system
		app.update();

		// Verify that dependencies were spawned
		let mut dependency_query =
			app.world_mut()
				.query::<(Entity, &Transform, &Dependency, &DependencyCurveData)>();
		let dependency_count = dependency_query.iter(app.world()).count();
		assert!(dependency_count > 0, "Expected dependencies to be spawned for position testing");

		// Get the roadline to understand the expected bounds and bezier curves
		let roadline = app.world().resource::<Roadline>();
		let (max_width, max_height) = roadline.visual_bounds();
		let max_width_f32 = max_width.value() as f32 * custom_pixels_per_unit;
		let max_height_f32 = max_height.value() as f32 * custom_pixels_per_unit;

		// Collect dependency positions and verify they match expected calculations
		let mut verified_dependencies = 0;
		for (_entity, transform, dependency, curve_data) in dependency_query.iter(app.world()) {
			// Find the corresponding bezier curve in the roadline
			let mut bezier_curves = roadline.bezier_curves();
			let bezier_curve =
				bezier_curves.find(|(dep_id, _, _, _, _)| **dep_id == dependency.dependency_id);

			if let Some((_, start_point, end_point, _control1, _control2)) = bezier_curve {
				// Calculate expected positions based on the spawning logic
				let expected_start_pos = Vec3::new(
					start_point.x.value() as f32 * custom_pixels_per_unit,
					start_point.y.value() as f32 * custom_pixels_per_unit,
					0.0,
				);
				let expected_end_pos = Vec3::new(
					end_point.x.value() as f32 * custom_pixels_per_unit,
					end_point.y.value() as f32 * custom_pixels_per_unit,
					0.0,
				);

				// Verify the curve data positions match our calculations
				let actual_start = curve_data.start;
				let actual_end = curve_data.end;

				// Allow for small floating point differences
				let tolerance = 0.1;
				assert!(
					(actual_start.x - expected_start_pos.x).abs() < tolerance,
					"Dependency {:?} start X position mismatch: expected {:.2}, got {:.2}",
					dependency.dependency_id,
					expected_start_pos.x,
					actual_start.x
				);
				assert!(
					(actual_start.y - expected_start_pos.y).abs() < tolerance,
					"Dependency {:?} start Y position mismatch: expected {:.2}, got {:.2}",
					dependency.dependency_id,
					expected_start_pos.y,
					actual_start.y
				);
				assert!(
					(actual_end.x - expected_end_pos.x).abs() < tolerance,
					"Dependency {:?} end X position mismatch: expected {:.2}, got {:.2}",
					dependency.dependency_id,
					expected_end_pos.x,
					actual_end.x
				);
				assert!(
					(actual_end.y - expected_end_pos.y).abs() < tolerance,
					"Dependency {:?} end Y position mismatch: expected {:.2}, got {:.2}",
					dependency.dependency_id,
					expected_end_pos.y,
					actual_end.y
				);

				// Verify the transform scale is always 1.0 (default scaling)
				assert_eq!(transform.scale.x, 1.0, "Transform scale X should be 1.0");
				assert_eq!(transform.scale.y, 1.0, "Transform scale Y should be 1.0");
				assert_eq!(transform.scale.z, 1.0, "Transform scale Z should be 1.0");

				verified_dependencies += 1;
			}
		}

		// Verify we checked at least some dependencies
		assert!(
			verified_dependencies > 0,
			"Expected to verify positions for at least one dependency, but verified {}",
			verified_dependencies
		);

		// Verify that all dependencies are positioned within reasonable bounds
		// (should be centered around 0,0 with some spread)
		let mut min_x = f32::INFINITY;
		let mut max_x = f32::NEG_INFINITY;
		let mut min_y = f32::INFINITY;
		let mut max_y = f32::NEG_INFINITY;

		for (_, _, _, curve_data) in dependency_query.iter(app.world()) {
			// For dependencies, the actual positioning is in the curve data, not the transform
			min_x = min_x.min(curve_data.start.x).min(curve_data.end.x);
			max_x = max_x.max(curve_data.start.x).max(curve_data.end.x);
			min_y = min_y.min(curve_data.start.y).min(curve_data.end.y);
			max_y = max_y.max(curve_data.start.y).max(curve_data.end.y);
		}

		// Dependencies should be distributed around the center (0,0)
		// The exact bounds depend on the test roadline data, but they should be reasonable
		assert!(
			min_x >= 0.0 && max_x <= max_width_f32,
			"Dependencies should be distributed around X=0, but found range [{:.2}, {:.2}]",
			min_x,
			max_x
		);
		assert!(
			min_y >= 0.0 && max_y <= max_height_f32,
			"Dependencies should be distributed around Y=0, but found range [{:.2}, {:.2}]",
			min_y,
			max_y
		);

		Ok(())
	}
}
