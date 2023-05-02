mod game;
mod utils;


fn main() {
	let gm = game::Game::init();
	gm.run();
}
