use board::Board;

mod piece;
mod board;
mod position;
mod engine;

fn main() {
    println!("Hello, world!");
    // Start game
    let board: Board = Board::default();
    println!("{board}");

    // Get legal moves

    // Execute move
}
