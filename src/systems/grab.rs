use amethyst::{
	core::SystemDesc, derive::SystemDesc, ecs::{Component, DenseVecStorage, Entities, Entity, Join, System, SystemData, World, WriteStorage}
};

#[derive(SystemDesc)]
pub struct GrabSystem;

impl<'s> System<'s> for GrabSystem {
	type SystemData = (WriteStorage<'s, GrabDesire>, WriteStorage<'s, Inventory>, Entities<'s>);

	fn run(&mut self, (mut desires, mut inventories, entities): Self::SystemData) {
		for (desire, _inventory) in (&mut desires, &mut inventories).join() {
			if let Some(target) = desire.target.take() {
				entities.delete(target).unwrap();
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

pub struct Inventory {}

impl Component for Inventory {
	type Storage = DenseVecStorage<Self>;
}

pub struct GrabTarget {
	pub item: String,
}

impl Component for GrabTarget {
	type Storage = DenseVecStorage<Self>;
}
