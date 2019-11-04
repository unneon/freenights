use crate::{
	balance::Balance, systems::{
		animation::Facing, combat::Fighting, grab::{GrabDesire, Inventory}, movement::Walking
	}
};
use amethyst::{
	assets::Handle, core::{math::Vector3, Transform}, ecs::{Component, DenseVecStorage}, prelude::*, renderer::{SpriteRender, SpriteSheet}
};
use std::collections::HashMap;

pub struct Player;

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}

pub fn initialize(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
	let movement_parameters = world.read_resource::<Balance>().player.walking.clone();
	let combat_parameters = world.read_resource::<Balance>().player.attack.clone();
	let mut transform = Transform::default();
	let mut scale = Vector3::zeros();
	scale.fill(0.02);
	transform.set_scale(scale);
	world
		.create_entity()
		.with(Player)
		.with(Walking::new(movement_parameters))
		.with(Fighting::new(combat_parameters))
		.with(GrabDesire { target: None })
		.with(Inventory { storage: HashMap::new() })
		.with(Facing::Right)
		.with(transform)
		.with(SpriteRender { sprite_sheet, sprite_number: 1 })
		.build();
}
