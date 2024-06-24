use crate::{piece::{Color, Piece}, position::Position};


/* -------------------------------------------------------------------------- */
/*                                 Definition                                 */
/* -------------------------------------------------------------------------- */
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

// Contains only the information necessary to represent the state of a chess game
// Has the same fields as the FEN notation
pub struct Board {
    squares: [Option<Piece>; 64],
    active_color: Color,
    halfmove: u16,
    fullmove: u16,
    castling_rights: CastlingRights,
    en_passant: Option<Position>,
}


/* -------------------------------------------------------------------------- */
/*                               Implementations                              */
/* -------------------------------------------------------------------------- */
impl Board {
    pub fn set_piece(&mut self, pos: Position, piece: Piece) {
        let index = pos.index() as usize;
        self.squares[index] = Some(piece);
    }

    pub fn remove_piece(&mut self, pos: Position) {
        let index = pos.index() as usize;
        self.squares[index] = None;
    }

    pub fn disable_castling_kingside(&mut self, color: Color) {
        match color {
            Color::White => self.castling_rights.white_kingside = false,
            Color::Black => self.castling_rights.black_kingside = false,
        }
    }

    pub fn disable_castling_queenside(&mut self, color: Color) {
        match color {
            Color::White => self.castling_rights.white_queenside = false,
            Color::Black => self.castling_rights.black_queenside = false,
        }
    }

    pub fn disable_all_castling(&mut self, color: Color) {
        match color {
            Color::White => {
                self.castling_rights.white_kingside = false;
                self.castling_rights.white_queenside = false;
            },
            Color::Black => {
                self.castling_rights.black_kingside = false;
                self.castling_rights.black_queenside = false;
            },
        }
    }

    pub fn queenside_castling_right(&self, color: Color) -> bool {
        match color {
            Color::White => self.castling_rights.white_queenside,
            Color::Black => self.castling_rights.black_queenside,
        }
    }

    pub fn kingside_castling_right(&self, color: Color) -> bool {
        match color {
            Color::White => self.castling_rights.white_kingside,
            Color::Black => self.castling_rights.black_kingside,
        }
    }

    pub fn increment_halfmove(&mut self) {
        self.halfmove += 1;
    }

    pub fn reset_halfmove(&mut self) {
        self.halfmove = 0;
    }

    pub fn get_halfmove(&self) -> u16 {
        self.halfmove
    }

    pub fn increment_fullmove(&mut self) {
        self.fullmove += 1;
    }

    pub fn get_fullmove(&self) -> u16 {
        self.fullmove
    }

    pub fn switch_active_color(&mut self) {
        match self.active_color {
            Color::White => self.active_color = Color::Black,
            Color::Black => self.active_color = Color::White,
        }
    }

    pub fn get_active_color(&self) -> Color {
        self.active_color
    }

    pub fn set_en_passant(&mut self, pos: Position) {
        self.en_passant = Some(pos)
    }

    pub fn clear_en_passant(&mut self) {
        self.en_passant = None
    }

    pub fn get_en_passant(&self) -> Option<Position> {
        self.en_passant
    }
}


/* -------------------------------------------------------------------------- */
/*                           Default Implementations                          */
/* -------------------------------------------------------------------------- */
impl Default for CastlingRights {
    fn default() -> Self {
        Self { white_kingside: true, white_queenside: true, black_kingside: true, black_queenside: true }
    }
}

impl Default for Board {
    fn default() -> Self {
        const EMPTY: Option<Piece> = None;
        let mut _squares = [EMPTY; 64];
        for i in 8..16 {
            _squares[i] = Some(Piece::Pawn(Color::White));
        };
        for i in 48..56 {
            _squares[i] = Some(Piece::Pawn(Color::Black));
        };
        _squares[0] = Some(Piece::Rook(Color::White));
        _squares[1] = Some(Piece::Knight(Color::White));
        _squares[2] = Some(Piece::Bishop(Color::White));
        _squares[3] = Some(Piece::Queen(Color::White));
        _squares[4] = Some(Piece::King(Color::White));
        _squares[5] = Some(Piece::Bishop(Color::White));
        _squares[6] = Some(Piece::Knight(Color::White));
        _squares[7] = Some(Piece::Rook(Color::White));
        _squares[56] = Some(Piece::Rook(Color::Black));
        _squares[57] = Some(Piece::Knight(Color::Black));
        _squares[58] = Some(Piece::Bishop(Color::Black));
        _squares[59] = Some(Piece::Queen(Color::Black));
        _squares[60] = Some(Piece::King(Color::Black));
        _squares[61] = Some(Piece::Bishop(Color::Black));
        _squares[62] = Some(Piece::Knight(Color::Black));
        _squares[63] = Some(Piece::Rook(Color::Black));
        Self {
            squares: _squares,
            en_passant: None,
            castling_rights: CastlingRights::default(),
            active_color: Color::White,
            halfmove: 0,
            fullmove: 1
        }
    }
}


/* -------------------------------------------------------------------------- */
/*                           Display Implementations                          */
/* -------------------------------------------------------------------------- */
// FEN notation for Castling Rights
impl std::fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("");
        if self.white_kingside || self.white_queenside || self.black_kingside || self.white_queenside {
            if self.white_kingside { result += "K" };
            if self.white_queenside { result += "Q" };
            if self.black_kingside { result += "k" };
            if self.black_queenside { result += "q" };
        } else {
            result += "-";
        }
        write!(f, "{}", result)
    }
}

// Represent board as a table
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
