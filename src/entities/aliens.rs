use crate::{
	balance::Balance, entities::camera::{ARENA_HEIGHT, ARENA_WIDTH}
};
use amethyst::{
	assets::Handle, core::{math::Vector3, Transform}, prelude::*, renderer::{SpriteRender, SpriteSheet}
};
use rand::Rng;

const ALIEN_WIDTH: f32 = 2.0;
const ALIEN_HEIGHT: f32 = 2.06;

pub fn initialize(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
	let count = world.read_resource::<Balance>().aliens.count;
	let mut rng = rand::thread_rng();
	for _ in 0..count {
		let x = rng.gen_range(-ARENA_WIDTH / 2. + ALIEN_WIDTH / 2., ARENA_WIDTH / 2. - ALIEN_WIDTH / 2.);
		let y = rng.gen_range(-ARENA_HEIGHT / 2. + ALIEN_HEIGHT / 2., ARENA_HEIGHT / 2. - ALIEN_HEIGHT / 2.);
		let mut transform = Transform::default();
		transform.set_translation_x(x);
		transform.set_translation_y(y);
		let mut scale = Vector3::zeros();
		scale.fill(0.02);
		transform.set_scale(scale);
		world.create_entity().with(transform).with(SpriteRender { sprite_sheet: sprite_sheet.clone(), sprite_number: 3 }).build();
	}
}
