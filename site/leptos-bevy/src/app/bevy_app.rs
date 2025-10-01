use bevy::prelude::App;
use leptos_bevy_canvas::prelude::*;
pub use roadline_bevy_renderer::events::interactions::output::task::TaskSelectedForExternEvent;
use roadline_bevy_renderer::{roadline_renderer::RoadlineRenderer, RoadlinePlugin};
use roadline_representation_core::roadline::Roadline;

/// Initialize the Bevy app and export the event to Leptos
pub fn init_bevy_app(
	task_selected_for_extern_sender: BevyEventSender<TaskSelectedForExternEvent>,
	roadline: Roadline,
) -> Result<App, anyhow::Error> {
	log::info!("Initializing Bevy app with roadline: {:#?}", roadline);

	let renderer = RoadlineRenderer::new().with_plugin(RoadlinePlugin::bevy_leptos_canvas());

	let mut app = renderer.create_app_with_roadline(roadline)?;

	app.export_event_to_leptos(task_selected_for_extern_sender);

	Ok(app)
}
