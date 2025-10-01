use leptos::prelude::mount_to_body;
use log::Level;
use roadline_site_leptos_bevy::app::leptos_app::App;

fn main() {
	console_error_panic_hook::set_once();
	console_log::init_with_level(Level::Info).expect("error initializing log");

	mount_to_body(App);
}
