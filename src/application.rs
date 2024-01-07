use ggez::{GameResult, GameError};
use ggez::event::EventHandler;
use ggez::Context;

use crate::scene::{Scene, SceneResult};

pub struct GlobalState {
    active_scene: Box<dyn Scene>,
}

impl GlobalState {
    pub fn new(scene: Box<dyn Scene>) -> GameResult<GlobalState> {
        let state = GlobalState {
            active_scene: scene,
        };
        Ok(state)
    }   
}

impl EventHandler<GameError> for GlobalState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if let SceneResult::SwitchScene(scene) = self.active_scene.update(ctx) {
            self.active_scene = scene;
        };
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.active_scene.draw(ctx)?;
        Ok(())
    }
}
