use bevy::prelude::App;
use leptos_bevy_canvas::prelude::*;
pub use roadline_bevy_renderer::events::interactions::output::task::TaskSelectedForExternEvent;
use roadline_bevy_renderer::roadline_renderer::RoadlineRenderer;

pub fn init_bevy_app(
	task_selected_for_extern_sender: BevyEventSender<TaskSelectedForExternEvent>,
) -> Result<App, anyhow::Error> {
	let renderer = RoadlineRenderer::new();

	let mut app = renderer.create_app();

	app.export_event_to_leptos(task_selected_for_extern_sender);

	Ok(app)
}
