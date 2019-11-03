use amethyst::{
	core::{SystemDesc, Time}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Entities, Join, ReadExpect, System, SystemData, World, WriteStorage}, renderer::resources::Tint
};

pub struct Life {
	pub health: f32,
	pub since_attack: f32,
}

impl Component for Life {
	type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct CycleOfLife;

impl<'s> System<'s> for CycleOfLife {
	type SystemData = (WriteStorage<'s, Life>, WriteStorage<'s, Tint>, ReadExpect<'s, Time>, Entities<'s>);

	fn run(&mut self, (mut lives, mut tints, time, entities): Self::SystemData) {
		let mut to_delete = Vec::new();
		for (life, tint, entity) in (&mut lives, &mut tints, &entities).join() {
			tint.0.red = (tint.0.red - time.delta_seconds() * 8.).max(1.);
			tint.0.green = (tint.0.green - time.delta_seconds() * 8.).max(1.);
			tint.0.blue = (tint.0.blue - time.delta_seconds() * 8.).max(1.);
			if life.since_attack == 0. {
				tint.0.red = 4.;
				tint.0.green = 4.;
				tint.0.blue = 4.;
			}
			life.since_attack += time.delta_seconds();
			if life.health <= 0.0 {
				to_delete.push(entity);
			}
		}
		for entity in to_delete {
			entities.delete(entity).unwrap();
		}
	}
}
