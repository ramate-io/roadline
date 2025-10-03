pub mod markdown_renderer;

use crate::app::bevy_app::{init_bevy_app, TaskSelectedForExternEvent};
use crate::components::panes::markdown_renderer::MarkdownPopupPane;
use leptos::prelude::Set;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_bevy_canvas::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;
use roadline_representation_core::roadline::Roadline;
use roadline_source_github_markdown::{GitHubMetadataCollector, GitHubSource, GitHubUrl};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct MarkdownState {
	/// The markdown content to display
	pub content: String,
	/// The metadata of the markdown content
	pub metadata: Arc<GitHubMetadataCollector>,
}

impl MarkdownState {
	pub fn new(content: String, metadata: GitHubMetadataCollector) -> Self {
		Self { content, metadata: Arc::new(metadata) }
	}

	pub fn get_anchor_for_event(&self, event: &TaskSelectedForExternEvent) -> Option<String> {
		self.metadata.get_fragment(&event.selected_task).cloned()
	}
}

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
	let (markdown_state, set_markdown_state) = signal::<Option<MarkdownState>>(None);
	let (anchor, set_anchor) = signal::<Option<String>>(None);

	let (loading, set_loading) = signal(true);
	let (error, set_error) = signal::<Option<String>>(None);

	// Task popup state management
	let (show_task_popup, set_show_task_popup) = signal(false);

	// Load roadline when path changes
	Effect::new(move || {
		let current_path = path();
		if current_path.is_empty() {
			return;
		}

		set_loading.set(true);
		set_error.set(None);

		spawn_local(async move {
			match load_roadline_and_content_from_github(&current_path).await {
				Ok((roadline, metadata, content)) => {
					set_roadline.set(Some(Arc::new(RwLock::new(roadline))));
					set_loading.set(false);
					set_markdown_state.set(Some(MarkdownState::new(content, metadata)));
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

	Effect::watch(
		move || task_selected_for_extern_receiver.get(),
		move |event, _prev_event, _| {
			if let Some(event) = event {
				let anchor =
					markdown_state.get().and_then(|state| state.get_anchor_for_event(&event));
				set_anchor.set(anchor);
				set_show_task_popup.set(true);
				log::info!("Popup visibility set to: {}", show_task_popup.get());
			}
		},
		false,
	);

	let task_selected_for_extern_sender_clone = task_selected_for_extern_sender.clone();

	view! {
		<div style="height: 100vh; width: 100vw;">
			{move || {
				if loading.get() {
					view! {
						<div style="display: flex; justify-content: center; align-items: center; height: 100vh; font-family: Arial, sans-serif;">
							<div>"Loading roadline from GitHub..."</div>
						</div>
					}.into_any()
				} else if let Some(err) = error.get() {
					view! {
						<div style="display: flex; justify-content: center; align-items: center; height: 100vh; font-family: Arial, sans-serif; color: red;">
							<div>
								<h2>"Error"</h2>
								<p>{err}</p>
								<p>"Path: " {path()}</p>
							</div>
						</div>
					}.into_any()
				} else if let Some(roadline_arc) = roadline.get() {
					let roadline_clone = roadline_arc.clone();
					let sender_clone = task_selected_for_extern_sender_clone.clone();
					view! {
						<BevyCanvas
							init=move || {
								let roadline = roadline_clone.blocking_read();
								init_bevy_app(sender_clone, roadline.clone()).unwrap()
							}
							{..}
							height="100%"
							width="100%"
							style="outline: none;"
						/>
					}.into_any()
				} else {
					view! {
						<div style="display: flex; justify-content: center; align-items: center; height: 100vh; font-family: Arial, sans-serif;">
							<div>"No roadline loaded"</div>
						</div>
					}.into_any()
				}
			}}

			// Task markdown popup - overlays over everything
			{move || {
				if let Some(state) = markdown_state.get() {
					let anchor = anchor.get();
					view! {
						<MarkdownPopupPane
							is_visible=show_task_popup
							set_visible=set_show_task_popup
							content=state.content
							title="Task Details".to_string()
							anchor=anchor
						/>
					}.into_any()
				} else {
					view! { <></> }.into_any()
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

/// Load a roadline and content from GitHub using the GitHub source
async fn load_roadline_and_content_from_github(
	path: &str,
) -> Result<(Roadline, GitHubMetadataCollector, String), anyhow::Error> {
	log::info!("Loading roadline from GitHub path: {}", path);

	// Parse the path: /gh/org/repo/path/to/file.md
	let path_parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();

	if path_parts.len() < 2 {
		return Err(anyhow::anyhow!(
			"Invalid GitHub path format. Expected: /org/repo/path/to/file.md"
		));
	}

	let org = path_parts[0];
	let repo = path_parts[1];
	let file_path = path_parts[2..].join("/");

	log::info!("Parsed GitHub path - org: {}, repo: {}, file: {}", org, repo, file_path);

	// Create GitHub source and fetch the roadline and content
	let source = GitHubSource::new();
	let repository_path = format!("{}/{}/{}", org, repo, file_path);

	log::info!("Fetching from repository path: {}", repository_path);

	let (builder, metadata, content) = source
		.from_github_url_with_metadata_and_content(&GitHubUrl::parse(&repository_path)?.0)
		.await?;
	let roadline = builder.build()?;

	log::info!("Roadline: {:#?}", roadline);
	log::info!("Successfully loaded roadline with {} tasks", roadline.graph().arena.tasks().len());

	Ok((roadline, metadata, content))
}
