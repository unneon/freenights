use crate::{balance, entities::aliens::Alien, systems::life::Life};
use amethyst::{
	core::{SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Join, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage}
};

pub struct Fighting {
	pub swing: Option<Swing>,
	pub cooldown: f32,
	pub parameters: balance::Attack,
}

#[derive(Debug)]
pub struct Swing;

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
	type SystemData = (WriteStorage<'s, Fighting>, WriteStorage<'s, Transform>, ReadStorage<'s, Alien>, WriteStorage<'s, Life>, ReadExpect<'s, Time>);

	fn run(&mut self, (mut fights, transforms, aliens, mut lives, time): Self::SystemData) {
		for (fight, attacker_transform) in (&mut fights, &transforms).join() {
			fight.cooldown = (fight.cooldown - time.delta_seconds()).max(0.0);
			if fight.cooldown == 0.0 {
				if let Some(Swing) = &fight.swing {
					for (defender_transform, alien, life) in (&transforms, &aliens, &mut lives).join() {
						let distance = (attacker_transform.translation() - defender_transform.translation()).norm();
						if distance <= fight.parameters.range + alien.radius {
							life.health -= fight.parameters.power;
							life.since_attack = 0.;
						}
					}
					fight.cooldown = fight.parameters.cooldown;
				}
			}
		}
	}
}
