use crate::entities::player::Player;
use amethyst::{
	core::{SystemDesc, Transform}, derive::SystemDesc, ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}
};

#[derive(SystemDesc)]
pub struct Movement;

impl<'s> System<'s> for Movement {
	type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Player>, Read<'s, InputHandler<StringBindings>>);

	fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
		for (_player, transform) in (&players, &mut transforms).join() {
			let movement_x = input.axis_value("move_horizontal").unwrap();
			let movement_y = input.axis_value("move_vertical").unwrap();
			transform.prepend_translation_x(movement_x);
			transform.prepend_translation_y(movement_y);
		}
	}
}
