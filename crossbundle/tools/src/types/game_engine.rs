#[derive(Clone, Copy, Debug)]
pub enum GameEngine {
    Bevy,
    Macroquad,
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::Macroquad
    }
}
