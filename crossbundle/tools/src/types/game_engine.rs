#[derive(Clone, Copy)]
pub enum GameEngine {
    Bevy,
    Macroquad,
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::Macroquad
    }
}