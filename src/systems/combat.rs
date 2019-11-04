use crate::{
	balance, systems::{aliens::Alien, life::Alive}
};
use amethyst::{
	core::{SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Join, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage}
};

#[derive(SystemDesc)]
pub struct Combat;

impl<'s> System<'s> for Combat {
	type SystemData = (WriteStorage<'s, Fighter>, WriteStorage<'s, Transform>, ReadStorage<'s, Alien>, WriteStorage<'s, Alive>, ReadExpect<'s, Time>);

	fn run(&mut self, (mut fights, transforms, aliens, mut lives, time): Self::SystemData) {
		for (fight, attacker_transform) in (&mut fights, &transforms).join() {
			fight.cooldown = (fight.cooldown - time.delta_seconds()).max(0.0);
			if fight.cooldown == 0.0 {
				if let Some(Attack) = &fight.attack {
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

pub struct Fighter {
	pub attack: Option<Attack>,
	pub cooldown: f32,
	pub parameters: balance::Attack,
}

impl Fighter {
	pub fn new(parameters: balance::Attack) -> Fighter {
		Fighter { attack: None, cooldown: 0.0, parameters }
	}
}

impl Component for Fighter {
	type Storage = DenseVecStorage<Fighter>;
}

pub struct Attack;
