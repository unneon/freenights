use amethyst::{
	core::SystemDesc, derive::SystemDesc, ecs::{
		Component, DenseVecStorage, Entities, Entity, Join, ReadStorage, System, SystemData, World, WriteStorage
	}
};
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct GrabSystem;

impl<'s> System<'s> for GrabSystem {
	type SystemData = (
		WriteStorage<'s, GrabDesire>,
		ReadStorage<'s, GrabTarget>,
		WriteStorage<'s, Inventory>,
		Entities<'s>,
	);

	fn run(&mut self, (mut desires, grab_targets, mut inventories, entities): Self::SystemData) {
		for (desire, inventory) in (&mut desires, &mut inventories).join() {
			if let Some(target) = desire.target.take() {
				entities.delete(target).unwrap();
				let item = grab_targets.get(target).unwrap().item.clone();
				*inventory.storage.entry(item).or_insert(0) += 1;
				eprintln!("{:?}", inventory.storage);
			}
		}
	}
}

pub struct GrabDesire {
	pub target: Option<Entity>,
}

impl Component for GrabDesire {
	type Storage = DenseVecStorage<Self>;
}

pub struct Inventory {
	pub storage: HashMap<String, i64>,
}

impl Component for Inventory {
	type Storage = DenseVecStorage<Self>;
}

pub struct GrabTarget {
	pub item: String,
}

impl Component for GrabTarget {
	type Storage = DenseVecStorage<Self>;
}
