use bevy::prelude::*;
use roadline_bevy_renderer::{RoadlineRenderConfig, RoadlineRenderer};

// Import test utilities for the canvas target
use roadline_bevy_renderer::test_utils::*;

fn main() -> Result<(), anyhow::Error> {
	println!("🚀 Starting Roadline Bevy Canvas Target");

	// Create a test reified representation for the canvas
	let reified = create_test_roadline()?;
	println!(
		"📊 Created reified representation with {} tasks and {} connections",
		reified.task_count(),
		reified.connection_count()
	);

	// Create the Bevy renderer with web-optimized visual config
	let config = RoadlineRenderConfig {
		unit_to_pixel_scale: 2.0, // Moderate scale for web performance
		milestone_color: Color::srgb(0.2, 0.8, 1.0), // Bright cyan milestones
		edge_color: Color::srgb(0.9, 0.9, 0.9), // Light gray edges
		milestone_radius: 12.0,   // Moderate milestone size
		edge_thickness: 3.0,      // Moderate edge thickness
		background_color: Color::srgb(1.0, 1.0, 1.0), // White background
	};

	let renderer = RoadlineRenderer::with_config(config);

	// Create and configure the Bevy app
	let mut app = renderer.create_app();

	// Add camera control systems for web interaction
	app.add_systems(Update, (keyboard_input_system, camera_control_system));

	// Render the reified data
	renderer.render(&mut app, reified)?;
	println!("🎯 Reified data rendered successfully");

	// Center and fit camera to content
	renderer.center_camera(&mut app);
	renderer.fit_camera_to_content(&mut app, 0.2); // 20% padding for better view
	println!("📷 Camera positioned and fitted to content");

	// Run the Bevy app
	app.run();

	Ok(())
}

/// System to handle keyboard input for camera controls and quitting
fn keyboard_input_system(
	keys: Res<ButtonInput<KeyCode>>,
	mut camera_query: Query<&mut Transform, With<Camera2d>>,
	mut exit: EventWriter<AppExit>,
) {
	if let Ok(mut transform) = camera_query.single_mut() {
		let move_speed = 10.0;

		// Camera movement
		if keys.pressed(KeyCode::KeyW) {
			transform.translation.y += move_speed;
		}
		if keys.pressed(KeyCode::KeyS) {
			transform.translation.y -= move_speed;
		}
		if keys.pressed(KeyCode::KeyA) {
			transform.translation.x -= move_speed;
		}
		if keys.pressed(KeyCode::KeyD) {
			transform.translation.x += move_speed;
		}

		// Reset camera position
		if keys.just_pressed(KeyCode::KeyR) {
			transform.translation = Vec3::ZERO;
		}
	}

	// Quit
	if keys.just_pressed(KeyCode::Escape) {
		exit.write(AppExit::Success);
	}
}

/// System for smooth camera controls
fn camera_control_system(
	time: Res<Time>,
	keys: Res<ButtonInput<KeyCode>>,
	mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
	if let Ok(mut transform) = camera_query.single_mut() {
		let move_speed = 200.0 * time.delta_secs();

		// Smooth camera movement
		let mut movement = Vec3::ZERO;

		if keys.pressed(KeyCode::ArrowUp) {
			movement.y += move_speed;
		}
		if keys.pressed(KeyCode::ArrowDown) {
			movement.y -= move_speed;
		}
		if keys.pressed(KeyCode::ArrowLeft) {
			movement.x -= move_speed;
		}
		if keys.pressed(KeyCode::ArrowRight) {
			movement.x += move_speed;
		}

		transform.translation += movement;
	}
}
