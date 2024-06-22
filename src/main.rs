use board::Board;

mod piece;
mod board;
mod position;
mod engine;

fn main() {
    println!("Rusty world");
    let board: Board = Board::default();
    println!("{board}");
}
