use crate::piece::Piece;
use std::option::Option;

pub struct Board {
    squares: [Option<Piece>; 64]
}