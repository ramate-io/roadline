use crate::app::bevy_app::{init_bevy_app, TaskSelectedForExternEvent};
use leptos::prelude::Set;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

const RENDER_WIDTH: u32 = 600;
const RENDER_HEIGHT: u32 = 500;

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

	Effect::new(move || {
		log::info!("Processing effect for task selected for extern receiver");
		if let Some(event) = task_selected_for_extern_receiver.get() {
			log::info!("Event received: {:#?}", event);
			set_event_str.set(format!("{:#?}", event));
		}
	});

	/*let (a, set_a) = signal(0);

	Effect::new(move |_| {
		// immediately prints "Value: 0" and subscribes to `a`
		log::info!("Value: {}", a.get());
	});

	log::info!("App component rendered");*/

	view! {
		<div class="flex w-full mx-auto max-w-[1400px] p-5 items-center">
			<Frame class="border-red-500 bg-red-500/5 flex-1">
				/*<button onclick=move || {
					log::info!("Incrementing a");
					 set_a.set(a.get() + 1)
				}>"Increment"</button>*/
				<h2 class="text-xl font-bold text-red-500 relative top-[-10px]">Bevy</h2>
				<div
					class="aspect-[6/5] rounded-lg overflow-hidden"
					style:max-width="100%"
					style:max-height="100%"
				>
					<BevyCanvas
						init=move || { init_bevy_app(task_selected_for_extern_sender).unwrap() }
						{..}
						width=RENDER_WIDTH
						height=RENDER_HEIGHT
					/>
				</div>
			</Frame>

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
