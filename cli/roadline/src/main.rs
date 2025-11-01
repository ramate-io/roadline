use clap::Parser;
use roadline::{Roadline, RoadlineError};

#[tokio::main]
async fn main() -> Result<(), RoadlineError> {
	let roadline = Roadline::parse();
	match roadline.execute().await {
		Ok(()) => Ok(()),
		Err(e) => {
			eprintln!("Error: {}", e);
			Ok(())
		}
	}
}
