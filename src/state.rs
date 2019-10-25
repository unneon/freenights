use crate::{
	entities::{aliens, camera, player}, graphics::{load_sprite_sheet, GlobalSpriteSheet}
};
use amethyst::prelude::*;

pub enum Game {
	Play,
}

impl SimpleState for Game {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let sprite_sheet = load_sprite_sheet(data.world);
		data.world.insert(GlobalSpriteSheet(sprite_sheet.clone()));
		camera::initialize(data.world);
		player::initialize(data.world, sprite_sheet.clone());
		aliens::initialize(data.world, sprite_sheet.clone());
	}
}

impl Default for Game {
	fn default() -> Self {
		Game::Play
	}
}
