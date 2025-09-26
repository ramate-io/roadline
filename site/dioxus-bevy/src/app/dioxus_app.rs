use dioxus::prelude::*;

#[component]
pub fn DioxusApp() -> Element {
	rsx! {
		div { id: "title",
			h1 { "HotDog! 🌭" }
		}
		div { id: "dogview",
			img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
		}
		div { id: "buttons",
			button { id: "skip", "skip" }
			button { id: "save", "save!" }
		}
	}
}
