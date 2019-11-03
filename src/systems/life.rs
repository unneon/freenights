use amethyst::{
	core::{SystemDesc, Time}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Entities, Join, ReadExpect, System, SystemData, World, WriteStorage}, renderer::resources::Tint
};
use rand::Rng;

pub struct Life {
	pub health: f32,
	pub since_attack: f32,
	pub loot: LootPool,
}

impl Component for Life {
	type Storage = DenseVecStorage<Self>;
}

pub struct LootPool(pub Vec<LootKind>);
pub struct LootKind {
	pub probability: f32,
	pub count: (i32, i32),
	pub item: &'static str,
}

impl LootPool {
	fn toss(&self) -> Vec<(i32, String)> {
		let mut rng = rand::thread_rng();
		let mut loot = Vec::new();
		for kind in &self.0 {
			if rng.gen_bool(kind.probability as f64) {
				loot.push((rng.gen_range(kind.count.0, kind.count.1 + 1), kind.item.to_owned()));
			}
		}
		loot
	}
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
				for (count, item) in life.loot.toss() {
					println!("Looted {}x {}!", count, item);
				}
			}
		}
		for entity in to_delete {
			entities.delete(entity).unwrap();
		}
	}
}
