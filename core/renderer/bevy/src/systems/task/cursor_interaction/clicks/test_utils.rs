use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
use crate::resources::{Roadline, SelectionResource};
use crate::test_utils::create_test_roadline;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::camera::{ComputedCameraValues, RenderTargetInfo};
use roadline_util::task::Id as TaskId;

/// Helper function to set up an app with all plugins and resources needed for cursor interaction testing
pub fn setup_cursor_interaction_test_app() -> App {
	let mut app = setup_task_test_app();

	// Add input plugins for mouse input
	app.add_plugins(bevy::input::InputPlugin);

	// Add window plugin to create primary window
	app.world_mut().spawn(Window::default());

	// Add required resources
	app.insert_resource(SelectionResource::default());
	app.insert_resource(bevy::input::ButtonInput::<bevy::input::mouse::MouseButton>::default());

	app.world_mut().spawn((
		Camera2d,
		Camera {
			order: 1,
			// Don't draw anything in the background, to see the previous camera.
			clear_color: ClearColorConfig::None,
			viewport: Some(Viewport {
				physical_position: UVec2::new(0, 0),
				physical_size: UVec2::new(200, 200),
				..default()
			}),
			computed: ComputedCameraValues {
				target_info: Some(RenderTargetInfo { scale_factor: 1.0, ..default() }),
				..default()
			},
			..default()
		},
	));

	// Add test roadline
	let core_roadline = create_test_roadline().expect("Failed to create test roadline");
	app.insert_resource(Roadline::from(core_roadline));

	app
}

/// Helper function to spawn a basic test task
pub fn spawn_test_task(
	app: &mut App,
	task_id: TaskId,
	position: Vec3,
	size: Vec2,
	title: String,
) -> Result<(), Box<dyn std::error::Error>> {
	let params = TestTasksParams::new().with_basic_task(task_id, position, size, title);
	app.world_mut().run_system_once(params.build())?;
	Ok(())
}

/// Helper function to simulate cursor movement to a specific world position
pub fn simulate_cursor_to_world_position(
	windows: &mut Query<(Entity, &mut Window)>,
	cameras: &Query<(&Camera, &GlobalTransform)>,
	world_pos: Vec3,
) -> Result<(), Box<dyn std::error::Error>> {
	let (_window_entity, mut window) = windows.single_mut().map_err(|_| "No window found")?;
	let (camera, camera_transform) = cameras.single().map_err(|_| "No camera found")?;

	// Convert world coordinates to screen coordinates
	let screen_pos = camera
		.world_to_viewport(camera_transform, world_pos)
		.map_err(|_| "Failed to convert world to viewport")?;

	window.set_cursor_position(Some(screen_pos));
	Ok(())
}

/// Helper function to simulate a mouse click
pub fn simulate_mouse_click(
	windows: &mut Query<(Entity, &mut Window)>,
	mouse_events: &mut bevy::ecs::event::EventWriter<bevy::input::mouse::MouseButtonInput>,
	cameras: &Query<(&Camera, &GlobalTransform)>,
	world_pos: Vec3,
) -> Result<(), Box<dyn std::error::Error>> {
	let (window_entity, mut window) = windows.single_mut().map_err(|_| "No window found")?;
	let (camera, camera_transform) = cameras.single().map_err(|_| "No camera found")?;

	// Convert world coordinates to screen coordinates
	let screen_pos = camera
		.world_to_viewport(camera_transform, world_pos)
		.map_err(|_| "Failed to convert world to viewport")?;

	window.set_cursor_position(Some(screen_pos));
	mouse_events.write(bevy::input::mouse::MouseButtonInput {
		button: bevy::input::mouse::MouseButton::Left,
		state: bevy::input::ButtonState::Pressed,
		window: window_entity,
	});
	Ok(())
}
