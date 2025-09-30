//! Basic usage example for the markdown parser.

use roadline_parser_markdown::RoadlineParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Example markdown content
	let markdown_content = r#"
### T0: Project Start
- **Starts:** T0 + 0 months
- **Depends-on:** $\emptyset$
- **Ends:** 1 month
- **Contents:**
    - **[T0.1](#t01-project-initialization)**: Project initialization and setup

### T1: Push Towards Validation
> [!IMPORTANT]
> **T1** focuses on readying OAC for validation.

- **Starts:** T0 + 1 month
- **Depends-on:** [T0](#t0-project-start)
- **Ends:** 1 month
- **Contents:**
    - **[T1.1](#t11-complete-draft-of-oart-1-bfa)**: Complete draft of **OART-1: BFA**
    - **[T1.2](#t12-complete-draft-of-oart-2-collaborative-transaction-routing)**: Complete draft of **OART-2: Collaborative Transaction Routing**
    - **[T1.3](#t13-begin-gwrdfa-implementation)**: Begin [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation

### T2: Validation and Accepting Contributions
- **Starts:** T1 + 1 month
- **Depends-on:** [T1](#t1-push-towards-validation)
- **Ends:** 1 month
- **Contents:**
    - **[T2.1](#t21-continue-sharing-and-updating-oart-1-bfa)**: Continue sharing and updating **OART-1: BFA**
    - **[T2.2](#t22-continue-sharing-and-updating-oart-2-collaborative-transaction-routing)**: Continue sharing and updating **OART-2: Collaborative Transaction Routing**
"#;

	// Create the parser
	let parser = RoadlineParser::new();

	// Parse the markdown content
	let tasks = parser.parse_tasks(markdown_content)?;

	println!("Parsed {} tasks:", tasks.len());
	for task in &tasks {
		println!("  - Task {}: {}", u8::from(*task.id()), task.title().as_ref());
		println!("    Dependencies: {:?}", task.depends_on());
		println!("    Subtasks: {}", task.subtasks().len());
	}

	// Build a roadline representation using the parser's convenience method
	let _roadline = parser.parse_and_build(markdown_content)?;

	println!("\nSuccessfully built roadline representation!");

	Ok(())
}
