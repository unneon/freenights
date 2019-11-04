use crate::{
	entities::{aliens, camera, player}, graphics::load_all_sprite_sheets
};
use amethyst::prelude::*;

pub enum Game {
	Play,
}

impl SimpleState for Game {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let sprite_sheet = load_all_sprite_sheets(data.world);
		camera::initialize(data.world);
		player::initialize(data.world, sprite_sheet.0["textures/spritesheet"].clone());
		aliens::initialize(data.world, sprite_sheet.0["textures/spritesheet"].clone());
		data.world.insert(sprite_sheet);
	}
}

impl Default for Game {
	fn default() -> Self {
		Game::Play
	}
}
