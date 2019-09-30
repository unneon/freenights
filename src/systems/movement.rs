use crate::{
	balance::Balance, entities::player::{Facing, Player}, util::scale_axes
};
use amethyst::{
	core::{SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Join, Read, ReadExpect, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}
};

#[derive(SystemDesc)]
pub struct Movement;

impl<'s> System<'s> for Movement {
	type SystemData =
		(WriteStorage<'s, Transform>, WriteStorage<'s, Player>, Read<'s, InputHandler<StringBindings>>, Read<'s, Time>, ReadExpect<'s, Balance>);

	fn run(&mut self, (mut transforms, mut players, input, time, balance): Self::SystemData) {
		for (player, transform) in (&mut players, &mut transforms).join() {
			let axes = scale_axes(input.axis_value("move_horizontal").unwrap(), input.axis_value("move_vertical").unwrap());
			match axes[0] {
				ax if ax > 0. => player.facing = Facing::Right,
				ax if ax < 0. => player.facing = Facing::Left,
				_ => (),
			}
			player.velocity += player.compute_acceleration(axes, &balance) * time.delta_seconds();
			let displacement = player.velocity * time.delta_seconds();
			transform.prepend_translation_x(displacement[0]);
			transform.prepend_translation_y(displacement[1]);
		}
	}
}
