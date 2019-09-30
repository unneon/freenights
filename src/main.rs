mod balance;
mod configs;
mod entities;
mod graphics;
mod state;
mod systems;
mod util;

use crate::state::Game;
use amethyst::{
	core::TransformBundle, input::{InputBundle, StringBindings}, renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle}, Application, GameDataBuilder
};

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());
	let root_dir = amethyst::utils::application_root_dir()?;
	let game_data = GameDataBuilder::default()
		.with_bundle(InputBundle::<StringBindings>::new().with_bindings(configs::bindings(&root_dir)?))?
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(RenderToWindow::from_config(configs::window(&root_dir)?).with_clear([0.675, 0.757, 0.184, 1.]))
				.with_plugin(RenderFlat2D::default()),
		)?
		.with_bundle(TransformBundle::new())?
		.with(systems::Movement, "movement", &["input_system"])
		.with(systems::Animation, "animation", &["movement"]);
	let mut game =
		Application::build(root_dir.join("assets"), Game::default())?.with_resource(balance::Balance::load(&root_dir)?).build(game_data)?;
	game.run();
	Ok(())
}
