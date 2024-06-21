use crate::{piece::{Color, Piece}, position::Position};

pub struct Board {
    squares: [Option<Piece>; 64]
}

impl Board {
    pub fn empty() -> Self {
        const EMPTY: Option<Piece> = None;
        Self {
            squares: [EMPTY; 64]
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
            squares: _squares
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
        let top_row = "╭───┬───┬───┬───┬───┬───┬───┬───╮";
        let mid_row = "├───┼───┼───┼───┼───┼───┼───┼───┤";
        let bot_row = "╰───┴───┴───┴───┴───┴───┴───┴───╯";
        table += top_row;
        table += "\n";
        for row in (0..8).rev() {
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
        write!(f, "{}", table)
    }
}