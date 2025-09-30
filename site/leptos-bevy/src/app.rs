pub mod bevy_app;
pub mod leptos_app;

use leptos::prelude::IntoView;

/// Errors thrown by the [RoadlineApp].
#[derive(Debug, thiserror::Error)]
pub enum RoadlineAppError {
	#[error("RoadlineApp runtime error: {0}")]
	Runtime(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The application instance.
pub struct RoadlineApp;

impl RoadlineApp {
	pub fn new() -> Self {
		Self
	}

	/// Run the application.
	///
	/// This is synchronous. Asynchronous runtimes should be spawned within.  
	/// That's the most readability way to address the reality that Dioxu spawns a runtime.
	pub fn run(&self) -> impl IntoView {
		leptos_app::App();
	}
}
