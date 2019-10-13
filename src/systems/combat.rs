use crate::balance;
use amethyst::{
	core::{SystemDesc, Time}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Join, ReadExpect, System, SystemData, World, WriteStorage}
};

pub struct Fighting {
	pub swing: Option<Swing>,
	pub cooldown: f32,
	pub parameters: balance::Combat,
}

#[derive(Debug)]
pub enum Swing {
	Up,
}

#[derive(SystemDesc)]
pub struct Combat;

impl Fighting {
	pub fn new(parameters: balance::Combat) -> Fighting {
		Fighting { swing: None, cooldown: 0.0, parameters }
	}
}

impl Component for Fighting {
	type Storage = DenseVecStorage<Fighting>;
}

impl<'s> System<'s> for Combat {
	type SystemData = (WriteStorage<'s, Fighting>, ReadExpect<'s, Time>);

	fn run(&mut self, (mut fights, time): Self::SystemData) {
		for (fight,) in (&mut fights,).join() {
			fight.cooldown = (fight.cooldown - time.delta_seconds()).max(0.0);
			if fight.cooldown == 0.0 {
				if let Some(swing) = &fight.swing {
					eprintln!("Swung {:?}!", swing);
					fight.cooldown = fight.parameters.attack_cooldown;
				}
			}
		}
	}
}
