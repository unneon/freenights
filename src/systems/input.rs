use crate::{entities::player::Player, systems::movement::Walking, util::scale_axes};
use amethyst::{
	core::SystemDesc, derive::SystemDesc, ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}
};

#[derive(SystemDesc)]
pub struct Input;

impl<'s> System<'s> for Input {
	type SystemData = (ReadStorage<'s, Player>, WriteStorage<'s, Walking>, Read<'s, InputHandler<StringBindings>>);

	fn run(&mut self, (players, mut walks, input): Self::SystemData) {
		for (_player, walk) in (&players, &mut walks).join() {
			let axes = scale_axes(input.axis_value("move_horizontal").unwrap(), input.axis_value("move_vertical").unwrap());
			walk.intent = axes;
		}
	}
}
