use crate::components::sections::markdown::MarkdownSection;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// A programmable markdown popup pane that can overlay most of the screen
#[component]
pub fn MarkdownPopupPane(
	mut on_close: impl FnMut(MouseEvent) + 'static,
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
			style:display="flex"
			style:position="fixed"
			style:top="0"
			style:left="0"
			style:right="0"
			style:bottom="0"
			style:z-index="1000"
			style:align-items="center"
			style:justify-content="center"
			style:background-color="rgba(0, 0, 0, 0.7)"
			style:backdrop-filter="blur(3px)"
		>
			<div
				style:position="relative"
				style:background-color="#ffffff"
				style:border-radius="12px"
				style:box-shadow="0 25px 50px -12px rgba(0, 0, 0, 0.4)"
				style:width="80vw"
				style:height="80vh"
				style:overflow="hidden"
				style:border="2px solid #000000"
				style:display="flex"
				style:flex-direction="column"
				style:padding="1.5rem"
			>
				// Close button
				<button
					style:position="absolute"
					style:top="1rem"
					style:right="1rem"
					style:z-index="10"
					style:background-color="transparent"
					style:border="none"
					style:border-radius="50%"
					style:padding="8px"
					style:cursor="pointer"
					style:color="#000000"
					style:display="flex"
					style:align-items="center"
					style:justify-content="center"
					on:click=move |ev| {
						log::info!("Closing markdown popup");
						on_close(ev)
					}
				>
					<svg
						style:width="24px"
						style:height="24px"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
					>
						<line x1="18" y1="6" x2="6" y2="18"></line>
						<line x1="6" y1="6" x2="18" y2="18"></line>
					</svg>
				</button>

				// Content area - scrollable
				<div
					style:overflow-y="auto"
					style:padding="1.5rem"
					style:flex="1"
					node_ref=content_ref
				>
					<MarkdownSection content=content />
				</div>
			</div>
		</div>
	}
}
