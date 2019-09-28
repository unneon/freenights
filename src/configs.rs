use amethyst::window::DisplayConfig;
use std::path::Path;

pub fn window(root_dir: &Path) -> amethyst::Result<DisplayConfig> {
	let raw = std::fs::read(root_dir.join("config").join("display.toml"))?;
	let config = toml::from_slice(&raw)?;
	Ok(config)
}
