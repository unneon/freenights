use amethyst::{
	core::SystemDesc, derive::SystemDesc, ecs::{Component, DenseVecStorage, Join, ReadStorage, System, SystemData, World, WriteStorage}, renderer::SpriteRender
};

#[derive(SystemDesc)]
pub struct Animation;

impl<'s> System<'s> for Animation {
	type SystemData = (ReadStorage<'s, WalkAnimation>, WriteStorage<'s, SpriteRender>);

	fn run(&mut self, (facing, mut sprites): Self::SystemData) {
		for (facing, sprite) in (&facing, &mut sprites).join() {
			let bit = match facing {
				WalkAnimation::Left => 1,
				WalkAnimation::Right => 0,
			};
			sprite.sprite_number = (sprite.sprite_number & !1) | bit;
		}
	}
}

pub enum WalkAnimation {
	Left,
	Right,
}

impl Component for WalkAnimation {
	type Storage = DenseVecStorage<Self>;
}
