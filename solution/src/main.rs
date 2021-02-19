mod enums;
mod errors;
mod board;
mod game;
mod node;
mod pieces;

// Nine men's morris, played through the command-line interface.

// During the initial phase, players are asked to input coordinates of a position on which to place their piece, e.g. "a7".
// During the second phase, players are asked to input the coordinates of two positions, in order to move a piece from the first to the second, e.g. "a7a4"

// On every turn, the board is printed to the console with the positions marked as either empty (·), occupied by a white piece (○), or occupied by a black piece (●)
// (note that on a black terminal background, the symbols can appear inversed - the white piece is a circle filled in with black,
//  whereas the black piece is a circle filled in with white).

use crate::game::Game;

fn main() {
    let mut game = Game::new();
	game.game_loop();
}
