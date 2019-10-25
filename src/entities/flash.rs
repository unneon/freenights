use amethyst::{
	assets::Handle, core::{math::Vector3, Transform}, ecs::{Entities, WriteStorage}, renderer::{SpriteRender, SpriteSheet}
};

pub fn create(
	source_pos: &Vector3<f32>,
	radius: f32,
	entities: &Entities,
	transforms: &mut WriteStorage<Transform>,
	sprite_renders: &mut WriteStorage<SpriteRender>,
	sprite_sheet: Handle<SpriteSheet>,
) {
	let mut transform = Transform::default();
	transform.set_translation_x(source_pos.x);
	transform.set_translation_y(source_pos.y);
	let mut scale = Vector3::zeros();
	scale.fill(radius / 100.0);
	transform.set_scale(scale);
	entities.build_entity().with(transform, transforms).with(SpriteRender { sprite_sheet, sprite_number: 4 }, sprite_renders).build();
}
