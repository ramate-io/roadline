use crate::components::containers::drawer::ResizableDrawer;
use crate::components::sections::markdown::MarkdownSection;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// A programmable markdown drawer pane that slides up from the bottom
#[component]
pub fn MarkdownDrawerPane(
	on_close: impl FnMut(MouseEvent) + 'static,
	#[prop(into)] content: String,
	#[prop(into)] anchor: ReadSignal<Option<String>>,
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
					log::info!("Scrolling to element: {}", anchor_id_clone);
					target_element.scroll_into_view();
				} else {
					set_tick.set(tick.get() + 1);
				}
			}
		}
	});

	view! {
		<div
			style:padding="20px"
			node_ref=content_ref
		>
			<MarkdownSection content=content />
		</div>
	}
}
