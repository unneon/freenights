use amethyst::SimpleState;

pub enum Game {
	Play,
}

impl SimpleState for Game {
}

impl Default for Game {
	fn default() -> Self {
		Game::Play
	}
}
