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
	grab_highlight: HighlightTint,
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
			self.grab_highlight.update(grab_target.clone(), &mut tints);
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
		Input { grab_highlight: HighlightTint::new([2.5, 2.5, 2.5]), grab_action: HoldableAction::new("grab", 0.2) }
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

struct HighlightTint {
	rgb: [f32; 3],
	curr: Option<Entity>,
}

impl HighlightTint {
	fn new(rgb: [f32; 3]) -> HighlightTint {
		HighlightTint { rgb, curr: None }
	}

	fn update(&mut self, next: Option<Entity>, tints: &mut WriteStorage<Tint>) {
		if next != self.curr {
			if let Some(curr) = self.curr.take() {
				if let Some(tint) = tints.get_mut(curr) {
					tint.0.red /= self.rgb[0];
					tint.0.green /= self.rgb[1];
					tint.0.blue /= self.rgb[2];
				}
			}
			if let Some(next) = &next {
				let tint = tints.get_mut(next.clone()).unwrap();
				tint.0.red *= self.rgb[0];
				tint.0.green *= self.rgb[1];
				tint.0.blue *= self.rgb[2];
			}
		}
		self.curr = next;
	}
}
