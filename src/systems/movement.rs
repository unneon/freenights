use crate::{balance, systems::animation::Facing};
use amethyst::{
	core::{math::Vector2, SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Join, Read, System, SystemData, World, WriteStorage}
};

pub struct Walking {
	pub intent: Vector2<f32>,
	pub velocity: Vector2<f32>,
	pub parameters: balance::Movement,
}

#[derive(SystemDesc)]
pub struct Movement;

impl Walking {
	pub fn new(parameters: balance::Movement) -> Self {
		Walking { intent: Vector2::zeros(), velocity: Vector2::zeros(), parameters }
	}
}

impl Component for Walking {
	type Storage = DenseVecStorage<Self>;
}

impl<'s> System<'s> for Movement {
	type SystemData = (WriteStorage<'s, Transform>, WriteStorage<'s, Walking>, WriteStorage<'s, Facing>, Read<'s, Time>);

	fn run(&mut self, (mut transforms, mut walks, mut faces, time): Self::SystemData) {
		for (transform, walk, face) in (&mut transforms, &mut walks, &mut faces).join() {
			let acceleration = walk.parameters.acceleration * walk.intent - walk.parameters.drag * walk.velocity;
			walk.velocity += acceleration * time.delta_seconds();
			let displacement = walk.velocity * time.delta_seconds();
			transform.prepend_translation_x(displacement[0]);
			transform.prepend_translation_y(displacement[1]);
			match displacement[0] {
				dx if dx > 0.001 => *face = Facing::Right,
				dx if dx < -0.001 => *face = Facing::Left,
				_ => (),
			}
		}
	}
}
