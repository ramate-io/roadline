use crate::app::bevy_app::{init_bevy_app, TaskSelectedForExternEvent};
use leptos::prelude::Set;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use roadline_bevy_renderer::test_utils::create_test_roadline;

#[derive(Copy, Clone)]
pub enum EventDirection {
	None,
	LeptosToBevy,
	BevyToLeptos,
}

#[component]
pub fn App() -> impl IntoView {
	log::info!("App component mounted");
	let (task_selected_for_extern_receiver, task_selected_for_extern_sender) =
		event_b2l::<TaskSelectedForExternEvent>();

	let (event_str, set_event_str) = signal(String::new());

	let roadline = create_test_roadline().expect("Failed to create test roadline");

	Effect::new(move || {
		log::info!("Processing effect for task selected for extern receiver");
		if let Some(event) = task_selected_for_extern_receiver.get() {
			log::info!("Event received: {:#?}", event);
			set_event_str.set(format!("{:#?}", event));
		}
	});

	view! {
		<div style="height: 100vh; width: 100vw;">
			<BevyCanvas
				init=move || { init_bevy_app(task_selected_for_extern_sender, roadline).unwrap() }
				{..}
				height="100%"
				width="100%"
				style="outline: none;"
			/>
			<EventDisplay event_str />
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
