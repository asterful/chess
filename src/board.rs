use crate::{piece::{Color, Piece}, position::Position};

// Castling rights
pub struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool
}

impl CastlingRights {
    pub fn new(white_kingside: bool, white_queenside: bool, black_kingside: bool, black_queenside: bool) -> Self {
        Self { white_kingside, white_queenside, black_kingside, black_queenside }
    }

    pub fn can_castle_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_kingside,
            Color::Black => self.black_kingside,
        }
    }

    pub fn can_castle_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_queenside,
            Color::Black => self.white_queenside,
        }
    }

    pub fn disable_kingside(&mut self, color: Color) {
        match color {
            Color::White => self.white_kingside = false,
            Color::Black => self.black_kingside = false,
        }
    }

    pub fn disable_queenside(&mut self, color: Color) {
        match color {
            Color::White => self.white_queenside = false,
            Color::Black => self.black_queenside = false,
        }
    }
    
    pub fn disable_all(&mut self, color: Color) {
        match color {
            Color::White => {
                self.white_kingside = false;
                self.white_queenside = false;
            },
            Color::Black => {
                self.black_kingside = false;
                self.black_queenside = false;
            }
        }
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self { white_kingside: true, white_queenside: true, black_kingside: true, black_queenside: true }
    }
}

// Board
pub struct Board {
    squares: [Option<Piece>; 64],
    en_passant: Option<Position>,
    castling_rights: CastlingRights,
    active_color: Color,
    halfmove: u16,
    fullmove: u16
}

impl Board {
    pub fn empty() -> Self {
        const EMPTY: Option<Piece> = None;
        Self {
            squares: [EMPTY; 64],
            en_passant: None,
            castling_rights: CastlingRights::new(false, false, false, false),
            active_color: Color::White,
            halfmove: 0,
            fullmove: 1
        }
    }

    pub fn default() -> Self {
        const EMPTY: Option<Piece> = None;
        let mut _squares = [EMPTY; 64];
        for i in 8..16 {
            _squares[i] = Some(Piece::Pawn(Color::White));
        };
        for i in 48..56 {
            _squares[i] = Some(Piece::Pawn(Color::Black));
        };
        Self {
            squares: _squares,
            en_passant: None,
            castling_rights: CastlingRights::default(),
            active_color: Color::White,
            halfmove: 0,
            fullmove: 1
        }
    }

    pub fn set_piece(&mut self, pos: Position, piece: Piece) {
        let index = pos.index() as usize;
        self.squares[index] = Some(piece);
    }

    pub fn remove_piece(&mut self, pos: Position) {
        let index = pos.index() as usize;
        self.squares[index] = None;
    }
}

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = String::from("");
        let vertical = "│";
        let top_row = "  ╭───┬───┬───┬───┬───┬───┬───┬───╮";
        let mid_row = "  ├───┼───┼───┼───┼───┼───┼───┼───┤";
        let bot_row = "  ╰───┴───┴───┴───┴───┴───┴───┴───╯";
        table += top_row;
        table += "\n";
        for row in (0..8).rev() {
            table += format!("{} ", row + 1).as_str();
            for col in 0..8 {
                table += vertical;
                match &self.squares[row*8 + col] {
                    Some(x) => table += format!(" {} ", x).as_str(),
                    None => table += "   ",
                }
            }
            table += vertical;
            table += "\n";
            if row == 0 { table += bot_row }
            else { table += mid_row }
            table += "\n";
        }
        table += "    A   B   C   D   E   F   G   H  \n";
        write!(f, "{}", table)
    }
}