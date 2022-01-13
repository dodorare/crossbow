#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEngine {
    Bevy,
    Macroquad,
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::Macroquad
    }
}
