//! I'm thinking in here will be the logic for determining the board, what pieces are on the board but maybe 
//! not the move_set for any piece. I think that logic can be seperated out into a different library.

#[derive(Debug)]
pub struct Piece { //The One Piece, the One Piece is real!!!!!
    pub x: u8,
    pub y: u8,
    pub colour_bool: bool,
    pub piece_type: String,
}

// Yeah I have no idea how we want to set up the board from this point.
