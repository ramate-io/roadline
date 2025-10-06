//! Code for a resizable drawer component.
//! Drawers are meant to be resizable and pulled from one side.

use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::web_sys;

/// A resizable drawer component that slides up from the bottom
#[component]
pub fn ResizableDrawer(
	children: Children,
	#[prop(optional)] initial_height: Option<String>,
	#[prop(optional)] min_height: Option<f64>,
	#[prop(optional)] _on_close: Option<Box<dyn FnMut(MouseEvent) + 'static>>,
) -> impl IntoView {
	let drawer_ref = NodeRef::new();
	let resize_handle_ref = NodeRef::new();

	// State for drawer height
	let default_height = initial_height.unwrap_or_else(|| "10vh".to_string());
	let (drawer_height, set_drawer_height) = signal(default_height);
	let (is_resizing, set_is_resizing) = signal(false);
	let (start_y, set_start_y) = signal(0.0);
	let (start_height, set_start_height) = signal(0.0);

	let min_height = min_height.unwrap_or(100.0);

	// Mouse down handler for resize handle
	let handle_mouse_down = move |ev: MouseEvent| {
		set_is_resizing.set(true);
		set_start_y.set(ev.client_y() as f64);

		// For now, just use a default height
		set_start_height.set(200.0);

		// Prevent text selection during resize
		if let Some(body) = document().body() {
			body.style().set_property("user-select", "none").unwrap();
		}

		ev.prevent_default();
	};

	// Global mouse move handler
	let handle_mouse_move = move |ev: web_sys::MouseEvent| {
		if !is_resizing.get() {
			return;
		}

		let dy = start_y.get() - ev.client_y() as f64;
		let new_height = (start_height.get() + dy).max(min_height);
		set_drawer_height.set(format!("{}px", new_height));
	};

	// Global mouse up handler
	let handle_mouse_up = move |_: web_sys::MouseEvent| {
		set_is_resizing.set(false);

		// Restore text selection
		if let Some(body) = document().body() {
			body.style().set_property("user-select", "").unwrap();
		}
	};

	// Add global event listeners when resizing starts
	/*Effect::new(move || {
		if is_resizing.get() {
			// For now, just set a reasonable height when resizing starts
			// This can be enhanced later with proper mouse tracking
			set_drawer_height.set("50vh".to_string());
		}
	});*/

	view! {
		<div
			class="drawer"
			node_ref=drawer_ref
			style:position="fixed"
			style:bottom="0"
			style:left="0"
			style:right="0"
			style:z-index="1000"
			style:background-color="#ffffff"
			style:display="flex"
			style:flex-direction="column"
			style:height=move || drawer_height.get()
		>
			// Resize handle with black border and centered ball
			<div
				class="resize-handle"
				node_ref=resize_handle_ref
				style:height="8px"
				style:background-color="#000000"
				style:cursor="ns-resize"
				style:display="flex"
				style:align-items="center"
				style:justify-content="center"
				on:mousedown=handle_mouse_down
				on:mousemove=handle_mouse_move
				on:mouseup=handle_mouse_up
			>
				// Small black ball in center
				<div
					style:width="6px"
					style:height="6px"
					style:background-color="#000000"
					style:border-radius="50%"
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
