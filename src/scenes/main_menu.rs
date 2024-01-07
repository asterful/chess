use ggez::{graphics::{Text, Canvas, DrawParam, Color}, GameError};
use ggez::input::keyboard::KeyCode;

use crate::scene::{Scene, SceneResult};

use super::fps_scene::FpsScene;

pub struct MainMenuScene {

}

impl Scene for MainMenuScene {
    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), GameError> {
        let fps_display = Text::new(format!("Hello world!"));

        let mut canvas = Canvas::from_frame(context, Color::BLACK);
        canvas.draw(&fps_display, DrawParam::from([0.0, 0.0]).color(Color::WHITE));
        canvas.finish(context)?;
        Ok(())
    }

    fn update(&mut self, context: &mut ggez::Context) -> SceneResult {
        let k_ctx = &context.keyboard;
        if k_ctx.is_key_pressed(KeyCode::Right) {
            let fps = FpsScene {};
            return SceneResult::SwitchScene(Box::from(fps));
        }
        SceneResult::None
    }
}