use amethyst::{
	core::SystemDesc, derive::SystemDesc, ecs::{Component, DenseVecStorage, Entities, Join, ReadStorage, System, SystemData, World}
};

pub struct Life {
	pub health: f32,
}

impl Component for Life {
	type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct CycleOfLife;

impl<'s> System<'s> for CycleOfLife {
	type SystemData = (ReadStorage<'s, Life>, Entities<'s>);

	fn run(&mut self, (lives, entities): Self::SystemData) {
		let mut to_delete = Vec::new();
		for (life, entity) in (&lives, &entities).join() {
			if life.health <= 0.0 {
				to_delete.push(entity);
			}
		}
		for entity in to_delete {
			entities.delete(entity).unwrap();
		}
	}
}
