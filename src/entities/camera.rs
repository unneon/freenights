use amethyst::{core::Transform, prelude::*, renderer::Camera};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub fn initialize(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 1.0);
	world.create_entity().with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT)).with(transform).build();
}
