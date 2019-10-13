use crate::{balance, systems::life::Life};
use amethyst::{
	core::{SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Join, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage}
};

pub struct Fighting {
	pub swing: Option<Swing>,
	pub cooldown: f32,
	pub parameters: balance::Attack,
}

#[derive(Debug)]
pub enum Swing {
	Up,
}

#[derive(SystemDesc)]
pub struct Combat;

impl Fighting {
	pub fn new(parameters: balance::Attack) -> Fighting {
		Fighting { swing: None, cooldown: 0.0, parameters }
	}
}

impl Component for Fighting {
	type Storage = DenseVecStorage<Fighting>;
}

impl<'s> System<'s> for Combat {
	type SystemData = (WriteStorage<'s, Fighting>, ReadStorage<'s, Transform>, WriteStorage<'s, Life>, ReadExpect<'s, Time>);

	fn run(&mut self, (mut fights, transforms, mut lives, time): Self::SystemData) {
		for (fight, attacker_transform) in (&mut fights, &transforms).join() {
			fight.cooldown = (fight.cooldown - time.delta_seconds()).max(0.0);
			if fight.cooldown == 0.0 {
				if let Some(swing) = &fight.swing {
					for (defender_transform, life) in (&transforms, &mut lives).join() {
						if (attacker_transform.translation() - defender_transform.translation()).norm() <= fight.parameters.range {
							life.health -= fight.parameters.power;
						}
					}
					fight.cooldown = fight.parameters.cooldown;
				}
			}
		}
	}
}
