use bevy::{asset::load_internal_binary_asset, prelude::*};
use roadline_bevy_renderer::{RoadlineRenderConfig, RoadlineRenderer};

// Import test utilities for the example
use roadline_bevy_renderer::test_utils::*;

fn main() -> Result<(), anyhow::Error> {
	println!("🚀 Starting Roadline Bevy Renderer Visual Example");

	// Create a complex test reified representation for a more interesting visualization
	let reified = create_test_roadline()?;
	println!(
		"📊 Created reified representation with {} tasks and {} connections",
		reified.task_count(),
		reified.connection_count()
	);

	// Create the Bevy renderer with attractive visual config
	let config = RoadlineRenderConfig {
		unit_to_pixel_scale: 3.0, // Larger scale for better visibility
		milestone_color: Color::srgb(0.2, 0.8, 1.0), // Bright cyan milestones
		edge_color: Color::srgb(0.9, 0.9, 0.9), // Light gray edges
		milestone_radius: 15.0,   // Larger milestones
		edge_thickness: 4.0,      // Thicker edges
		background_color: Color::srgb(1.0, 1.0, 1.0), // Dark blue background
	};

	let renderer = RoadlineRenderer::with_config(config);

	// Create and configure the Bevy app
	let mut app = renderer.create_app();

	// Add some additional systems for better visual experience
	app.add_systems(Update, (keyboard_input_system, camera_control_system, info_display_system));

	/*load_internal_binary_asset!(
		app,
		TextStyle::default().font,
		"../assets/fonts/FiraSans-Bold.ttf",
		|bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
	);*/

	// Render the reified data
	renderer.render(&mut app, reified)?;
	println!("🎯 Reified data rendered successfully");

	// Get and display visual bounds
	if let Some((min_x, max_x, min_y, max_y)) = renderer.get_visual_bounds(&app) {
		println!(
			"📏 Visual bounds: x=[{:.1}, {:.1}], y=[{:.1}, {:.1}]",
			min_x, max_x, min_y, max_y
		);
	}

	// Center and fit camera to content
	renderer.center_camera(&mut app);
	renderer.fit_camera_to_content(&mut app, 0.2); // 20% padding for better view
	println!("📷 Camera positioned and fitted to content");

	// Add instructions text
	app.world_mut().spawn((
		Text::new("Roadline Visualization\n\nControls:\n- WASD: Move camera\n- Q/E: Zoom in/out\n- R: Reset camera\n- ESC: Quit"),
		TextFont {
			font_size: 16.0,
			..default()
		},
		TextColor(Color::BLACK),
		Node {
			position_type: PositionType::Absolute,
			top: Val::Px(10.0),
			left: Val::Px(10.0),
			..default()
		},
	));

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

/// System to display runtime information
fn info_display_system(
	camera_query: Query<&Transform, With<Camera2d>>,
	mut text_query: Query<&mut Text>,
) {
	if let Ok(transform) = camera_query.single() {
		if let Ok(mut text) = text_query.single_mut() {
			text.0 = format!(
				"Roadline Visualization\n\nCamera Position: ({:.1}, {:.1})\n\nControls:\n- WASD/Arrows: Move camera\n- R: Reset camera\n- ESC: Quit",
				transform.translation.x,
				transform.translation.y,
			);
		}
	}
}
