use crate::enums::Color;
use crate::enums::State;
use crate::enums::Direction;

#[derive(Clone, Copy)]
pub struct Node {
    pub state: State,

    // neighbours
    pub above: Option<&'static str>,
    pub left:  Option<&'static str>,
    pub right: Option<&'static str>,
    pub below: Option<&'static str>,
}

impl Node {
    pub fn new(above: Option<&'static str>, left: Option<&'static str>, right: Option<&'static str>, below: Option<&'static str>) -> Self {
        Self {
            state: State::Empty,
            above,
            left,
            right,
            below
        }
    }

    pub fn adjacent(&self) -> Vec<&str> {
        let neighbours = vec![self.above, self.left, self.right, self.below];

        neighbours.iter().filter(|elem| elem.is_some()).map(|elem| elem.unwrap()).collect()
    }

    // Returns the neighbour in a given direction - None if there is no neighbour in that direction, or Some(position of the neighbour) if there is.
    pub fn get_neighbour(&self, direction: Direction) -> Option<&str> {
        match direction {
            Direction::Above => self.above,
            Direction::Left => self.left,
            Direction::Right => self.right,
            Direction::Below => self.below
        }
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.state {
            State::Empty                  => write!(f, "·"),
            State::Occupied(Color::White) => write!(f, "○"),
            State::Occupied(Color::Black) => write!(f, "●"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let d7 = Node::new(None, Some("a7"), Some("g7"), Some("d6"));
        assert_eq!(d7.above, None);
        assert_eq!(d7.left,  Some("a7"));
        assert_eq!(d7.right, Some("g7"));
        assert_eq!(d7.below, Some("d6"));

        let b4 = Node::new(Some("b6"), Some("a4"), Some("c4"), Some("b2"));
        assert_eq!(b4.above, Some("b6"));
        assert_eq!(b4.left,  Some("a4"));
        assert_eq!(b4.right, Some("c4"));
        assert_eq!(b4.below, Some("b2"));

        let g1 = Node::new(Some("g4"), Some("d1"), None, None);
        assert_eq!(g1.above, Some("g4"));
        assert_eq!(g1.left,  Some("d1"));
        assert_eq!(g1.right, None);
        assert_eq!(g1.below, None);
    }

    #[test]
    fn test_adjacent() {
        let d7 = Node::new(None, Some("a7"), Some("g7"), Some("d6"));
        assert_eq!(d7.adjacent(), vec!["a7", "g7", "d6"]);

        let b4 = Node::new(Some("b6"), Some("a4"), Some("c4"), Some("b2"));
        assert_eq!(b4.adjacent(), vec!["b6", "a4", "c4", "b2"]);

        let g1 = Node::new(Some("g4"), Some("d1"), None, None);
        assert_eq!(g1.adjacent(), vec!["g4", "d1"]);
    }

    #[test]
    fn test_direction() {
        let d7 = Node::new(None, Some("a7"), Some("g7"), Some("d6"));
        assert_eq!(d7.get_neighbour(Direction::Above), None);
        assert_eq!(d7.get_neighbour(Direction::Left), Some("a7"));
        assert_eq!(d7.get_neighbour(Direction::Right), Some("g7"));
        assert_eq!(d7.get_neighbour(Direction::Below), Some("d6"));

        let b4 = Node::new(Some("b6"), Some("a4"), Some("c4"), Some("b2"));
        assert_eq!(b4.get_neighbour(Direction::Above), Some("b6"));
        assert_eq!(b4.get_neighbour(Direction::Left), Some("a4"));
        assert_eq!(b4.get_neighbour(Direction::Right), Some("c4"));
        assert_eq!(b4.get_neighbour(Direction::Below), Some("b2"));
    }
}
