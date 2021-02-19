// player colors
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn other(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "white (○)"),
            Color::Black => write!(f, "black (●)"),
        }
    }
}

// the two main phases of the game - placing pieces (until each player places all 9 of theirs) and moving the already placed pieces
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Phase {
    Placing,
    Moving
}

// the state of a position on the board
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum State {
    Empty,
    Occupied(Color)
}

// one of the four directions on the board
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Above,
    Left,
    Right,
    Below
}