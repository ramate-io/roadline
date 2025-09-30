use crate::app::bevy_app::{init_bevy_app, TaskSelectedForExternEvent};
use leptos::prelude::Set;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_bevy_canvas::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;
use roadline_bevy_renderer::test_utils::create_test_roadline;
use roadline_representation_core::roadline::Roadline;
use roadline_source_github_markdown::{GitHubSource, GitHubUrl};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Copy, Clone)]
pub enum EventDirection {
	None,
	LeptosToBevy,
	BevyToLeptos,
}

#[component]
pub fn App() -> impl IntoView {
	log::info!("App component mounted");

	view! {
		<Router>
			<Routes fallback=|| "Not found.">
				<Route path=path!("/") view=HomePage />
				<Route path=path!("/gh/*path") view=GitHubRoadlinePage />
			</Routes>
		</Router>
	}
}

#[component]
pub fn HomePage() -> impl IntoView {
	view! {
		<div style="padding: 20px; font-family: Arial, sans-serif;">
			<h1>"Roadline Viewer"</h1>
			<p>"Enter a GitHub path to view a roadline:"</p>
			<p style="font-family: monospace; background: #f5f5f5; padding: 10px; border-radius: 4px;">
				"/gh/org/repo/path/to/file.md"
			</p>
			<p>"Example:"</p>
			<p style="font-family: monospace; background: #f5f5f5; padding: 10px; border-radius: 4px;">
				"/gh/ramate-io/oac/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md"
			</p>
		</div>
	}
}

#[component]
pub fn GitHubRoadlinePage() -> impl IntoView {
	let params = use_params_map();
	let path = move || params.with(|params| params.get("path").unwrap_or_default());

	let (roadline, set_roadline) = signal::<Option<Arc<RwLock<Roadline>>>>(None);
	let (loading, set_loading) = signal(true);
	let (error, set_error) = signal::<Option<String>>(None);

	// Load roadline when path changes
	Effect::new(move || {
		let current_path = path();
		if current_path.is_empty() {
			return;
		}

		set_loading.set(true);
		set_error.set(None);

		spawn_local(async move {
			match load_roadline_from_github(&current_path).await {
				Ok(roadline) => {
					set_roadline.set(Some(Arc::new(RwLock::new(roadline))));
					set_loading.set(false);
				}
				Err(e) => {
					set_error.set(Some(format!("Failed to load roadline: {}", e)));
					set_loading.set(false);
				}
			}
		});
	});

	let (task_selected_for_extern_receiver, task_selected_for_extern_sender) =
		event_b2l::<TaskSelectedForExternEvent>();

	let (event_str, set_event_str) = signal(String::new());

	Effect::new(move || {
		log::info!("Processing effect for task selected for extern receiver");
		if let Some(event) = task_selected_for_extern_receiver.get() {
			log::info!("Event received: {:#?}", event);
			set_event_str.set(format!("{:#?}", event));
		}
	});

	view! {
		<div style="height: 100vh; width: 100vw;">
			{move || {
				if loading.get() {
					view! {
						<div style="display: flex; justify-content: center; align-items: center; height: 100vh; font-family: Arial, sans-serif;">
							<div>"Loading roadline from GitHub..."</div>
						</div>
					}.into_view()
				} else if let Some(err) = error.get() {
					view! {
						<div style="display: flex; justify-content: center; align-items: center; height: 100vh; font-family: Arial, sans-serif; color: red;">
							<div>
								<h2>"Error"</h2>
								<p>{err}</p>
								<p>"Path: " {path()}</p>
							</div>
						</div>
					}.into_view()
				} else if let Some(roadline_arc) = roadline.get() {
					let roadline_clone = roadline_arc.clone();
					view! {
						<BevyCanvas
							init=move || {
								let roadline = roadline_clone.blocking_read();
								init_bevy_app(task_selected_for_extern_sender, roadline.clone()).unwrap()
							}
							{..}
							height="100%"
							width="100%"
							style="outline: none;"
						/>
						<EventDisplay event_str />
					}.into_view()
				} else {
					view! {
						<div style="display: flex; justify-content: center; align-items: center; height: 100vh; font-family: Arial, sans-serif;">
							<div>"No roadline loaded"</div>
						</div>
					}.into_view()
				}
			}}
		</div>
	}
}

#[component]
pub fn EventDisplay(event_str: ReadSignal<String>) -> impl IntoView {
	view! {
		<div class="flex-1 px-5 relative">
			<pre>{event_str}</pre>
		</div>
	}
}

#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
	view! { <div class=format!("border-2 border-solid {class} rounded-lg p-5")>{children()}</div> }
}

/// Load a roadline from GitHub using the GitHub source
async fn load_roadline_from_github(path: &str) -> Result<Roadline, anyhow::Error> {
	log::info!("Loading roadline from GitHub path: {}", path);

	// Parse the path: /gh/org/repo/path/to/file.md
	let path_parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();

	if path_parts.len() < 3 || path_parts[0] != "gh" {
		return Err(anyhow::anyhow!(
			"Invalid GitHub path format. Expected: /gh/org/repo/path/to/file.md"
		));
	}

	let org = path_parts[1];
	let repo = path_parts[2];
	let file_path = path_parts[3..].join("/");

	log::info!("Parsed GitHub path - org: {}, repo: {}, file: {}", org, repo, file_path);

	// Create GitHub source and fetch the roadline
	let source = GitHubSource::new();
	let repository_path = format!("{}/{}/{}", org, repo, file_path);

	log::info!("Fetching from repository path: {}", repository_path);

	let (builder, _metadata) = source
		.from_github_url_with_metadata(&GitHubUrl::parse(&repository_path)?.0)
		.await?;
	let roadline = builder.build()?;

	log::info!("Successfully loaded roadline with {} tasks", roadline.graph().arena.tasks().len());

	Ok(roadline)
}
