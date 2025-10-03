use crate::components::sections::markdown::MarkdownSection;
use leptos::prelude::*;

/// A programmable markdown popup pane that can overlay most of the screen
#[component]
pub fn MarkdownPopupPane(
	#[prop(into)] is_visible: ReadSignal<bool>,
	#[prop(into)] set_visible: WriteSignal<bool>,
	#[prop(into)] content: String,
	#[prop(optional)] title: Option<String>,
	#[prop(into)] anchor: Option<String>,
) -> impl IntoView {
	view! {
		<div
			style:display=move || if is_visible.get() { "flex" } else { "none" }
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
					on:click=move |_| {
						log::info!("Closing markdown popup");
						set_visible.set(false)
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

				// Header with title (if provided)
				{title.map(|title_text| view! {
					<div
						style:border-bottom="2px solid #000000"
						style:padding="1.5rem 1.5rem 1rem 1.5rem"
						style:flex-shrink="0"
					>
						<h2
							style:font-size="1.25rem"
							style:font-weight="600"
							style:color="#000000"
							style:margin="0"
						>
							{title_text}
						</h2>
					</div>
				})}

				// Content area - scrollable
				<div
					style:overflow-y="auto"
					style:padding="1.5rem"
					style:flex="1"
					/*node_ref=move |el: NodeRef<HtmlElement>| {
						// Store reference for anchor scrolling
						let el_copy = el.clone();
						let anchor_copy = anchor.clone();

						// Effect to scroll to anchor when popup becomes visible
						Effect::new(move || {
							let is_visible_val = is_visible.get();

							if is_visible_val {
								if let Some(anchor_id) = anchor_copy.as_ref() {
									log::info!("Looking for anchor: {}", anchor_id);

									// Try different anchor selectors
									let queries = vec![
										format!("#{}", anchor_id),
										format!("[id='{}']", anchor_id),
										format!(r#"[name="{}"]"#, anchor_id),
									];

									for query in queries {
										if let Ok(Some(target_el)) = el_copy.query_selector(&query) {
											log::info!("Found anchor element with query: {}", query);
											// Use the standard scrollIntoView method
											target_el.scroll_into_view();
											break;
										}
									}
								}
							}
						}),
					}*/
				>
					<MarkdownSection content=content />
				</div>
			</div>
		</div>
	}
}
