use clap::Parser;
use roadline_bevy_renderer::RoadlineRenderer;
use roadline_parser_markdown::RoadlineParser;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum BevyError {
	#[error("Encountered an internal error: {0}")]
	Internal(#[from] Box<dyn std::error::Error>),
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Bevy {
	/// The path to the markdown file to render
	#[clap(long)]
	pub path: PathBuf,
}

impl Bevy {
	pub async fn execute(&self) -> Result<(), BevyError> {
		let parser = RoadlineParser::new();
		let content =
			std::fs::read_to_string(&self.path).map_err(|e| BevyError::Internal(Box::new(e)))?;
		let roadline =
			parser.parse_and_build(&content).map_err(|e| BevyError::Internal(Box::new(e)))?;

		let renderer = RoadlineRenderer::new();

		let mut app = renderer
			.create_app_with_roadline(roadline)
			.map_err(|e| BevyError::Internal(Box::new(e)))?;

		app.run();

		Ok(())
	}
}
