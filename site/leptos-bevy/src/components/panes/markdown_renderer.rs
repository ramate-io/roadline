use crate::components::containers::drawer::ResizableDrawer;
use crate::components::sections::markdown::MarkdownSection;
use leptos::prelude::*;
use roadline_util::task::Id as TaskId;

#[derive(Debug, Clone)]
pub struct HeaderInfo {
	pub id: TaskId,
	pub fragment: String,
}

/// A programmable markdown drawer pane that slides up from the bottom
#[component]
pub fn MarkdownDrawerPane(
	#[prop(into)] content: String,
	#[prop(into)] anchor: ReadSignal<Option<String>>,
	#[prop(into)] header_info: ReadSignal<Option<HeaderInfo>>,
) -> impl IntoView {
	// NodeRef for the scrollable content container
	let content_ref = NodeRef::new();

	let (tick, set_tick) = signal(0 as u32);

	// Effect to scroll to anchor after content is rendered
	Effect::new(move || {
		if let Some(anchor_id) = anchor.get() {
			// Use spawn_local with a small delay to ensure DOM is updated
			let anchor_id_clone = anchor_id.clone();
			// Find the anchor element within the content container
			if let Some(_content_element) = content_ref.get() {
				let document = document();
				let target_element = document.get_element_by_id(&anchor_id_clone);
				if let Some(target_element) = target_element {
					target_element.scroll_into_view();
				} else {
					set_tick.set(tick.get() + 1);
				}
			}
		}
	});

	view! {
		<ResizableDrawer
			initial_height="30vh".to_string()
			min_height=100.0
			max_height=1000.0
		>
			<div
				style:padding="4rem"
				node_ref=content_ref
				class="markdown-body"
			>
				<section>
					{move || {
						if let Some(header_info) = header_info.get() {

							let fragment = format!("#{}", header_info.fragment);
							view! {
								<h3>{format!("Selected task: T{}", header_info.id.value())} <a href={fragment.clone()}>{fragment.clone()}</a></h3>
							}.into_any()
						} else {
							view! { <></> }.into_any()
						}
					}}
				</section>
				<MarkdownSection content=content />
			</div>
		</ResizableDrawer>
	}
}
