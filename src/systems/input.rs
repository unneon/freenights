use crate::{
	entities::player::Player, systems::{
		combat::{Fighting, Swing}, grab::{GrabDesire, GrabTarget}, movement::Walking
	}, util::scale_axes
};
use amethyst::{
	core::{SystemDesc, Transform}, derive::SystemDesc, ecs::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}
};

#[derive(SystemDesc)]
pub struct Input;

impl<'s> System<'s> for Input {
	type SystemData = (
		ReadStorage<'s, Player>,
		ReadStorage<'s, Transform>,
		WriteStorage<'s, Walking>,
		WriteStorage<'s, Fighting>,
		WriteStorage<'s, GrabDesire>,
		ReadStorage<'s, GrabTarget>,
		Read<'s, InputHandler<StringBindings>>,
		Entities<'s>,
	);

	fn run(&mut self, (players, transforms, mut walks, mut fights, mut desires, grab_targets, input, entities): Self::SystemData) {
		for (_player, transform, walk, fight, desire) in (&players, &transforms, &mut walks, &mut fights, &mut desires).join() {
			let axes = scale_axes(input.axis_value("move_horizontal").unwrap(), input.axis_value("move_vertical").unwrap());
			walk.intent = axes;
			fight.swing = if input.action_is_down("attack").unwrap() { Some(Swing) } else { None };
			if input.action_is_down("grab").unwrap() {
				if let Some((_, entity)) = (&transforms, &grab_targets, &entities)
					.join()
					.map(|(item_transform, _grab_target, entity)| {
						let item_pos = item_transform.translation();
						let distance = (item_pos - transform.translation()).norm();
						(distance, entity)
					})
					.filter(|(distance, _)| *distance < 4.0)
					.fold(None, |acc, target| match acc {
						Some((acc_dist, _)) if acc_dist <= target.0 => acc,
						_ => Some(target),
					}) {
					desire.target = Some(entity);
				}
			}
		}
	}
}
