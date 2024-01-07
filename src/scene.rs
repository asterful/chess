use ggez::{Context, GameError};

pub enum SceneResult {
    None,
    SwitchScene(Box<dyn Scene>), 
}

pub trait Scene {
    fn update(&mut self, context: &mut Context) -> SceneResult;
    fn draw(&mut self, context: &mut Context) -> Result<(), GameError>;
}
