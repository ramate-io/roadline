use clap::Parser;

#[derive(Debug, thiserror::Error)]
pub enum BevyError {
	#[error("Encountered an internal error: {0}")]
	Internal(#[from] anyhow::Error),
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Bevy {}

impl Bevy {
	pub async fn execute(&self) -> Result<(), BevyError> {
		unimplemented!()
	}
}
