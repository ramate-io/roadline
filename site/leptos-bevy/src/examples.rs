use crate::components::{
	panes::markdown_renderer::MarkdownPopupPane, sections::markdown::MarkdownSection,
};
use leptos::prelude::*;

/// Example component showing how to use both the markdown section and popup pane
#[component]
pub fn ExampleUsage() -> impl IntoView {
	let (show_popup, set_show_popup) = signal(false);

	let sample_markdown = r#" 
# Example Markdown Content

This is a **sample markdown** document that demonstrates the capabilities of our components.

## Features

- *Italic text*
- **Bold text**  
- `Code snippets`
- Lists and more!

### Code Example

```rust
fn main() {
    println!("Hello World!");
}
```

### Links

[Visit our website](https://example.com)

This content can be displayed both as a section and in a popup overlay.
"#
	.to_string();

	view! {
		<div class="example-container">
			<h1 class="example-title">Markdown Components Example</h1>

			// Markdown Section Demo
			<div class="example-section">
				<h2 class="example-subtitle">Markdown Section</h2>
				<MarkdownSection content=sample_markdown.clone() />
			</div>

			// Button to trigger popup
			<div class="example-button-container">
				<button
					class="example-button"
					on:click=move |_| set_show_popup.set(true)
				>
					"Open Markdown Popup"
				</button>
			</div>

			// Popup Pane
			<MarkdownPopupPane
				is_visible=show_popup
				set_visible=set_show_popup
				content=sample_markdown
				title="Example Popup".to_string()
				class="custom-popup"
			/>
		</div>
	}
}
