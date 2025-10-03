use leptos::prelude::*;

/// A section component that renders markdown content
#[component]
pub fn MarkdownSection(
	#[prop(into)] content: String,
	#[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
	view! {
		<section
			class=format!(
				"markdown-body {}",
				class.unwrap_or("")
			)
			inner_html=content
		/>
	}
}
