use crate::{
	entities::player::Player, systems::{
		combat::{Attack, Fighter}, grab::{GrabDesire, GrabTarget}, movement::Walking
	}, util::scale_axes
};
use amethyst::{
	core::{math::Vector3, SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}, renderer::resources::Tint
};

#[derive(SystemDesc)]
pub struct Input {
	last_grab_target: Option<Entity>,
	grab_action: HoldableAction,
}

impl<'s> System<'s> for Input {
	type SystemData = (
		ReadStorage<'s, Player>,
		ReadStorage<'s, Transform>,
		WriteStorage<'s, Walking>,
		WriteStorage<'s, Fighter>,
		WriteStorage<'s, GrabDesire>,
		WriteStorage<'s, Tint>,
		ReadStorage<'s, GrabTarget>,
		Read<'s, InputHandler<StringBindings>>,
		ReadExpect<'s, Time>,
		Entities<'s>,
	);

	fn run(&mut self, (players, transforms, mut walks, mut fights, mut desires, mut tints, grab_targets, input, time, entities): Self::SystemData) {
		for (_player, transform, walk, fight, desire) in (&players, &transforms, &mut walks, &mut fights, &mut desires).join() {
			let axes = scale_axes(input.axis_value("move_horizontal").unwrap(), input.axis_value("move_vertical").unwrap());
			walk.intent = axes;
			fight.attack = if input.action_is_down("attack").unwrap() { Some(Attack) } else { None };
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
			if self.grab_action.update(&input, &time) {
				if let Some(entity) = grab_target {
					desire.target = Some(entity);
				}
			}
		}
	}
}

impl Default for Input {
	fn default() -> Self {
		Input { last_grab_target: None, grab_action: HoldableAction::new("grab", 0.2) }
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

struct HoldableAction {
	key: &'static str,
	weak_delay: f32,
	since_last: f32,
}

impl HoldableAction {
	fn new(key: &'static str, weak_delay: f32) -> HoldableAction {
		HoldableAction { key, weak_delay, since_last: std::f32::INFINITY }
	}

	fn update(&mut self, input: &InputHandler<StringBindings>, time: &Time) -> bool {
		self.since_last += time.delta_seconds();
		if input.action_is_down(self.key).unwrap() {
			if self.since_last >= self.weak_delay {
				self.since_last = 0.;
				true
			} else {
				false
			}
		} else {
			self.since_last = std::f32::INFINITY;
			false
		}
	}
}
