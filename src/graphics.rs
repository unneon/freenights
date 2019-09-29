use crate::configs::SpriteSheetTOML;
use amethyst::{
	assets::{AssetStorage, Handle, Loader}, prelude::*, renderer::{ImageFormat, SpriteSheet, Texture}
};

pub fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load("textures/pong_spritesheet.png", ImageFormat::default(), (), &texture_storage)
	};
	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load("textures/pong_spritesheet.toml", SpriteSheetTOML(texture_handle), (), &sprite_sheet_store)
}
