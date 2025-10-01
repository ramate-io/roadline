pub mod bundles;
pub mod components;
pub mod events;
pub mod resources;
pub mod roadline_renderer;
pub mod systems;

pub mod test_utils;

use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseMotion};
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::SystemCursorIcon;
use bevy::window::WindowResized;
use bevy::winit::cursor::CursorIcon;
use bevy_ui_anchor::AnchorUiPlugin;

use crate::resources::{Roadline, SelectionResource};

pub use roadline_renderer::RoadlineRenderer;

/// Marker component for the UI camera
#[derive(Component)]
pub struct UiCameraMarker;

/// Main plugin for the Roadline Bevy renderer
#[derive(Debug, Clone)]
pub struct RoadlinePlugin {
	canvas_id: String,
}

impl Default for RoadlinePlugin {
	fn default() -> Self {
		Self { canvas_id: "#roadline-canvas".to_string() }
	}
}

impl RoadlinePlugin {
	pub fn bevy_leptos_canvas() -> Self {
		Self { canvas_id: "#bevy_canvas".to_string() }
	}
}

impl Plugin for RoadlinePlugin {
	fn build(&self, app: &mut App) {
		// Add default plugins with windowing
		app.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Roadline Renderer".to_string(),
				canvas: Some(self.canvas_id.clone()), // For web integration
				fit_canvas_to_parent: true,
				prevent_default_event_handling: false,
				..default()
			}),
			..default()
		}));

		// Add common roadline setup
		self.add_common_setup(app);
	}
}

impl RoadlinePlugin {
	/// Build the plugin with headless plugins (for testing)
	pub fn build_headless(&self, app: &mut App) {
		// Add headless plugins
		app.add_plugins(MinimalPlugins)
			.add_plugins(AssetPlugin::default())
			.add_plugins(bevy::scene::ScenePlugin)
			.add_plugins(bevy::render::mesh::MeshPlugin)
			.add_plugins(bevy::transform::TransformPlugin)
			.add_plugins(bevy::render::view::VisibilityPlugin)
			.add_plugins(bevy::input::InputPlugin);

		// Add common roadline setup
		self.add_common_setup(app);

		// Add headless-specific setup
		self.add_headless_setup(app);
	}

	/// Common setup shared between normal and headless builds
	fn add_common_setup(&self, app: &mut App) {
		let task_cursor_interaction_system = systems::TaskCursorInteractionSystem::default();

		app
			// Add bevy_ui_anchor plugin
			.add_plugins(AnchorUiPlugin::<UiCameraMarker>::new())
			// Set white background
			.insert_resource(ClearColor(Color::WHITE))
			// Add selection resource
			.insert_resource(SelectionResource::new())
			// Add required events for cursor interaction systems
			.add_event::<crate::events::interactions::TaskSelectionChangedEvent>()
			.add_event::<crate::events::interactions::output::task::TaskSelectedForExternEvent>()
			// Add render update event
			.add_event::<crate::resources::RenderUpdateEvent>()
			// Add required resources for cursor interaction systems
			.insert_resource(systems::task::cursor_interaction::clicks::events::TaskSelectionChangedEventSystem::default())
			.insert_resource(systems::task::cursor_interaction::clicks::events::output::task_selected_for_extern::TouchDurationTracker::default())
			// Add render config resource
			.insert_resource(RoadlineRenderConfig::default())
			// Add camera positioning resource
			.insert_resource(CameraPositioned::default())
			// Add our custom systems
			.add_systems(Startup, setup_camera)
			.add_systems(Update, (camera_control_system, camera_panning_system))
			.add_systems(Update, position_camera_on_leftmost_task.run_if(resource_exists::<Roadline>))
			.add_systems(
				Update,
				(
					systems::TaskSpawningSystem::default().build(),
					systems::DependencySpawningSystem::default().build(),
					systems::DependencyHoverSystem::default().build(),
					task_cursor_interaction_system.build(),
					// systems::click_selection_system,
				),
			);
	}

	/// Additional headless-specific setup
	fn add_headless_setup(&self, app: &mut App) {
		app.init_asset::<ColorMaterial>()
			.init_asset::<Mesh>()
			.register_type::<Visibility>()
			.register_type::<InheritedVisibility>()
			.register_type::<ViewVisibility>()
			.register_type::<MeshMaterial2d<ColorMaterial>>();
	}
}

/// Setup the 2D camera for rendering
fn setup_camera(mut commands: Commands) {
	// Spawn the sprite camera
	commands.spawn((
		Camera2d,
		Camera {
			order: 1,
			// Don't draw anything in the background, to see the previous camera.
			clear_color: ClearColorConfig::None,
			..default()
		},
		Projection::Orthographic(OrthographicProjection {
			scale: 1.0, // This would now control the zoom level
			..OrthographicProjection::default_2d()
		}),
		Transform::from_xyz(0.0, 0.0, 0.0),
		// This camera will only render entities which are on the same render layer.
		RenderLayers::layer(2),
	));

	commands.spawn((
		Camera2d,
		IsDefaultUiCamera,
		UiCameraMarker, // Mark this camera for bevy_ui_anchor
		Projection::Orthographic(OrthographicProjection {
			scale: 1.0, // This would now control the zoom level
			..OrthographicProjection::default_2d()
		}),
		Transform::from_xyz(0.0, 0.0, 0.0),
		RenderLayers::layer(1),
	));
}

/// Resource to track if camera has been positioned
#[derive(Resource, Default)]
struct CameraPositioned(bool);

/// Position camera on the leftmost task after roadline is loaded
fn position_camera_on_leftmost_task(
	roadline: Res<Roadline>,
	mut camera_query: Query<&mut Transform, With<Camera2d>>,
	mut positioned: ResMut<CameraPositioned>,
) {
	// Only position once
	if positioned.0 {
		return;
	}

	// Find the leftmost task position
	let mut leftmost_x = f32::MAX;
	let mut leftmost_y = 0.0;
	let mut _leftmost_task_id = None;

	// Get visual bounds for scaling
	let (max_width, max_height) = roadline.visual_bounds();
	let max_width_f32 = max_width.value() as f32;
	let _max_height_f32 = max_height.value() as f32;

	// Calculate offsets (same as task spawning system)
	let content_width_pixels = max_width_f32 * 75.0; // pixels_per_unit
	let content_height_pixels = _max_height_f32 * 75.0;
	let offset_x = -content_width_pixels / 2.0;
	let offset_y = -content_height_pixels / 2.0;

	// Find the leftmost task
	for (task_id, start_x, start_y, end_x, _end_y) in roadline.task_rectangles() {
		let _width = end_x - start_x;
		let pixel_x = start_x as f32 * 75.0 + offset_x;
		let pixel_y = start_y as f32 * 75.0 + offset_y;
		let _sprite_width = _width as f32 * 75.0;

		// Use the left edge of the task (not the center)
		let left_edge_x = pixel_x;

		if left_edge_x < leftmost_x {
			leftmost_x = left_edge_x;
			leftmost_y = pixel_y;
			_leftmost_task_id = Some(task_id);
		}
	}

	// If we found a task, position camera to left-align the node with screen
	if leftmost_x != f32::MAX {
		// To left-align the node with the screen, we need to position the camera
		// so that the node's left edge appears at the left edge of the viewport
		// Since the camera is at the center of the viewport, we need to offset
		// by half the viewport width to the right
		// For now, use a reasonable default viewport width
		let viewport_width = 1200.0; // Default viewport width
		let camera_x = leftmost_x + (viewport_width / 2.0);
		
		println!(
			"Left-aligning camera: node at ({:.1}, {:.1}), camera at ({:.1}, {:.1})",
			leftmost_x, leftmost_y, camera_x, leftmost_y
		);

		// Update camera position
		for mut transform in camera_query.iter_mut() {
			transform.translation.x = camera_x;
			transform.translation.y = leftmost_y;
		}
	} else {
		println!("No tasks found, keeping camera at origin");
	}
	
	// Mark as positioned
	positioned.0 = true;
}

fn camera_control_system(
	time: Res<Time>,
	keys: Res<ButtonInput<KeyCode>>,
	mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
	for mut transform in camera_query.iter_mut() {
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

/// System to handle window resize events and update camera orthographic projection
pub fn update_camera_on_resize(
	mut resize_events: EventReader<WindowResized>,
	mut query: Query<&mut Projection, With<Camera2d>>,
) {
	for event in resize_events.read() {
		println!("Window resized: {}x{}", event.width, event.height);

		// Calculate appropriate scale based on window size
		// Smaller windows = larger scale (more zoomed in)
		// Larger windows = smaller scale (more zoomed out)
		let base_width = 1000.0_f32; // Reference width
		let base_height = 1000.0_f32; // Reference height

		let width_ratio = base_width / event.width;
		let height_ratio = base_height / event.height;

		// Use the larger ratio to ensure content fits
		let scale = width_ratio.max(height_ratio);

		println!("Calculated scale: {}", scale);

		for mut projection in query.iter_mut() {
			if let Projection::Orthographic(ref mut orthographic) = *projection {
				orthographic.scale = scale;
			}
		}
	}
}

/// System to handle mouse/touch panning for camera movement
fn camera_panning_system(
	mut commands: Commands,
	mut mouse_button_events: EventReader<MouseButtonInput>,
	mut mouse_motion_events: EventReader<MouseMotion>,
	mut touch_events: EventReader<TouchInput>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut camera_query: Query<&mut Transform, With<Camera2d>>,
	windows: Query<(Entity, &Window)>,
	mut last_mouse_pos: Local<Option<Vec2>>,
	mut is_panning: Local<bool>,
	mut pan_mode_active: Local<bool>,
) {
	// Check if pan mode should be active (Ctrl/Cmd held down)
	let ctrl_or_cmd_pressed = keyboard_input.pressed(KeyCode::ControlLeft) 
		|| keyboard_input.pressed(KeyCode::ControlRight)
		|| keyboard_input.pressed(KeyCode::SuperLeft)  // Cmd on Mac
		|| keyboard_input.pressed(KeyCode::SuperRight); // Cmd on Mac
	
	// Update pan mode state and cursor
	if ctrl_or_cmd_pressed != *pan_mode_active {
		*pan_mode_active = ctrl_or_cmd_pressed;
		
		if let Ok((window_entity, _)) = windows.single() {
			if *pan_mode_active {
				// Enter pan mode - show grab cursor
				commands
					.entity(window_entity)
					.insert(CursorIcon::System(SystemCursorIcon::Grab));
			} else {
				// Exit pan mode - reset cursor and stop panning
				commands
					.entity(window_entity)
					.insert(CursorIcon::System(SystemCursorIcon::Default));
				*is_panning = false;
				*last_mouse_pos = None;
			}
		}
	}

	// Handle mouse button events
	for event in mouse_button_events.read() {
		// Only allow panning if pan mode is active (Ctrl/Cmd held) or middle mouse button
		let can_pan = *pan_mode_active || event.button == MouseButton::Middle;
		
		if can_pan && (event.button == MouseButton::Left || event.button == MouseButton::Middle) {
			match event.state {
				bevy::input::ButtonState::Pressed => {
					if let Ok((window_entity, window)) = windows.single() {
						if let Some(cursor_pos) = window.cursor_position() {
							*last_mouse_pos = Some(cursor_pos);
							*is_panning = true;
							// Set cursor to grabbing hand
							commands
								.entity(window_entity)
								.insert(CursorIcon::System(SystemCursorIcon::Grabbing));
						}
					}
				}
				bevy::input::ButtonState::Released => {
					*is_panning = false;
					*last_mouse_pos = None;
					// Reset cursor based on pan mode state
					if let Ok((window_entity, _)) = windows.single() {
						if *pan_mode_active {
							// Still in pan mode, show grab cursor
							commands
								.entity(window_entity)
								.insert(CursorIcon::System(SystemCursorIcon::Grab));
						} else {
							// Not in pan mode, show default cursor
							commands
								.entity(window_entity)
								.insert(CursorIcon::System(SystemCursorIcon::Default));
						}
					}
				}
			}
		}
	}

	// Handle mouse motion for panning
	if *is_panning {
		for event in mouse_motion_events.read() {
			if last_mouse_pos.is_some() {
				let delta = event.delta;

				// Convert screen delta to world delta
				// Note: This is a simple conversion - you might want to adjust based on your camera scale
				let world_delta = Vec3::new(-delta.x, delta.y, 0.0);

				for mut transform in camera_query.iter_mut() {
					transform.translation += world_delta;
				}

				// Update last mouse position
				if let Ok((_, window)) = windows.single() {
					if let Some(cursor_pos) = window.cursor_position() {
						*last_mouse_pos = Some(cursor_pos);
					}
				}
			}
		}
	}

	// Handle touch events for mobile panning
	for event in touch_events.read() {
		match event.phase {
			TouchPhase::Started => {
				*last_mouse_pos = Some(event.position);
				*is_panning = true;
				// Set cursor to pan hand for touch devices (if supported)
				if let Ok((window_entity, _)) = windows.single() {
					commands
						.entity(window_entity)
						.insert(CursorIcon::System(SystemCursorIcon::Grabbing));
				}
			}
			TouchPhase::Moved => {
				if let Some(last_pos) = *last_mouse_pos {
					let delta = event.position - last_pos;
					let world_delta = Vec3::new(-delta.x, delta.y, 0.0);

					for mut transform in camera_query.iter_mut() {
						transform.translation += world_delta;
					}

					*last_mouse_pos = Some(event.position);
				}
			}
			TouchPhase::Ended | TouchPhase::Canceled => {
				*is_panning = false;
				*last_mouse_pos = None;
				// Reset cursor to default
				if let Ok((window_entity, _)) = windows.single() {
					commands
						.entity(window_entity)
						.insert(CursorIcon::System(SystemCursorIcon::Default));
				}
			}
		}
	}
}

/// Configuration for the renderer
#[derive(Resource, Debug, Clone)]
pub struct RoadlineRenderConfig {
	/// Scale factor for converting reified units to pixels
	pub unit_to_pixel_scale: f32,
	/// Background color
	pub background_color: Color,
	/// Default milestone color
	pub milestone_color: Color,
	/// Default edge color  
	pub edge_color: Color,
	/// Milestone radius in pixels
	pub milestone_radius: f32,
	/// Edge thickness in pixels
	pub edge_thickness: f32,
}

impl Default for RoadlineRenderConfig {
	fn default() -> Self {
		Self {
			unit_to_pixel_scale: 1.0,
			background_color: Color::srgb(1.0, 1.0, 1.0),
			milestone_color: Color::srgb(0.2, 0.7, 1.0),
			edge_color: Color::srgb(0.8, 0.8, 0.8),
			milestone_radius: 8.0,
			edge_thickness: 20.423,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::resources::Roadline;
	use crate::test_utils::create_test_roadline;

	#[test]
	fn test_roadline_plugin_basic_setup() {
		let mut app = App::new();

		// Add the roadline plugin in headless mode for testing
		RoadlinePlugin::default().build_headless(&mut app);

		// Create test roadline data
		let reified = create_test_roadline().expect("Failed to create test roadline");

		// Insert the roadline data
		app.insert_resource(Roadline::new(reified));

		// Update the app to run startup systems
		app.update();

		// Verify that required resources exist
		assert!(app.world().contains_resource::<SelectionResource>());
		assert!(app.world().contains_resource::<systems::task::cursor_interaction::clicks::events::TaskSelectionChangedEventSystem>());

		// Verify that required events are registered
		assert!(app
			.world()
			.contains_resource::<Events<crate::events::interactions::TaskSelectionChangedEvent>>());
		assert!(app.world().contains_resource::<Events<crate::events::interactions::output::task::TaskSelectedForExternEvent>>());

		// Verify that cameras were spawned
		let mut camera_query = app.world_mut().query::<&Camera2d>();
		let cameras: Vec<_> = camera_query.iter(app.world()).collect();
		assert_eq!(cameras.len(), 2, "Should have spawned 2 cameras (sprite and UI)");

		// Verify that the roadline data is accessible
		let roadline =
			app.world().get_resource::<Roadline>().expect("Roadline resource should exist");
		assert!(roadline.task_count() > 0, "Roadline should contain tasks");
	}
}
