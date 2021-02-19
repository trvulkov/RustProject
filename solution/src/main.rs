mod enums;
mod errors;
mod board;
mod game;
mod node;
mod pieces;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
	game.game_loop();
}
