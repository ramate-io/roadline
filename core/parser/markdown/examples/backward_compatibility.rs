//! Backward compatibility example for the markdown parser.

use roadline_prarser_markdown::{RoadmapParser, date::EndDateFormat};

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
    
    // Test the date parser directly
    println!("\nTesting date format detection:");
    let date_parser = parser.task_parser.date_parser;
    
    let old_format = date_parser.parse_end_date_format("T1 + 1 month")?;
    match old_format {
        EndDateFormat::TaskReference(task_ref, duration) => {
            println!("  Old format detected: task_ref='{}', duration={:?}", task_ref, duration);
        }
        _ => println!("  Unexpected format for old input"),
    }
    
    let new_format = date_parser.parse_end_date_format("1 month")?;
    match new_format {
        EndDateFormat::Duration(duration) => {
            println!("  New format detected: duration={:?}", duration);
        }
        _ => println!("  Unexpected format for new input"),
    }
    
    println!("\nBackward compatibility test completed successfully!");
    
    Ok(())
}
