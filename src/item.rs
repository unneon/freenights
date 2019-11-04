use crate::graphics::GlobalSpriteSheet;
use amethyst::renderer::SpriteRender;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ItemSprite {
	pub sheet: String,
	pub index: usize,
}

#[derive(Deserialize)]
pub struct ItemData {
	pub name: String,
	pub weight: f32,
	pub sprite: ItemSprite,
}

#[derive(Deserialize)]
pub struct ItemDatabase(pub HashMap<String, ItemData>);

impl ItemSprite {
	pub fn create_render(&self, sprite_sheets: &GlobalSpriteSheet) -> SpriteRender {
		SpriteRender { sprite_sheet: sprite_sheets.0[&self.sheet].clone(), sprite_number: self.index }
	}
}
