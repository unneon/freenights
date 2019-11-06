use crate::{
	balance::Balance, entities::camera::{ARENA_HEIGHT, ARENA_WIDTH}, graphics::GlobalSpriteSheet, systems::{animation::WalkAnimation, life::Alive, movement::Walking}
};
use amethyst::{
	core::{math::Vector3, SystemDesc, Transform}, derive::SystemDesc, ecs::{Component, DenseVecStorage, Entities, Join, ReadExpect, System, SystemData, World, WriteStorage}, renderer::{palette::rgb::Srgba, resources::Tint, SpriteRender}
};
use rand::{rngs::ThreadRng, Rng};

#[derive(SystemDesc)]
pub struct Aliens;

impl<'s> System<'s> for Aliens {
	type SystemData = (
		WriteStorage<'s, Alien>,
		WriteStorage<'s, Alive>,
		WriteStorage<'s, SpriteRender>,
		WriteStorage<'s, Tint>,
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Walking>,
		WriteStorage<'s, WalkAnimation>,
		ReadExpect<'s, Balance>,
		ReadExpect<'s, GlobalSpriteSheet>,
		Entities<'s>,
	);

	fn run(
		&mut self,
		(mut aliens, mut lives, mut sprites, mut tints, mut transforms, mut walks, mut walk_animations, balance, sprite_sheets, entities): Self::SystemData,
	) {
		let live_count = count_aliens(&mut aliens);
		let to_spawn = balance.aliens.count - live_count;
		for _ in 0..to_spawn {
			let mut rng = rand::thread_rng();
			let [x, y] = gen_near_rectangle(
				&mut rng,
				-ARENA_WIDTH / 2. - Alien::WIDTH / 2.,
				-ARENA_HEIGHT / 2. - Alien::HEIGHT / 2.,
				ARENA_WIDTH / 2. + Alien::WIDTH / 2.,
				ARENA_HEIGHT / 2. + Alien::HEIGHT / 2.,
				2.0,
			);
			let mut transform = Transform::default();
			transform.set_translation_x(x);
			transform.set_translation_y(y);
			let mut scale = Vector3::zeros();
			scale.fill(0.02);
			transform.set_scale(scale);
			entities
				.build_entity()
				.with(
					Alien {
						timeout: Alien::gen_timeout_standing(&mut rng),
						state: AlienState::Standing,
						radius: Alien::WIDTH.max(Alien::HEIGHT) / 2.0,
					},
					&mut aliens,
				)
				.with(
					Alive { health: balance.aliens.base_health, since_attack: std::f32::INFINITY, loot: balance.aliens.loot_pool.clone() },
					&mut lives,
				)
				.with(Walking::new(balance.aliens.walking.clone()), &mut walks)
				.with(if rng.gen() { WalkAnimation::Left } else { WalkAnimation::Right }, &mut walk_animations)
				.with(transform, &mut transforms)
				.with(SpriteRender { sprite_sheet: sprite_sheets.0["textures/spritesheet"].clone(), sprite_number: 3 }, &mut sprites)
				.with(Tint(Srgba::new(1., 1., 1., 1.)), &mut tints)
				.build();
		}
	}
}

fn count_aliens(aliens: &mut WriteStorage<Alien>) -> i32 {
	let mut count = 0;
	for (_alien,) in (aliens,).join() {
		count += 1;
	}
	count
}

fn gen_near_rectangle(rng: &mut ThreadRng, x1: f32, y1: f32, x2: f32, y2: f32, offset: f32) -> [f32; 2] {
	let sign = if rng.gen() { -1. } else { 1. };
	if rng.gen() {
		let local_y = (y2 - y1) / 2. + rng.gen_range(0., offset);
		[rng.gen_range(x1, x2), (y1 + y2) / 2. + sign * local_y]
	} else {
		let local_x = (x2 - x1) / 2. + rng.gen_range(0., offset);
		[(x1 + x2) / 2. + sign * local_x, rng.gen_range(y1, y2)]
	}
}

pub struct Alien {
	pub state: AlienState,
	pub timeout: f32,
	pub radius: f32,
}

impl Alien {
	const HEIGHT: f32 = 2.06;
	const WIDTH: f32 = 2.0;

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

#[derive(Debug)]
pub enum AlienState {
	Standing,
	Walking { to_middle: bool },
}
