mod configs;
mod state;

use crate::state::Game;
use amethyst::{
	renderer::{types::DefaultBackend, RenderToWindow, RenderingBundle}, Application, GameDataBuilder
};

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());
	let root_dir = amethyst::utils::application_root_dir()?;
	let game_data = GameDataBuilder::default().with_bundle(
		RenderingBundle::<DefaultBackend>::new().with_plugin(RenderToWindow::from_config(configs::window(&root_dir)?).with_clear([0., 0., 0., 1.])),
	)?;
	let mut game = Application::build(root_dir.join("assets"), Game::default())?.build(game_data)?;
	game.run();
	Ok(())
}
