use ggez::event;
use ggez::GameResult;
use ggez::conf::{WindowMode, WindowSetup};

pub mod application;
pub mod scene;
pub mod scenes;

use scenes::main_menu::MainMenuScene;

fn main() -> GameResult {
    let window_setup = WindowSetup::default().title("Aster Chess").vsync(false);
    let window_mode = WindowMode::default()
        .resizable(true)
        .min_dimensions(640.0, 480.0);

    let (ctx, event_loop) = ggez::ContextBuilder::new("aster_chess", "asterful")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    let start_scene = MainMenuScene {};
    let state = application::GlobalState::new(Box::from(start_scene))?;
    event::run(ctx, event_loop, state)
}
