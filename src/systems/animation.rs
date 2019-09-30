use crate::entities::player::{Facing, Player};
use amethyst::{
	core::SystemDesc, derive::SystemDesc, ecs::{Join, ReadStorage, System, SystemData, World, WriteStorage}, renderer::SpriteRender
};

#[derive(SystemDesc)]
pub struct Animation;

impl<'s> System<'s> for Animation {
	type SystemData = (ReadStorage<'s, Player>, WriteStorage<'s, SpriteRender>);

	fn run(&mut self, (players, mut sprites): Self::SystemData) {
		for (player, sprite) in (&players, &mut sprites).join() {
			sprite.sprite_number = match player.facing {
				Facing::Left => 1,
				Facing::Right => 0,
			};
		}
	}
}
