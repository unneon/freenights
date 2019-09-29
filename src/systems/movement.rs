use crate::{balance::Balance, entities::player::Player};
use amethyst::{
	core::{math::Vector2, SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}
};

#[derive(SystemDesc)]
pub struct Movement;

impl<'s> System<'s> for Movement {
	type SystemData =
		(WriteStorage<'s, Transform>, ReadStorage<'s, Player>, Read<'s, InputHandler<StringBindings>>, Read<'s, Time>, ReadExpect<'s, Balance>);

	fn run(&mut self, (mut transforms, players, input, time, balance): Self::SystemData) {
		for (_player, transform) in (&players, &mut transforms).join() {
			let axis = Vector2::new(input.axis_value("move_horizontal").unwrap(), input.axis_value("move_vertical").unwrap());
			if let Some(axis) = axis.try_normalize(0.) {
				let delta = axis * time.delta_seconds() * balance.player.max_speed;
				transform.prepend_translation_x(delta[0]);
				transform.prepend_translation_y(delta[1]);
			}
		}
	}
}
