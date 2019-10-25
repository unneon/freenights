use crate::{
	balance, entities::{aliens::Alien, flash}, graphics::GlobalSpriteSheet, systems::life::Life
};
use amethyst::{
	core::{SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Entities, Join, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage}, renderer::{resources::Tint, SpriteRender}
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
	type SystemData = (
		WriteStorage<'s, Fighting>,
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Alien>,
		WriteStorage<'s, Life>,
		WriteStorage<'s, SpriteRender>,
		WriteStorage<'s, Tint>,
		ReadExpect<'s, Time>,
		ReadExpect<'s, GlobalSpriteSheet>,
		Entities<'s>,
	);

	fn run(
		&mut self,
		(mut fights, mut transforms, aliens, mut lives, mut sprite_renders, mut tints, time, sprite_sheet, entities): Self::SystemData,
	) {
		let mut flashes = Vec::new();
		for (fight, attacker_transform) in (&mut fights, &transforms).join() {
			fight.cooldown = (fight.cooldown - time.delta_seconds()).max(0.0);
			if fight.cooldown == 0.0 {
				if let Some(Swing) = &fight.swing {
					for (defender_transform, alien, life, tint) in (&transforms, &aliens, &mut lives, &mut tints).join() {
						let distance = (attacker_transform.translation() - defender_transform.translation()).norm();
						if distance <= fight.parameters.range + alien.radius {
							flashes.push((attacker_transform.translation().clone(), fight.parameters.range));
							tint.0.red = 4.;
							tint.0.green = 4.;
							tint.0.blue = 4.;
							life.health -= fight.parameters.power;
						}
					}
					fight.cooldown = fight.parameters.cooldown;
				}
			}
		}
		for (_alien, tint) in (&aliens, &mut tints).join() {
			tint.0.red = (tint.0.red - time.delta_seconds() * 8.).max(1.);
			tint.0.green = (tint.0.green - time.delta_seconds() * 8.).max(1.);
			tint.0.blue = (tint.0.blue - time.delta_seconds() * 8.).max(1.);
		}
		for (pos, radius) in flashes {
			flash::create(&pos, radius, &entities, &mut transforms, &mut sprite_renders, sprite_sheet.0.clone());
		}
	}
}
