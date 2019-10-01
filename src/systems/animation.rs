use amethyst::{
	core::SystemDesc, derive::SystemDesc, ecs::{Component, DenseVecStorage, Join, ReadStorage, System, SystemData, World, WriteStorage}, renderer::SpriteRender
};

#[derive(SystemDesc)]
pub struct Animation;

impl<'s> System<'s> for Animation {
	type SystemData = (ReadStorage<'s, Facing>, WriteStorage<'s, SpriteRender>);

	fn run(&mut self, (facing, mut sprites): Self::SystemData) {
		for (facing, sprite) in (&facing, &mut sprites).join() {
			let bit = match facing {
				Facing::Left => 1,
				Facing::Right => 0,
			};
			sprite.sprite_number = (sprite.sprite_number & !1) | bit;
		}
	}
}

pub enum Facing {
	Left,
	Right,
}

impl Component for Facing {
	type Storage = DenseVecStorage<Self>;
}
