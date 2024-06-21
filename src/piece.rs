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

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Piece::Pawn(Color::Black) =>   "♙",
            Piece::Bishop(Color::Black) => "♗",
            Piece::Knight(Color::Black) => "♘",
            Piece::Rook(Color::Black) =>   "♖",
            Piece::Queen(Color::Black) =>  "♕",
            Piece::King(Color::Black) =>   "♔",
            Piece::Pawn(Color::White) =>   "♟",
            Piece::Bishop(Color::White) => "♝",
            Piece::Knight(Color::White) => "♞",
            Piece::Rook(Color::White) =>   "♜",
            Piece::Queen(Color::White) =>  "♛",
            Piece::King(Color::White) =>   "♚",
        };
        write!(f, "{}", symbol)
    }
}