use std::time::Instant;

use ggez::{GameResult, GameError};
use ggez::event::EventHandler;
use ggez::Context;
use ggez::graphics::{Text, DrawParam, Canvas, Color};

pub struct GlobalState {
    launch_time: Instant,
}

impl GlobalState {
    pub fn new() -> GameResult<GlobalState> {
        let state = GlobalState { launch_time: Instant::now() };
        Ok(state)
    }   
}

impl EventHandler<GameError> for GlobalState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let time = self.launch_time.elapsed().as_secs();
        let fps = ctx.time.fps();
        let time_elapsed = Text::new(format!("Time Elapsed: {time}s"));
        let fps_display = Text::new(format!("FPS: {fps}"));

        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&fps_display, DrawParam::from([0.0, 0.0]).color(Color::WHITE));
        canvas.draw(&time_elapsed, DrawParam::from([0.0, 15.0]).color(Color::WHITE));
        canvas.finish(ctx)?;
        Ok(())
    }
}