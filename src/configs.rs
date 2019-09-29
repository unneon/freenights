use amethyst::{
	assets::{Format, Handle}, renderer::{sprite::SpriteList, SpriteSheet, Texture}, window::DisplayConfig
};
use std::path::Path;

pub fn window(root_dir: &Path) -> amethyst::Result<DisplayConfig> {
	let raw = std::fs::read(root_dir.join("config").join("display.toml"))?;
	let config = toml::from_slice(&raw)?;
	Ok(config)
}

#[derive(Clone, Debug)]
/// TOML replacement for [`amethyst::renderer::SpriteSheetFormat`].
pub struct SpriteSheetTOML(pub Handle<Texture>);

impl Format<SpriteSheet> for SpriteSheetTOML {
	fn name(&self) -> &'static str {
		"SPRITE_SHEET_TOML"
	}

	fn import_simple(&self, bytes: Vec<u8>) -> Result<SpriteSheet, amethyst::Error> {
		let sprite_list: SpriteList = toml::from_slice(&bytes)?;
		Ok(SpriteSheet { texture: self.0.clone(), sprites: sprite_list.build_sprites() })
	}
}
