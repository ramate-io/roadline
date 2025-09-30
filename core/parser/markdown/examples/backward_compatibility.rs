//! Backward compatibility example for the markdown parser.

use roadline_parser_markdown::RoadmapParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Example with old format end dates
	let old_format_content = r#"
### T1: Old Format Example
- **Starts:** T0 + 0 months
- **Depends-on:** $\emptyset$
- **Ends:** T1 + 1 month
- **Contents:**
    - **[T1.1](#t11-task)**: Example task
"#;

	// Example with new format end dates
	let new_format_content = r#"
### T1: New Format Example
- **Starts:** T0 + 0 months
- **Depends-on:** $\emptyset$
- **Ends:** 1 month
- **Contents:**
    - **[T1.1](#t11-task)**: Example task
"#;

	let parser = RoadmapParser::new();

	// Test old format parsing
	println!("Testing old format (T1 + 1 month):");
	let old_tasks = parser.parse_tasks(old_format_content)?;
	println!("  Parsed {} tasks", old_tasks.len());

	// Test new format parsing
	println!("Testing new format (1 month):");
	let new_tasks = parser.parse_tasks(new_format_content)?;
	println!("  Parsed {} tasks", new_tasks.len());

	// Test the range parser directly
	println!("\nTesting range parser:");
	let range_parser = &parser.task_parser.range_parser;

	// Test parsing directly
	let _old_result = range_parser.parse(
		Some("T0 + 0 months"),
		Some("T1 + 1 month"),
		&roadline_util::task::Id::new(1),
	)?;
	println!("  Old format parsed successfully");

	let _new_result = range_parser.parse(
		Some("T0 + 0 months"),
		Some("1 month"),
		&roadline_util::task::Id::new(1),
	)?;
	println!("  New format parsed successfully");

	println!("\nBackward compatibility test completed successfully!");

	Ok(())
}
