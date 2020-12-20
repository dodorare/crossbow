#[derive(Debug, Clone)]
pub enum Target {
    Bin(String),
    Example(String),
    Lib,
}
