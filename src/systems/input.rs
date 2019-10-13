use crate::{
	entities::player::Player, systems::{
		combat::{Fighting, Swing}, movement::Walking
	}, util::scale_axes
};
use amethyst::{
	core::SystemDesc, derive::SystemDesc, ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}
};

#[derive(SystemDesc)]
pub struct Input;

impl<'s> System<'s> for Input {
	type SystemData = (ReadStorage<'s, Player>, WriteStorage<'s, Walking>, WriteStorage<'s, Fighting>, Read<'s, InputHandler<StringBindings>>);

	fn run(&mut self, (players, mut walks, mut fights, input): Self::SystemData) {
		for (_player, walk, fight) in (&players, &mut walks, &mut fights).join() {
			let axes = scale_axes(input.axis_value("move_horizontal").unwrap(), input.axis_value("move_vertical").unwrap());
			walk.intent = axes;
			fight.swing = if input.action_is_down("attack_up").unwrap() { Some(Swing::Up) } else { None };
		}
	}
}
