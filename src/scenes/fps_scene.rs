use ggez::{graphics::{Text, Canvas, DrawParam, Color}, GameError};
use ggez::input::keyboard::KeyCode;

use crate::scene::{Scene, SceneResult};

use super::main_menu::MainMenuScene;

pub struct FpsScene {

}

impl Scene for FpsScene {
    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), GameError> {
        let fps = context.time.fps();
        let fps_display = Text::new(format!("FPS: {fps}"));

        let mut canvas = Canvas::from_frame(context, Color::BLACK);
        canvas.draw(&fps_display, DrawParam::from([0.0, 0.0]).color(Color::WHITE));
        canvas.finish(context)?;
        Ok(())
    }

    fn update(&mut self, context: &mut ggez::Context) -> SceneResult {
        let k_ctx = &context.keyboard;
        if k_ctx.is_key_pressed(KeyCode::Left) {
            let main_menu = MainMenuScene {};
            return SceneResult::SwitchScene(Box::from(main_menu));
        }
        SceneResult::None
    }
}
