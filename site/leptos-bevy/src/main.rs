use leptos::prelude::mount_to_body;
use log::Level;
use roadline_site_leptos_bevy::app::leptos_app::App;

pub const RENDER_WIDTH: f32 = 600.0;
pub const RENDER_HEIGHT: f32 = 500.0;

fn main() {
	console_error_panic_hook::set_once();
	console_log::init_with_level(Level::Info).expect("error initializing log");

	mount_to_body(App);
}
