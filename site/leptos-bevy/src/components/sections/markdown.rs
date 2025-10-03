use comrak::{
	adapters::{HeadingAdapter, HeadingMeta},
	markdown_to_html_with_plugins,
	nodes::Sourcepos,
	ComrakOptions, Plugins,
};
use leptos::prelude::*;

/*pub struct HeadingAdapterImpl;

impl HeadingAdapter for HeadingAdapterImpl {
	// Required methods
	fn enter(
		&self,
		output: &mut dyn core::fmt::Write,
		heading: &HeadingMeta,
		sourcepos: Option<Sourcepos>,
	) -> core::fmt::Result {
		Ok(())
	}
	fn exit(&self, output: &mut dyn core::fmt::Write, heading: &HeadingMeta) -> core::fmt::Result {
		Ok(())
	}
}*/

/// A section component that renders markdown content
#[component]
pub fn MarkdownSection(
	#[prop(into)] content: String,
	#[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
	let mut options = ComrakOptions::default();
	options.extension.strikethrough = true;
	options.extension.tagfilter = true;
	options.extension.table = true;
	options.extension.autolink = true;
	options.extension.strikethrough = true;
	options.extension.tagfilter = true;
	options.extension.tasklist = true;
	options.extension.superscript = true;
	options.extension.subscript = true;
	options.extension.footnotes = true;
	options.extension.math_dollars = true;
	options.extension.math_code = true;
	options.extension.header_ids = Some("".to_string());

	let content = markdown_to_html_with_plugins(&content, &options, &Plugins::default());

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
