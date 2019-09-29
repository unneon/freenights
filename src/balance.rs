use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct Balance {
	pub player: Player,
}

impl Balance {
	pub fn load(root_dir: &Path) -> amethyst::Result<Balance> {
		Ok(toml::from_slice(&std::fs::read(root_dir.join("config").join("balance.toml"))?)?)
	}
}

#[derive(Deserialize)]
pub struct Player {
	pub max_speed: f32,
}
