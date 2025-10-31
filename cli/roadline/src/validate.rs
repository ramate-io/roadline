use clap::Parser;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
	#[error("Encountered an internal error: {0}")]
	Internal(#[from] anyhow::Error),
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Validate {}

impl Validate {
	pub async fn execute(&self) -> Result<(), ValidationError> {
		unimplemented!()
	}
}
