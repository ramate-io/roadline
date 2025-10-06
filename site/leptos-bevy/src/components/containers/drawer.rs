//! Code for a resizable drawer component.
//! Drawers are meant to be resizable and pulled from one side.

use leptos::ev;
use leptos::html::Div;
use leptos::prelude::*;

/// A resizable drawer component that slides up from the bottom
#[component]
pub fn ResizableDrawer(
	children: Children,
	#[prop(optional)] initial_height: Option<String>,
	#[prop(optional)] min_height: Option<f64>,
	#[prop(optional)] max_height: Option<f64>,
) -> impl IntoView {
	let drawer_ref = NodeRef::<Div>::new();

	// State for drawer height
	let default_height = initial_height.unwrap_or_else(|| "30vh".to_string());
	let (drawer_height, set_drawer_height) = signal(default_height);
	let (is_resizing, set_is_resizing) = signal(false);

	let on_mouse_down = move |ev: ev::MouseEvent| {
		ev.prevent_default();
		set_is_resizing.set(true);
	};

	let on_mouse_move = move |ev: ev::MouseEvent| {
		ev.prevent_default();

		if !is_resizing.get() {
			return;
		}

		if let Some(drawer) = drawer_ref.get() {
			let bounds = drawer.get_bounding_client_rect();
			let delta = bounds.bottom() - ev.client_y() as f64;
			let mut new_height = delta + 50.0;
			if let Some(min_height) = min_height {
				if new_height < min_height {
					new_height = min_height;
				}
			}
			if let Some(max_height) = max_height {
				if new_height > max_height {
					new_height = max_height;
				}
			}
			set_drawer_height.set(format!("{}px", new_height)); // new height is just the delta from the bottom of the drawer
		}
	};

	let stop_resize = move |ev: ev::MouseEvent| {
		ev.prevent_default();
		set_is_resizing.set(false);
	};

	view! {
		<div
			class="drawer"
			node_ref=drawer_ref
			style:position="fixed"
			style:bottom="0"
			style:left="0"
			style:right="0"
			style:display="flex"
			style:flex-direction="column"
			style:height=move || drawer_height.get()
			on:mousemove=on_mouse_move
			on:mouseleave=stop_resize
			on:mouseup=stop_resize
		>
			// Extra padding for the drag
			<div style:height="40px" style:width="100%" style:pointer-events="none" style:z-index="0">
			</div>
			// Resize handle with black border and centered ball
			<div
				class="resize-handle"
				style:height="8px"
				style:background-color="#000000"
				style:cursor="ns-resize"
				style:display="flex"
				style:align-items="center"
				style:justify-content="center"
				on:mousedown=on_mouse_down
				on:mouseup=stop_resize
			>
				// Small black ball in center
				<div
					style:width="18px"
					style:height="18px"
					style:background-color="#000000"
					style:border-radius="50%"
					style:z-index="1000"
				></div>
			</div>

			// Content area
			<div
				style:flex="1"
				style:overflow-y="auto"
			>
				{children()}
			</div>
		</div>
	}
}
