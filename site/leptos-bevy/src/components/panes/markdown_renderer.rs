use leptos::prelude::*;

/// A programmable markdown popup pane that can overlay most of the screen
#[component]
pub fn MarkdownPopupPane(
	#[prop(into)] is_visible: ReadSignal<bool>,
	#[prop(into)] set_visible: WriteSignal<bool>,
	#[prop(into)] content: String,
	#[prop(optional)] title: Option<String>,
	#[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
	view! {
		<div
			class="markdown-popup-overlay"
			style:display=move || if is_visible.get() { "flex" } else { "none" }
		>
			<div class="markdown-popup-content">
				// Close button
				<button
					class="markdown-popup-close-button"
					on:click=move |_| set_visible.set(false)
				>
					<svg class="markdown-popup-close-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<line x1="18" y1="6" x2="6" y2="18"></line>
						<line x1="6" y1="6" x2="18" y2="18"></line>
					</svg>
				</button>

				// Header with title (if provided)
				{title.map(|title_text| view! {
					<div class="markdown-popup-header">
						<h2 class="markdown-popup-title">
							{title_text}
						</h2>
					</div>
				})}

				// Content area
				<div class="markdown-popup-body">
					<section
						class=format!(
							"markdown-body markdown-popup-content-body {}",
							class.unwrap_or("")
						)
						inner_html=content
					/>
				</div>
			</div>
		</div>
	}
}
