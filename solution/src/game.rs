use crate::pieces::Pieces;
use crate::board::Board;
use crate::enums::Phase;
use crate::enums::Color;

use crate::errors::PlacingError;
use crate::errors::MovingError;
use crate::errors::RemovingError;

use std::io;
use std::io::BufRead;


pub struct Game {
    white: Pieces, // stores information about the white player's pieces
    black: Pieces, // stores information about the black player's pieces

    pub board: Board,
    phase: Phase,

    pub current: Color // the player who should play on the current turn. Changes to the other color every turn
}
impl Game {
    pub fn new() -> Self {
        Self {
            white: Pieces::new(), 
            black: Pieces::new(), 

            board: Board::new(),
            phase: Phase::Placing,

            current: Color::White 
        }
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "white: {} unplaced, {} placed, at {:?}", self.white.unplaced, self.white.placed, self.white.positions).expect("ERROR when printing game");
        writeln!(f, "black: {} unplaced, {} placed, at {:?}", self.black.unplaced, self.black.placed, self.black.positions).expect("ERROR when printing game");
        writeln!(f, "{}", self.board).expect("ERROR when printing game");

        Ok(())
    }
}

impl Game { // game actions
    // Requests the coordinates of a single position, reads them from the standard input and calls Board.place() with the color of the current player and said position.
    // Handles that function's errors and prints an approriate message if there is one.
    // If the placement is successful, calls the Pieces.place() function of the current player and returns the position.
    pub fn place_piece<R: BufRead>(&mut self, mut input: R) -> String {
        println!("{} player, PLACE your piece:", self.current);

        loop {
            let mut position = String::new();
            match input.read_line(&mut position) {
                Err(error) => println!("ERROR: input error - {}", error),
                Ok(_) => {
                    let trimmed_position = position.trim_end();
                    match self.board.place_piece(self.current, trimmed_position) {
                        Err(PlacingError::InvalidPosition) => println!("ERROR: Invalid position - {}!", trimmed_position),
                        Err(PlacingError::PlaceAtOccupied) => println!("ERROR: Position {} is already occupied!", trimmed_position),
                        Ok(()) => {
                            match self.current {
                                Color::White => self.white.place_piece(trimmed_position.to_string()),
                                Color::Black => self.black.place_piece(trimmed_position.to_string())
                            }

                            return trimmed_position.to_string();
                        }
                    }
                }
            }
        }
    }
    
    // Requests the coordinates of two positions, reads a single line from the standard input and checks if it is of the appropriate length.
    // If yes, also checks whether the current player can "fly" their pieces and calls Board.move() with the color of the current player, both positions,
    // and an appropriate boolean value for the "flying".
    // Handles that function's errors and prints an approriate message if there is one.
    // If the movement is successful, calls the Pieces.move() function of the current player and returns the second position.
    pub fn move_piece<R: BufRead>(&mut self, mut input: R) -> String {
        let flying = match self.current {
            Color::White => self.white.placed <= 3,
            Color::Black => self.black.placed <= 3
        };
        if flying {
            println!("{} player, MOVE your piece to any position ('fly'):", self.current);
        } else {
            println!("{} player, MOVE your piece to an adjacent position:", self.current);
        }

        loop {
            let mut positions = String::new();
            match input.read_line(&mut positions) {
                Err(error) => println!("ERROR: input error - {}", error),
                Ok(6) => { // letter + digit + letter + digit + \r + \n
                    let trimmed_positions = positions.trim_end();
                    let start = &trimmed_positions[..2];
                    let end = &trimmed_positions[2..];



                    match self.board.move_piece(self.current, start, end, flying) {
                        Err(MovingError::InvalidMoveFrom)    => println!("ERROR: Invalid first position - {}!", start),
                        Err(MovingError::InvalidMoveTo)      => println!("ERROR: Invalid second position - {}!", end),
                        Err(MovingError::MoveToSame)         => println!("ERROR: The two positions are identical!"),
                        Err(MovingError::MoveFromEmpty)      => println!("ERROR: The starting position {} doesn't have a piece to move!", start),
                        Err(MovingError::MoveFromWrongColor) => println!("ERROR: The starting position {} isn't occupied by you!", start),
                        Err(MovingError::MoveToOccupied)     => println!("ERROR: The target position {} is already occupied!", end),
                        Err(MovingError::NotAdjacent)        => println!("ERROR: Can't move from {} to {}!", start, end),
                        Ok(()) => {
                            match self.current {
                                Color::White => self.white.move_piece(start.to_string(), end.to_string()),
                                Color::Black => self.black.move_piece(start.to_string(), end.to_string())
                            }

                            return end.to_string();
                        }
                    }
                }
                Ok(_) => println!("ERROR: Invalid input - must be 4 symbols (e.g. a7a4)!"),
            }
        }
    }

    // Requests the coordinates of a single position, reads them from the standard input and calls Board.remove() 
    // with the color of the current player, the position and an appropriate boolean value determining whether pieces can be removed from mills or not.
    // Handles that function's errors and prints an approriate message if there is one.
    // If the removal is successful, calls the Pieces.remove() function of the other player.
    fn remove_piece<R: BufRead>(&mut self, mut input: R) -> String {
        println!("{} player, REMOVE opponent's piece:", self.current);

        loop {
            let mut position = String::new();
            match input.read_line(&mut position) {
                Err(error) => println!("ERROR: input error - {}", error),
                Ok(_) => {
                    // If all of the other player's pieces are in a mill, they can be removed without issue, so check_for_mills is false. 
                    // Otherwise, only pieces not in a mill can be removed, so check_for_mills is true.
                    let check_for_mills = match self.current {
                        Color::White => !self.black.positions.iter().all(|position| self.board.in_mill(Color::Black, position)),
                        Color::Black => !self.white.positions.iter().all(|position| self.board.in_mill(Color::White, position)),
                    };

                    let trimmed_position = position.trim_end();
                    match self.board.remove_piece(self.current.other(), trimmed_position, check_for_mills) {
                        Err(RemovingError::InvalidPosition)      => println!("ERROR: Invalid position - {}!", trimmed_position),
                        Err(RemovingError::RemoveFromEmpty)      => println!("ERROR: Cannot remove from empty position {}!", trimmed_position),
                        Err(RemovingError::RemoveFromWrongColor) => println!("ERROR: Cannot remove your own pieces (from position {})!", trimmed_position),
                        Err(RemovingError::RemoveFromMill)       => println!("ERROR: Cannot remove from opponent's mills (from position {})!", trimmed_position),
                        Ok(()) => {
                            match self.current {
                                Color::White => self.black.remove_piece(trimmed_position.to_string()),
                                Color::Black => self.white.remove_piece(trimmed_position.to_string())
                            }

                            return trimmed_position.to_string();
                        }
                    }
                }
            }
        }
    
    }
}

impl Game { // checking if the game can continue
    // checks if the player of the given color can move at least one of their pieces
    fn can_move(&self, color: Color) -> bool {
        let flying = match self.current {
            Color::White => self.white.placed <= 3,
            Color::Black => self.black.placed <= 3
        };
        if flying { // there are at most 18 pieces on a board of 24 positions, so there will always be an available position to fly to
            return true;
        }

        match color {
            Color::White => self.white.positions.iter().any(|position| self.board.can_move(Color::White, position)),
            Color::Black => self.black.positions.iter().any(|position| self.board.can_move(Color::Black, position))
        }
    }

    // checks if the player of the given color can continue playing the game, i.e. if they have enough pieces and can move at least one
    fn can_play(&self, color: Color) -> bool {
        match color {
            Color::White => self.white.placed >= 3 && self.can_move(Color::White),
            Color::Black => self.black.placed >= 3 && self.can_move(Color::Black)
        }
    }
}

impl Game { // game loop
    // Requests information about which player should move first.
    // After that, loops while the game can continue (both players have enough pieces and can move at least one of them), with each iteration
    // calling either place() or move() depending on the phase of the game. If a mill is formed, calls remove().
    // After the looping condition becomes false, prints an appropriate message for the end of the game, 
    // describing who won and by what cause in the case of victory, or that the outcome is a draw.
    pub fn game_loop(&mut self) {
        loop {
            println!("Who should move first? (white or black)?");

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Err(error) => println!("ERROR: input error - {}", error),
                Ok(_) => {
                    let trimmed = input.trim_end();

                    if trimmed == "white" {
                        self.current = Color::White;
                        break;
                    } else if trimmed == "black" {
                        self.current = Color::Black;
                        break;
                    } else {
                        println!("ERROR: Invalid input!");
                    }
                }
            }
        }    

        print!("\nINSTRUCTIONS:\n\
                At the start of every turn, the board is printed, with the occupied positions marked by ○ for white pieces and ● for black pieces.\n\
                
                Depending on the phase of the game, the players are asked to input coordinates of positions:\n\
                - during the placement phase, input the coordinates of a single position (e.g. 'a7') to place a piece there. The position should be unoccupied.\n\
                - during the movement phase, input the coordinates of two positions (e.g. 'a7a4') to move a piece from the first to the second. \
                The first position should have a piece of your color, and the second should be adjacent to it and unoccupied.\n\
                If you have only 3 pieces left however, you can move to non-adjacent positions.\n\
                \n\
                If a mill is formed at any point, a message will be printed and you will need to input the coordinates of a single position, \
                from which to remove a piece belonging to your opponent.\n\

                In the case of invalid input (positions that don't exist, placing on already occupied positions, moving your opponent's pieces, etc.) \
                an appropriate error message is printed and the turn is repeated (until correct input is given).
                \n"
        );

        while self.phase == Phase::Placing || (self.phase == Phase::Moving && self.can_play(Color::White) && self.can_play(Color::Black)) {
            print!("{}", self);

            let position = match self.phase {
                Phase::Placing => self.place_piece(io::stdin().lock()),
                Phase::Moving  => self.move_piece(io::stdin().lock())
            };

            if self.board.in_mill(self.current, &position) {
                print!("{}", self);
                println!("{} player FORMED A MILL!", self.current);

                self.remove_piece(io::stdin().lock());
            }

            self.current = self.current.other();

            if self.white.unplaced == 0 && self.black.unplaced == 0 {
                self.phase = Phase::Moving;
            }
        }

        print!("{}", self);
        if self.white.placed == 2 {
            println!("VICTORY for BLACK player - white player has less than 3 pieces!");
        } else if self.black.placed == 2 {
            println!("VICTORY for WHITE player - black player has less than 3 pieces!");
        } else if self.can_move(Color::White) == false && self.can_move(Color::Black) {
            println!("VICTORY for BLACK player - white player cannot move their pieces!")
        } else if self.can_move(Color::White) && self.can_move(Color::Black) == false {
            println!("VICTORY for WHITE player - black player cannot move their pieces!")
        } else if self.can_move(Color::White) == false && self.can_move(Color::Black) == false {
            println!("DRAW - neither player can move their pieces!")
        } else {
            println!("ERROR: invalid end state of game!")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::State;

    #[test]
    fn test_place() {
        let mut game = Game::new();

        assert_eq!(game.place_piece(&b"a7\r\n"[..]), "a7");
        game.current = game.current.other();
        assert_eq!(game.board.get_node_ref("a7").unwrap().state, State::Occupied(Color::White));
        
        assert_eq!(game.place_piece(&b"a1\r\n"[..]), "a1");
        game.current = game.current.other();
        assert_eq!(game.board.get_node_ref("a1").unwrap().state, State::Occupied(Color::Black));

        assert_eq!(game.place_piece(&b"d7\r\n"[..]), "d7");
        game.current = game.current.other();
        assert_eq!(game.board.get_node_ref("d7").unwrap().state, State::Occupied(Color::White));

        assert_eq!(game.place_piece(&b"d1\r\n"[..]), "d1");
        game.current = game.current.other();
        assert_eq!(game.board.get_node_ref("d1").unwrap().state, State::Occupied(Color::Black));

        assert_eq!(game.place_piece(&b"g7\r\n"[..]), "g7");
        game.current = game.current.other();
        assert_eq!(game.board.get_node_ref("g7").unwrap().state, State::Occupied(Color::White));
        assert!(game.board.in_mill(Color::White, "g7"));
    }
    #[test]
    fn test_move() {
        let mut game = Game::new();

        assert_eq!(game.place_piece(&b"a7\r\n"[..]), "a7");
        game.current = game.current.other();
        assert_eq!(game.place_piece(&b"a1\r\n"[..]), "a1");
        game.current = game.current.other();

        assert_eq!(game.move_piece(&b"a7a4\r\n"[..]), "a4");
        game.current = game.current.other();
        assert_eq!(game.board.get_node_ref("a7").unwrap().state, State::Empty);
        assert_eq!(game.board.get_node_ref("a4").unwrap().state, State::Occupied(Color::White));

        assert_eq!(game.move_piece(&b"a1d1\r\n"[..]), "d1");
        game.current = game.current.other();
        assert_eq!(game.board.get_node_ref("a1").unwrap().state, State::Empty);
        assert_eq!(game.board.get_node_ref("d1").unwrap().state, State::Occupied(Color::Black));
    }
    #[test]
    fn test_remove() {
        let mut game = Game::new();

        assert_eq!(game.place_piece(&b"a7\r\n"[..]), "a7");
        game.current = game.current.other();
        assert_eq!(game.place_piece(&b"a1\r\n"[..]), "a1");
        game.current = game.current.other();

        assert_eq!(game.place_piece(&b"d7\r\n"[..]), "d7");
        game.current = game.current.other();
        assert_eq!(game.place_piece(&b"d1\r\n"[..]), "d1");
        game.current = game.current.other();

        assert_eq!(game.place_piece(&b"g7\r\n"[..]), "g7");
        assert!(game.board.in_mill(Color::White, "g7"));
        assert_eq!(game.remove_piece(&b"d1\r\n"[..]), "d1");
        assert_eq!(game.board.get_node_ref("d1").unwrap().state, State::Empty);

        game.current = game.current.other();
    }

    #[test]
    fn test_can_move() {
        let mut game = Game::new();

        assert_eq!(game.place_piece(&b"a1\r\n"[..]), "a1");
        assert_eq!(game.place_piece(&b"a4\r\n"[..]), "a4");
        assert_eq!(game.place_piece(&b"a7\r\n"[..]), "a7");
        assert_eq!(game.place_piece(&b"b4\r\n"[..]), "b4");

        game.current = game.current.other();
        assert_eq!(game.place_piece(&b"d1\r\n"[..]), "d1");
        assert_eq!(game.place_piece(&b"d7\r\n"[..]), "d7");
        assert_eq!(game.place_piece(&b"b2\r\n"[..]), "b2");
        assert_eq!(game.place_piece(&b"b6\r\n"[..]), "b6");
        assert_eq!(game.place_piece(&b"c4\r\n"[..]), "c4");

        assert!(game.can_move(Color::White) == false);
        assert!(game.can_move(Color::Black));
    }
    #[test]
    fn test_can_move_flying() {
        let mut game = Game::new();

        assert_eq!(game.place_piece(&b"a1\r\n"[..]), "a1");
        assert_eq!(game.place_piece(&b"a4\r\n"[..]), "a4");
        assert_eq!(game.place_piece(&b"a7\r\n"[..]), "a7");

        game.current = game.current.other();
        assert_eq!(game.place_piece(&b"d1\r\n"[..]), "d1");
        assert_eq!(game.place_piece(&b"b4\r\n"[..]), "b4");
        assert_eq!(game.place_piece(&b"d7\r\n"[..]), "d7");

        assert!(game.can_move(Color::White));
        assert!(game.can_move(Color::Black));
    }
    #[test]
    fn test_can_play() {
        let mut game = Game::new();

        assert_eq!(game.place_piece(&b"a1\r\n"[..]), "a1");
        assert_eq!(game.place_piece(&b"a4\r\n"[..]), "a4");
        assert_eq!(game.place_piece(&b"a7\r\n"[..]), "a7");

        game.current = game.current.other();
        assert_eq!(game.place_piece(&b"d1\r\n"[..]), "d1");
        assert_eq!(game.place_piece(&b"b4\r\n"[..]), "b4");
        assert_eq!(game.place_piece(&b"d7\r\n"[..]), "d7");

        assert!(game.can_play(Color::White));
        assert!(game.can_play(Color::Black));

        game.current = game.current.other();
        assert_eq!(game.remove_piece(&b"b4\r\n"[..]), "b4");

        assert!(game.can_play(Color::Black) == false);
    }
}