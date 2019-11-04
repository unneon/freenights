use crate::{
	entities::player::Player, systems::{
		combat::{Fighting, Swing}, grab::{GrabDesire, GrabTarget}, movement::Walking
	}, util::scale_axes
};
use amethyst::{
	core::{math::Vector3, SystemDesc, Transform}, derive::SystemDesc, ecs::{Entities, Entity, Join, Read, ReadStorage, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}, renderer::resources::Tint
};

#[derive(Default, SystemDesc)]
pub struct Input {
	pub last_grab_target: Option<Entity>,
}

impl<'s> System<'s> for Input {
	type SystemData = (
		ReadStorage<'s, Player>,
		ReadStorage<'s, Transform>,
		WriteStorage<'s, Walking>,
		WriteStorage<'s, Fighting>,
		WriteStorage<'s, GrabDesire>,
		WriteStorage<'s, Tint>,
		ReadStorage<'s, GrabTarget>,
		Read<'s, InputHandler<StringBindings>>,
		Entities<'s>,
	);

	fn run(&mut self, (players, transforms, mut walks, mut fights, mut desires, mut tints, grab_targets, input, entities): Self::SystemData) {
		for (_player, transform, walk, fight, desire) in (&players, &transforms, &mut walks, &mut fights, &mut desires).join() {
			let axes = scale_axes(input.axis_value("move_horizontal").unwrap(), input.axis_value("move_vertical").unwrap());
			walk.intent = axes;
			fight.swing = if input.action_is_down("attack").unwrap() { Some(Swing) } else { None };
			let grab_target = get_neareast_grab_target(transform.translation(), &transforms, &grab_targets, &entities);
			if grab_target != self.last_grab_target {
				if let Some(last_grab_target) = self.last_grab_target.take() {
					if let Some(tint) = tints.get_mut(last_grab_target) {
						tint.0.red = 1.;
						tint.0.green = 1.;
						tint.0.blue = 1.;
					}
				}
				if let Some(grab_target) = &grab_target {
					let tint = tints.get_mut(grab_target.clone()).unwrap();
					tint.0.red = 2.5;
					tint.0.green = 2.5;
					tint.0.blue = 2.5;
				}
			}
			self.last_grab_target = grab_target.clone();
			if input.action_is_down("grab").unwrap() {
				if let Some(entity) = grab_target {
					desire.target = Some(entity);
				}
			}
		}
	}
}

fn get_neareast_grab_target(
	pos: &Vector3<f32>,
	transforms: &ReadStorage<Transform>,
	grab_targets: &ReadStorage<GrabTarget>,
	entities: &Entities,
) -> Option<Entity> {
	let mut nearest = None;
	for (grab_transform, _grab_target, entity) in (transforms, grab_targets, entities).join() {
		let grab_pos = grab_transform.translation();
		let dist = (grab_pos - pos).norm();
		if dist < 2.5 {
			nearest = match nearest {
				Some((acc_dist, _)) if acc_dist <= dist => nearest,
				_ => Some((dist, entity)),
			}
		}
	}
	nearest.map(|(_, entity)| entity)
}
