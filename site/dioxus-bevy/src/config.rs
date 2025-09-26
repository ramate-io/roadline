use crate::app::RoadlineApp;

// Errors thrown by the [RoadlineConfig].
#[derive(Debug, thiserror::Error)]
pub enum RoadlineConfigError {
	#[error("RoadlineConfig failed to build RoadlineApps: {0}")]
	Build(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The configuration used to build the [RoadlineApp].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RoadlineConfig {}

impl RoadlineConfig {
	/// Build the [RoadlineApp] with the given configuration.
	pub fn build(&self) -> Result<RoadlineApp, RoadlineConfigError> {
		Ok(RoadlineApp::new())
	}
}
