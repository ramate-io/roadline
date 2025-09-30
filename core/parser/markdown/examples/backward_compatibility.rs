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

	// Test the end date parser directly
	println!("\nTesting end date format detection:");
	let end_date_parser = &parser.task_parser.end_date_parser;

	// Test parsing directly
	let old_result = end_date_parser.parse("T1 + 1 month")?;
	println!("  Old format parsed successfully");

	let new_result = end_date_parser.parse("1 month")?;
	println!("  New format parsed successfully");

	println!("\nBackward compatibility test completed successfully!");

	Ok(())
}
