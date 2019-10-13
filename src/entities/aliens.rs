use crate::{
	balance::Balance, entities::camera::{ARENA_HEIGHT, ARENA_WIDTH}, systems::{animation::Facing, life::Life, movement::Walking}
};
use amethyst::{
	assets::Handle, core::{math::Vector3, Transform}, ecs::{Component, DenseVecStorage}, prelude::*, renderer::{SpriteRender, SpriteSheet}
};
use rand::{rngs::ThreadRng, Rng};

const ALIEN_WIDTH: f32 = 2.0;
const ALIEN_HEIGHT: f32 = 2.06;

pub struct Alien {
	pub action: Action,
	pub timeout: f32,
}

pub enum Action {
	Standing,
	Walking,
}

impl Alien {
	pub fn gen_timeout_standing(rng: &mut ThreadRng) -> f32 {
		rng.gen_range(0., 12.)
	}

	pub fn gen_timeout_walking(rng: &mut ThreadRng) -> f32 {
		rng.gen_range(1., 2.)
	}
}

impl Component for Alien {
	type Storage = DenseVecStorage<Self>;
}

pub fn initialize(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
	let parameters = world.read_resource::<Balance>().aliens.clone();
	let mut rng = rand::thread_rng();
	for _ in 0..parameters.count {
		let x = rng.gen_range(-ARENA_WIDTH / 2. + ALIEN_WIDTH / 2., ARENA_WIDTH / 2. - ALIEN_WIDTH / 2.);
		let y = rng.gen_range(-ARENA_HEIGHT / 2. + ALIEN_HEIGHT / 2., ARENA_HEIGHT / 2. - ALIEN_HEIGHT / 2.);
		let mut transform = Transform::default();
		transform.set_translation_x(x);
		transform.set_translation_y(y);
		let mut scale = Vector3::zeros();
		scale.fill(0.02);
		transform.set_scale(scale);
		world
			.create_entity()
			.with(Alien { timeout: Alien::gen_timeout_standing(&mut rng), action: Action::Standing })
			.with(Life { health: parameters.base_health })
			.with(Walking::new(parameters.walking.clone()))
			.with(if rng.gen() { Facing::Left } else { Facing::Right })
			.with(transform)
			.with(SpriteRender { sprite_sheet: sprite_sheet.clone(), sprite_number: 3 })
			.build();
	}
}
