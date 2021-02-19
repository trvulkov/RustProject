use std::collections::HashMap;

use crate::node::Node;
use crate::enums::Color;
use crate::enums::State;
use crate::enums::Direction;

use crate::errors::PlacingError;
use crate::errors::MovingError;
use crate::errors::RemovingError;

use splitmut::{SplitMut, SplitMutError};

#[derive(Clone)]
pub struct Board {
    nodes: HashMap<&'static str, Node>
}

impl Board {
    pub fn new() -> Self {
        let nodes: HashMap<&str, Node> = [
            // neighbours:   above       left        right       below
            ("a7", Node::new(None,       None,       Some("d7"), Some("a4"))),
            ("d7", Node::new(None,       Some("a7"), Some("g7"), Some("d6"))),
            ("g7", Node::new(None,       Some("d7"), None,       Some("g4"))),

            ("b6", Node::new(None,       None,       Some("d6"), Some("b4"))),
            ("d6", Node::new(Some("d7"), Some("b6"), Some("f6"), Some("d5"))),
            ("f6", Node::new(None,       Some("d6"), None,       Some("f4"))),

            ("c5", Node::new(None,       None,       Some("d5"), Some("c4"))),
            ("d5", Node::new(Some("d6"), Some("c5"), Some("e5"), None      )),
            ("e5", Node::new(None,       Some("d5"), None,       Some("e4"))),

            ("a4", Node::new(Some("a7"), None,       Some("b4"), Some("a1"))),
            ("b4", Node::new(Some("b6"), Some("a4"), Some("c4"), Some("b2"))),
            ("c4", Node::new(Some("c5"), Some("b4"), None,       Some("c3"))),
            ("e4", Node::new(Some("e5"), None,       Some("f4"), Some("e3"))),
            ("f4", Node::new(Some("f6"), Some("e4"), Some("g4"), Some("f2"))),
            ("g4", Node::new(Some("g7"), Some("f4"), None,       Some("g1"))),

            ("c3", Node::new(Some("c4"), None,       Some("d3"), None      )),
            ("d3", Node::new(None,       Some("c3"), Some("e3"), Some("d2"))),
            ("e3", Node::new(Some("e4"), Some("d3"), None,       None      )),

            ("b2", Node::new(Some("b4"), None,       Some("d2"), None      )),
            ("d2", Node::new(Some("d3"), Some("b2"), Some("f2"), Some("d1"))),
            ("f2", Node::new(Some("f4"), Some("d2"), None,       None      )),

            ("a1", Node::new(Some("a4"), None,       Some("d1"), None      )),
            ("d1", Node::new(Some("d2"), Some("a1"), Some("g1"), None      )),
            ("g1", Node::new(Some("g4"), Some("d1"), None,       None      )),
        ].iter().cloned().collect();

        Self { nodes }
    }

    pub fn get_node_ref(&self, position: &str) -> Option<&Node> { // only used for testing
        self.nodes.get(position)
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, " 7 {}-----------{}-----------{}", self.nodes["a7"], self.nodes["d7"], self.nodes["g7"]).expect("ERROR when printing board");
        writeln!(f, "   |           |           |").expect("ERROR when printing board");
        writeln!(f, " 6 |   {}-------{}-------{}   |", self.nodes["b6"], self.nodes["d6"], self.nodes["f6"]).expect("ERROR when printing board");
        writeln!(f, "   |   |       |       |   |").expect("ERROR when printing board");
        writeln!(f, " 5 |   |   {}---{}---{}   |   |", self.nodes["c5"], self.nodes["d5"], self.nodes["e5"]).expect("ERROR when printing board");
        writeln!(f, "   |   |   |       |   |   |").expect("ERROR when printing board");
        writeln!(f, " 4 {}---{}---{}       {}---{}---{}", self.nodes["a4"], self.nodes["b4"], self.nodes["c4"], self.nodes["e4"], self.nodes["f4"], self.nodes["g4"]).expect("ERROR when printing board");
        writeln!(f, "   |   |   |       |   |   |").expect("ERROR when printing board");
        writeln!(f, " 3 |   |   {}---{}---{}   |   |", self.nodes["c3"], self.nodes["d3"], self.nodes["e3"]).expect("ERROR when printing board");  
        writeln!(f, "   |   |       |       |   |").expect("ERROR when printing board");
        writeln!(f, " 2 |   {}-------{}-------{}   |", self.nodes["b2"], self.nodes["d2"], self.nodes["f2"]).expect("ERROR when printing board");  
        writeln!(f, "   |           |           |").expect("ERROR when printing board");
        writeln!(f, " 1 {}-----------{}-----------{}", self.nodes["a1"], self.nodes["d1"], self.nodes["g1"]).expect("ERROR when printing board");  
        writeln!(f, "   a   b   c   d   e   f   g").expect("ERROR when printing board");

        Ok(())
    }
}

impl Board { // game logic - manipulating pieces
    // Places a piece of the given color at the given position (if the position isn't actually on the board, or it is but is already occupied - an error is returned).
    pub fn place_piece(&mut self, color: Color, position: &str) -> Result<(), PlacingError> {
        match self.nodes.get_mut(position) {
            None => Err(PlacingError::InvalidPosition),
            Some(node) =>
                match node.state {
                    State::Occupied(_) => Err(PlacingError::PlaceAtOccupied),
                    State::Empty => {
                        node.state = State::Occupied(color);
                        Ok(())
                    }
                }
        }
    }

    // Moves a piece of a given color between two positions
    // (if either of the positions isn't on the board, the first one isn't occupied by a piece of the given color, or the second one isn't empty - an error is returned).
    // The boolean argument determines whether the piece can "fly" - if not, a check is made for adjacency between the positions (and an error is returned if it fails).
    pub fn move_piece(&mut self, color: Color, from: &str, to: &str, flying: bool) -> Result<(), MovingError> {
        let (start, end) = self.nodes.get2_mut(from, to);
        match start {
            Err(_) => Err(MovingError::InvalidMoveFrom),
            Ok(start) =>
                match end {
                    Err(SplitMutError::SameValue) => Err(MovingError::MoveToSame),
                    Err(SplitMutError::NoValue) => Err(MovingError::InvalidMoveTo),
                    Ok(end) => {
                        match &start.state {
                            State::Empty => Err(MovingError::MoveFromEmpty),
                            State::Occupied(start_color) => {
                                if *start_color != color {
                                    return Err(MovingError::MoveFromWrongColor);
                                } 

                                match &end.state {
                                    State::Occupied(_) => Err(MovingError::MoveToOccupied),
                                    State::Empty => {
                                        if flying == false && start.adjacent().contains(&to) == false {
                                            return Err(MovingError::NotAdjacent);
                                        }

                                        start.state = State::Empty;
                                        end.state = State::Occupied(color);
                                        Ok(())
                                    }
                                }
                            }
                        }
                    }
            }
        }
    }

    // Removes a piece of the given color from the given position
    // (if the position isn't actually on the board, or there isn't a piece of the given color on it, an error is returned;
    // if 'check_for_mills' is true, and an attempt is made to remove a piece that is currently in a mill, an error is returned).
    pub fn remove_piece(&mut self, color: Color, from: &str, check_for_mills: bool) -> Result<(), RemovingError> {
        let position_in_mill = self.in_mill(color, from);

        match self.nodes.get_mut(from) {
            None => Err(RemovingError::InvalidPosition),
            Some(node) =>
                match &node.state {
                    State::Empty => Err(RemovingError::RemoveFromEmpty),
                    State::Occupied(node_color) => {
                        if *node_color != color {
                            return Err(RemovingError::RemoveFromWrongColor);
                        }

                        /*if check_for_mills {
                            if self.in_mill(color, from) {
                                return Err(RemovingError::RemoveFromMill);
                            }
                        }*/
                        if check_for_mills && position_in_mill {
                            return Err(RemovingError::RemoveFromMill);
                        }

                        node.state = State::Empty;
                        Ok(())
                    }
                        
                }
        }
    }

    // Checks if a piece of the given color can move from the given position (i.e. if there are any unoccupied positions adjacent to it).
    pub fn can_move(&self, color: Color, position: &str) -> bool {
        if let Some(node) = self.nodes.get(position)  {
            if node.state == State::Occupied(color) {
                for neighbour in node.adjacent() {
                    if let Some(neighbour_node) = self.nodes.get(neighbour) {
                        if neighbour_node.state == State::Empty {
                            return true;
                        }
                    }
                }
            } 
        }

        false
    }
}

impl Board { // game logic - mills
    // Checks if a piece is in a mill, by calling the middle_of_mill() and edge_of_mill() functions
    pub fn in_mill(&self, color: Color, position: &str) -> bool {
        if let Some(node) = self.nodes.get(position) {
            return self.middle_of_mill(color, *node) || self.edge_of_mill(color, *node)
        }

        false
    }

    // Checks if a piece is in the middle of mill, i.e. if it forms a mill with two of it's neighbours (either the ones to the left and to the right, or above and below).
    // Only called by the in_mill() function.
    fn middle_of_mill(&self, color: Color, node: Node) -> bool {
        // check for horizontal mill
        if let Some(left) = node.left {
            if let Some(right) = node.right {
                if let Some(left_node) = self.nodes.get(left) {
                    if let Some(right_node) = self.nodes.get(right) {
                        if left_node.state == State::Occupied(color) && right_node.state == State::Occupied(color) {
                            return true;
                        }
                    }
                }
            }
        }

        // check for vertical mill
        if let Some(above) = node.above {
            if let Some(below) = node.below {
                if let Some(above_node) = self.nodes.get(above) {
                    if let Some(below_node) = self.nodes.get(below) {
                        if above_node.state == State::Occupied(color) && below_node.state == State::Occupied(color) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    // Checks if a piece is the edge of mill, i.e. if it forms a mill with a neighbour and that neighbour's neighbour in one of the four directions.
    // Only called by the in_mill() function.
    fn edge_of_mill(&self, color: Color, node: Node) -> bool {
        self.check_direction(color, node, Direction::Above, 1) ||
        self.check_direction(color, node, Direction::Left,  1) ||
        self.check_direction(color, node, Direction::Right, 1) ||
        self.check_direction(color, node, Direction::Below, 1)
    }
    // Checks if the given node forms a mill in the given direction (i.e. the neighbour and that neighbour's neighbour in said direction are the same color).
    // The 'found' parameter has to be 1 at the first call.
    fn check_direction(&self, color: Color, node: Node, direction: Direction, found: u8) -> bool {
        if found == 3 {
            return true;
        }

        if let Some(neighbour) = node.get_neighbour(direction) {
            if let Some(neighbour_node) = self.nodes.get(neighbour) {
                return neighbour_node.state == State::Occupied(color) && self.check_direction(color, *neighbour_node, direction, found + 1)
            }
        }

        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place() {
        let mut board = Board::new();

        assert_eq!(board.place_piece(Color::White, "a7"), Ok(()));
        let a7 = board.nodes.get("a7");
        assert!(a7.is_some());
        assert_eq!(a7.unwrap().state, State::Occupied(Color::White));

        assert_eq!(board.place_piece(Color::Black, "a4"), Ok(()));
        let a4 = board.nodes.get("a4");
        assert!(a4.is_some());
        assert_eq!(a4.unwrap().state, State::Occupied(Color::Black));

        assert_eq!(board.place_piece(Color::White, "a7"), Err(PlacingError::PlaceAtOccupied));
        assert_eq!(board.place_piece(Color::Black, "a7"), Err(PlacingError::PlaceAtOccupied));
        assert_eq!(board.place_piece(Color::White, "а8"), Err(PlacingError::InvalidPosition));
    }
    #[test]
    fn test_move() {
        let mut board = Board::new();

        assert_eq!(board.place_piece(Color::White, "a7"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a4"), Ok(()));
        assert_eq!(board.place_piece(Color::Black, "g1"), Ok(()));

        assert_eq!(board.move_piece(Color::White, "a8", "a1", false), Err(MovingError::InvalidMoveFrom));
        assert_eq!(board.move_piece(Color::White, "a7", "a8", false), Err(MovingError::InvalidMoveTo));
        assert_eq!(board.move_piece(Color::White, "a1", "d1", false), Err(MovingError::MoveFromEmpty));
        assert_eq!(board.move_piece(Color::White, "g1", "d1", false), Err(MovingError::MoveFromWrongColor));
        assert_eq!(board.move_piece(Color::White, "a7", "a4", false), Err(MovingError::MoveToOccupied));
        assert_eq!(board.move_piece(Color::White, "a7", "a7", false), Err(MovingError::MoveToSame));
        assert_eq!(board.move_piece(Color::White, "a7", "a1", false), Err(MovingError::NotAdjacent));

        assert_eq!(board.move_piece(Color::White, "a7", "d7", false), Ok(()));
        assert_eq!(board.nodes.get("a7").unwrap().state, State::Empty);
        assert_eq!(board.nodes.get("d7").unwrap().state, State::Occupied(Color::White));

        assert_eq!(board.move_piece(Color::White, "d7", "d1", true), Ok(()));
        assert_eq!(board.nodes.get("d7").unwrap().state, State::Empty);
        assert_eq!(board.nodes.get("d1").unwrap().state, State::Occupied(Color::White));


        assert_eq!(board.move_piece(Color::Black, "g1", "g4", true), Ok(()));
        assert_eq!(board.nodes.get("g1").unwrap().state, State::Empty);
        assert_eq!(board.nodes.get("g4").unwrap().state, State::Occupied(Color::Black));

        assert_eq!(board.move_piece(Color::Black, "g4", "a1", true), Ok(()));
        assert_eq!(board.nodes.get("g4").unwrap().state, State::Empty);
        assert_eq!(board.nodes.get("a1").unwrap().state, State::Occupied(Color::Black));
    }
    #[test]
    fn test_remove() {
        let mut board = Board::new();

        assert_eq!(board.place_piece(Color::White, "a7"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a4"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "g7"), Ok(()));
        assert_eq!(board.place_piece(Color::Black, "g1"), Ok(()));

        assert_eq!(board.remove_piece(Color::White, "a8", false), Err(RemovingError::InvalidPosition));
        assert_eq!(board.remove_piece(Color::White, "g4", false), Err(RemovingError::RemoveFromEmpty));
        assert_eq!(board.remove_piece(Color::White, "g1", false), Err(RemovingError::RemoveFromWrongColor));
        assert_eq!(board.remove_piece(Color::White, "a7", true), Err(RemovingError::RemoveFromMill));

        assert_eq!(board.remove_piece(Color::White, "a7", false), Ok(()));
        assert_eq!(board.nodes.get("a7").unwrap().state, State::Empty);

        assert_eq!(board.remove_piece(Color::White, "a4", true), Ok(()));
        assert_eq!(board.nodes.get("a4").unwrap().state, State::Empty);

        assert_eq!(board.remove_piece(Color::Black, "g1", true), Ok(()));
        assert_eq!(board.nodes.get("g1").unwrap().state, State::Empty);
    }

    #[test]
    fn test_can_move() {
        let mut board = Board::new();

        assert_eq!(board.place_piece(Color::White, "a7"), Ok(()));
        assert_eq!(board.place_piece(Color::Black, "a4"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "b4"), Ok(()));
        assert_eq!(board.place_piece(Color::Black, "d1"), Ok(()));


        assert!(board.can_move(Color::White, "a4") == false);
        assert!(board.can_move(Color::White, "a8") == false);
        assert!(board.can_move(Color::Black, "a7") == false);

        assert!(board.can_move(Color::White, "a7"));
        assert!(board.can_move(Color::White, "b4"));
        assert!(board.can_move(Color::Black, "d1"));

        assert!(board.can_move(Color::Black, "a4") == false);
        assert!(board.can_move(Color::White, "a1") == false);
    }

    #[test]
    fn test_mills() {
        let mut board = Board::new();

        assert_eq!(board.place_piece(Color::White, "a7"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a4"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "d1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "g1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "g7"), Ok(()));

        assert_eq!(board.place_piece(Color::Black, "e3"), Ok(()));
        assert_eq!(board.place_piece(Color::Black, "e4"), Ok(()));
        assert_eq!(board.place_piece(Color::Black, "e5"), Ok(()));
        assert_eq!(board.place_piece(Color::Black, "d3"), Ok(()));

        assert!(board.in_mill(Color::White, "a7"));
        assert!(board.in_mill(Color::White, "a4"));
        assert!(board.in_mill(Color::White, "a1"));
        assert!(board.in_mill(Color::White, "d1"));
        assert!(board.in_mill(Color::White, "g1"));
        assert!(board.in_mill(Color::White, "g7") == false);
        assert!(board.in_mill(Color::Black, "a7") == false);

        assert!(board.in_mill(Color::White, "e3") == false);
        assert!(board.in_mill(Color::Black, "e3"));
        assert!(board.in_mill(Color::Black, "e4"));
        assert!(board.in_mill(Color::Black, "e5"));
        assert!(board.in_mill(Color::Black, "d3") == false);
    }
    #[test]
    fn test_edge_of_mill() {
        let mut board = Board::new();

        assert_eq!(board.place_piece(Color::White, "a7"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a4"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "d1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "g1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "g7"), Ok(()));


        assert!(board.edge_of_mill(Color::White, *board.nodes.get("a7").unwrap()));
        assert!(board.edge_of_mill(Color::White, *board.nodes.get("a4").unwrap()) == false);
        assert!(board.edge_of_mill(Color::White, *board.nodes.get("a1").unwrap()));
        assert!(board.edge_of_mill(Color::White, *board.nodes.get("d1").unwrap()) == false);
        assert!(board.edge_of_mill(Color::White, *board.nodes.get("g1").unwrap()));
        assert!(board.edge_of_mill(Color::White, *board.nodes.get("g7").unwrap()) == false);

        assert!(board.check_direction(Color::White, *board.nodes.get("a7").unwrap(), Direction::Above, 1) == false);
        assert!(board.check_direction(Color::White, *board.nodes.get("a7").unwrap(), Direction::Left, 1) == false);
        assert!(board.check_direction(Color::White, *board.nodes.get("a7").unwrap(), Direction::Right, 1) == false);
        assert!(board.check_direction(Color::White, *board.nodes.get("a7").unwrap(), Direction::Below, 1));

        assert!(board.check_direction(Color::White, *board.nodes.get("a4").unwrap(), Direction::Above, 1) == false);
        assert!(board.check_direction(Color::White, *board.nodes.get("a4").unwrap(), Direction::Below, 1) == false);

        assert!(board.check_direction(Color::White, *board.nodes.get("a1").unwrap(), Direction::Right, 1));
        assert!(board.check_direction(Color::White, *board.nodes.get("g1").unwrap(), Direction::Left, 1));
    }
    #[test]
    fn test_middle_of_mill() {
        let mut board = Board::new();

        assert_eq!(board.place_piece(Color::White, "a7"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a4"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "a1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "d1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "g1"), Ok(()));
        assert_eq!(board.place_piece(Color::White, "g7"), Ok(()));

        assert!(board.middle_of_mill(Color::White, *board.nodes.get("a7").unwrap()) == false);
        assert!(board.middle_of_mill(Color::White, *board.nodes.get("a4").unwrap()));
        assert!(board.middle_of_mill(Color::White, *board.nodes.get("a1").unwrap()) == false);
        assert!(board.middle_of_mill(Color::White, *board.nodes.get("d1").unwrap()));
        assert!(board.middle_of_mill(Color::White, *board.nodes.get("g1").unwrap()) == false);
        assert!(board.middle_of_mill(Color::White, *board.nodes.get("g7").unwrap()) == false);
    }
}