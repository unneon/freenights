use crate::balance::Balance;
use amethyst::{
	assets::Handle, core::{
		math::{Vector2, Vector3}, Transform
	}, ecs::{Component, DenseVecStorage}, prelude::*, renderer::{SpriteRender, SpriteSheet}
};

pub struct Player {
	pub velocity: Vector2<f32>,
	pub facing: Facing,
}

pub enum Facing {
	Left,
	Right,
}

impl Player {
	pub fn compute_acceleration(&self, axes: Vector2<f32>, balance: &Balance) -> Vector2<f32> {
		balance.player.acceleration * axes - balance.player.drag * self.velocity
	}
}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}

pub fn initialize(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
	let mut transform = Transform::default();
	let mut scale = Vector3::zeros();
	scale.fill(0.02);
	transform.set_scale(scale);
	world
		.create_entity()
		.with(Player { velocity: Vector2::new(0., 0.), facing: Facing::Right })
		.with(transform)
		.with(SpriteRender { sprite_sheet, sprite_number: 1 })
		.build();
}
