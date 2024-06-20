pub enum Color {
    White,
    Black,
}

pub enum Piece {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}