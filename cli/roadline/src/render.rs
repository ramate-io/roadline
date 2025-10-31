pub mod bevy;

use clap::Subcommand;

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
	#[error("Encountered an error while rendering with Bevy: {0}")]
	BevyError(#[from] bevy::BevyError),
}

#[derive(Subcommand)]
pub enum Render {
	/// Load and run a RISC-V ELF file
	Bevy(bevy::Bevy),
}

impl Render {
	pub async fn execute(&self) -> Result<(), RenderError> {
		match self {
			Render::Bevy(bevy) => bevy.execute().await.map_err(RenderError::BevyError),
		}
	}
}
