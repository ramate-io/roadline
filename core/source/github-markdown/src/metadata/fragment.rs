//! Extract links and fragments from markdown documents
use std::collections::{HashMap, HashSet};

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd};

/// Returns the default markdown extensions used by lychee.
/// Sadly, `|` is not const for `Options` so we can't use a const global.
fn md_extensions() -> Options {
	Options::ENABLE_HEADING_ATTRIBUTES
		| Options::ENABLE_MATH
		| Options::ENABLE_WIKILINKS
		| Options::ENABLE_FOOTNOTES
}

/// Extract fragments/anchors from a Markdown string.
///
/// Fragments are generated from headings using the same unique kebab case method as GitHub.
/// If a [heading attribute](https://github.com/raphlinus/pulldown-cmark/blob/master/specs/heading_attrs.txt)
/// is present,
/// this will be added to the fragment set **alongside** the other generated fragment.
/// It means a single heading such as `## Frag 1 {#frag-2}` would generate two fragments.
pub(crate) fn extract_markdown_fragments(input: &str) -> HashSet<String> {
	let mut in_heading = false;
	let mut heading_text = String::new();
	let mut heading_id: Option<CowStr<'_>> = None;
	let mut id_generator = HeadingIdGenerator::default();

	let mut out = HashSet::new();

	for event in Parser::new_ext(input, md_extensions()) {
		match event {
			Event::Start(Tag::Heading { id, .. }) => {
				heading_id = id;
				in_heading = true;
			}
			Event::End(TagEnd::Heading(_)) => {
				if let Some(frag) = heading_id.take() {
					out.insert(frag.to_string());
				}

				if !heading_text.is_empty() {
					let id = id_generator.generate(&heading_text);
					out.insert(id);
					heading_text.clear();
				}

				in_heading = false;
			}
			Event::Text(text) | Event::Code(text) => {
				if in_heading {
					heading_text.push_str(&text);
				}
			}

			// An HTML node
			Event::Html(_html) | Event::InlineHtml(_html) => {
				// skip
			}

			// Silently skip over other events
			_ => (),
		}
	}
	out
}

#[derive(Default)]
struct HeadingIdGenerator {
	counter: HashMap<String, usize>,
}

impl HeadingIdGenerator {
	fn generate(&mut self, heading: &str) -> String {
		let mut id = Self::into_kebab_case(heading);
		let count = self.counter.entry(id.clone()).or_insert(0);
		if *count != 0 {
			id = format!("{}-{}", id, *count);
		}
		*count += 1;

		id
	}

	/// Converts text into kebab case
	#[must_use]
	fn into_kebab_case(text: &str) -> String {
		text.to_lowercase()
			.chars()
			.filter_map(|ch| {
				if ch.is_alphanumeric() || ch == '_' || ch == '-' {
					Some(ch)
				} else if ch.is_whitespace() {
					Some('-')
				} else {
					None
				}
			})
			.collect::<String>()
	}
}
