use crate::{board::{self, Board}, position::Position};

// What is the actual state of the game
pub enum GameState {
    ShowingPieceLegalMoves,
    WaitingForTurn,
    PlayerTurn,
    GameOver,
}

// Game Messages
pub enum GameMessage {
    SquareSelected(Position),
    SquareDeselected(Position),
}

// Has all the state necesary to run an interactive chess game
pub struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    pub fn new(board: Board) -> Self {
        Self {
            board: board,
            state: GameState::PlayerTurn
        }
    }

    pub fn update(&mut self, message: GameMessage) {
        match self.state {
            GameState::ShowingPieceLegalMoves => todo!(),
            GameState::WaitingForTurn => todo!(),
            GameState::PlayerTurn => todo!(),
            GameState::GameOver => todo!(),
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
}