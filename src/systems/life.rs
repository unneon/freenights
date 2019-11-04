use crate::{graphics::GlobalSpriteSheet, item::ItemDatabase, systems::grab::GrabTarget};
use amethyst::{
	core::{math::Vector3, SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Entities, Join, ReadExpect, System, SystemData, World, WriteStorage}, renderer::{palette::Srgba, resources::Tint, SpriteRender}
};
use rand::Rng;
use serde::Deserialize;

pub struct Life {
	pub health: f32,
	pub since_attack: f32,
	pub loot: LootPool,
}

impl Component for Life {
	type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Deserialize)]
pub struct LootPool(pub Vec<LootKind>);

#[derive(Clone, Deserialize)]
pub struct LootKind {
	pub probability: f32,
	pub count: (i32, i32),
	pub item: String,
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
	type SystemData = (
		WriteStorage<'s, Life>,
		WriteStorage<'s, Tint>,
		WriteStorage<'s, SpriteRender>,
		WriteStorage<'s, Transform>,
		WriteStorage<'s, GrabTarget>,
		ReadExpect<'s, GlobalSpriteSheet>,
		ReadExpect<'s, ItemDatabase>,
		ReadExpect<'s, Time>,
		Entities<'s>,
	);

	fn run(
		&mut self,
		(mut lives, mut tints, mut renders, mut transforms, mut grab_targets, sprite_sheets, item_database, time, entities): Self::SystemData,
	) {
		let mut to_delete = Vec::new();
		let mut to_spawn = Vec::new();
		for (life, tint, transform, entity) in (&mut lives, &mut tints, &transforms, &entities).join() {
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
					to_spawn.push((transform.clone(), count, item))
				}
			}
		}
		for entity in to_delete {
			entities.delete(entity).unwrap();
		}
		let mut rng = rand::thread_rng();
		for (mut transform, count, item) in to_spawn {
			let mut scale = Vector3::zeros();
			scale.fill(0.08);
			transform.set_scale(scale);
			let item_data = &item_database.0[&item];
			for _ in 0..count {
				let mut transform = transform.clone();
				let translation = transform.translation_mut();
				translation.x += rng.gen_range(-1., 1.);
				translation.y += rng.gen_range(-1., 1.);
				entities
					.build_entity()
					.with(item_data.sprite.create_render(&sprite_sheets), &mut renders)
					.with(Tint(Srgba::new(1., 1., 1., 1.)), &mut tints)
					.with(transform.clone(), &mut transforms)
					.with(GrabTarget { item: item.clone() }, &mut grab_targets)
					.build();
			}
		}
	}
}
