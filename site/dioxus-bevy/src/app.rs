pub mod dioxus_app;

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
	pub fn run(&self) -> Result<(), RoadlineAppError> {
		// Desktop/server: spawn a Tokio background task before launching UI
		// Only compile this on native (non-wasm) targets and when the `server` feature is enabled
		#[cfg(all(not(target_arch = "wasm32"), feature = "server"))]
		{
			use std::time::Duration;
			let runtime = tokio::runtime::Builder::new_multi_thread()
				.enable_all()
				.build()
				.map_err(|e| RoadlineAppError::Runtime(e.into()))?;

			runtime.spawn(async move {
				loop {
					// Replace with real logging (e.g., tracing::info!)
					println!("[bg] heartbeat (server)");
					tokio::time::sleep(Duration::from_secs(5)).await;
				}
			});

			// Keep runtime in scope while the UI is running
			let _runtime_guard = runtime;
		}

		// Web: spawn a background task using the browser event loop
		#[cfg(target_arch = "wasm32")]
		{
			use gloo_timers::future::TimeoutFuture;
			use wasm_bindgen_futures::spawn_local;
			use web_sys::console;

			spawn_local(async move {
				loop {
					console::log_1(&"[bg] heartbeat (web)".into());
					TimeoutFuture::new(5_000).await;
				}
			});
		}

		// Launch Dioxus UI (blocks on desktop; returns on web)
		dioxus::launch(dioxus_app::DioxusApp);

		Ok(())
	}
}
