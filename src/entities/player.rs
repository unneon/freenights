use amethyst::{
	assets::Handle, core::Transform, ecs::{Component, DenseVecStorage}, prelude::*, renderer::{SpriteRender, SpriteSheet}
};

pub struct Player {}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}

pub fn initialize(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
	world.create_entity().with(Player {}).with(Transform::default()).with(SpriteRender { sprite_sheet, sprite_number: 0 }).build();
}
