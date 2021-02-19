// errors that can occur when placing a piece
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlacingError {
    InvalidPosition,
    PlaceAtOccupied
}

// errors that can occur when moving a piece between positions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MovingError {
    InvalidMoveFrom,
    InvalidMoveTo,

    MoveToSame,
    MoveFromEmpty,
    MoveFromWrongColor,
    MoveToOccupied,

    NotAdjacent
}

// errors that can occur when removing a piece
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RemovingError {
    InvalidPosition,
    RemoveFromEmpty,
    RemoveFromWrongColor,
    RemoveFromMill
}
