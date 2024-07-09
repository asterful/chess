mod piece;
mod board;
mod position;
mod engine;
mod game;
mod frontend;

fn main() {
    let mut cli_client = frontend::cli::Client::new();
    cli_client.start();
}
