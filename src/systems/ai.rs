use crate::{
	entities::camera::{ARENA_HEIGHT, ARENA_WIDTH}, systems::{
		aliens::{Alien, AlienState}, movement::Walking
	}
};
use amethyst::{
	core::{math::Vector2, SystemDesc, Time, Transform}, derive::SystemDesc, ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage}
};
use rand::{rngs::ThreadRng, Rng};

#[derive(SystemDesc)]
pub struct AI;

impl<'s> System<'s> for AI {
	type SystemData = (WriteStorage<'s, Alien>, WriteStorage<'s, Walking>, ReadStorage<'s, Transform>, Read<'s, Time>);

	fn run(&mut self, (mut aliens, mut walks, transforms, time): Self::SystemData) {
		let mut rng = rand::thread_rng();
		for (alien, walk, transform) in (&mut aliens, &mut walks, &transforms).join() {
			alien.timeout -= time.delta_seconds();
			if alien.timeout <= 0. {
				match alien.state {
					AlienState::Standing => {
						let pos = Vector2::new(transform.translation().x, transform.translation().y);
						let direction = gen_direction(&mut rng, &pos);
						walk.intent = direction;
						alien.state = AlienState::Walking;
						alien.timeout = Alien::gen_timeout_walking(&mut rng);
					},
					AlienState::Walking { .. } => {
						walk.intent = Vector2::zeros();
						alien.state = AlienState::Standing;
						alien.timeout = Alien::gen_timeout_standing(&mut rng);
					},
				}
			}
		}
	}
}

fn gen_direction(rng: &mut ThreadRng, pos: &Vector2<f32>) -> Vector2<f32> {
	let path = if is_inside_arena(&pos) {
		let target = Vector2::new(rng.gen_range(-ARENA_WIDTH / 4., ARENA_WIDTH / 4.), rng.gen_range(-ARENA_HEIGHT / 4., ARENA_HEIGHT / 4.));
		target - pos
	} else {
		Vector2::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.))
	};
	path.normalize()
}

fn is_inside_arena(pos: &Vector2<f32>) -> bool {
	pos[0].abs() >= ARENA_WIDTH / 2. || pos[1].abs() >= ARENA_HEIGHT / 2.
}
