use std::io::{self, Write};

use crate::{board::Board, game::Game};
use crate::game::{GameMessage, GameState};
use crate::position::Position;

pub struct Client {
    game: Game,
}

impl Client {
    pub fn new() -> Self {
        let board = Board::default();
        Self {
            game: Game::new(board),
        }
    }

    pub fn start(&mut self) {
        println!("\nWelcome to chess!");
        let lines = io::stdin().lines();
        
        println!("{}", self.game.get_board());
        print!("Select a piece: ");
        io::stdout().flush().unwrap();
        
        for line in lines {
            match line {
                Ok(input) => {},
                Err(_) => break,
            }
        }
    }

    fn update(&mut self, input: &str) {
        let pos = Position::from_str(input);
        match self.game.get_state() {
            GameState::ShowingPieceLegalMoves => {},
            GameState::WaitingForTurn => {},
            GameState::PlayerTurn => {
                match pos {
                    Some(_pos) => {
                        self.game.update(GameMessage::SquareSelected(_pos))
                    },
                    None => {},
                }
            },
            GameState::GameOver => {},
        }
    }
}