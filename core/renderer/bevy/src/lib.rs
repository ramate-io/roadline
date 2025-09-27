pub mod bundles;
pub mod components;
pub mod events;
pub mod resources;
pub mod roadline_renderer;
pub mod systems;

pub mod test_utils;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_ui_anchor::AnchorUiPlugin;

use crate::resources::SelectionResource;

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
			// Add our custom systems
			.add_systems(Startup, setup_camera)
			.add_systems(Update, camera_control_system)
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
		Transform::from_xyz(0.0, 0.0, 0.0),
		RenderLayers::layer(1),
	));
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
