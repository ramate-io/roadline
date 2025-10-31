pub mod render;
pub mod validate;

use clap::Parser;
use clap_markdown_ext::Markdown;

#[derive(Debug, thiserror::Error)]
pub enum RoadlineError {
	#[error("Encountered an error while generating documentation: {0}")]
	MarkdownError(#[from] anyhow::Error),
	#[error("Encountered an error while running the program: {0}")]
	RenderError(#[from] render::RenderError),
	#[error("Encountered an error while validating the roadline: {0}")]
	ValidationError(#[from] validate::ValidationError),
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Roadline {
	/// Generate CLI documentation
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Render the roadline
	#[clap(subcommand)]
	Render(render::Render),
	/// Validate the roadline
	Validate(validate::Validate),
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
			Roadline::Validate(validate) => {
				validate.execute().await?;
			}
		}

		Ok(())
	}
}
