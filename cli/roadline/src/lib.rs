pub mod render;

use clap::Parser;
use clap_markdown_ext::Markdown;

#[derive(Debug, thiserror::Error)]
pub enum RoadlineError {
	#[error("Encountered an error while generating documentation: {0}")]
	MarkdownError(#[from] anyhow::Error),
	#[error("Encountered an error while running the program: {0}")]
	RenderError(#[from] render::RenderError),
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Roadline {
	/// Generate CLI documentation
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Run a RISC-V program in the box
	#[clap(subcommand)]
	Render(render::Render),
}

impl Roadline {
	pub async fn execute(&self) -> Result<(), RoadlineError> {
		match self {
			Roadline::Markdown(markdown) => {
				markdown.execute::<Self>().await?;
			}
			Roadline::Render(render) => {
				render.execute().await?;
			}
		}

		Ok(())
	}
}
