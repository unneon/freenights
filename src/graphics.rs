use crate::configs::SpriteSheetTOML;
use amethyst::{
	assets::{AssetStorage, Handle, Loader}, prelude::*, renderer::{ImageFormat, SpriteSheet, Texture}
};
use std::collections::HashMap;

pub fn load_sprite_sheet(world: &mut World, name: &str) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(format!("{}.png", name), ImageFormat::default(), (), &texture_storage)
	};
	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(format!("{}.toml", name), SpriteSheetTOML(texture_handle), (), &sprite_sheet_store)
}

const REGISTERED_SPRITE_SHEETS: &[&'static str] = &["textures/spritesheet", "textures/food"];

pub fn load_all_sprite_sheets(world: &mut World) -> GlobalSpriteSheet {
	let mut sheets = HashMap::new();
	for name in REGISTERED_SPRITE_SHEETS {
		sheets.insert((*name).to_owned(), load_sprite_sheet(world, *name));
	}
	GlobalSpriteSheet(sheets)
}

pub struct GlobalSpriteSheet(pub HashMap<String, Handle<SpriteSheet>>);
