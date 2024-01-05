use ggez::event;
use ggez::GameResult;
use ggez::conf::{WindowMode, WindowSetup};

pub mod application;

fn main() -> GameResult {
    let window_setup = WindowSetup::default().title("Aster Chess").vsync(false);
    let window_mode = WindowMode::default().borderless(true);

    let (ctx, event_loop) = ggez::ContextBuilder::new("aster_chess", "asterful")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    let state = application::GlobalState::new()?;
    event::run(ctx, event_loop, state)
}
