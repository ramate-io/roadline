use roadline_site_dioxus_bevy::config::RoadlineConfig;

pub fn main() -> Result<(), anyhow::Error> {
	let config = RoadlineConfig::default();
	let app = config.build()?;
	app.run()?;
	Ok(())
}
