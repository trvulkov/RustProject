use std::collections::HashSet;

// stores information about the pieces of a player
pub struct Pieces {
    pub unplaced: u8,
    pub placed: u8,

    pub positions: HashSet<String>
}

impl Pieces {
    pub fn new() -> Self {
        Self {
            unplaced: 9,
            placed: 0,

            positions: HashSet::new()
        }
    }

    // manipulating pieces - these functions are called by functions of the Game class, in order to reflect the changes to the pieces
    pub fn place_piece(&mut self, position: String) { // called by Game.place()
        if self.unplaced > 0 {
            self.unplaced -= 1;
            self.placed += 1;

            self.positions.insert(position);
        }
    }
    pub fn remove_piece(&mut self, position: String) { // called by Game.remove()
        if self.placed > 0 {
            self.placed -= 1;

            self.positions.remove(&position);
        }
    }
    pub fn move_piece(&mut self, from: String, to: String) {
        self.positions.remove(&from);
        self.positions.insert(to);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place() {
        let mut pieces = Pieces::new();

        assert_eq!(pieces.unplaced, 9);
        assert_eq!(pieces.placed, 0);
        assert!(pieces.positions.is_empty());

        pieces.place_piece("a7".to_string());
        assert_eq!(pieces.unplaced, 8);
        assert_eq!(pieces.placed, 1);
        assert_eq!(pieces.positions, ["a7".to_string()].iter().cloned().collect());

        pieces.place_piece("g7".to_string());
        assert_eq!(pieces.unplaced, 7);
        assert_eq!(pieces.placed, 2);
        assert_eq!(pieces.positions, ["a7".to_string(), "g7".to_string()].iter().cloned().collect());

        pieces.place_piece("a1".to_string());
        pieces.place_piece("a4".to_string());
        pieces.place_piece("g1".to_string());
        pieces.place_piece("g4".to_string());
        pieces.place_piece("d1".to_string());
        pieces.place_piece("d7".to_string());
        pieces.place_piece("d3".to_string());
        assert_eq!(pieces.unplaced, 0);
        assert_eq!(pieces.placed, 9);
        assert_eq!(pieces.positions, ["a1".to_string(), "a4".to_string(), "a7".to_string(),
                                      "g1".to_string(), "g4".to_string(), "g7".to_string(),
                                      "d1".to_string(), "d3".to_string(), "d7".to_string()].iter().cloned().collect());

        pieces.place_piece("d6".to_string());
        assert_eq!(pieces.unplaced, 0);
        assert_eq!(pieces.placed, 9);
        assert_eq!(pieces.positions, ["a1".to_string(), "a4".to_string(), "a7".to_string(),
                                      "g1".to_string(), "g4".to_string(), "g7".to_string(),
                                      "d1".to_string(), "d3".to_string(), "d7".to_string()].iter().cloned().collect());                                  
    }

    #[test]
    fn test_remove() {
        let mut pieces = Pieces::new();
        pieces.place_piece("a7".to_string());
        assert_eq!(pieces.unplaced, 8);
        assert_eq!(pieces.placed, 1);
        assert_eq!(pieces.positions, ["a7".to_string()].iter().cloned().collect());

        pieces.remove_piece("a7".to_string());
        assert_eq!(pieces.unplaced, 8);
        assert_eq!(pieces.placed, 0);
        assert!(pieces.positions.is_empty());

        pieces.place_piece("a1".to_string());
        pieces.place_piece("a4".to_string());
        pieces.place_piece("d1".to_string());
        assert_eq!(pieces.unplaced, 5);
        assert_eq!(pieces.placed, 3);
        assert_eq!(pieces.positions, ["a1".to_string(), "a4".to_string(), "d1".to_string(), ].iter().cloned().collect());

        pieces.remove_piece("a1".to_string());
        pieces.remove_piece("a4".to_string());
        assert_eq!(pieces.unplaced, 5);
        assert_eq!(pieces.placed, 1);
        assert_eq!(pieces.positions, ["d1".to_string()].iter().cloned().collect());

        pieces.remove_piece("d1".to_string());
        assert_eq!(pieces.unplaced, 5);
        assert_eq!(pieces.placed, 0);
        assert!(pieces.positions.is_empty());

        pieces.remove_piece("d6".to_string());
        assert_eq!(pieces.unplaced, 5);
        assert_eq!(pieces.placed, 0);
        assert!(pieces.positions.is_empty());
    }

    #[test]
    fn test_move() {
        let mut pieces = Pieces::new();
        pieces.place_piece("a1".to_string());
        pieces.place_piece("d7".to_string());
        assert_eq!(pieces.unplaced, 7);
        assert_eq!(pieces.placed, 2);
        assert_eq!(pieces.positions, ["a1".to_string(), "d7".to_string()].iter().cloned().collect());

        pieces.move_piece("a1".to_string(), "a4".to_string());
        assert_eq!(pieces.unplaced, 7);
        assert_eq!(pieces.placed, 2);
        assert_eq!(pieces.positions, ["a4".to_string(), "d7".to_string()].iter().cloned().collect());

        pieces.move_piece("a4".to_string(), "a7".to_string());
        assert_eq!(pieces.unplaced, 7);
        assert_eq!(pieces.placed, 2);
        assert_eq!(pieces.positions, ["a7".to_string(), "d7".to_string()].iter().cloned().collect());
    }
}