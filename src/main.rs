mod configs;
mod entities;
mod graphics;
mod state;

use crate::state::Game;
use amethyst::{
	core::TransformBundle, renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle}, Application, GameDataBuilder
};

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());
	let root_dir = amethyst::utils::application_root_dir()?;
	let game_data = GameDataBuilder::default()
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(RenderToWindow::from_config(configs::window(&root_dir)?).with_clear([0., 0., 0., 1.]))
				.with_plugin(RenderFlat2D::default()),
		)?
		.with_bundle(TransformBundle::new())?;
	let mut game = Application::build(root_dir.join("assets"), Game::default())?.build(game_data)?;
	game.run();
	Ok(())
}
